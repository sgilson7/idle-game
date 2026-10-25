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
    /// The bottom bar's five groups (P21).
    pub groups: Vec<Tab>,
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
    /// What the app tells the player while it is closed (T17): the device
    /// schedules these when the page hides and drops them when it shows.
    /// Empty on the web.
    pub alerts: Vec<Alert>,
}

/// One notification for the device to schedule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Alert {
    /// The same kind of alert keeps the same id, so a new one replaces it.
    pub id: u32,
    /// Wall-clock milliseconds to show it at.
    pub at: u64,
    pub title: String,
    pub body: String,
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
    /// What the bank search box holds (IG-Q41 A); not saved.
    search: String,
    /// Skill screens show every row, not only the newest (T15); not saved.
    all: bool,
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
    let mut state = file.state;
    // Version 1 had twelve skills; Crafting and Farming were added at the end
    // (T5). Version 2 had fourteen; the four engine skills follow (T11 to T14).
    if (state.v == 1 && state.xp.len() == 12) || (state.v == 2 && state.xp.len() == 14) {
        state.xp.resize(c::SKILLS.len(), 0);
        state.upkeep_hour = state.sim_ms / c::HOUR;
        state.v = 3;
    }
    // Version 3 had six rooms and one tier of each engine; the regions add
    // six rooms and a tier each (T15). Ranks, rooms and goods carry over.
    if state.v == 3 {
        state.v = SAVE_VERSION;
    }
    state.rooms.resize(c::ROOMS.len(), 0);
    if state.xp.len() != c::SKILLS.len()
        || state.tools.len() != c::TOOL_SKILLS.len()
        || state.style >= c::STYLES.len()
    {
        return Err("a save with the wrong shape".into());
    }
    Ok(state)
}

impl Game {
    /// Starts from the saved text, or a new game when there is none. A save
    /// that cannot be read starts a new game and asks the page to keep the
    /// old text (IG-5).
    pub fn new(
        strings_json: &str,
        glossary_md: &str,
        saved: &str,
        device_json: &str,
        now: u64,
    ) -> Result<Game, String> {
        let text = Text::parse(strings_json)?;
        let hints = glossary::parse(glossary_md)
            .map_err(|e| e.message())?
            .into_iter()
            .map(|e| Hint {
                term: e.term,
                definition: e.definition,
            })
            .collect();
        let device: Device = serde_json::from_str(device_json).unwrap_or_default();
        let mut g = Game {
            text,
            hints,
            s: State::new(now),
            device,
            sheet: None,
            toasts: Vec::new(),
            draft: Flag::default(),
            stash: false,
            search: String::new(),
            all: false,
        };
        g.seal();
        if saved.trim().is_empty() {
            g.sheet = Some(g.welcome());
            return Ok(g);
        }
        match read_save(saved) {
            Ok(s) => {
                g.s = s;
                g.seal();
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
        serde_json::to_string(&SaveFile {
            game: GAME_ID.into(),
            v: SAVE_VERSION,
            state: self.s.clone(),
        })
        .unwrap_or_default()
    }

    /// The save as a file to keep (long-term save).
    pub fn export(&mut self, now: u64) -> String {
        self.s.exported_wall = now;
        self.s.note(now, "export", "", 0);
        serde_json::to_string_pretty(&SaveFile {
            game: GAME_ID.into(),
            v: SAVE_VERSION,
            state: self.s.clone(),
        })
        .unwrap_or_default()
    }

    pub fn import(&mut self, text: &str, now: u64) -> String {
        match read_save(text) {
            Ok(s) => {
                self.s = s;
                self.seal();
                self.s.last_wall = now;
                self.s.note(now, "import", "", 0);
                self.sheet = Some(self.notice("import_ok_title", self.text.s("import_ok_text")));
            }
            Err(why) => {
                self.sheet = Some(self.notice(
                    "import_bad_title",
                    self.text.t("import_bad_text", &[("why", why)]),
                ));
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
                        "xp_per_hour": (self.s.xp[i] * 3_600_000).checked_div(ms).unwrap_or(0),
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
            "regions_open": self.s.regions_open(),
            "engines": {
                "ranks": self.s.ranks,
                "region_ranks": self.s.region_ranks,
                "rooms": self.s.rooms,
                "recipes": self.s.recipes,
                "blueprints": self.s.blueprints,
                "servants": self.s.servants.iter().filter(|s| !s.job.is_empty()).count(),
                "orders": self.s.counts.get("orders").copied().unwrap_or(0),
                "caravans": self.s.counts.get("caravans").copied().unwrap_or(0),
                "income": self.s.counts.get("income").copied().unwrap_or(0),
            },
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
        let credited = elapsed.min(self.s.away_cap());
        let mut r = Report::begin(&self.s, elapsed, elapsed - credited);
        let growing: Vec<usize> = (0..self.s.patches_open())
            .filter(|&i| self.s.grow_left(i).unwrap_or(0) > 0)
            .collect();
        self.s.advance(credited, &mut r);
        let mut ripe: Vec<String> = growing
            .into_iter()
            .filter(|&i| self.s.ready(i))
            .map(|i| self.s.patch(i).crop)
            .collect();
        ripe.sort();
        ripe.dedup();
        for crop in &ripe {
            self.s.note(now, "crop_ready", crop, 0);
        }
        let away = elapsed >= c::RETURN_AFTER_MS;
        self.log_report(&r, now, away);
        if away {
            self.s.count("away_ms", elapsed);
            if r.actions > 0
                || r.stop.is_some()
                || !ripe.is_empty()
                || r.gold > 0
                || !r.research.is_empty()
                || !r.chests.is_empty()
            {
                let sheet = self.view().return_sheet(&r);
                self.sheet = Some(sheet);
            }
        } else {
            self.s.count("seen_ms", elapsed);
            self.toasts = self.toasts_for(&r);
            for crop in &ripe {
                self.toasts
                    .push(self.text.t("toast_ripe", &[("what", self.text.action(crop))]));
            }
        }
    }

    /// An engine act: whatever it changes, its levels and finds are logged
    /// like any action's, and it says so in a toast.
    fn engine(&mut self, now: u64, name: &str, value: &str) {
        let mut r = Report::begin(&self.s, 0, 0);
        let (k, rest) = value.split_once(':').unwrap_or((value, ""));
        let num_k = k.parse::<usize>().ok();
        // A rise names its region first ("2" or "2:tribute"); none is the Vale.
        let reg = num_k.filter(|&g| g < c::REGIONS.len()).unwrap_or(0);
        let done = match name {
            "room.build" => num_k.is_some_and(|i| i < c::ROOMS.len() && self.s.build_room(i, &mut r)),
            "rise.noble" => self.s.rise_noble_in(reg, value.ends_with("tribute"), &mut r),
            "rise.trader" => self.s.rise_trader_in(reg),
            "rise.inventor" => self.s.rise_inventor_in(reg, &mut r),
            "rise.cosmic" => self.s.rise_cosmic_in(reg, &mut r),
            "servant.set" => {
                let job = self.s.active.clone().unwrap_or_default();
                num_k.is_some_and(|i| self.s.set_servant(i, &job))
            }
            "servant.dismiss" => {
                if let Some(i) = num_k {
                    self.s.dismiss_servant(i);
                }
                num_k.is_some()
            }
            "order.fill" => match (num_k, rest.parse::<u64>()) {
                (Some(t), Ok(slot)) if t < c::TOWNS.len() => self.s.fill_order(t, slot, &mut r),
                _ => false,
            },
            "caravan.send" | "caravan.guarded" => {
                let guards = name == "caravan.guarded";
                match num_k {
                    Some(t) if t < c::TOWNS.len() => self.s.send_caravan(t, rest, guards, &mut r),
                    _ => false,
                }
            }
            "research" => self.s.start_research(value, &mut r),
            "machine.toggle" => {
                self.s.toggle_machine(value);
                true
            }
            _ => false,
        };
        if done {
            self.s.note(now, name, value, 0);
            let key = format!("toast_{}", name.replace('.', "_"));
            if self.text.ui.contains_key(&key) {
                self.toasts.push(self.text.s(&key));
            }
        }
        self.log_report(&r, now, false);
        let more = self.toasts_for(&r);
        self.toasts.extend(more);
    }

    /// A farming act: plant or harvest, with its levels logged like any action's.
    fn farm(&mut self, now: u64, what: &str, value: &str) {
        let mut r = Report::begin(&self.s, 0, 0);
        match what {
            "plant" => {
                if self.s.plant(value, &mut r) {
                    self.s.note(now, "plant", value, 0);
                }
            }
            "harvest" => {
                let crop = value
                    .parse::<usize>()
                    .map(|i| self.s.patch(i).crop)
                    .unwrap_or_default();
                let n = value
                    .parse::<usize>()
                    .map(|i| self.s.harvest(i, &mut r))
                    .unwrap_or(0);
                if n > 0 {
                    self.s.note(now, "harvest", &crop, n);
                    self.toasts.push(self.text.t(
                        "toast_harvest",
                        &[("n", n.to_string()), ("item", self.text.item(&crop))],
                    ));
                }
            }
            _ => {
                let n = self.s.harvest_all(&mut r);
                if n > 0 {
                    self.s.note(now, "harvest", "all", n);
                    self.toasts
                        .push(self.text.t("toast_harvest_all", &[("n", n.to_string())]));
                }
                self.sheet = None;
            }
        }
        self.log_report(&r, now, false);
        let more = self.toasts_for(&r);
        self.toasts.extend(more);
    }

    fn log_report(&mut self, r: &Report, now: u64, away: bool) {
        for &(skill, level, at) in &r.levels {
            self.s
                .note_at(now, at, "level", c::SKILLS[skill].id, level as u64);
        }
        for (id, cp) in &r.checkpoints {
            self.s.note(now, "mastery", id, *cp as u64);
        }
        for (item, n) in &r.rare {
            self.s.note(now, "rare", item, *n);
        }
        for id in &r.recipes {
            self.s.note(now, "recipe", id, 0);
        }
        for (t, gold, bandits) in &r.caravans {
            self.s.note(
                now,
                if *bandits { "caravan_bandits" } else { "caravan" },
                c::TOWNS[*t].id,
                *gold,
            );
        }
        for id in &r.research {
            self.s.note(now, "blueprint", id, 0);
        }
        for id in &r.chests {
            self.s.note(now, "chest", id, 0);
        }
        if r.masterworks > 0 {
            self.s.note(now, "masterwork", "", r.masterworks);
        }
        if r.income > 0 {
            self.s.count("income", r.income);
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
            self.s.note(
                now,
                "away",
                self.s.active.clone().unwrap_or_default().as_str(),
                r.elapsed,
            );
        }
    }

    fn toasts_for(&self, r: &Report) -> Vec<String> {
        let t = &self.text;
        let mut out = Vec::new();
        for (i, sk) in c::SKILLS.iter().enumerate() {
            let l = self.s.level(i);
            if l > r.lv0[i] {
                out.push(t.t(
                    "toast_level",
                    &[("skill", t.skill(sk.id).name.clone()), ("n", l.to_string())],
                ));
            }
        }
        for (id, cp) in &r.checkpoints {
            out.push(t.t("toast_mastery", &[("what", t.action(id)), ("n", cp.to_string())]));
        }
        for item in r.rare.keys() {
            out.push(t.t("toast_rare", &[("item", t.item(item))]));
        }
        for id in &r.recipes {
            out.push(t.t("toast_recipe", &[("what", t.action(id))]));
        }
        for (town, gold, _) in &r.caravans {
            out.push(t.t(
                "toast_caravan",
                &[
                    ("town", t.s(&format!("town_{}", c::TOWNS[*town].id))),
                    ("n", num(*gold)),
                ],
            ));
        }
        for id in &r.research {
            out.push(t.t("toast_blueprint", &[("what", t.s(&format!("bp_{id}")))]));
        }
        for id in &r.chests {
            out.push(t.t("toast_chest", &[("what", t.s(&format!("dungeon_{id}")))]));
        }
        if r.masterworks > 0 {
            out.push(t.t("toast_masterwork", &[("n", num(r.masterworks))]));
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
                self.all = false;
            }
            "do" => {
                self.sheet = None;
                if self.s.active.as_deref() == Some(value) {
                    self.s.stop();
                    self.s.note(now, "stop", value, 0);
                } else if self.s.unlocked(value) && self.s.missing(value).is_none() {
                    self.s.start(value, now);
                    self.s.screen = match c::action_index(value) {
                        Some(a) => crate::view::tab_of_skill(c::ACTIONS[a].skill).into(),
                        None => "combat".into(),
                    };
                    self.all = false;
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
                    self.toasts = vec![self
                        .text
                        .t("toast_planned", &[("n", self.s.plan.len().to_string())])];
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
                self.s.note(
                    now,
                    "plan",
                    &self
                        .s
                        .plan
                        .iter()
                        .map(|p| p.id.clone())
                        .collect::<Vec<_>>()
                        .join(","),
                    0,
                );
            }
            "plan.clear" => self.s.plan.clear(),
            "sell" => {
                let got = self.s.sell(value);
                if got > 0 {
                    self.s.note(now, "sell", value, got);
                    self.toasts = vec![self
                        .text
                        .t("toast_sold", &[("item", self.text.item(value)), ("n", num(got))])];
                }
            }
            "room.build" | "rise.noble" | "rise.trader" | "rise.inventor" | "rise.cosmic" | "servant.set"
            | "servant.dismiss" | "order.fill" | "caravan.send" | "caravan.guarded" | "research"
            | "machine.toggle" => self.engine(now, name, value),
            "farm.plant" => self.farm(now, "plant", value),
            "farm.harvest" => self.farm(now, "harvest", value),
            "farm.harvest_all" => self.farm(now, "all", value),
            "buy.seed" => {
                if let Some(ci) = c::crop_index(value) {
                    let cr = &c::CROPS[ci];
                    let cost = cr.seed_cost * c::SEED_PACK;
                    if self.s.gold >= cost && self.s.level(c::FARMING) >= cr.level {
                        self.s.gold -= cost;
                        *self.s.bank.entry(cr.seed.to_string()).or_insert(0) += c::SEED_PACK;
                        self.s.note(now, "buy", cr.seed, c::SEED_PACK);
                    }
                }
            }
            "buy" => {
                if let Ok(line) = value.parse::<usize>() {
                    if self.s.buy(line) {
                        self.s.note(
                            now,
                            "buy",
                            c::SKILLS[c::TOOL_SKILLS[line]].id,
                            self.s.tools[line] as u64,
                        );
                    }
                }
            }
            "prestige.ask" if self.s.can_prestige() => self.sheet = Some(self.prestige_sheet()),
            "prestige.do" if self.s.can_prestige() => {
                self.s.do_prestige(now);
                self.sheet = None;
                self.toasts = vec![self
                    .text
                    .t("toast_prestige", &[("n", self.s.prestige.to_string())])];
            }
            "bank.search" => self.search = value.chars().take(40).collect(),
            "list.all" => self.all = !self.all,
            "sheet.close" => self.sheet = None,
            "help.home" => self.sheet = Some(self.home_help()),
            "reset.ask" => {
                self.sheet = Some(Sheet {
                    kind: "notice".into(),
                    title: self.text.s("reset_title"),
                    lines: vec![self.text.s("reset_text")],
                    buttons: vec![
                        button(self.text.s("cancel"), "sheet.close", ""),
                        button(self.text.s("reset"), "reset.do", "").tone("stop"),
                    ],
                    ..Sheet::default()
                })
            }
            "reset.do" => {
                let keep = (
                    self.s.log.clone(),
                    self.s.flags.clone(),
                    self.s.counts.clone(),
                    self.s.first_wall,
                );
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
                self.toasts = vec![self
                    .text
                    .t("toast_flagged", &[("n", self.s.flags.len().to_string())])];
            }
            "seen.hide" => {
                self.s.note(now, "hide", "", 0);
            }
            "seen.show" => {
                self.s.note(now, "show", "", 0);
            }
            // The store's answers, as the device reports them (IG-Q46 B).
            "store.owned" => {
                let was = self.s.sealed;
                self.device.owned = value == "1";
                self.seal();
                self.s.note(now, "store_owned", value, 0);
                if was && !self.s.sealed {
                    self.toasts = vec![self.text.s("toast_store_owned")];
                }
            }
            "store.price" => {
                self.device.price = value.into();
            }
            "store.pending" => {
                self.toasts = vec![self.text.s("toast_store_pending")];
            }
            "store.failed" => {
                self.s.note(now, "store_failed", value, 0);
                self.toasts = vec![self.text.s("toast_store_failed")];
            }
            "store.none" => {
                self.toasts = vec![self.text.s("toast_store_none")];
            }
            "playtest.sent" => {
                self.toasts = vec![self.text.s("toast_playtest")];
            }
            _ => {}
        }
    }

    // ---------- sheets ----------

    fn view(&self) -> View<'_> {
        View {
            t: &self.text,
            s: &self.s,
            device: &self.device,
            search: &self.search,
            all: self.all,
        }
    }

    fn welcome(&self) -> Sheet {
        let t = &self.text;
        Sheet {
            kind: "notice".into(),
            title: t.s("welcome_title"),
            lines: vec![
                t.s("welcome_1"),
                t.t("welcome_2", &[("cap", crate::text::dur(self.s.away_cap()))]),
                t.s("welcome_3"),
            ],
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
            big: t.t(
                "mult_change",
                &[
                    ("a", crate::text::times(self.s.mult_bp())),
                    ("b", crate::text::times(next)),
                ],
            ),
            lines: vec![t.s("prestige_warn")],
            buttons: vec![
                button(t.s("not_yet"), "sheet.close", ""),
                button(t.s("tab_prestige"), "prestige.do", "").tone("pres"),
            ],
            ..Sheet::default()
        }
    }

    fn flag_sheet(&self) -> Sheet {
        let t = &self.text;
        let fun = (1..=4u8)
            .map(|k| {
                button(t.s(&format!("fun_{k}")), "flag.fun", &k.to_string()).pressed(self.draft.fun == k)
            })
            .collect();
        let pace = (1..=3u8)
            .map(|k| {
                button(t.s(&format!("pace_{k}")), "flag.pace", &k.to_string()).pressed(self.draft.pace == k)
            })
            .collect();
        Sheet {
            kind: "flag".into(),
            title: t.s("flag_title"),
            lines: vec![t.s("flag_intro")],
            blocks: vec![
                Block {
                    title: t.s("flag_fun"),
                    buttons: fun,
                    ..block("choice")
                },
                Block {
                    title: t.s("flag_pace"),
                    buttons: pace,
                    ..block("choice")
                },
                Block {
                    title: t.s("flag_note"),
                    act: "flag.note".into(),
                    value: self.draft.note.clone(),
                    text: t.s("flag_note_hint"),
                    ..block("input")
                },
            ],
            buttons: vec![
                button(t.s("cancel"), "sheet.close", ""),
                button(t.s("flag_save"), "flag.save", "").tone("primary"),
            ],
            ..Sheet::default()
        }
    }

    // ---------- drawing ----------

    pub fn ui(&mut self) -> Ui {
        let v = self.view();
        // The page's one timer wakes for whichever comes first: the running
        // action, an engine's timer (T11 to T14), or the next crop to ripen
        // (T5). 0 means nothing to wake for.
        let event = Some(self.s.next_event())
            .filter(|&n| n < u64::MAX)
            .map(|n| n.max(1));
        let crop = (0..self.s.patches_open())
            .filter_map(|i| self.s.grow_left(i))
            .filter(|&left| left > 0)
            .min();
        let next_ms = event.into_iter().chain(crop).min().unwrap_or(0);
        let ui = Ui {
            title: self.text.title.clone(),
            top: v.top(),
            tabs: v.tabs(),
            groups: v.groups(),
            screen: self.s.screen.clone(),
            blocks: v.screen(),
            sheet: self.sheet.clone(),
            toasts: std::mem::take(&mut self.toasts),
            next_ms,
            autosave_ms: c::AUTOSAVE_MS,
            stash: self.stash,
            hints: self.hints.clone(),
            alerts: self.alerts(),
        };
        self.stash = false;
        ui
    }

    /// The regions past the Vale wait for the purchase in the app.
    fn seal(&mut self) {
        self.s.sealed = self.device.app && !self.device.owned;
    }

    /// One alert of each kind, for the first moment it happens: a crop
    /// ripens, a caravan comes home, research finishes, and time away stops
    /// counting. Worked out from the state as it is; what the player does
    /// next can change them, so the device asks again on each hide.
    pub fn alerts(&self) -> Vec<Alert> {
        if !self.device.app {
            return Vec::new();
        }
        let now = self.s.last_wall;
        let crop = (0..self.s.patches_open())
            .filter_map(|i| self.s.grow_left(i))
            .filter(|&left| left > 0)
            .min();
        let caravan = self
            .s
            .caravans
            .iter()
            .map(|cv| cv.back.saturating_sub(self.s.sim_ms))
            .filter(|&left| left > 0)
            .min();
        let research = self.s.research_need().filter(|&left| left > 0);
        let cap = self.s.away_cap();
        let hours = (cap / c::HOUR).to_string();
        [
            (1, "crop", crop, Vec::new()),
            (2, "caravan", caravan, Vec::new()),
            (3, "research", research, Vec::new()),
            (4, "away", Some(cap), vec![("n", hours)]),
        ]
        .into_iter()
        .filter_map(|(id, kind, left, args)| {
            left.filter(|&l| l <= cap).map(|l| Alert {
                id,
                at: now + l,
                title: self.text.s(&format!("alert_{kind}_title")),
                body: self.text.t(&format!("alert_{kind}_text"), &args),
            })
        })
        .collect()
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
    Block {
        kind: kind.into(),
        ..Block::default()
    }
}

/// The tuning numbers the playtest file carries, so a reader can match a
/// complaint to a number.
fn tuning() -> serde_json::Value {
    serde_json::json!({
        "away_cap_ms": c::AWAY_CAP_MS,
        "engine_at": c::ENGINE_AT.iter().map(|(_, at)| *at).collect::<Vec<_>>(),
        "recipes": c::RECIPES.iter().map(|r| serde_json::json!([r.id, r.enemy, r.one_in])).collect::<Vec<_>>(),
        "towns": c::TOWNS.iter().map(|t| serde_json::json!([t.id, t.level, t.trip_ms, t.danger])).collect::<Vec<_>>(),
        "regions": c::REGIONS,
        "noble_ranks": c::NOBLE_RANKS.iter().flatten().map(|r| serde_json::json!([r.id, r.gold, r.rooms, r.income])).collect::<Vec<_>>(),
        "blueprints": c::BLUEPRINTS.iter().map(|b| serde_json::json!([b.id, b.level, b.gold, b.ms])).collect::<Vec<_>>(),
        "machines": c::MACHINES.iter().map(|m| serde_json::json!([m.id, m.ms])).collect::<Vec<_>>(),
        "plan_steps": c::PLAN_STEPS,
        "return_after_ms": c::RETURN_AFTER_MS,
        "prestige_base": c::PRESTIGE_BASE,
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

    /// Every skill at 40 (total level 720, so all four regions are open),
    /// every room built once, a rank on each ladder, a recipe and a
    /// blueprint known, and goods for every screen to show.
    fn engines_open(g: &mut Game) {
        let s = g.state_mut();
        for i in 0..c::SKILLS.len() {
            s.xp[i] = crate::xp::at(40);
        }
        s.rooms = vec![1; c::ROOMS.len()];
        s.ranks = [1, 2, 1, 1];
        s.recipes.push("farmers_stew".into());
        s.blueprints.push("sawmill".into());
        s.gold = 50_000;
        for (item, n) in [
            ("logs", 500),
            ("cart", 2),
            ("shrimp", 50),
            ("wood_parts", 100),
            ("auto_smelter", 1),
            ("axe_gadget", 1),
            ("starlight", 50),
            ("iron_ore", 20),
            ("bronze_sword", 1),
        ] {
            s.bank.insert(item.into(), n);
        }
        s.servants.push(crate::game::Servant {
            job: "copper".into(),
            progress: 0,
            unpaid: true,
        });
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
        assert_eq!(v["tabs"].as_array().unwrap().len(), 23);
        assert_eq!(v["groups"].as_array().unwrap().len(), 5);
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
        for (name, value) in [
            ("do", "tree"),
            ("flag.open", ""),
            ("help.home", ""),
            ("reset.ask", ""),
            ("do", "chicken"),
        ] {
            all_text(&ui(&g.act(name, value, 3)), &mut texts);
        }
        g.state_mut().start("tree", 3);
        all_text(&ui(&g.tick(3 + 2 * 3_600_000)), &mut texts);
        // Every engine open, with something in each, and a return screen that
        // has a caravan, a blueprint, a recipe and the estate's hours in it.
        let mut g = game(1);
        engines_open(&mut g);
        for tab in TABS {
            all_text(&ui(&g.act("tab", tab, 2)), &mut texts);
        }
        g.act("do", "tree", 2);
        g.act("servant.set", "0", 2);
        g.act("caravan.send", "0:logs", 2);
        g.act("research", "axe_gadget", 2);
        for tab in [
            "trader",
            "estate",
            "workshop",
            "observatory",
            "farming",
            "recipes",
            "combat",
            "regions",
            "bank",
        ] {
            all_text(&ui(&g.act("tab", tab, 3)), &mut texts);
        }
        g.act("bank.search", "log", 3);
        all_text(&ui(&g.act("tab", "bank", 3)), &mut texts);
        g.act("bank.search", "zzz", 3);
        all_text(&ui(&g.act("tab", "bank", 3)), &mut texts);
        g.act("tab", "woodcutting", 3);
        all_text(&ui(&g.act("list.all", "", 3)), &mut texts);
        all_text(&ui(&g.act("do", "sunken_ship", 3)), &mut texts);
        all_text(&ui(&g.tick(3 + 2 * 3_600_000)), &mut texts);
        texts.sort();
        texts.dedup();
        let missing: Vec<&String> = texts
            .iter()
            .filter(|s| s.starts_with('[') && s.ends_with(']'))
            .collect();
        assert!(missing.is_empty(), "{missing:?}");
        assert!(
            !texts.iter().any(|s| s.contains('{') && s.contains('}')),
            "an unfilled placeholder"
        );
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
        assert!(v["toasts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|t| t.as_str().unwrap().contains("Woodcutting")));
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
        let newer = saved.replace("\"v\":4", "\"v\":99");
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
        assert!(
            p["summary"]["skills"]["woodcutting"]["level_ups"]
                .as_array()
                .unwrap()
                .len()
                >= 10
        );
        assert!(
            p["summary"]["skills"]["woodcutting"]["xp_per_hour"]
                .as_u64()
                .unwrap()
                > 0
        );
        assert!(p["save"]["state"]["xp"].is_array());
    }

    #[test]
    fn a_version_one_save_gains_crafting_and_farming() {
        let mut g = game(0);
        g.act("do", "tree", 0);
        g.tick(30_000);
        let v2 = g.save();
        let mut old: serde_json::Value = serde_json::from_str(&v2).unwrap();
        old["v"] = 1.into();
        old["state"]["v"] = 1.into();
        old["state"]["xp"].as_array_mut().unwrap().truncate(12);
        old["state"].as_object_mut().unwrap().remove("patches");
        let loaded = Game::new(STRINGS, GLOSSARY, &old.to_string(), "{}", 40_000).unwrap();
        assert_eq!(loaded.state().xp.len(), 18);
        assert_eq!(loaded.state().xp[0], 100);
        assert_eq!(loaded.state().xp[c::FARMING], 0);
    }

    #[test]
    fn a_version_two_save_gains_the_engine_skills_and_is_paid_from_its_own_hour() {
        let mut g = game(0);
        g.act("do", "tree", 0);
        g.tick(30_000);
        let mut old: serde_json::Value = serde_json::from_str(&g.save()).unwrap();
        old["v"] = 2.into();
        old["state"]["v"] = 2.into();
        old["state"]["sim_ms"] = (50 * c::HOUR).into();
        old["state"]["xp"].as_array_mut().unwrap().truncate(14);
        for k in ["recipes", "ranks", "rooms", "servants", "caravans", "upkeep_hour"] {
            old["state"].as_object_mut().unwrap().remove(k);
        }
        let loaded = Game::new(STRINGS, GLOSSARY, &old.to_string(), "{}", 40_000).unwrap();
        assert_eq!(loaded.state().xp.len(), 18);
        assert_eq!(loaded.state().xp[0], 100);
        assert_eq!(loaded.state().upkeep_hour, 50);
        assert_eq!(loaded.state().rooms, vec![0; c::ROOMS.len()]);
    }

    #[test]
    fn a_version_three_save_gains_twelve_rooms_and_keeps_its_ranks() {
        let mut g = game(0);
        g.act("do", "tree", 0);
        g.tick(30_000);
        let mut old: serde_json::Value = serde_json::from_str(&g.save()).unwrap();
        old["v"] = 3.into();
        old["state"]["v"] = 3.into();
        old["state"]["rooms"] = serde_json::json!([2, 1, 0, 0, 1, 0]);
        old["state"]["ranks"] = serde_json::json!([1, 3, 0, 0]);
        for k in ["region_ranks", "dungeon_stage"] {
            old["state"].as_object_mut().unwrap().remove(k);
        }
        let loaded = Game::new(STRINGS, GLOSSARY, &old.to_string(), "{}", 40_000).unwrap();
        assert_eq!(loaded.state().v, SAVE_VERSION);
        assert_eq!(loaded.state().rooms, vec![2, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(loaded.state().ranks, [1, 3, 0, 0]);
        assert_eq!(loaded.state().region_ranks, [[0; 4]; 3]);
    }

    #[test]
    fn the_bank_groups_items_by_the_tab_that_makes_them_and_searches_by_name() {
        let mut g = game(0);
        g.act("sheet.close", "", 0);
        for (item, n) in [("logs", 5), ("oak_logs", 2), ("copper_ore", 3), ("shrimp", 4)] {
            g.state_mut().bank.insert(item.into(), n);
        }
        let titles = |v: &serde_json::Value| -> Vec<String> {
            v["blocks"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|b| b["kind"] == "bank")
                .map(|b| b["title"].as_str().unwrap().to_string())
                .collect()
        };
        let v = ui(&g.act("tab", "bank", 0));
        assert_eq!(titles(&v), ["Woodcutting", "Mining", "Cooking"]);
        let v = ui(&g.act("bank.search", "LOG", 0));
        assert_eq!(titles(&v), ["Woodcutting"]);
        let woods = v["blocks"]
            .as_array()
            .unwrap()
            .iter()
            .find(|b| b["kind"] == "bank")
            .unwrap()["buttons"]
            .as_array()
            .unwrap()
            .len();
        assert_eq!(woods, 2);
    }

    #[test]
    fn a_long_skill_shows_its_newest_eight_until_you_ask_for_all() {
        // Woodcutting 99 with every region open: 36 trees, all unlocked.
        let mut g = game(0);
        g.act("sheet.close", "", 0);
        for i in 0..c::SKILLS.len() {
            g.state_mut().xp[i] = crate::xp::at(99);
        }
        let cards = |v: &serde_json::Value| {
            v["blocks"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|b| b["kind"] == "cards")
                .map(|b| b["cards"].as_array().unwrap().len())
                .sum::<usize>()
        };
        let v = ui(&g.act("tab", "woodcutting", 0));
        assert_eq!(cards(&v), crate::view::NEWEST);
        let v = ui(&g.act("list.all", "", 0));
        assert_eq!(cards(&v), 36);
        let v = ui(&g.act("tab", "mining", 0));
        assert_eq!(cards(&v), crate::view::NEWEST, "a new tab shows the newest again");
    }

    #[test]
    fn the_bottom_bar_has_five_groups_and_a_group_opens_on_its_running_tab() {
        let mut g = game(0);
        g.act("sheet.close", "", 0);
        g.state_mut().bank.insert("copper_ore".into(), 5);
        g.state_mut().bank.insert("tin_ore".into(), 5);
        let v = ui(&g.act("do", "bronze_bar", 0));
        let groups = v["groups"].as_array().unwrap();
        let ids: Vec<&str> = groups.iter().map(|t| t["id"].as_str().unwrap()).collect();
        assert_eq!(ids, ["gather", "make", "fight", "engines", "more"]);
        // Smithing runs, so Make opens on Smithing; Gather on its first tab.
        assert_eq!(groups[1]["value"], "smithing");
        assert_eq!(groups[1]["running"], true);
        assert_eq!(groups[0]["value"], "woodcutting");
        // the strip shows the group on screen
        let strip: Vec<&str> = v["tabs"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|t| t["strip"] == true)
            .map(|t| t["id"].as_str().unwrap())
            .collect();
        assert_eq!(
            strip,
            [
                "firemaking",
                "fletching",
                "smithing",
                "cooking",
                "crafting",
                "runecrafting",
                "construction"
            ]
        );
    }

    #[test]
    fn an_engine_tab_before_its_level_says_what_opens_it() {
        let mut g = game(0);
        let v = ui(&g.act("tab", "trader", 0));
        let tab = v["tabs"]
            .as_array()
            .unwrap()
            .iter()
            .find(|t| t["id"] == "trader")
            .unwrap()
            .clone();
        assert_eq!(tab["locked"], true);
        let text = v["blocks"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|b| b["text"].as_str())
            .collect::<Vec<_>>()
            .join(" ");
        assert!(text.contains("150"), "{text}");
    }

    #[test]
    fn engine_acts_go_through_the_door() {
        let mut g = game(0);
        g.act("sheet.close", "", 0);
        engines_open(&mut g);
        g.state_mut().ranks = [0, 0, 0, 0];
        g.state_mut().rooms = vec![0; c::ROOMS.len()];
        g.state_mut().bank.insert("plank".into(), 10);
        g.state_mut().bank.insert("nails".into(), 20);
        g.state_mut().bank.insert("limestone".into(), 10);
        g.act("room.build", "0", 0);
        assert_eq!(g.state().rooms[c::KITCHEN], 1);
        g.act("rise.noble", "", 0);
        assert_eq!(g.state().ranks[1], 1);
        let v = ui(&g.act("caravan.send", "0:logs", 0));
        assert_eq!(g.state().caravans.len(), 1);
        assert!(v["toasts"].as_array().unwrap().len() == 1);
        // Nothing else runs: the page is told to wake in an hour, for the caravan.
        assert_eq!(v["next_ms"], c::HOUR);
        g.tick(c::HOUR);
        assert!(g.state().caravans.is_empty());
        assert_eq!(g.state().have("spices"), 10);
    }

    #[test]
    fn away_time_counts_for_longer_at_cosmic_ranks() {
        let mut g = game(0);
        engines_open(&mut g);
        g.state_mut().ranks[3] = 3;
        g.act("do", "tree", 0);
        g.tick(60 * c::HOUR);
        assert_eq!(g.state().sim_ms, 48 * c::HOUR);
    }

    #[test]
    fn seeds_bought_planted_grown_and_harvested_through_the_door() {
        let mut g = game(0);
        g.act("sheet.close", "", 0);
        g.state_mut().gold = 25;
        g.act("buy.seed", "potato", 0);
        assert_eq!(g.state().gold, 5);
        assert_eq!(g.state().have("potato_seed"), 10);
        g.act("farm.plant", "potato", 0);
        g.act("farm.plant", "potato", 0);
        assert_eq!(g.state().have("potato_seed"), 8);
        // Potatoes take 10 minutes. Watching (ticks under 10 minutes apart),
        // the second tick finds them ripe and says so.
        g.tick(5 * 60_000);
        let v = ui(&g.tick(10 * 60_000));
        assert!(v.get("sheet").is_none());
        assert!(v["toasts"]
            .as_array()
            .unwrap()
            .iter()
            .any(|t| t.as_str().unwrap().contains("Potato")));
        g.act("farm.harvest_all", "", 10 * 60_000);
        assert!(g.state().have("potato") >= 6);
        assert!(g.state().patch(0).crop.is_empty());
    }

    #[test]
    fn with_nothing_running_the_page_still_wakes_for_a_ripening_crop() {
        // Potato: 10 minutes. Planted at 0, nothing else running.
        let mut g = game(0);
        g.act("sheet.close", "", 0);
        g.state_mut().bank.insert("potato_seed".into(), 1);
        let v = ui(&g.act("farm.plant", "potato", 0));
        assert_eq!(v["next_ms"], 600_000);
    }

    #[test]
    fn crops_that_ripen_while_away_open_the_return_screen() {
        let mut g = game(0);
        g.act("sheet.close", "", 0);
        g.state_mut().bank.insert("potato_seed".into(), 1);
        g.act("farm.plant", "potato", 0);
        let v = ui(&g.tick(60 * 60_000));
        assert_eq!(v["sheet"]["kind"], "return");
        let ready: Vec<String> = v["sheet"]["ready"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["title"].as_str().unwrap().to_string())
            .collect();
        assert!(ready.iter().any(|t| t.contains("potato")), "{ready:?}");
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

    // ---------- the app (T17) ----------

    fn app(owned: bool, now: u64) -> Game {
        let device = if owned {
            r#"{"app":true,"owned":true,"price":"$0.99"}"#
        } else {
            r#"{"app":true,"price":"$0.99"}"#
        };
        Game::new(STRINGS, GLOSSARY, "", device, now).unwrap()
    }

    fn all_at(g: &mut Game, level: u32) {
        for i in 0..c::SKILLS.len() {
            g.state_mut().xp[i] = crate::xp::at(level);
        }
    }

    #[test]
    fn the_app_keeps_the_later_regions_shut_until_the_purchase() {
        // Level 40 in every skill is past all three region gates.
        let mut web = game(1);
        all_at(&mut web, 40);
        assert_eq!(web.state().regions_open(), 4);
        let mut g = app(false, 1);
        all_at(&mut g, 40);
        assert_eq!(g.state().regions_open(), 1);
        assert!(g.state().region_reached(1));
        let v = ui(&g.act("store.owned", "1", 2));
        assert_eq!(g.state().regions_open(), 4);
        assert_eq!(v["toasts"][0], "Purchase made: the regions are open to you");
        g.act("store.owned", "0", 3);
        assert_eq!(g.state().regions_open(), 1);
        let mut owned = app(true, 1);
        all_at(&mut owned, 40);
        assert_eq!(owned.state().regions_open(), 4);
    }

    #[test]
    fn a_save_loaded_or_imported_into_the_app_stays_shut_without_the_purchase() {
        let mut web = game(1);
        all_at(&mut web, 40);
        let text = web.save();
        let g = Game::new(STRINGS, GLOSSARY, &text, r#"{"app":true}"#, 2).unwrap();
        assert_eq!(g.state().regions_open(), 1);
        let mut g = app(false, 1);
        g.import(&text, 2);
        assert_eq!(g.state().regions_open(), 1);
    }

    #[test]
    fn the_regions_screen_offers_the_purchase_only_in_the_app_and_only_until_it_is_made() {
        let labels = |v: &serde_json::Value| {
            let mut t = Vec::new();
            all_text(&v["blocks"], &mut t);
            t
        };
        let mut g = app(false, 1);
        let t = labels(&ui(&g.act("tab", "regions", 2)));
        assert!(t.iter().any(|s| s == "Buy for $0.99"));
        assert!(t.iter().any(|s| s == "Restore purchase"));
        let t = labels(&ui(&g.act("store.owned", "1", 3)));
        assert!(!t.iter().any(|s| s == "Restore purchase"));
        let mut web = game(1);
        let t = labels(&ui(&web.act("tab", "regions", 2)));
        assert!(!t.iter().any(|s| s == "Restore purchase"));
    }

    #[test]
    fn the_app_lists_one_alert_of_each_kind_within_time_away() {
        let mut g = app(false, 1_000);
        g.tick(1_000);
        let s = g.state_mut();
        let sim = s.sim_ms;
        for back in [30 * c::HOUR, 2 * c::HOUR, 5 * c::HOUR] {
            s.caravans.push(crate::game::Caravan {
                back: sim + back,
                ..Default::default()
            });
        }
        let v = ui(&g.tick(1_000));
        let alerts = v["alerts"].as_array().unwrap();
        // The caravan home soonest (2 hours) and time away running out (24 hours).
        assert_eq!(alerts.len(), 2);
        assert_eq!(alerts[0]["id"], 2);
        assert_eq!(alerts[0]["at"], 1_000 + 7_200_000);
        assert_eq!(alerts[1]["id"], 4);
        assert_eq!(alerts[1]["at"], 1_000 + 86_400_000);
        assert_eq!(
            alerts[1]["body"],
            "Time away counts for 24 hours. Open the game to carry on."
        );
        // A caravan home only after time away runs out gets no alert.
        g.state_mut().caravans.retain(|cv| cv.back == sim + 30 * c::HOUR);
        let v = ui(&g.tick(1_000));
        assert_eq!(v["alerts"].as_array().unwrap().len(), 1);
        assert_eq!(v["alerts"][0]["id"], 4);
        let mut web = game(1_000);
        assert_eq!(ui(&web.tick(1_000))["alerts"].as_array().unwrap().len(), 0);
    }
}
