//! The game's state and its one rule for time (P4): the page hands in its
//! clock, and `advance` credits the milliseconds since it last asked. Watching,
//! a background tab and a closed tab all go through `advance`, so an hour away
//! and an hour watching give the same result.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::content::{self as c, Kind};
use crate::rng::Rng;
use crate::xp;

/// The save format's version (IG-5). A save from a newer version is not read.
pub const SAVE_VERSION: u32 = 1;

/// What is running: an action id or an enemy id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Step {
    pub id: String,
    /// How many to do; 0 means until the inputs run out, or until you come back.
    pub n: u64,
    pub done: u64,
}

/// One line of the playtest log (P12).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Event {
    /// The page's clock.
    pub w: u64,
    /// Game time credited so far.
    pub s: u64,
    pub k: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub a: String,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub n: u64,
}

fn is_zero(n: &u64) -> bool {
    *n == 0
}

/// A moment the player flagged, with how it felt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Flag {
    pub w: u64,
    pub s: u64,
    pub screen: String,
    pub doing: String,
    /// 1 bored · 2 fine · 3 fun · 4 great; 0 not given.
    pub fun: u8,
    /// 1 too slow · 2 about right · 3 too fast; 0 not given.
    pub pace: u8,
    pub note: String,
    pub total: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct State {
    pub v: u32,
    pub xp: Vec<u64>,
    pub mxp: BTreeMap<String, u64>,
    pub bank: BTreeMap<String, u64>,
    pub gold: u64,
    pub tools: Vec<usize>,
    pub active: Option<String>,
    pub progress_ms: u64,
    pub plan: Vec<Step>,
    pub style: usize,
    pub hp: u64,
    pub foe_hp: u64,
    pub kills: BTreeMap<String, u64>,
    pub prestige: u32,
    pub sim_ms: u64,
    pub last_wall: u64,
    pub first_wall: u64,
    pub exported_wall: u64,
    pub rng: Rng,
    pub screen: String,
    pub log: Vec<Event>,
    pub flags: Vec<Flag>,
    pub counts: BTreeMap<String, u64>,
}

/// Why an action could not go on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stop {
    /// An item ran out (or a held item is missing).
    Missing(String),
    /// Out of food in a fight: the player retreats and loses nothing (IG-Q16).
    Food,
}

/// What one stretch of time did, for the return screen and the toasts.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Report {
    pub elapsed: u64,
    pub capped: u64,
    pub actions: u64,
    pub lv0: Vec<u32>,
    pub xp: Vec<u64>,
    pub m0: BTreeMap<String, u32>,
    pub items: BTreeMap<String, u64>,
    pub rare: BTreeMap<String, u64>,
    pub used: BTreeMap<String, u64>,
    pub kills: BTreeMap<String, u64>,
    pub gold: u64,
    pub steps: Vec<String>,
    pub stop: Option<Stop>,
    pub stopped_on: String,
    pub checkpoints: Vec<(String, u32)>,
    /// (skill, level, game time) for each level reached, at the moment it was reached.
    pub levels: Vec<(usize, u32, u64)>,
}

fn add(map: &mut BTreeMap<String, u64>, k: &str, n: u64) {
    *map.entry(k.to_string()).or_insert(0) += n;
}

impl State {
    pub fn new(now: u64) -> State {
        let mut xp = vec![0; c::SKILLS.len()];
        xp[c::HITPOINTS] = xp::at(c::HITPOINTS_START_LEVEL);
        let mut s = State {
            v: SAVE_VERSION,
            xp,
            mxp: BTreeMap::new(),
            bank: BTreeMap::new(),
            gold: 0,
            tools: vec![0; c::TOOL_SKILLS.len()],
            active: None,
            progress_ms: 0,
            plan: Vec::new(),
            style: 0,
            hp: 0,
            foe_hp: 0,
            kills: BTreeMap::new(),
            prestige: 0,
            sim_ms: 0,
            last_wall: now,
            first_wall: now,
            exported_wall: 0,
            rng: Rng::new(now),
            screen: "woodcutting".into(),
            log: Vec::new(),
            flags: Vec::new(),
            counts: BTreeMap::new(),
        };
        s.hp = s.max_hp();
        s.note(now, "new", "", 0);
        s
    }

    // ---------- reading the state ----------

    pub fn level(&self, skill: usize) -> u32 {
        xp::level(self.xp[skill])
    }

    pub fn total_level(&self) -> u32 {
        (0..c::SKILLS.len()).map(|i| self.level(i)).sum()
    }

    pub fn mastery(&self, id: &str) -> u32 {
        xp::mastery_level(self.mxp.get(id).copied().unwrap_or(0))
    }

    pub fn have(&self, item: &str) -> u64 {
        self.bank.get(item).copied().unwrap_or(0)
    }

    pub fn max_hp(&self) -> u64 {
        self.level(c::HITPOINTS) as u64 * c::HP_PER_LEVEL
    }

    /// The skill-XP multiplier in basis points (P9).
    pub fn mult_bp(&self) -> u64 {
        10_000 + c::PRESTIGE_STEP_BP * self.prestige as u64
    }

    pub fn prestige_needs(&self) -> u32 {
        c::PRESTIGE_BASE
    }

    /// How long one action takes now, after mastery and tools.
    pub fn duration(&self, id: &str) -> u64 {
        if c::enemy_index(id).is_some() {
            return c::STYLES[self.style].ms;
        }
        let Some(i) = c::action_index(id) else {
            return 1_000;
        };
        let a = &c::ACTIONS[i];
        let mut ms = a.ms;
        if self.mastery(a.id) >= c::CHECKPOINTS[1] {
            ms -= c::MASTERY_FASTER_MS;
        }
        if let Some(t) = c::TOOL_SKILLS.iter().position(|&s| s == a.skill) {
            ms = ms * (100 - c::TOOL_TIERS[self.tools[t]].1) / 100;
        }
        ms
    }

    /// Chance of a double for a gathering action, in percent (P6).
    pub fn double_pct(&self, id: &str) -> u64 {
        let m = self.mastery(id) as u64;
        let base = if m >= 50 {
            c::DOUBLE_AT_50
        } else if m >= 10 {
            c::DOUBLE_AT_10
        } else {
            0
        };
        base + m / 10
    }

    pub fn keep_pct(&self, id: &str) -> u64 {
        let m = self.mastery(id);
        if m >= 99 {
            c::KEEP_AT_99
        } else if m >= 50 {
            c::KEEP_AT_50
        } else {
            0
        }
    }

    /// Experience one action gives its skill, after prestige and the
    /// mastery-10 bonus (making only). Firemaking gives XP only (T4, IG-Q14 A).
    pub fn xp_for(&self, a: &c::Action) -> u64 {
        let kind = c::SKILLS[a.skill].kind;
        let mut pct = 100;
        if kind == Kind::Make && self.mastery(a.id) >= c::CHECKPOINTS[0] {
            pct += c::MAKE_XP_AT_10;
        }
        a.xp * self.mult_bp() * pct / 1_000_000
    }

    /// The first missing input, if any.
    pub fn missing(&self, id: &str) -> Option<String> {
        if c::enemy_index(id).is_some() {
            let s = &c::STYLES[self.style];
            if !s.held.is_empty() && !s.held.iter().any(|(i, _)| self.have(i) > 0) {
                return Some(s.held[s.held.len() - 1].0.to_string());
            }
            if !s.spent.is_empty() && !s.spent.iter().any(|(i, _)| self.have(i) > 0) {
                return Some(s.spent[s.spent.len() - 1].0.to_string());
            }
            return None;
        }
        let a = &c::ACTIONS[c::action_index(id)?];
        a.needs
            .iter()
            .find(|(i, n)| self.have(i) < *n)
            .map(|(i, _)| i.to_string())
    }

    pub fn unlocked(&self, id: &str) -> bool {
        match c::action_index(id) {
            Some(i) => self.level(c::ACTIONS[i].skill) >= c::ACTIONS[i].level,
            None => c::enemy_index(id).is_some(),
        }
    }

    /// The skill an action or enemy trains.
    pub fn skill_of(&self, id: &str) -> usize {
        match c::action_index(id) {
            Some(i) => c::ACTIONS[i].skill,
            None => c::STYLES[self.style].skill,
        }
    }

    /// Best tier held or spendable for the current style.
    pub fn tier(&self, list: &[(&str, u64)]) -> (String, u64) {
        list.iter()
            .find(|(i, _)| self.have(i) > 0)
            .map(|(i, t)| (i.to_string(), *t))
            .unwrap_or_default()
    }

    pub fn max_hit(&self, enemy: &c::Enemy) -> u64 {
        let s = &c::STYLES[self.style];
        let tiers = self.tier(s.held).1 + self.tier(s.spent).1;
        let strong = if enemy.weak == s.skill {
            c::MAX_HIT_STRONG
        } else {
            0
        };
        c::MAX_HIT_BASE
            + self.level(s.skill) as u64 * c::MAX_HIT_PER_LEVEL
            + tiers * c::MAX_HIT_PER_TIER
            + strong
    }

    pub fn food(&self) -> Option<(&'static str, u64)> {
        c::FOOD.iter().copied().find(|(i, _)| self.have(i) > 0)
    }

    // ---------- changing the state ----------

    pub fn note(&mut self, wall: u64, k: &str, a: &str, n: u64) {
        self.note_at(wall, self.sim_ms, k, a, n);
    }

    /// An event at a given game time, for things that happened during an absence.
    pub fn note_at(&mut self, wall: u64, at: u64, k: &str, a: &str, n: u64) {
        self.log.push(Event {
            w: wall,
            s: at,
            k: k.into(),
            a: a.into(),
            n,
        });
        if self.log.len() > c::LOG_CAP {
            let extra = self.log.len() - c::LOG_CAP;
            self.log.drain(0..extra);
        }
    }

    pub fn count(&mut self, k: &str, n: u64) {
        add(&mut self.counts, k, n);
    }

    fn take(&mut self, item: &str, n: u64, r: &mut Report) {
        let have = self.have(item);
        self.bank.insert(item.to_string(), have.saturating_sub(n));
        add(&mut r.used, item, n);
    }

    fn put(&mut self, item: &str, n: u64, r: &mut Report, rare: bool) {
        add(&mut self.bank, item, n);
        add(if rare { &mut r.rare } else { &mut r.items }, item, n);
    }

    fn gain(&mut self, skill: usize, xp: u64, r: &mut Report) {
        let before = xp::level(self.xp[skill]);
        self.xp[skill] += xp;
        r.xp[skill] += xp;
        for l in (before + 1)..=xp::level(self.xp[skill]) {
            r.levels.push((skill, l, self.sim_ms));
        }
    }

    /// Starts an action or a fight by hand. A plan is cleared (P3).
    pub fn start(&mut self, id: &str, wall: u64) {
        self.plan.clear();
        self.begin(id);
        self.note(wall, "start", id, 0);
    }

    fn begin(&mut self, id: &str) {
        if self.active.as_deref() != Some(id) {
            self.progress_ms = 0;
        }
        self.active = Some(id.to_string());
        if let Some(e) = c::enemy_index(id) {
            self.foe_hp = c::ENEMIES[e].hp;
        }
    }

    pub fn stop(&mut self) {
        self.active = None;
        self.progress_ms = 0;
    }

    /// Credits `ms` of time (P4). Every action that completes inside it is done
    /// in order; time left over is progress toward the next one. Nothing in
    /// here depends on how the time is split, which `one_long_step_equals_many_short_ones` holds.
    pub fn advance(&mut self, ms: u64, r: &mut Report) {
        let mut left = ms;
        while let Some(id) = self.active.clone() {
            // An action that cannot start ends here, before its time is spent.
            if let Some(item) = self.missing(&id) {
                r.stop = Some(Stop::Missing(item));
                r.stopped_on = id.clone();
                self.next_step(r);
                continue;
            }
            let need = self.duration(&id).saturating_sub(self.progress_ms);
            if left < need {
                self.progress_ms += left;
                self.sim_ms += left;
                left = 0;
                break;
            }
            left -= need;
            self.sim_ms += need;
            self.progress_ms = 0;
            let skill = self.skill_of(&id);
            self.count(&format!("ms:{}", c::SKILLS[skill].id), self.duration(&id));
            match self.once(&id, r) {
                Ok(()) => {
                    r.actions += 1;
                    self.step_done(&id, r);
                }
                Err(stop) => {
                    r.stop = Some(stop);
                    r.stopped_on = id.clone();
                    self.next_step(r);
                }
            }
        }
        self.sim_ms += left;
    }

    fn step_done(&mut self, id: &str, r: &mut Report) {
        if let Some(step) = self.plan.first_mut() {
            if step.id == id {
                step.done += 1;
                if step.n > 0 && step.done >= step.n {
                    r.steps.push(id.to_string());
                    self.plan.remove(0);
                    self.follow_plan();
                }
            }
        }
    }

    fn next_step(&mut self, r: &mut Report) {
        if self.plan.is_empty() {
            self.stop();
            return;
        }
        r.steps.push(self.plan[0].id.clone());
        self.plan.remove(0);
        self.follow_plan();
    }

    fn follow_plan(&mut self) {
        match self.plan.first().map(|s| s.id.clone()) {
            Some(id) => self.begin(&id),
            None => self.stop(),
        }
    }

    /// One action or one swing.
    fn once(&mut self, id: &str, r: &mut Report) -> Result<(), Stop> {
        if let Some(e) = c::enemy_index(id) {
            return self.swing(e, r);
        }
        let Some(ai) = c::action_index(id) else {
            return Err(Stop::Missing(String::new()));
        };
        let a = &c::ACTIONS[ai];
        // `advance` checks the inputs before the wait starts (R3.6).
        r.m0.entry(a.id.to_string()).or_insert_with(|| self.mastery(a.id));
        let keep = self.keep_pct(a.id);
        if !a.needs.is_empty() && !(keep > 0 && self.rng.percent(keep)) {
            for (item, n) in a.needs {
                self.take(item, *n, r);
            }
        }
        let gained = self.xp_for(a);
        self.gain(a.skill, gained, r);
        let before = self.mastery(a.id);
        add(&mut self.mxp, a.id, a.xp);
        let after = self.mastery(a.id);
        for cp in c::CHECKPOINTS {
            if before < cp && after >= cp {
                r.checkpoints.push((a.id.to_string(), cp));
            }
        }
        let mut n = a.makes.1;
        if a.needs.is_empty() {
            if self.rng.percent(self.double_pct(a.id)) {
                n += a.makes.1;
            }
            if after >= 99 {
                n += 1;
            }
        }
        self.put(a.makes.0, n, r, false);
        if a.skill == c::WOODCUTTING && self.rng.one_in(c::NEST_ONE_IN) {
            self.put("bird_nest", 1, r, true);
        }
        Ok(())
    }

    fn swing(&mut self, e: usize, r: &mut Report) -> Result<(), Stop> {
        let enemy = &c::ENEMIES[e];
        let style = &c::STYLES[self.style];
        let (spent, _) = self.tier(style.spent);
        if !spent.is_empty() {
            self.take(&spent, 1, r);
        }
        if self.foe_hp == 0 {
            self.foe_hp = enemy.hp;
        }
        let strong = enemy.weak == style.skill;
        let hit_pct = if strong { c::HIT_PCT_STRONG } else { c::HIT_PCT };
        let max = self.max_hit(enemy);
        if self.rng.percent(hit_pct) {
            let dmg = (1 + self.rng.below(max)).min(self.foe_hp);
            self.foe_hp -= dmg;
            let m = self.mult_bp();
            self.gain(style.skill, dmg * c::STYLE_XP_TENTHS * m / 100_000, r);
            self.gain(c::HITPOINTS, dmg * c::HP_XP_THOUSANDTHS * m / 10_000_000, r);
        }
        if self.rng.percent(c::ENEMY_HIT_PCT) {
            let hit = self.rng.below(enemy.hit + 1);
            self.hp = self.hp.saturating_sub(hit);
        }
        let max_hp = self.max_hp();
        if self.hp * 100 < max_hp * c::EAT_BELOW_PCT {
            if let Some((food, heal)) = self.food() {
                self.take(food, 1, r);
                self.hp = (self.hp + heal).min(max_hp);
            }
        }
        if self.foe_hp == 0 {
            add(&mut self.kills, enemy.id, 1);
            add(&mut r.kills, enemy.id, 1);
            self.gold += enemy.gold;
            r.gold += enemy.gold;
            for (item, n, one_in) in enemy.drops {
                if self.rng.one_in(*one_in) {
                    self.put(item, *n, r, *one_in > 1);
                }
            }
            self.foe_hp = enemy.hp;
        }
        if self.hp <= enemy.hit && self.food().is_none() {
            self.hp = max_hp;
            self.foe_hp = enemy.hp;
            return Err(Stop::Food);
        }
        Ok(())
    }

    // ---------- the plan (P3) ----------

    pub fn plan_add(&mut self, id: &str) -> bool {
        if self.plan.len() >= c::PLAN_STEPS || !self.unlocked(id) {
            return false;
        }
        self.plan.push(Step {
            id: id.into(),
            n: 0,
            done: 0,
        });
        true
    }

    pub fn plan_count(&mut self, at: usize, n: u64) {
        if let Some(s) = self.plan.get_mut(at) {
            s.n = n;
        }
    }

    pub fn plan_remove(&mut self, at: usize) {
        if at < self.plan.len() {
            self.plan.remove(at);
        }
    }

    pub fn plan_start(&mut self) {
        for s in &mut self.plan {
            s.done = 0;
        }
        self.follow_plan();
    }

    // ---------- gold (P10) ----------

    pub fn sell(&mut self, item: &str) -> u64 {
        let got = c::value(item) * self.have(item);
        if got > 0 {
            self.bank.insert(item.to_string(), 0);
            self.gold += got;
        }
        got
    }

    pub fn buy(&mut self, line: usize) -> bool {
        let Some(&tier) = self.tools.get(line) else {
            return false;
        };
        let Some(&(cost, _)) = c::TOOL_TIERS.get(tier + 1) else {
            return false;
        };
        if self.gold < cost {
            return false;
        }
        self.gold -= cost;
        self.tools[line] = tier + 1;
        true
    }

    // ---------- prestige (P9) ----------

    pub fn can_prestige(&self) -> bool {
        self.total_level() >= self.prestige_needs()
    }

    /// Back to level 1 with a bigger multiplier, and nothing else from the game
    /// kept: mastery and kill counts start over too (T6, IG-Q18 B). The
    /// playtest log, flags and counts stay; they are about the player.
    pub fn do_prestige(&mut self, wall: u64) {
        let keep = self.clone();
        let mut s = State::new(wall);
        s.prestige = keep.prestige + 1;
        s.log = keep.log;
        s.flags = keep.flags;
        s.counts = keep.counts;
        s.first_wall = keep.first_wall;
        s.exported_wall = keep.exported_wall;
        s.sim_ms = keep.sim_ms;
        s.rng = keep.rng;
        s.screen = "woodcutting".into();
        *self = s;
        let p = self.prestige as u64;
        self.note(wall, "prestige", "", p);
    }
}

impl Report {
    pub fn begin(s: &State, elapsed: u64, capped: u64) -> Report {
        Report {
            elapsed,
            capped,
            lv0: (0..c::SKILLS.len()).map(|i| s.level(i)).collect(),
            xp: vec![0; c::SKILLS.len()],
            ..Report::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(s: &mut State, ms: u64) -> Report {
        let mut r = Report::begin(s, ms, 0);
        s.advance(ms, &mut r);
        r
    }

    #[test]
    fn a_new_game_starts_with_hitpoints_ten_and_full_health() {
        let s = State::new(5);
        assert_eq!(s.level(c::HITPOINTS), 10);
        assert_eq!(s.hp, 100);
        assert_eq!(s.total_level(), 21);
    }

    #[test]
    fn chopping_a_tree_for_thirty_seconds_gives_ten_logs_and_a_hundred_xp() {
        // tree: 3,000 ms, 10 XP, one log; 30 s is 10 chops. No mastery yet, so no doubles.
        let mut s = State::new(1);
        s.start("tree", 0);
        let r = run(&mut s, 30_000);
        assert_eq!(r.actions, 10);
        assert_eq!(s.xp[0], 100);
        assert!(s.have("logs") >= 10);
        assert_eq!(s.progress_ms, 0);
    }

    #[test]
    fn time_short_of_an_action_is_kept_as_progress() {
        let mut s = State::new(1);
        s.start("tree", 0);
        run(&mut s, 2_000);
        assert_eq!(s.xp[0], 0);
        assert_eq!(s.progress_ms, 2_000);
        run(&mut s, 1_000);
        assert_eq!(s.xp[0], 10);
    }

    #[test]
    fn one_long_step_equals_many_short_ones() {
        // IG-3: 8 hours at once equals 28,800 one-second steps, same seed.
        let mut a = State::new(99);
        a.bank.insert("logs".into(), 50);
        a.plan_add("burn_logs");
        a.plan_add("tree");
        a.plan_start();
        let mut b = a.clone();
        run(&mut a, 8 * 3_600_000);
        for _ in 0..28_800 {
            run(&mut b, 1_000);
        }
        assert_eq!(a, b);
    }

    #[test]
    fn a_level_unlocks_the_next_tree() {
        let mut s = State::new(1);
        assert!(!s.unlocked("oak"));
        s.xp[0] = xp::at(15);
        assert!(s.unlocked("oak"));
    }

    #[test]
    fn making_uses_its_inputs_and_stops_when_they_run_out() {
        let mut s = State::new(1);
        s.bank.insert("copper_ore".into(), 2);
        s.bank.insert("tin_ore".into(), 5);
        s.start("bronze_bar", 0);
        let r = run(&mut s, 30_000);
        assert_eq!(s.have("bronze_bar"), 2);
        assert_eq!(s.have("copper_ore"), 0);
        assert_eq!(s.have("tin_ore"), 3);
        assert_eq!(r.stop, Some(Stop::Missing("copper_ore".into())));
        assert_eq!(s.active, None);
    }

    #[test]
    fn a_plan_runs_its_steps_in_order() {
        // 3 chops (9 s), then burn those 3 logs (6 s), then chop again.
        let mut s = State::new(3);
        s.plan_add("tree");
        s.plan_add("burn_logs");
        s.plan_add("tree");
        s.plan_count(0, 3);
        s.plan_start();
        let r = run(&mut s, 9_000 + 6_000 + 2_000);
        assert_eq!(s.xp[c::FIREMAKING], 120);
        assert_eq!(r.steps, vec!["tree".to_string(), "burn_logs".to_string()]);
        assert_eq!(s.active.as_deref(), Some("tree"));
        // 17 s − 9 s of chops − 6 s of burns: the failed fourth burn costs nothing,
        // so the next chop is 2 s in.
        assert_eq!(s.progress_ms, 2_000);
    }

    #[test]
    fn picking_an_action_by_hand_clears_the_plan() {
        let mut s = State::new(3);
        s.plan_add("tree");
        s.plan_add("essence");
        s.plan_start();
        s.start("copper", 0);
        assert!(s.plan.is_empty());
    }

    #[test]
    fn burning_logs_gives_firemaking_xp_and_nothing_else() {
        // T4: a log burned is 2 s and 40 XP; the tree after it still gives 10.
        let mut s = State::new(4);
        s.bank.insert("logs".into(), 1);
        s.start("burn_logs", 0);
        run(&mut s, 2_000);
        assert_eq!(s.xp[c::FIREMAKING], 40);
        s.start("tree", 0);
        run(&mut s, 3_000);
        assert_eq!(s.xp[0], 10);
    }

    #[test]
    fn mastery_25_takes_two_tenths_of_a_second_off() {
        let mut s = State::new(1);
        assert_eq!(s.duration("tree"), 3_000);
        s.mxp.insert("tree".into(), xp::mastery_at(25));
        assert_eq!(s.duration("tree"), 2_800);
    }

    #[test]
    fn gathering_mastery_raises_the_double_chance() {
        let mut s = State::new(1);
        assert_eq!(s.double_pct("tree"), 0);
        s.mxp.insert("tree".into(), xp::mastery_at(10));
        assert_eq!(s.double_pct("tree"), 6);
        s.mxp.insert("tree".into(), xp::mastery_at(50));
        assert_eq!(s.double_pct("tree"), 20);
    }

    #[test]
    fn making_mastery_keeps_inputs_sometimes() {
        let mut s = State::new(1);
        s.mxp.insert("bronze_bar".into(), xp::mastery_at(50));
        assert_eq!(s.keep_pct("bronze_bar"), 10);
        s.mxp.insert("bronze_bar".into(), xp::mastery_at(99));
        assert_eq!(s.keep_pct("bronze_bar"), 20);
    }

    #[test]
    fn tools_make_their_skill_faster() {
        let mut s = State::new(1);
        s.gold = 4_500;
        assert!(s.buy(0));
        assert!(s.buy(0));
        assert_eq!(s.gold, 0);
        // steel axe: 10% faster; a tree takes 2,700 ms
        assert_eq!(s.duration("tree"), 2_700);
        assert_eq!(s.duration("copper"), 3_000);
        assert!(!s.buy(0));
    }

    #[test]
    fn selling_pays_the_value_of_the_stack() {
        let mut s = State::new(1);
        s.bank.insert("oak_logs".into(), 10);
        assert_eq!(s.sell("oak_logs"), 30);
        assert_eq!(s.gold, 30);
        assert_eq!(s.have("oak_logs"), 0);
    }

    #[test]
    fn a_fight_needs_its_styles_supplies() {
        let mut s = State::new(1);
        s.style = 0;
        assert_eq!(s.missing("chicken"), Some("bronze_sword".into()));
        s.bank.insert("bronze_sword".into(), 1);
        assert_eq!(s.missing("chicken"), None);
        s.style = 1;
        s.bank.insert("shortbow".into(), 1);
        assert_eq!(s.missing("chicken"), Some("bronze_arrows".into()));
    }

    #[test]
    fn fighting_chickens_with_a_sword_gives_kills_feathers_and_xp() {
        let mut s = State::new(11);
        s.bank.insert("bronze_sword".into(), 1);
        s.bank.insert("shrimp".into(), 100);
        s.start("chicken", 0);
        let r = run(&mut s, 10 * 60_000);
        assert!(s.kills["chicken"] > 10);
        assert!(s.have("feathers") >= 50);
        assert!(r.xp[c::MELEE] > 0 && r.xp[c::HITPOINTS] > 0);
        assert_eq!(s.active.as_deref(), Some("chicken"));
    }

    #[test]
    fn out_of_food_the_player_retreats_and_loses_nothing() {
        let mut s = State::new(12);
        s.bank.insert("bronze_sword".into(), 1);
        s.start("green_dragon", 0);
        let r = run(&mut s, 60 * 60_000);
        assert_eq!(r.stop, Some(Stop::Food));
        assert_eq!(s.active, None);
        assert_eq!(s.have("bronze_sword"), 1);
        assert_eq!(s.hp, s.max_hp());
    }

    #[test]
    fn the_right_style_hits_harder() {
        let mut s = State::new(1);
        s.xp[c::MAGIC] = xp::at(10);
        s.bank.insert("air_rune".into(), 5);
        s.style = 2;
        let goblin = &c::ENEMIES[c::enemy_index("goblin").unwrap()];
        let chicken = &c::ENEMIES[c::enemy_index("chicken").unwrap()];
        // 20 + 10·3 + air rune tier 1·10, +15 against the weak style
        assert_eq!(s.max_hit(chicken), 60);
        assert_eq!(s.max_hit(goblin), 75);
    }

    #[test]
    fn every_prestige_needs_600_and_keeps_only_the_multiplier() {
        let mut s = State::new(1);
        assert_eq!(s.prestige_needs(), 600);
        assert!(!s.can_prestige());
        for i in 0..c::SKILLS.len() {
            s.xp[i] = xp::at(50);
        }
        assert!(s.can_prestige());
        s.mxp.insert("tree".into(), 500);
        s.kills.insert("cow".into(), 3);
        s.gold = 99;
        s.do_prestige(10);
        assert_eq!(s.prestige, 1);
        assert_eq!(s.prestige_needs(), 600);
        assert_eq!(s.mult_bp(), 12_500);
        assert_eq!(s.level(0), 1);
        assert_eq!(s.gold, 0);
        assert!(s.mxp.is_empty());
        assert!(s.kills.is_empty());
        // a tree's 10 XP becomes 12 (12.5, rounded down)
        s.start("tree", 0);
        run(&mut s, 3_000);
        assert_eq!(s.xp[0], 12);
    }

    #[test]
    fn levels_reached_while_away_keep_the_game_time_they_came_at() {
        // Trees: 3 s and 10 XP. Level 2 needs 83 XP: the 9th chop, at 27 s.
        // Level 3 needs 174: the 18th chop, at 54 s.
        let mut s = State::new(1);
        s.start("tree", 0);
        let r = run(&mut s, 60_000);
        assert_eq!(r.levels[0], (0, 2, 27_000));
        assert_eq!(r.levels[1], (0, 3, 54_000));
    }

    #[test]
    fn the_log_keeps_its_newest_events() {
        let mut s = State::new(1);
        for i in 0..(c::LOG_CAP as u64 + 10) {
            s.note(i, "x", "", i);
        }
        assert_eq!(s.log.len(), c::LOG_CAP);
        assert_eq!(s.log.last().unwrap().n, c::LOG_CAP as u64 + 9);
    }
}
