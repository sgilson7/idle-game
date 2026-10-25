//! The door into the core (R4). The shim holds one `Game` and forwards each
//! call; every call takes the page's clock in milliseconds and returns the
//! whole screen as JSON. Settings, screens and flags come through one function,
//! `act`, with a dotted name. (After Peter's Mandible, and Level Twenty's
//! `Game` held in the module so the state is not copied on every tick.)

use serde::Serialize;

use crate::content as c;
use crate::game::{Flag, Report, State, Stop, SAVE_VERSION};
use crate::glossary;
use crate::text::{num, Text};
use crate::view::{button, Block, Device, Sheet, Tab, Top, View, TABS};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Hint {
    pub term: String,
    pub definition: String,
}

/// Everything the page draws.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Ui {
    pub title: String,
    pub top: Top,
    pub tabs: Vec<Tab>,
    pub screen: String,
    pub blocks: Vec<Block>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet: Option<Sheet>,
    pub toasts: Vec<String>,
    /// Milliseconds until the running action completes; 0 when nothing runs.
    pub next_ms: u64,
    pub autosave_ms: u64,
    /// The saved text could not be read: the page keeps a copy of it.
    pub stash: bool,
    pub hints: Vec<Hint>,
}

pub struct Game {
    text: Text,
    hints: Vec<Hint>,
    s: State,
    device: Device,
    sheet: Option<Sheet>,
    toasts: Vec<String>,
    draft: Flag,
    stash: bool,
}

/// A save as it is written to a file: a little about where it came from, then
/// the state.
#[derive(Serialize, serde::Deserialize)]
struct SaveFile {
    game: String,
    v: u32,
    state: State,
}

const GAME_ID: &str = "idle-game";

fn read_save(text: &str) -> Result<State, String> {
    let file: SaveFile = serde_json::from_str(text).map_err(|e| e.to_string())?;
    if file.game != GAME_ID {
        return Err("not a save from this game".into());
    }
    if file.v > SAVE_VERSION || file.state.v > SAVE_VERSION {
        return Err("a save from a newer version".into());
    }
    if file.state.xp.len() != c::SKILLS.len() || file.state.tools.len() != c::TOOL_SKILLS.len() || file.state.style >= c::STYLES.len() {
        return Err("a save with the wrong shape".into());
    }
    Ok(file.state)
}

impl Game {
    /// Starts from the saved text, or a new game when there is none. A save
    /// that cannot be read starts a new game and asks the page to keep the
    /// old text (IG-5).
    pub fn new(strings_json: &str, glossary_md: &str, saved: &str, device_json: &str, now: u64) -> Result<Game, String> {
        let text = Text::parse(strings_json)?;
        let hints = glossary::parse(glossary_md)
            .map_err(|e| e.message())?
            .into_iter()
            .map(|e| Hint { term: e.term, definition: e.definition })
            .collect();
        let device: Device = serde_json::from_str(device_json).unwrap_or_default();
        let mut g = Game { text, hints, s: State::new(now), device, sheet: None, toasts: Vec::new(), draft: Flag::default(), stash: false };
        if saved.trim().is_empty() {
            g.sheet = Some(g.welcome());
            return Ok(g);
        }
        match read_save(saved) {
            Ok(s) => {
                g.s = s;
                g.s.note(now, "load", "", 0);
            }
            Err(why) => {
                g.stash = true;
                g.s.note(now, "bad_save", &why, 0);
                g.sheet = Some(g.notice("bad_save_title", g.text.s("bad_save_text")));
            }
        }
        Ok(g)
    }

    // ---------- the calls the shim forwards ----------

    /// Credits the time since the page last asked, then draws.
    pub fn tick(&mut self, now: u64) -> String {
        self.credit(now);
        self.ui_json()
    }

    pub fn act(&mut self, name: &str, value: &str, now: u64) -> String {
        self.credit(now);
        self.s.count(&format!("act:{name}"), 1);
        self.apply(name, value, now);
        self.ui_json()
    }

    pub fn save(&self) -> String {
        serde_json::to_string(&SaveFile { game: GAME_ID.into(), v: SAVE_VERSION, state: self.s.clone() }).unwrap_or_default()
    }

    /// The save as a file to keep (long-term save).
    pub fn export(&mut self, now: u64) -> String {
        self.s.exported_wall = now;
        self.s.note(now, "export", "", 0);
        serde_json::to_string_pretty(&SaveFile { game: GAME_ID.into(), v: SAVE_VERSION, state: self.s.clone() }).unwrap_or_default()
    }

    pub fn import(&mut self, text: &str, now: u64) -> String {
        match read_save(text) {
            Ok(s) => {
                self.s = s;
                self.s.last_wall = now;
                self.s.note(now, "import", "", 0);
                self.sheet = Some(self.notice("import_ok_title", self.text.s("import_ok_text")));
            }
            Err(why) => {
                self.sheet = Some(self.notice("import_bad_title", self.text.t("import_bad_text", &[("why", why)])));
            }
        }
        self.ui_json()
    }

    /// The playtest file (P12): the save, the log, the flagged moments, and a
    /// summary of pace worked out here so the reader does not have to.
    pub fn playtest(&mut self, now: u64) -> String {
        self.credit(now);
        self.s.note(now, "playtest", "", self.s.flags.len() as u64);
        let skills: serde_json::Map<String, serde_json::Value> = c::SKILLS
            .iter()
            .enumerate()
            .map(|(i, sk)| {
                let ms = self.s.counts.get(&format!("ms:{}", sk.id)).copied().unwrap_or(0);
                let times: Vec<serde_json::Value> = self
                    .s
                    .log
                    .iter()
                    .filter(|e| e.k == "level" && e.a == sk.id)
                    .map(|e| serde_json::json!({ "level": e.n, "game_ms": e.s, "wall": e.w }))
                    .collect();
                (
                    sk.id.to_string(),
                    serde_json::json!({
                        "level": self.s.level(i),
                        "xp": self.s.xp[i],
                        "ms_spent": ms,
                        "xp_per_hour": if ms > 0 { self.s.xp[i] * 3_600_000 / ms } else { 0 },
                        "level_ups": times,
                    }),
                )
            })
            .collect();
        let aways: Vec<&crate::game::Event> = self.s.log.iter().filter(|e| e.k == "away").collect();
        let summary = serde_json::json!({
            "total_level": self.s.total_level(),
            "prestige": self.s.prestige,
            "gold": self.s.gold,
            "game_ms": self.s.sim_ms,
            "real_ms_since_first_play": now.saturating_sub(self.s.first_wall),
            "returns": aways.len(),
            "away_ms": aways.iter().map(|e| e.n).sum::<u64>(),
            "flags": self.s.flags.len(),
            "kills": self.s.kills,
            "skills": skills,
        });
        let doc = serde_json::json!({
            "kind": "idle-game-playtest",
            "save_version": SAVE_VERSION,
            "made_wall": now,
            "first_wall": self.s.first_wall,
            "device": self.device,
            "summary": summary,
            "flags": self.s.flags,
            "counts": self.s.counts,
            "events": self.s.log,
            "tuning": tuning(),
            "save": SaveFile { game: GAME_ID.into(), v: SAVE_VERSION, state: self.s.clone() },
        });
        serde_json::to_string_pretty(&doc).unwrap_or_default()
    }

    // ---------- time ----------

    fn credit(&mut self, now: u64) {
        let elapsed = now.saturating_sub(self.s.last_wall);
        self.s.last_wall = self.s.last_wall.max(now);
        if elapsed == 0 {
            return;
        }
        let credited = elapsed.min(c::AWAY_CAP_MS);
        let mut r = Report::begin(&self.s, elapsed, elapsed - credited);
        self.s.advance(credited, &mut r);
        let away = elapsed >= c::RETURN_AFTER_MS;
        self.log_report(&r, now, away);
        if away {
            self.s.count("away_ms", elapsed);
            if r.actions > 0 || r.stop.is_some() {
                let sheet = self.view().return_sheet(&r);
                self.sheet = Some(sheet);
            }
        } else {
            self.s.count("seen_ms", elapsed);
            self.toasts = self.toasts_for(&r);
        }
    }

    fn log_report(&mut self, r: &Report, now: u64, away: bool) {
        for (i, sk) in c::SKILLS.iter().enumerate() {
            for l in (r.lv0[i] + 1)..=self.s.level(i) {
                self.s.note(now, "level", sk.id, l as u64);
            }
        }
        for (id, cp) in &r.checkpoints {
            self.s.note(now, "mastery", id, *cp as u64);
        }
        for (item, n) in &r.rare {
            self.s.note(now, "rare", item, *n);
        }
        if let Some(stop) = &r.stop {
            let what = match stop {
                Stop::Food => "food".to_string(),
                Stop::Missing(i) => i.clone(),
            };
            self.s.note(now, "out", &what, 0);
        }
        for st in &r.steps {
            self.s.note(now, "plan_step", st, 0);
        }
        if away {
            self.s.note(now, "away", self.s.active.clone().unwrap_or_default().as_str(), r.elapsed);
        }
    }

    fn toasts_for(&self, r: &Report) -> Vec<String> {
        let t = &self.text;
        let mut out = Vec::new();
        for (i, sk) in c::SKILLS.iter().enumerate() {
            let l = self.s.level(i);
            if l > r.lv0[i] {
                out.push(t.t("toast_level", &[("skill", t.skill(sk.id).name.clone()), ("n", l.to_string())]));
            }
        }
        for (id, cp) in &r.checkpoints {
            out.push(t.t("toast_mastery", &[("what", t.action(id)), ("n", cp.to_string())]));
        }
        for item in r.rare.keys() {
            out.push(t.t("toast_rare", &[("item", t.item(item))]));
        }
        if !r.steps.is_empty() && self.s.active.is_some() {
            out.push(t.s("toast_plan"));
        }
        if let Some(stop) = &r.stop {
            out.push(match stop {
                Stop::Food => t.s("toast_food"),
                Stop::Missing(i) => t.t("toast_out", &[("item", t.item(i))]),
            });
        }
        out
    }

    // ---------- actions ----------

    fn apply(&mut self, name: &str, value: &str, now: u64) {
        match name {
            "tab" if TABS.contains(&value) => {
                self.s.screen = value.into();
                self.sheet = None;
                self.s.count(&format!("tab:{value}"), 1);
            }
            "do" => {
                self.sheet = None;
                if self.s.active.as_deref() == Some(value) {
                    self.s.stop();
                    self.s.note(now, "stop", value, 0);
                } else if self.s.unlocked(value) && self.s.missing(value).is_none() {
                    self.s.start(value, now);
                    self.s.screen = match c::action_index(value) {
                        Some(a) => c::SKILLS[c::ACTIONS[a].skill].id.into(),
                        None => "combat".into(),
                    };
                }
            }
            "style" => {
                if let Ok(k) = value.parse::<usize>() {
                    if k < c::STYLES.len() {
                        self.s.style = k;
                        if let Some(id) = self.s.active.clone() {
                            if self.s.missing(&id).is_some() {
                                self.s.stop();
                            }
                        }
                    }
                }
            }
            "plan.add" => {
                if self.s.plan_add(value) {
                    self.toasts = vec![self.text.t("toast_planned", &[("n", self.s.plan.len().to_string())])];
                }
            }
            "plan.count" => {
                if let Some((k, n)) = value.split_once(':') {
                    if let (Ok(k), Ok(n)) = (k.parse::<usize>(), n.parse::<u64>()) {
                        if c::PLAN_COUNTS.contains(&n) {
                            self.s.plan_count(k, n);
                        }
                    }
                }
            }
            "plan.remove" => {
                if let Ok(k) = value.parse::<usize>() {
                    self.s.plan_remove(k);
                }
            }
            "plan.start" => {
                self.s.plan_start();
                self.s.note(now, "plan", &self.s.plan.iter().map(|p| p.id.clone()).collect::<Vec<_>>().join(","), 0);
            }
            "plan.clear" => self.s.plan.clear(),
            "sell" => {
                let got = self.s.sell(value);
                if got > 0 {
                    self.s.note(now, "sell", value, got);
                    self.toasts = vec![self.text.t("toast_sold", &[("item", self.text.item(value)), ("n", num(got))])];
                }
            }
            "buy" => {
                if let Ok(line) = value.parse::<usize>() {
                    if self.s.buy(line) {
                        self.s.note(now, "buy", c::SKILLS[c::TOOL_SKILLS[line]].id, self.s.tools[line] as u64);
                    }
                }
            }
            "prestige.ask" if self.s.can_prestige() => self.sheet = Some(self.prestige_sheet()),
            "prestige.do" if self.s.can_prestige() => {
                self.s.do_prestige(now);
                self.sheet = None;
                self.toasts = vec![self.text.t("toast_prestige", &[("n", self.s.prestige.to_string())])];
            }
            "sheet.close" => self.sheet = None,
            "help.home" => self.sheet = Some(self.home_help()),
            "reset.ask" => {
                self.sheet = Some(Sheet {
                    kind: "notice".into(),
                    title: self.text.s("reset_title"),
                    lines: vec![self.text.s("reset_text")],
                    buttons: vec![button(self.text.s("cancel"), "sheet.close", ""), button(self.text.s("reset"), "reset.do", "").tone("stop")],
                    ..Sheet::default()
                })
            }
            "reset.do" => {
                let keep = (self.s.log.clone(), self.s.flags.clone(), self.s.counts.clone(), self.s.first_wall);
                self.s = State::new(now);
                (self.s.log, self.s.flags, self.s.counts, self.s.first_wall) = keep;
                self.s.note(now, "reset", "", 0);
                self.sheet = None;
            }
            "flag.open" => {
                self.draft = Flag::default();
                self.sheet = Some(self.flag_sheet());
            }
            "flag.fun" => {
                self.draft.fun = value.parse().unwrap_or(0).min(4);
                self.sheet = Some(self.flag_sheet());
            }
            "flag.pace" => {
                self.draft.pace = value.parse().unwrap_or(0).min(3);
                self.sheet = Some(self.flag_sheet());
            }
            "flag.note" => {
                self.draft.note = value.chars().take(c::NOTE_MAX_CHARS).collect();
            }
            "flag.save" => {
                let mut f = self.draft.clone();
                f.w = now;
                f.s = self.s.sim_ms;
                f.screen = self.s.screen.clone();
                f.doing = self.s.active.clone().unwrap_or_default();
                f.total = self.s.total_level();
                self.s.flags.push(f);
                self.s.note(now, "flag", "", self.s.flags.len() as u64);
                self.draft = Flag::default();
                self.sheet = None;
                self.toasts = vec![self.text.t("toast_flagged", &[("n", self.s.flags.len().to_string())])];
            }
            "seen.hide" => {
                self.s.note(now, "hide", "", 0);
            }
            "seen.show" => {
                self.s.note(now, "show", "", 0);
            }
            "playtest.sent" => {
                self.toasts = vec![self.text.s("toast_playtest")];
            }
            _ => {}
        }
    }

    // ---------- sheets ----------

    fn view(&self) -> View<'_> {
        View { t: &self.text, s: &self.s, device: &self.device }
    }

    fn welcome(&self) -> Sheet {
        let t = &self.text;
        Sheet {
            kind: "notice".into(),
            title: t.s("welcome_title"),
            lines: vec![t.s("welcome_1"), t.t("welcome_2", &[("cap", crate::text::dur(c::AWAY_CAP_MS))]), t.s("welcome_3")],
            buttons: vec![button(t.s("welcome_go"), "sheet.close", "").tone("primary")],
            ..Sheet::default()
        }
    }

    fn notice(&self, title_key: &str, line: String) -> Sheet {
        Sheet {
            kind: "notice".into(),
            title: self.text.s(title_key),
            lines: vec![line],
            buttons: vec![button(self.text.s("ok"), "sheet.close", "").tone("primary")],
            ..Sheet::default()
        }
    }

    fn home_help(&self) -> Sheet {
        let t = &self.text;
        Sheet {
            kind: "notice".into(),
            title: t.s("home_title"),
            lines: vec![t.s("home_1"), t.s("home_2"), t.s("home_3"), t.s("home_4")],
            buttons: vec![button(t.s("ok"), "sheet.close", "").tone("primary")],
            ..Sheet::default()
        }
    }

    fn prestige_sheet(&self) -> Sheet {
        let t = &self.text;
        let next = self.s.mult_bp() + c::PRESTIGE_STEP_BP;
        Sheet {
            kind: "notice".into(),
            title: t.t("prestige_q", &[("n", (self.s.prestige + 1).to_string())]),
            big: t.t("mult_change", &[("a", crate::text::times(self.s.mult_bp())), ("b", crate::text::times(next))]),
            lines: vec![t.s("prestige_warn")],
            buttons: vec![button(t.s("not_yet"), "sheet.close", ""), button(t.s("tab_prestige"), "prestige.do", "").tone("pres")],
            ..Sheet::default()
        }
    }

    fn flag_sheet(&self) -> Sheet {
        let t = &self.text;
        let fun = (1..=4u8).map(|k| button(t.s(&format!("fun_{k}")), "flag.fun", &k.to_string()).pressed(self.draft.fun == k)).collect();
        let pace = (1..=3u8).map(|k| button(t.s(&format!("pace_{k}")), "flag.pace", &k.to_string()).pressed(self.draft.pace == k)).collect();
        Sheet {
            kind: "flag".into(),
            title: t.s("flag_title"),
            lines: vec![t.s("flag_intro")],
            blocks: vec![
                Block { title: t.s("flag_fun"), buttons: fun, ..block("choice") },
                Block { title: t.s("flag_pace"), buttons: pace, ..block("choice") },
                Block { title: t.s("flag_note"), act: "flag.note".into(), value: self.draft.note.clone(), text: t.s("flag_note_hint"), ..block("input") },
            ],
            buttons: vec![button(t.s("cancel"), "sheet.close", ""), button(t.s("flag_save"), "flag.save", "").tone("primary")],
            ..Sheet::default()
        }
    }

    // ---------- drawing ----------

    pub fn ui(&mut self) -> Ui {
        let v = self.view();
        let next_ms = match &self.s.active {
            Some(id) => self.s.duration(id).saturating_sub(self.s.progress_ms).max(1),
            None => 0,
        };
        let ui = Ui {
            title: self.text.title.clone(),
            top: v.top(),
            tabs: v.tabs(),
            screen: self.s.screen.clone(),
            blocks: v.screen(),
            sheet: self.sheet.clone(),
            toasts: std::mem::take(&mut self.toasts),
            next_ms,
            autosave_ms: c::AUTOSAVE_MS,
            stash: self.stash,
            hints: self.hints.clone(),
        };
        self.stash = false;
        ui
    }

    fn ui_json(&mut self) -> String {
        serde_json::to_string(&self.ui()).unwrap_or_default()
    }

    /// For tests: the state as it is.
    pub fn state(&self) -> &State {
        &self.s
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.s
    }
}

fn block(kind: &str) -> Block {
    Block { kind: kind.into(), ..Block::default() }
}

/// The tuning numbers the playtest file carries, so a reader can match a
/// complaint to a number.
fn tuning() -> serde_json::Value {
    serde_json::json!({
        "away_cap_ms": c::AWAY_CAP_MS,
        "return_after_ms": c::RETURN_AFTER_MS,
        "bonfire_ms_per_log": c::BONFIRE_MS_PER_LOG,
        "bonfire_xp_pct": c::BONFIRE_XP_PCT,
        "prestige_base": c::PRESTIGE_BASE,
        "prestige_rise": c::PRESTIGE_RISE,
        "prestige_step_bp": c::PRESTIGE_STEP_BP,
        "mastery_scale": crate::xp::MASTERY_SCALE,
        "nest_one_in": c::NEST_ONE_IN,
        "actions": c::ACTIONS.iter().map(|a| serde_json::json!([a.id, a.level, a.ms, a.xp])).collect::<Vec<_>>(),
        "enemies": c::ENEMIES.iter().map(|e| serde_json::json!([e.id, e.hp, e.hit, e.gold])).collect::<Vec<_>>(),
        "tools": c::TOOL_TIERS,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const STRINGS: &str = include_str!("../../../data/strings.json");
    const GLOSSARY: &str = include_str!("../../../GLOSSARY.md");

    fn ui(json: &str) -> serde_json::Value {
        serde_json::from_str(json).unwrap()
    }

    fn game(now: u64) -> Game {
        Game::new(STRINGS, GLOSSARY, "", "{}", now).unwrap()
    }

    fn all_text(v: &serde_json::Value, out: &mut Vec<String>) {
        match v {
            serde_json::Value::String(s) => out.push(s.clone()),
            serde_json::Value::Array(a) => a.iter().for_each(|x| all_text(x, out)),
            serde_json::Value::Object(o) => o.values().for_each(|x| all_text(x, out)),
            _ => {}
        }
    }

    #[test]
    fn a_new_game_welcomes_the_player_on_the_woodcutting_tab() {
        let mut g = game(1_000);
        let v = ui(&g.tick(1_000));
        assert_eq!(v["screen"], "woodcutting");
        assert_eq!(v["sheet"]["kind"], "notice");
        assert_eq!(v["tabs"].as_array().unwrap().len(), 14);
        assert_eq!(v["next_ms"], 0);
    }

    #[test]
    fn every_screen_and_sheet_finds_all_its_words() {
        // A missing key shows as [key]; no screen may show one.
        let mut g = game(1);
        g.state_mut().bank.insert("bronze_sword".into(), 1);
        g.state_mut().bank.insert("logs".into(), 5);
        let mut texts = Vec::new();
        for tab in TABS {
            all_text(&ui(&g.act("tab", tab, 2)), &mut texts);
        }
        for (name, value) in [("do", "tree"), ("flag.open", ""), ("help.home", ""), ("reset.ask", ""), ("do", "chicken")] {
            all_text(&ui(&g.act(name, value, 3)), &mut texts);
        }
        g.state_mut().start("tree", 3);
        all_text(&ui(&g.tick(3 + 2 * 3_600_000)), &mut texts);
        let missing: Vec<&String> = texts.iter().filter(|s| s.starts_with('[') && s.ends_with(']')).collect();
        assert!(missing.is_empty(), "{missing:?}");
        assert!(!texts.iter().any(|s| s.contains('{') && s.contains('}')), "an unfilled placeholder");
    }

    #[test]
    fn coming_back_after_ten_minutes_shows_the_return_screen() {
        let mut g = game(0);
        g.act("do", "tree", 0);
        let v = ui(&g.tick(10 * 60_000));
        assert_eq!(v["sheet"]["kind"], "return");
        assert_eq!(v["sheet"]["big"], "10m");
        // 600 s / 3 s = 200 chops at 10 XP
        assert_eq!(g.state().xp[0], 2_000);
    }

    #[test]
    fn a_short_absence_gives_toasts_not_the_return_screen() {
        let mut g = game(0);
        g.act("do", "tree", 0);
        g.act("sheet.close", "", 0);
        let v = ui(&g.tick(9 * 60_000));
        assert!(v.get("sheet").is_none());
        assert!(v["toasts"].as_array().unwrap().iter().any(|t| t.as_str().unwrap().contains("Woodcutting")));
    }

    #[test]
    fn away_time_counts_for_a_day_at_most() {
        let mut g = game(0);
        g.act("do", "tree", 0);
        g.tick(30 * 3_600_000);
        // 24 h / 3 s = 28,800 chops
        assert_eq!(g.state().sim_ms, 24 * 3_600_000);
        let v = ui(&g.ui_json());
        assert!(v["sheet"]["notes"][0].as_str().unwrap().contains("6h"));
    }

    #[test]
    fn a_clock_that_goes_back_credits_nothing() {
        let mut g = game(10_000);
        g.act("do", "tree", 10_000);
        g.tick(5_000);
        assert_eq!(g.state().sim_ms, 0);
        g.tick(13_000);
        assert_eq!(g.state().xp[0], 10);
    }

    #[test]
    fn the_save_round_trips_and_a_bad_save_starts_over_and_is_kept() {
        let mut g = game(0);
        g.act("do", "tree", 0);
        g.tick(30_000);
        let saved = g.save();
        let g2 = Game::new(STRINGS, GLOSSARY, &saved, "{}", 40_000).unwrap();
        assert_eq!(g2.state().xp, g.state().xp);
        let mut g3 = Game::new(STRINGS, GLOSSARY, "{not json", "{}", 0).unwrap();
        let v = ui(&g3.tick(0));
        assert_eq!(v["stash"], true);
        assert_eq!(g3.state().xp[0], 0);
        let newer = saved.replace("\"v\":1", "\"v\":99");
        let mut g4 = Game::new(STRINGS, GLOSSARY, &newer, "{}", 0).unwrap();
        assert_eq!(ui(&g4.tick(0))["stash"], true);
    }

    #[test]
    fn export_then_import_gives_the_same_game() {
        let mut g = game(0);
        g.act("do", "tree", 0);
        g.tick(60_000);
        let file = g.export(60_000);
        let mut fresh = game(0);
        fresh.import(&file, 70_000);
        assert_eq!(fresh.state().xp, g.state().xp);
        let v = ui(&fresh.import("{\"game\":\"other\"}", 70_000));
        assert!(v["sheet"]["title"].as_str().unwrap().len() > 1);
        assert_eq!(fresh.state().xp, g.state().xp);
    }

    #[test]
    fn a_flagged_moment_lands_in_the_playtest_file() {
        let mut g = game(0);
        g.act("do", "tree", 0);
        g.act("flag.open", "", 1_000);
        g.act("flag.fun", "3", 1_000);
        g.act("flag.pace", "1", 1_000);
        g.act("flag.note", "logs come in slowly at the start", 1_000);
        g.act("flag.save", "", 2_000);
        g.tick(2_000 + 10 * 60_000);
        let p: serde_json::Value = serde_json::from_str(&g.playtest(11 * 60_000)).unwrap();
        assert_eq!(p["kind"], "idle-game-playtest");
        assert_eq!(p["flags"][0]["fun"], 3);
        assert_eq!(p["flags"][0]["pace"], 1);
        assert_eq!(p["flags"][0]["note"], "logs come in slowly at the start");
        assert_eq!(p["flags"][0]["doing"], "tree");
        assert_eq!(p["summary"]["returns"], 1);
        assert!(p["summary"]["skills"]["woodcutting"]["level_ups"].as_array().unwrap().len() >= 10);
        assert!(p["summary"]["skills"]["woodcutting"]["xp_per_hour"].as_u64().unwrap() > 0);
        assert!(p["save"]["state"]["xp"].is_array());
    }

    #[test]
    fn prestige_asks_first_and_only_when_ready() {
        let mut g = game(0);
        let v = ui(&g.act("prestige.ask", "", 0));
        assert_eq!(v["sheet"]["kind"], "notice");
        for i in 0..c::SKILLS.len() {
            g.state_mut().xp[i] = crate::xp::at(50);
        }
        g.act("prestige.do", "", 0);
        assert_eq!(g.state().prestige, 1);
    }

    #[test]
    fn the_plan_is_built_from_cards_and_started_from_its_tab() {
        let mut g = game(0);
        g.act("sheet.close", "", 0);
        g.act("plan.add", "tree", 0);
        g.act("plan.add", "burn_logs", 0);
        g.act("plan.count", "0:100", 0);
        g.act("plan.start", "", 0);
        assert_eq!(g.state().active.as_deref(), Some("tree"));
        assert_eq!(g.state().plan[0].n, 100);
        g.act("plan.count", "0:7", 0);
        assert_eq!(g.state().plan[0].n, 100);
    }
}
