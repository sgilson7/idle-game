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
/// 2 added Crafting, Farming and patches (T5); 3 added the four engines and
/// their skills (T10 to T14); 4 the regions, their engine tiers, twelve
/// rooms and dungeons (T15); 5 Performance, Fame, followers and advanced
/// plans (T18). An older save is brought up to it.
pub const SAVE_VERSION: u32 = 5;

/// One step of a plan. A plain step runs an action or an enemy id; in
/// advanced mode (T18, IG-Q53 A) a step can also stop on a condition
/// (`until`), go back (`repeat`) or jump on a condition (`if`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Step {
    pub id: String,
    /// How many to do; 0 means until the inputs run out, or until you come back.
    pub n: u64,
    pub done: u64,
    /// "" runs `id`; "repeat" and "if" are advanced steps.
    #[serde(default)]
    pub kind: String,
    /// A plain step ends early once this holds.
    #[serde(default)]
    pub until: Option<Cond>,
    /// An "if" step's test.
    #[serde(default)]
    pub cond: Option<Cond>,
    /// The "if" jumps when you have fewer than `cond`'s amount, not at least.
    #[serde(default)]
    pub below: bool,
    /// Where a "repeat" or an "if" goes: a step's index.
    #[serde(default)]
    pub to: usize,
    /// How many times a "repeat" goes back; 0 is forever.
    #[serde(default)]
    pub times: u64,
    /// How many times it has gone back this time through.
    #[serde(default)]
    pub used: u64,
}

/// Something a plan can test: an item's count, a skill's level, or gold.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cond {
    /// "item", "level" or "gold".
    pub kind: String,
    /// The item or skill id; empty for gold.
    pub id: String,
    pub n: u64,
}

/// A farming patch: empty, or a crop and the game time it was planted (T5).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Patch {
    pub crop: String,
    pub planted: u64,
    /// How long this crop takes, set at planting by compost and the harvest
    /// moon (T8, T13); 0 in an older save means the crop's own time.
    #[serde(default)]
    pub ms: u64,
}

/// A servant (T11): runs `job` at half speed alongside you.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Servant {
    pub job: String,
    pub progress: u64,
    /// No wage or no food at the last hour: the servant waits for the next.
    pub unpaid: bool,
}

/// A show booked at a venue (T18): it pays when the game clock reaches `back`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Show {
    pub venue: String,
    pub back: u64,
}

/// A follower (T18, IG-Q54 A): gathers alongside you and grows with work.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Follower {
    pub job: String,
    pub xp: u64,
    pub progress: u64,
}

/// One line of the combat log (T19): what happened in a swing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Blow {
    /// "hit", "miss", "hurt", "eat", "kill" or "drop".
    pub kind: String,
    /// The enemy, food or item it names.
    pub what: String,
    pub n: u64,
}

/// A caravan on the road (T12).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Caravan {
    pub town: usize,
    pub item: String,
    pub n: u64,
    /// Game time it comes back.
    pub back: u64,
    pub guards: bool,
}

/// Research in progress (T14).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Research {
    pub id: String,
    pub progress: u64,
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
    /// The step running now, when the plan runs (T18: steps stay, a pointer moves).
    #[serde(default)]
    pub plan_at: usize,
    #[serde(default)]
    pub plan_on: bool,
    /// Plan steps passed since an action last finished; a plan that goes
    /// round without doing anything stops (IG-Q53 A).
    #[serde(default)]
    pub plan_idle: u64,
    /// The plan screen's Advanced switch.
    #[serde(default)]
    pub plan_advanced: bool,
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
    #[serde(default)]
    pub patches: Vec<Patch>,
    // ---- the engines (T10 to T14); an older save has none of them ----
    /// Recipes found (T10). Kept through prestige (IG-Q26 A).
    #[serde(default)]
    pub recipes: Vec<String>,
    /// Game time the Goblin hot pot's extra gold lasts until.
    #[serde(default)]
    pub hot_pot_until: u64,
    /// Rank on each engine's ladder: Trader, Nobility, Inventor, Cosmic.
    #[serde(default)]
    pub ranks: [usize; 4],
    /// Each estate room's level, in `ROOMS` order (six before T15, twelve since).
    #[serde(default)]
    pub rooms: Vec<u64>,
    /// Each later region's rank on each engine ladder (T15).
    #[serde(default)]
    pub region_ranks: [[usize; 4]; 3],
    /// How far into the dungeon you are (T15).
    #[serde(default)]
    pub dungeon_stage: usize,
    #[serde(default)]
    pub servants: Vec<Servant>,
    #[serde(default)]
    pub caravans: Vec<Caravan>,
    /// Orders filled, as "window:town:slot".
    #[serde(default)]
    pub filled: Vec<String>,
    #[serde(default)]
    pub blueprints: Vec<String>,
    #[serde(default)]
    pub research: Option<Research>,
    /// Each machine's progress through its cycle.
    #[serde(default)]
    pub machine_ms: BTreeMap<String, u64>,
    /// Machines the player switched off.
    #[serde(default)]
    pub machines_off: Vec<String>,
    /// The last game hour the estate was paid for.
    #[serde(default)]
    pub upkeep_hour: u64,
    /// The app without the purchase: the regions past the Vale stay shut
    /// (IG-Q46 B). The api sets it from the device on every load; not saved.
    #[serde(skip)]
    pub sealed: bool,
    // ---- the bard (T18) ----
    /// Rank on each region's Fame ladder.
    #[serde(default)]
    pub fame: [usize; 4],
    #[serde(default)]
    pub show: Option<Show>,
    /// Kept through prestige; three at most.
    #[serde(default)]
    pub followers: Vec<Follower>,
    /// Game time each song's strength lasts until: "fight", "farm", "gather".
    #[serde(default)]
    pub buffs: BTreeMap<String, u64>,
    /// The newest lines of the combat log, oldest first (T19).
    #[serde(default)]
    pub blows: Vec<Blow>,
    /// Sounds and the phone's buzz (T18, IG-Q50 B); on unless turned off.
    #[serde(default = "on")]
    pub sound: bool,
    /// The app's purchase sheet has been shown at the Coast (T18).
    #[serde(default)]
    pub offered: bool,
    /// The moments the app has asked for a rating at, once each (T18).
    #[serde(default)]
    pub asked: Vec<String>,
    /// Weapon mastery (T23): each weapon's XP, by name, kept through prestige.
    #[serde(default)]
    pub weapon_xp: BTreeMap<String, u64>,
    /// The weapon chosen for each style, by the style's skill id.
    #[serde(default)]
    pub wield: BTreeMap<String, String>,
    /// The lists' filter and order (T25): 0 shows every region, r + 1 only
    /// region r; the order is "" (level, lowest first), "high" or "name".
    #[serde(default)]
    pub view_region: usize,
    #[serde(default)]
    pub view_sort: String,
}

fn on() -> bool {
    true
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
    /// Recipes found.
    pub recipes: Vec<String>,
    /// Caravans home: (town, gold, bandits met).
    pub caravans: Vec<(usize, u64, bool)>,
    /// Blueprints finished.
    pub research: Vec<String>,
    /// Gold the estate paid, and wages paid.
    pub income: u64,
    pub wages: u64,
    /// A servant went unpaid.
    pub unpaid: bool,
    /// Dungeon chests opened.
    pub chests: Vec<String>,
    /// Masterworks, sold as they were made, and the gold they brought.
    pub masterworks: u64,
    /// Shows that ended: (venue, gold, renown).
    pub shows: Vec<(String, u64, u64)>,
    /// The plan stopped at this step: it went round doing nothing.
    pub plan_stuck: Option<usize>,
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
            plan_at: 0,
            plan_on: false,
            plan_idle: 0,
            plan_advanced: false,
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
            patches: Vec::new(),
            recipes: Vec::new(),
            hot_pot_until: 0,
            ranks: [0; 4],
            rooms: vec![0; c::ROOMS.len()],
            region_ranks: [[0; 4]; 3],
            dungeon_stage: 0,
            servants: Vec::new(),
            caravans: Vec::new(),
            filled: Vec::new(),
            blueprints: Vec::new(),
            research: None,
            machine_ms: BTreeMap::new(),
            machines_off: Vec::new(),
            upkeep_hour: 0,
            sealed: false,
            fame: [0; 4],
            show: None,
            followers: Vec::new(),
            buffs: BTreeMap::new(),
            blows: Vec::new(),
            sound: true,
            offered: false,
            asked: Vec::new(),
            weapon_xp: BTreeMap::new(),
            wield: BTreeMap::new(),
            view_region: 0,
            view_sort: String::new(),
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

    /// A skill's mastery pool (T27): its actions' mastery levels added up,
    /// and the most they can add up to (every one at 99).
    pub fn pool(&self, skill: usize) -> (u64, u64) {
        let ids = c::skill_actions(skill);
        let have = ids.iter().map(|&i| self.mastery(c::ACTIONS[i].id) as u64).sum();
        (have, ids.len() as u64 * xp::MAX_LEVEL as u64)
    }

    /// The pool as a percentage, rounded down.
    pub fn pool_pct(&self, skill: usize) -> u64 {
        let (have, most) = self.pool(skill);
        have * 100 / most.max(1)
    }

    /// How many of the pool's five steps a skill has reached.
    pub fn pool_steps(&self, skill: usize) -> usize {
        let pct = self.pool_pct(skill);
        c::POOL_STEPS.iter().filter(|&&s| pct >= s).count()
    }

    /// Your highest mastery level among a skill's actions (T27).
    pub fn top_mastery(&self, skill: usize) -> u32 {
        c::skill_actions(skill)
            .iter()
            .map(|&i| self.mastery(c::ACTIONS[i].id))
            .max()
            .unwrap_or(1)
    }

    fn pool_luck(&self, id: &str) -> u64 {
        c::action_index(id)
            .filter(|&i| self.pool_steps(c::ACTIONS[i].skill) >= 3)
            .map_or(0, |_| c::POOL_LUCK_PCT)
    }

    pub fn have(&self, item: &str) -> u64 {
        self.bank.get(item).copied().unwrap_or(0)
    }

    /// Hitpoints from the level, the regions' armour and warding jewellery.
    pub fn max_hp(&self) -> u64 {
        self.level(c::HITPOINTS) as u64 * c::HP_PER_LEVEL
            + self.armour().2
            + self.trinket(c::Trinket::Warding)
    }

    /// The best enchanted jewellery of one kind you hold (Q40 A).
    pub fn trinket(&self, kind: c::Trinket) -> u64 {
        c::TRINKETS
            .iter()
            .filter(|(i, k, _)| *k == kind && self.have(i) > 0)
            .map(|(_, _, v)| *v)
            .max()
            .unwrap_or(0)
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
        if c::enemy_index(id).is_some() || c::dungeon_index(id).is_some() {
            return self.buffed("fight", c::STYLES[self.style].ms);
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
        if c::GADGETS
            .iter()
            .any(|&(g, sk)| sk == a.skill && self.have(g) > 0)
        {
            ms = ms * (100 - c::GADGET_PCT) / 100;
        }
        let room = self.room_speed_pct(a.skill);
        if room > 0 {
            ms = ms * (100 - room) / 100;
        }
        if matches!(a.skill, c::WOODCUTTING | c::MINING | c::FISHING) {
            ms = self.buffed("gather", ms);
        }
        if self.pool_steps(a.skill) >= 2 {
            ms = ms * (100 - c::POOL_FASTER_PCT) / 100;
        }
        ms
    }

    /// A song's strength (IG-Q57 A): 10% off while it lasts.
    pub fn buffed(&self, kind: &str, ms: u64) -> u64 {
        if self.buff_on(kind) {
            ms * (100 - c::BUFF_PCT) / 100
        } else {
            ms
        }
    }

    pub fn buff_on(&self, kind: &str) -> bool {
        self.buffs.get(kind).is_some_and(|&until| until > self.sim_ms)
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
        base + m / 10 + self.pool_luck(id)
    }

    pub fn keep_pct(&self, id: &str) -> u64 {
        let m = self.mastery(id);
        let base = if m >= 99 {
            c::KEEP_AT_99
        } else if m >= 50 {
            c::KEEP_AT_50
        } else {
            0
        };
        let needs = c::action_index(id).is_some_and(|i| !c::ACTIONS[i].needs.is_empty());
        base + if needs { self.pool_luck(id) } else { 0 }
    }

    /// Experience one action gives its skill, after prestige and the
    /// mastery-10 bonus (making only). Firemaking gives XP only (T4, IG-Q14 A).
    pub fn xp_for(&self, a: &c::Action) -> u64 {
        let kind = c::SKILLS[a.skill].kind;
        let mut pct = 100;
        if kind == Kind::Make && self.mastery(a.id) >= c::CHECKPOINTS[0] {
            pct += c::MAKE_XP_AT_10;
        }
        let steps = self.pool_steps(a.skill);
        if steps >= 1 {
            pct += c::POOL_XP_PCT;
        }
        if steps >= 4 {
            pct += c::POOL_XP_MORE_PCT;
        }
        a.xp * self.mult_bp() * pct / 1_000_000
    }

    /// The first missing input, if any.
    pub fn missing(&self, id: &str) -> Option<String> {
        if c::enemy_index(id).is_some() || c::dungeon_index(id).is_some() {
            let s = &c::STYLES[self.style];
            if !s.held_optional && !s.held.is_empty() && !s.held.iter().any(|(i, _)| self.have(i) > 0) {
                return Some(s.held[s.held.len() - 1].0.to_string());
            }
            if !s.spent.is_empty() && !s.spent.iter().any(|(i, _)| self.have(i) > 0) {
                return Some(s.spent[s.spent.len() - 1].0.to_string());
            }
            return None;
        }
        let a = &c::ACTIONS[c::action_index(id)?];
        if let Some(&(_, inst)) = c::SONG_INSTRUMENTS.iter().find(|(s, _)| *s == a.id) {
            if self.have(inst) == 0 {
                return Some(inst.to_string());
            }
        }
        a.needs
            .iter()
            .find(|(i, n)| self.have(i) < *n)
            .map(|(i, _)| i.to_string())
    }

    /// Whether an action or enemy can be picked: its level, its engine, and
    /// any recipe, blueprint or rank it needs.
    pub fn unlocked(&self, id: &str) -> bool {
        match c::action_index(id) {
            Some(i) => {
                let a = &c::ACTIONS[i];
                self.level(a.skill) >= a.level
                    && self.skill_open(a.skill)
                    && self.region_open(a.region)
                    && self.gate_open(a.id)
            }
            None => match (c::enemy_index(id), c::dungeon_index(id)) {
                (Some(e), _) => {
                    self.region_open(c::ENEMIES[e].region)
                        && (!c::is_champion(id) || self.engine_open(c::Engine::Nobility))
                }
                (_, Some(d)) => self.region_open(c::DUNGEONS[d].region),
                _ => false,
            },
        }
    }

    /// A recipe, blueprint or rank an action waits on (T10, T13, T14).
    pub fn gate_open(&self, id: &str) -> bool {
        if c::recipe_index(id).is_some() {
            return self.recipes.iter().any(|r| r == id);
        }
        if let Some(b) = c::blueprint_of_build(id).map(|b| &c::BLUEPRINTS[b]) {
            return self.blueprints.iter().any(|k| k == b.id) && self.have(b.id) == 0;
        }
        if id == "astral_runes" {
            return self.engine_open(c::Engine::Cosmic) && self.ranks[3] >= c::ASTRAL_RANK;
        }
        true
    }

    /// The skill an action or enemy trains.
    pub fn skill_of(&self, id: &str) -> usize {
        // enemies and dungeons train the style in use
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

    /// Adds a line to the combat log, keeping the newest `COMBAT_LOG`.
    fn blow(&mut self, kind: &str, what: &str, n: u64) {
        if self.blows.len() >= c::COMBAT_LOG {
            self.blows.remove(0);
        }
        self.blows.push(Blow {
            kind: kind.into(),
            what: what.into(),
            n,
        });
    }

    /// A style's max hit from its level alone (T19): what each level adds.
    pub fn level_hit(&self, skill: usize) -> u64 {
        c::MAX_HIT_BASE + self.level(skill) as u64 * c::MAX_HIT_PER_LEVEL
    }

    /// What a style's weapon in hand and its ammunition add to its max hit:
    /// their tiers, and the weapon's own level (T23).
    pub fn gear_hit(&self, style: usize) -> u64 {
        let s = &c::STYLES[style];
        let (weapon, tier) = self.weapon_of(style);
        (tier + self.tier(s.spent).1) * c::MAX_HIT_PER_TIER + self.weapon_hit(&weapon)
    }

    // ---------- weapon mastery (T23, IG-Q58 to IG-Q61, all A) ----------

    /// The weapon a style fights with: the one chosen, while you have it,
    /// or else your best. Empty when you hold none.
    pub fn weapon_of(&self, style: usize) -> (String, u64) {
        let s = &c::STYLES[style];
        self.wield
            .get(c::SKILLS[s.skill].id)
            .and_then(|w| s.held.iter().find(|(i, _)| *i == w.as_str() && self.have(i) > 0))
            .map(|(i, t)| (i.to_string(), *t))
            .unwrap_or_else(|| self.tier(s.held))
    }

    pub fn weapon_level(&self, item: &str) -> u32 {
        xp::level(self.weapon_xp.get(item).copied().unwrap_or(0))
    }

    /// Max hit a weapon's level adds: one every `WEAPON_HIT_EVERY` levels.
    pub fn weapon_hit(&self, item: &str) -> u64 {
        if item.is_empty() {
            return 0;
        }
        self.weapon_level(item) as u64 / c::WEAPON_HIT_EVERY
    }

    /// Chance to land a weapon's level adds, in percent.
    pub fn weapon_land(&self, item: &str) -> u64 {
        if item.is_empty() {
            return 0;
        }
        self.weapon_level(item) as u64 / c::WEAPON_LAND_EVERY
    }

    /// The gate (IG-Q60 A): how many levels a weapon's weaker weapons (the
    /// same style, a lower tier) must add up to for it to learn at full
    /// speed, and how many they add up to now. A style with few weapons
    /// (Magic has four staves) asks at most `WEAPON_GATE_EACH` per weaker
    /// kind, so its top staff can still reach full speed.
    pub fn weapon_gate(&self, style: usize, item: &str) -> (u64, u64) {
        let s = &c::STYLES[style];
        let tier = s.held.iter().find(|(i, _)| *i == item).map_or(0, |(_, t)| *t);
        let weaker: Vec<&str> = s
            .held
            .iter()
            .filter(|(_, t)| *t < tier)
            .map(|(i, _)| *i)
            .collect();
        let need = (tier * c::WEAPON_GATE_PER_TIER).min(weaker.len() as u64 * c::WEAPON_GATE_EACH);
        let have = weaker.iter().map(|i| self.weapon_level(i) as u64).sum();
        (need, have)
    }

    /// A weapon's XP speed in percent: full once the gate is met, never
    /// below `WEAPON_RATE_MIN`; full for a style's weakest weapons.
    pub fn weapon_rate(&self, style: usize, item: &str) -> u64 {
        let (need, have) = self.weapon_gate(style, item);
        if need == 0 {
            return 100;
        }
        (have * 100 / need).clamp(c::WEAPON_RATE_MIN, 100)
    }

    /// Chooses the weapon a style fights with (IG-Q58 A).
    pub fn wield_weapon(&mut self, item: &str) -> bool {
        let Some(k) = c::STYLES
            .iter()
            .position(|s| s.held.iter().any(|(i, _)| *i == item))
        else {
            return false;
        };
        if self.have(item) == 0 {
            return false;
        }
        self.wield
            .insert(c::SKILLS[c::STYLES[k].skill].id.to_string(), item.to_string());
        true
    }

    pub fn max_hit(&self, enemy: &c::Enemy) -> u64 {
        let s = &c::STYLES[self.style];
        let (weapon, tier) = self.weapon_of(self.style);
        let tiers = tier + self.tier(s.spent).1;
        let strong = if enemy.weak == s.skill {
            c::MAX_HIT_STRONG
        } else {
            0
        };
        let hit = c::MAX_HIT_BASE
            + self.level(s.skill) as u64 * c::MAX_HIT_PER_LEVEL
            + tiers * c::MAX_HIT_PER_TIER
            + self.weapon_hit(&weapon)
            + strong;
        hit * (100 + self.trinket(c::Trinket::Power)) / 100
    }

    /// The best armour in the bank: its tier, and the hitpoints the regions' armour adds (T5, T15).
    pub fn armour(&self) -> (&'static str, u64, u64) {
        c::ARMOUR
            .iter()
            .copied()
            .find(|(i, _, _)| self.have(i) > 0)
            .unwrap_or(("", 0, 0))
    }

    /// The enemy you face now: the one picked, or the dungeon's current stage.
    pub fn foe(&self, id: &str) -> Option<usize> {
        match c::dungeon_index(id) {
            Some(d) => {
                let st = c::DUNGEONS[d].stages;
                c::enemy_index(st[self.dungeon_stage.min(st.len() - 1)])
            }
            None => c::enemy_index(id),
        }
    }

    // ---------- farming (T5) ----------

    /// Patches open: by Farming level, and one more for each Garden level (T11).
    pub fn patches_open(&self) -> usize {
        let l = self.level(c::FARMING);
        c::PATCH_LEVELS.iter().filter(|&&need| l >= need).count()
            + c::PATCH_PER_REGION * self.regions_open().saturating_sub(1)
            + self.rooms_of(c::RoomEffect::Patch) as usize
    }

    /// The patch at `i` as it stands (patches not yet stored are empty).
    pub fn patch(&self, i: usize) -> Patch {
        self.patches.get(i).cloned().unwrap_or_default()
    }

    /// Game time left before the crop in patch `i` is ready; `None` when empty.
    pub fn grow_left(&self, i: usize) -> Option<u64> {
        let p = self.patch(i);
        let crop = &c::CROPS[c::crop_index(&p.crop)?];
        let ms = if p.ms > 0 { p.ms } else { crop.grow_ms };
        Some((p.planted + ms).saturating_sub(self.sim_ms))
    }

    /// How long `crop` would take if planted now: compost (T8) and the
    /// harvest moon (T13) each take a quarter off.
    pub fn grow_ms(&self, crop: &c::Crop) -> u64 {
        let mut ms = crop.grow_ms;
        match self.compost() {
            Some("fine_ashes") => ms = ms * c::FINE_COMPOST_GROW_PCT / 100,
            Some(_) => ms = ms * c::COMPOST_GROW_PCT / 100,
            None => {}
        }
        // a harvest moon in each sky that shows Farming
        for _ in 0..self.signs_for(c::FARMING) {
            ms = ms * c::HARVEST_MOON_PCT / 100;
        }
        self.buffed("farm", ms)
    }

    /// The compost planting would use: fine ashes first (T15), then ashes.
    pub fn compost(&self) -> Option<&'static str> {
        ["fine_ashes", "ashes"].into_iter().find(|a| self.have(a) > 0)
    }

    pub fn ready(&self, i: usize) -> bool {
        self.grow_left(i) == Some(0)
    }

    /// Plants a seed of `crop` in the first empty open patch.
    pub fn plant(&mut self, crop: &str, r: &mut Report) -> bool {
        let Some(ci) = c::crop_index(crop) else {
            return false;
        };
        let cr = &c::CROPS[ci];
        let Some(slot) = (0..self.patches_open()).find(|&i| self.patch(i).crop.is_empty()) else {
            return false;
        };
        if self.level(c::FARMING) < cr.level || self.have(cr.seed) == 0 || !self.region_open(cr.region) {
            return false;
        }
        let ms = self.grow_ms(cr);
        self.take(cr.seed, 1, r);
        if let Some(a) = self.compost() {
            self.take(a, 1, r);
        }
        if self.patches.len() <= slot {
            self.patches.resize(slot + 1, Patch::default());
        }
        self.patches[slot] = Patch {
            crop: cr.id.into(),
            planted: self.sim_ms,
            ms,
        };
        let xp = cr.xp_plant * self.mult_bp() / 10_000;
        self.gain(c::FARMING, xp, r);
        true
    }

    /// Harvests patch `i` if its crop is ready: three to six of it, and XP for each.
    pub fn harvest(&mut self, i: usize, r: &mut Report) -> u64 {
        if !self.ready(i) {
            return 0;
        }
        let cr = &c::CROPS[c::crop_index(&self.patch(i).crop).unwrap_or(0)];
        let n = cr.yield_min + self.rng.below(cr.yield_max - cr.yield_min + 1);
        self.put(cr.produce, n, r, false);
        let xp = cr.xp_each * n * self.mult_bp() / 10_000;
        self.gain(c::FARMING, xp, r);
        self.patches[i] = Patch::default();
        n
    }

    pub fn harvest_all(&mut self, r: &mut Report) -> u64 {
        (0..self.patches.len()).map(|i| self.harvest(i, r)).sum()
    }

    pub fn food(&self) -> Option<(&'static str, u64)> {
        c::FOOD.iter().copied().find(|(i, _)| self.have(i) > 0)
    }

    /// One action's gathering finds and makers' luck (T15, Q40 A): rare trees,
    /// gems, treasure, masterworks and perfect dishes. Your own work only.
    fn luck(&mut self, a: &c::Action, n: &mut u64, r: &mut Report) {
        if !self.mechanics_open() {
            return;
        }
        match a.skill {
            c::WOODCUTTING if self.rare_tree() == Some(a.id) => *n *= 2,
            c::MINING if self.rng.one_in(c::GEM_ONE_IN) => {
                let gem = c::GEMS
                    .iter()
                    .rev()
                    .find(|(_, at)| a.level >= *at)
                    .map(|(g, _)| *g)
                    .unwrap_or("sapphire");
                self.put(gem, 1, r, true);
            }
            c::FISHING if self.rng.one_in(c::TREASURE_ONE_IN) => self.treasure(a.level, r),
            c::SMITHING if self.rng.one_in(c::MASTERWORK_ONE_IN) => {
                let gold = c::value(a.makes.0) * *n * c::MASTERWORK_TIMES;
                self.gold += gold;
                r.gold += gold;
                r.masterworks += gold;
            }
            c::COOKING if self.rng.one_in(c::PERFECT_ONE_IN) => *n += 1,
            _ => {}
        }
    }

    /// Treasure from the water: seeds, runes or gold, by a roll.
    fn treasure(&mut self, level: u32, r: &mut Report) {
        match self.rng.below(3) {
            0 => {
                let open: Vec<&'static str> = c::CROPS
                    .iter()
                    .filter(|cr| self.region_open(cr.region))
                    .map(|cr| cr.seed)
                    .collect();
                let seed = open[self.rng.below(open.len() as u64) as usize];
                self.put(seed, 5, r, true);
            }
            1 => {
                let runes = ["air_rune", "water_rune", "fire_rune", "chaos_rune", "death_rune"];
                let rune = runes[self.rng.below(runes.len() as u64) as usize];
                self.put(rune, 30, r, true);
            }
            _ => {
                let gold = 10 * level as u64;
                self.gold += gold;
                r.gold += gold;
            }
        }
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

    pub(crate) fn take(&mut self, item: &str, n: u64, r: &mut Report) {
        if n == 0 {
            return;
        }
        let have = self.have(item);
        self.bank.insert(item.to_string(), have.saturating_sub(n));
        add(&mut r.used, item, n);
    }

    pub(crate) fn put(&mut self, item: &str, n: u64, r: &mut Report, rare: bool) {
        if n == 0 || item.is_empty() {
            return;
        }
        add(&mut self.bank, item, n);
        add(if rare { &mut r.rare } else { &mut r.items }, item, n);
    }

    pub(crate) fn gain(&mut self, skill: usize, xp: u64, r: &mut Report) {
        let before = xp::level(self.xp[skill]);
        self.xp[skill] += xp;
        r.xp[skill] += xp;
        for l in (before + 1)..=xp::level(self.xp[skill]) {
            r.levels.push((skill, l, self.sim_ms));
        }
    }

    /// Starts an action or a fight by hand. A running plan stops and keeps
    /// its steps (P3; cleared before T18).
    pub fn start(&mut self, id: &str, wall: u64) {
        self.plan_on = false;
        self.begin(id);
        self.note(wall, "start", id, 0);
    }

    fn begin(&mut self, id: &str) {
        if self.active.as_deref() != Some(id) {
            self.progress_ms = 0;
            if c::dungeon_index(id).is_some() {
                self.dungeon_stage = 0;
            }
        }
        self.active = Some(id.to_string());
        if let Some(e) = self.foe(id) {
            self.foe_hp = c::ENEMIES[e].hp;
        }
    }

    pub fn stop(&mut self) {
        self.active = None;
        self.progress_ms = 0;
    }

    /// Credits `ms` of time (P4). Every action that completes inside it is done
    /// in order; time left over is progress toward the next one. The engines'
    /// timers (servants, machines, research, caravans, the estate's hours) run
    /// alongside: the loop moves to whichever comes first, does it, and goes
    /// on, so nothing depends on how the time is split, which
    /// `one_long_step_equals_many_short_ones` holds.
    pub fn advance(&mut self, ms: u64, r: &mut Report) {
        let mut left = ms;
        loop {
            self.settle(r);
            let timers = self.timers();
            let dt = self.next_event_with(&timers).min(left);
            self.pass(dt, &timers);
            left -= dt;
            self.fire(r);
            if left == 0 {
                self.settle(r);
                break;
            }
        }
    }

    /// An action that cannot start ends here, before its time is spent (R3.6).
    fn settle(&mut self, r: &mut Report) {
        while let Some(id) = self.active.clone() {
            let Some(item) = self.missing(&id) else {
                break;
            };
            r.stop = Some(Stop::Missing(item));
            r.stopped_on = id.clone();
            self.next_step(r);
        }
    }

    /// What every timer needs, worked out once for a step of the loop.
    pub(crate) fn timers(&self) -> crate::engines::Timers {
        let open = self.open_now();
        crate::engines::Timers {
            servants: (0..self.servants.len())
                .map(|i| self.servant_need_in(i, &open))
                .collect(),
            machines: c::MACHINES
                .iter()
                .map(|m| self.machine_need_in(m, &open))
                .collect(),
            followers: (0..self.followers.len())
                .map(|i| self.follower_need_in(i, &open))
                .collect(),
        }
    }

    /// Game time until the next thing completes.
    pub fn next_event(&self) -> u64 {
        self.next_event_with(&self.timers())
    }

    fn next_event_with(&self, t: &crate::engines::Timers) -> u64 {
        let mut next = u64::MAX;
        if let Some(id) = &self.active {
            next = next.min(self.duration(id).saturating_sub(self.progress_ms));
        }
        for (i, need) in t.servants.iter().enumerate() {
            if let Some(need) = need {
                next = next.min(need.saturating_sub(self.servants[i].progress));
            }
        }
        for (m, need) in c::MACHINES.iter().zip(&t.machines) {
            if let Some(need) = need {
                next = next.min(need.saturating_sub(self.machine_ms.get(m.id).copied().unwrap_or(0)));
            }
        }
        if let Some(need) = self.research_need() {
            next = next.min(need);
        }
        for cv in &self.caravans {
            next = next.min(cv.back.saturating_sub(self.sim_ms));
        }
        for (i, need) in t.followers.iter().enumerate() {
            if let Some(need) = need {
                next = next.min(need.saturating_sub(self.followers[i].progress));
            }
        }
        if let Some(sh) = &self.show {
            next = next.min(sh.back.saturating_sub(self.sim_ms));
        }
        for &until in self.buffs.values() {
            if until > self.sim_ms {
                next = next.min(until - self.sim_ms);
            }
        }
        if self.engine_open(c::Engine::Nobility) {
            next = next
                .min((self.upkeep_hour + 1) * c::HOUR - self.sim_ms.min((self.upkeep_hour + 1) * c::HOUR));
        }
        next
    }

    /// Moves every running timer on by `dt`.
    fn pass(&mut self, dt: u64, t: &crate::engines::Timers) {
        self.sim_ms += dt;
        if self.active.is_some() {
            self.progress_ms += dt;
        }
        for (i, need) in t.servants.iter().enumerate() {
            if need.is_some() {
                self.servants[i].progress += dt;
            }
        }
        for (m, need) in c::MACHINES.iter().zip(&t.machines) {
            if need.is_some() {
                *self.machine_ms.entry(m.id.to_string()).or_insert(0) += dt;
            }
        }
        for (i, need) in t.followers.iter().enumerate() {
            if need.is_some() {
                self.followers[i].progress += dt;
            }
        }
        if self.research_need().is_some() {
            if let Some(rs) = self.research.as_mut() {
                rs.progress += dt;
            }
        }
    }

    /// Does whatever is due now, in a fixed order: your action, servants,
    /// machines, research, caravans, then the estate's hour.
    fn fire(&mut self, r: &mut Report) {
        if let Some(id) = self.active.clone() {
            let ms = self.duration(&id);
            if self.progress_ms >= ms {
                self.progress_ms = 0;
                let skill = self.skill_of(&id);
                self.count(&format!("ms:{}", c::SKILLS[skill].id), ms);
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
        }
        self.fire_engines(r);
    }

    fn step_done(&mut self, id: &str, r: &mut Report) {
        self.plan_idle = 0;
        if !self.plan_on {
            return;
        }
        let at = self.plan_at;
        let Some(step) = self.plan.get_mut(at) else {
            return;
        };
        if step.id != id {
            return;
        }
        step.done += 1;
        let counted = step.n > 0 && step.done >= step.n;
        let until = step.until.clone();
        if counted || until.is_some_and(|u| self.holds(&u)) {
            r.steps.push(id.to_string());
            self.plan_at += 1;
            self.follow_plan(r);
        }
    }

    /// The running step cannot go on: the plan moves to the next, or the
    /// action stops when no plan runs.
    fn next_step(&mut self, r: &mut Report) {
        if !self.plan_on {
            self.stop();
            return;
        }
        if let Some(st) = self.plan.get(self.plan_at) {
            r.steps.push(st.id.clone());
        }
        self.plan_at += 1;
        self.follow_plan(r);
    }

    /// Whether a plan's condition holds now (at least its amount).
    pub fn holds(&self, cd: &Cond) -> bool {
        self.cond_value(cd) >= cd.n
    }

    pub fn cond_value(&self, cd: &Cond) -> u64 {
        match cd.kind.as_str() {
            "item" => self.have(&cd.id),
            "level" => c::skill_index(&cd.id).map_or(0, |i| self.level(i) as u64),
            _ => self.gold,
        }
    }

    /// Runs the plan from `plan_at`: control steps are followed at once, a
    /// plain step whose `until` already holds is passed, and the first step
    /// with something to do begins. The plan ends after its last step. A
    /// plan that passes more steps than twice its length without an action
    /// finishing goes round doing nothing, so it stops.
    fn follow_plan(&mut self, r: &mut Report) {
        while self.plan_on {
            let at = self.plan_at;
            let Some(st) = self.plan.get(at).cloned() else {
                break;
            };
            self.plan_idle += 1;
            if self.plan_idle > 2 * self.plan.len() as u64 + 1 {
                r.plan_stuck = Some(at);
                break;
            }
            match st.kind.as_str() {
                "repeat" => {
                    if st.to < at && (st.times == 0 || st.used < st.times) {
                        self.plan[at].used += 1;
                        for s in &mut self.plan[st.to..at] {
                            s.done = 0;
                            if s.kind == "repeat" {
                                s.used = 0;
                            }
                        }
                        self.plan_at = st.to;
                    } else {
                        self.plan[at].used = 0;
                        self.plan_at += 1;
                    }
                }
                "if" => {
                    let jump = st.cond.as_ref().is_some_and(|cd| self.holds(cd) != st.below);
                    self.plan_at = if jump && st.to < self.plan.len() {
                        st.to
                    } else {
                        at + 1
                    };
                }
                _ => {
                    if st.until.as_ref().is_some_and(|u| self.holds(u)) || !self.unlocked(&st.id) {
                        self.plan_at += 1;
                    } else {
                        self.begin(&st.id);
                        return;
                    }
                }
            }
        }
        self.plan_on = false;
        self.plan_at = 0;
        self.stop();
    }

    /// One action or one swing.
    fn once(&mut self, id: &str, r: &mut Report) -> Result<(), Stop> {
        if let Some(d) = c::dungeon_index(id) {
            return self.dungeon_swing(d, r);
        }
        if let Some(e) = c::enemy_index(id) {
            return self.swing(e, r).map(|_| ());
        }
        let Some(ai) = c::action_index(id) else {
            return Err(Stop::Missing(String::new()));
        };
        let a = &c::ACTIONS[ai];
        // `advance` checks the inputs before the wait starts (R3.6).
        r.m0.entry(a.id.to_string()).or_insert_with(|| self.mastery(a.id));
        self.produce(ai, true, r);
        Ok(())
    }

    /// One action's inputs, XP and goods. A servant's work (`yours` false)
    /// trains no mastery and gathers no starlight.
    pub(crate) fn produce(&mut self, ai: usize, yours: bool, r: &mut Report) {
        let a = &c::ACTIONS[ai];
        let keep = self.keep_pct(a.id);
        if !a.needs.is_empty() && !(keep > 0 && self.rng.percent(keep)) {
            for (item, n) in a.needs {
                self.take(item, *n, r);
            }
        }
        let gained = self.xp_for(a);
        self.gain(a.skill, gained, r);
        if yours {
            if let Some(&(_, kind)) = c::SONG_BUFFS.iter().find(|(s, _)| *s == a.id) {
                self.buffs.insert(kind.to_string(), self.sim_ms + c::BUFF_MS);
            }
        }
        let before = self.mastery(a.id);
        if yours {
            // Your highest mastery in the skill speeds every action's (T27).
            let m = a.xp * (100 + self.top_mastery(a.skill) as u64) / 100;
            add(&mut self.mxp, a.id, m);
        }
        let after = self.mastery(a.id);
        for cp in c::CHECKPOINTS {
            if before < cp && after >= cp {
                r.checkpoints.push((a.id.to_string(), cp));
            }
        }
        let mut n = a.makes.1;
        if yours {
            self.luck(a, &mut n, r);
        }
        let made = if a.makes.0 == "ashes" && a.level >= c::FINE_ASH_LEVEL && self.mechanics_open() {
            "fine_ashes"
        } else {
            a.makes.0
        };
        if a.id.starts_with("take_apart") {
            n += c::INVENTOR_RANK_PARTS * self.rank_sum(c::Engine::Inventor) as u64;
            if self.rng.one_in(c::FINE_PARTS_ONE_IN) {
                self.put("fine_parts", 1, r, true);
            }
        }
        if a.needs.is_empty() {
            if self.rng.percent(self.double_pct(a.id)) {
                n += a.makes.1;
            }
            if after >= 99 {
                n += 1;
            }
        }
        if self.pool_steps(a.skill) >= 5 {
            n += 1;
        }
        self.put(made, n, r, false);
        if a.skill == c::WOODCUTTING && self.rng.one_in(c::NEST_ONE_IN) {
            self.put("bird_nest", 1, r, true);
        }
        if a.skill == c::MINING && self.engine_open(c::Engine::Cosmic) && self.rng.one_in(c::STARDUST_ONE_IN)
        {
            self.put("stardust", 1, r, true);
        }
        if yours {
            self.starlight(a.skill, r);
        }
    }

    /// One swing in a dungeon: the stage's enemy, then the next; a chest at the end.
    fn dungeon_swing(&mut self, d: usize, r: &mut Report) -> Result<(), Stop> {
        let dg = &c::DUNGEONS[d];
        let stage = self.dungeon_stage.min(dg.stages.len() - 1);
        let e = c::enemy_index(dg.stages[stage]).unwrap_or(0);
        match self.swing(e, r) {
            Ok(true) => {
                self.dungeon_stage = stage + 1;
                if self.dungeon_stage >= dg.stages.len() {
                    self.dungeon_stage = 0;
                    self.chest(d, r);
                }
                if let Some(next) = self.foe(dg.id) {
                    self.foe_hp = c::ENEMIES[next].hp;
                }
                Ok(())
            }
            Ok(false) => Ok(()),
            Err(stop) => {
                self.dungeon_stage = 0;
                Err(stop)
            }
        }
    }

    /// A dungeon's chest: twice the run's gold, fine parts, and a chance of a recipe.
    fn chest(&mut self, d: usize, r: &mut Report) {
        let dg = &c::DUNGEONS[d];
        let gold: u64 = dg
            .stages
            .iter()
            .filter_map(|s| c::enemy_index(s))
            .map(|e| c::ENEMIES[e].gold)
            .sum::<u64>()
            * c::CHEST_GOLD_TIMES;
        self.gold += gold;
        r.gold += gold;
        self.put("fine_parts", c::CHEST_FINE_PARTS, r, true);
        if self.rng.one_in(c::CHEST_RECIPE_ONE_IN) {
            let mine: Vec<&c::Recipe> = c::RECIPES.iter().filter(|rc| rc.region == dg.region).collect();
            let rc = mine[self.rng.below(mine.len() as u64) as usize];
            self.find_recipe(rc, r);
        }
        r.chests.push(dg.id.to_string());
    }

    /// One swing at enemy `e`; `Ok(true)` when it falls.
    fn swing(&mut self, e: usize, r: &mut Report) -> Result<bool, Stop> {
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
        let (weapon, _) = self.weapon_of(self.style);
        let base_pct = if strong { c::HIT_PCT_STRONG } else { c::HIT_PCT };
        let hit_pct = (base_pct + self.weapon_land(&weapon)).min(100);
        let max = self.max_hit(enemy);
        if self.rng.percent(hit_pct) {
            let dmg = (1 + self.rng.below(max)).min(self.foe_hp);
            self.foe_hp -= dmg;
            let m = self.mult_bp();
            let xp = dmg * c::STYLE_XP_TENTHS * m / 100_000;
            if weapon.is_empty() {
                // Magic without a staff: the style learns directly.
                self.gain(style.skill, xp, r);
            } else {
                // The weapon learns twice the hit's XP at its gated speed,
                // and the style gets half of it (IG-Q61 A): at full speed,
                // the style learns as it did before T23.
                let w = 2 * xp * self.weapon_rate(self.style, &weapon) / 100;
                add(&mut self.weapon_xp, &weapon, w);
                self.gain(style.skill, w / 2, r);
            }
            self.gain(c::HITPOINTS, dmg * c::HP_XP_THOUSANDTHS * m / 10_000_000, r);
            self.blow("hit", enemy.id, dmg);
        } else {
            self.blow("miss", enemy.id, 0);
        }
        if self.rng.percent(c::ENEMY_HIT_PCT) {
            let hit = self.rng.below(enemy.hit + 1) * (100 - self.armour().1 * c::ARMOUR_PCT_PER_TIER) / 100;
            self.hp = self.hp.saturating_sub(hit);
            self.blow("hurt", enemy.id, hit);
        }
        let max_hp = self.max_hp();
        if self.hp * 100 < max_hp * c::EAT_BELOW_PCT {
            if let Some((food, heal)) = self.food() {
                self.take(food, 1, r);
                let mut heal = heal * (100 + self.trinket(c::Trinket::Vigour)) / 100;
                if self.have("spices") > 0 {
                    self.take("spices", 1, r);
                    heal = heal * (100 + c::SPICE_HEAL_PCT) / 100;
                }
                if food == "goblin_hot_pot" {
                    self.hot_pot_until = self.sim_ms + c::HOT_POT_MS;
                }
                self.hp = (self.hp + heal).min(max_hp);
                self.blow("eat", food, heal);
            }
        }
        if self.foe_hp == 0 {
            add(&mut self.kills, enemy.id, 1);
            add(&mut r.kills, enemy.id, 1);
            let mut gold = if self.sim_ms < self.hot_pot_until {
                enemy.gold * (100 + c::HOT_POT_GOLD_PCT) / 100
            } else {
                enemy.gold
            };
            gold = gold * (100 + self.trinket(c::Trinket::Wealth)) / 100;
            self.gold += gold;
            r.gold += gold;
            self.blow("kill", enemy.id, gold);
            for (item, n, one_in) in enemy.drops {
                if self.rng.one_in(*one_in) {
                    self.put(item, *n, r, *one_in > 1);
                    self.blow("drop", item, *n);
                }
            }
            for rc in c::RECIPES.iter().filter(|rc| rc.enemy == enemy.id) {
                if self.rng.one_in(rc.one_in) {
                    self.find_recipe(rc, r);
                }
            }
            self.starlight(style.skill, r);
            self.foe_hp = enemy.hp;
            if self.hp <= enemy.hit && self.food().is_none() {
                self.hp = max_hp;
                return Err(Stop::Food);
            }
            return Ok(true);
        }
        if self.hp <= enemy.hit && self.food().is_none() {
            self.hp = max_hp;
            self.foe_hp = enemy.hp;
            return Err(Stop::Food);
        }
        Ok(false)
    }

    // ---------- the plan (P3) ----------

    /// How many steps a plan may hold: more in advanced mode (IG-Q53 A).
    pub fn plan_max(&self) -> usize {
        if self.plan_advanced {
            c::PLAN_STEPS_ADVANCED
        } else {
            c::PLAN_STEPS
        }
    }

    pub fn plan_add(&mut self, id: &str) -> bool {
        if self.plan.len() >= self.plan_max() || !self.unlocked(id) {
            return false;
        }
        self.plan.push(Step {
            id: id.into(),
            ..Step::default()
        });
        true
    }

    /// Adds a "repeat" (back to the first step, forever) or an "if" (on
    /// gold, to the first step) in advanced mode; the player then edits it.
    pub fn plan_add_control(&mut self, kind: &str) -> bool {
        if !self.plan_advanced || self.plan.len() >= self.plan_max() || !matches!(kind, "repeat" | "if") {
            return false;
        }
        self.plan.push(Step {
            kind: kind.into(),
            cond: (kind == "if").then(|| Cond {
                kind: "gold".into(),
                id: String::new(),
                n: c::PLAN_AMOUNTS[0],
            }),
            below: kind == "if",
            ..Step::default()
        });
        true
    }

    /// The Advanced switch. It turns off only when no step uses it.
    pub fn plan_set_advanced(&mut self, on: bool) -> bool {
        if !on && (self.plan_uses_advanced() || self.plan.len() > c::PLAN_STEPS) {
            return false;
        }
        self.plan_advanced = on;
        true
    }

    pub fn plan_uses_advanced(&self) -> bool {
        self.plan.iter().any(|s| !s.kind.is_empty() || s.until.is_some())
    }

    pub fn plan_count(&mut self, at: usize, n: u64) {
        if let Some(s) = self.plan.get_mut(at) {
            s.n = n;
        }
    }

    /// Removes a step; jumps past it move up one, and jumps to it go to
    /// the step that takes its place.
    pub fn plan_remove(&mut self, at: usize) {
        if at >= self.plan.len() {
            return;
        }
        self.plan.remove(at);
        for s in &mut self.plan {
            if s.to > at {
                s.to -= 1;
            }
        }
        if self.plan_on {
            self.plan_on = false;
            self.stop();
        }
        self.plan_at = 0;
    }

    /// Moves a step to another place in the plan, as a dragged block does
    /// (T24). Jumps keep pointing at the steps they pointed at.
    pub fn plan_move(&mut self, from: usize, to: usize) -> bool {
        let len = self.plan.len();
        if from >= len || to >= len || from == to {
            return false;
        }
        let mut order: Vec<usize> = (0..len).collect();
        let moved = order.remove(from);
        order.insert(to, moved);
        let mut place = vec![0; len];
        for (new, &old) in order.iter().enumerate() {
            place[old] = new;
        }
        let mut steps: Vec<Step> = order.iter().map(|&old| self.plan[old].clone()).collect();
        for s in &mut steps {
            if !s.kind.is_empty() {
                s.to = place[s.to.min(len - 1)];
            }
        }
        self.plan = steps;
        self.plan_halt();
        true
    }

    /// Puts a new "repeat" or "if" at a place in the plan, as a block
    /// dragged from the palette (T24); jumps past it move down one.
    pub fn plan_insert(&mut self, kind: &str, at: usize) -> bool {
        let at = at.min(self.plan.len());
        if !self.plan_add_control(kind) {
            return false;
        }
        let new = self.plan.pop().unwrap_or_default();
        for s in &mut self.plan {
            if !s.kind.is_empty() && s.to >= at {
                s.to += 1;
            }
        }
        self.plan.insert(at, new);
        self.plan_halt();
        true
    }

    /// A changed plan starts again from its first step when next started.
    fn plan_halt(&mut self) {
        if self.plan_on {
            self.plan_on = false;
            self.stop();
        }
        self.plan_at = 0;
    }

    pub fn plan_clear(&mut self) {
        self.plan.clear();
        self.plan_at = 0;
        if self.plan_on {
            self.plan_on = false;
            self.stop();
        }
    }

    pub fn plan_start(&mut self) {
        let mut r = Report::default();
        self.plan_start_with(&mut r);
    }

    pub fn plan_start_with(&mut self, r: &mut Report) {
        for s in &mut self.plan {
            s.done = 0;
            s.used = 0;
        }
        self.plan_at = 0;
        self.plan_idle = 0;
        self.plan_on = !self.plan.is_empty();
        self.follow_plan(r);
    }

    /// Edits a step: the amount, the until, a jump's target, a repeat's
    /// times, an if's test. Values outside the offered lists are refused.
    pub fn plan_edit(&mut self, at: usize, what: &str, value: &str) -> bool {
        let len = self.plan.len();
        let advanced = self.plan_advanced;
        let Some(st) = self.plan.get(at).cloned() else {
            return false;
        };
        let made = self.plan_items();
        let number = value.parse::<u64>().ok();
        let edited = match (what, st.kind.as_str()) {
            ("to", "repeat") => number
                .filter(|&t| (t as usize) < at)
                .map(|t| Step { to: t as usize, ..st }),
            ("to", "if") => number
                .filter(|&t| (t as usize) < len && t as usize != at)
                .map(|t| Step { to: t as usize, ..st }),
            ("times", "repeat") => number
                .filter(|n| c::PLAN_TIMES.contains(n))
                .map(|n| Step { times: n, ..st }),
            ("below", "if") => Some(Step {
                below: value == "1",
                ..st
            }),
            ("on", "if") => {
                let cd = if value == "gold" {
                    Some(Cond {
                        kind: "gold".into(),
                        id: String::new(),
                        n: c::PLAN_AMOUNTS[0],
                    })
                } else {
                    made.iter().any(|i| i == value).then(|| Cond {
                        kind: "item".into(),
                        id: value.into(),
                        n: c::PLAN_AMOUNTS[0],
                    })
                };
                cd.map(|cd| Step { cond: Some(cd), ..st })
            }
            ("amount", "if") => number.filter(|n| c::PLAN_AMOUNTS.contains(n)).and_then(|n| {
                st.cond.clone().map(|cd| Step {
                    cond: Some(Cond { n, ..cd }),
                    ..st.clone()
                })
            }),
            ("count", "") => number
                .filter(|n| c::PLAN_COUNTS.contains(n))
                .map(|n| Step { n, ..st }),
            ("until", "") if advanced => self.until_from(&st, value).map(|u| Step { until: u, ..st }),
            _ => None,
        };
        match edited {
            Some(s) => {
                self.plan[at] = s;
                true
            }
            None => false,
        }
    }

    /// Parses an until choice for a plain step: "none", "item:N" (what the
    /// step makes), "level:N" (its skill) or "gold:N".
    fn until_from(&self, st: &Step, value: &str) -> Option<Option<Cond>> {
        if value == "none" {
            return Some(None);
        }
        let (kind, n) = value.split_once(':')?;
        let n = n.parse::<u64>().ok()?;
        let (id, ok) = match kind {
            "item" => (self.makes_of(&st.id)?, c::PLAN_AMOUNTS.contains(&n)),
            "level" => (
                c::SKILLS[self.skill_of(&st.id)].id.to_string(),
                n > self.level(self.skill_of(&st.id)) as u64 && n <= xp::MAX_LEVEL as u64,
            ),
            "gold" => (String::new(), c::PLAN_AMOUNTS.contains(&n)),
            _ => return None,
        };
        ok.then(|| {
            Some(Cond {
                kind: kind.into(),
                id,
                n,
            })
        })
    }

    /// What a plan step's action makes, if it is an action.
    pub fn makes_of(&self, id: &str) -> Option<String> {
        c::action_index(id).map(|i| c::ACTIONS[i].makes.0.to_string())
    }

    /// The items the plan's plain steps make, in order, once each: what an
    /// "if" can test.
    pub fn plan_items(&self) -> Vec<String> {
        let mut out: Vec<String> = Vec::new();
        for s in &self.plan {
            if let Some(m) = self.makes_of(&s.id) {
                if !out.contains(&m) {
                    out.push(m);
                }
            }
        }
        out
    }

    // ---------- gold (P10) ----------

    pub fn sell(&mut self, item: &str) -> u64 {
        self.sell_n(item, self.have(item))
    }

    /// Sells `n` of an item, or all you have if you have fewer (T22).
    pub fn sell_n(&mut self, item: &str, n: u64) -> u64 {
        let n = n.min(self.have(item));
        let got = c::value(item) * n;
        if got > 0 {
            self.bank.insert(item.to_string(), self.have(item) - n);
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
        if self.gold < cost || !self.tool_tier_open(tier + 1) {
            return false;
        }
        self.gold -= cost;
        self.tools[line] = tier + 1;
        true
    }

    /// The top tool tiers need the Trader's rank (IG-Q55 A).
    pub fn tool_tier_open(&self, tier: usize) -> bool {
        tier <= c::TOOL_TIERS_FREE
            || (self.engine_open(c::Engine::Trader)
                && self.rank_in(c::Engine::Trader, 0) >= c::TOOL_TIER_RANK)
    }

    /// Every shelf: (region, rank, items, open). The Vale's come from
    /// `SHELVES`; each later region's opens at its first Trader rank there.
    pub fn shelves(&self) -> Vec<(usize, usize, Vec<&'static str>, bool)> {
        let trader = self.engine_open(c::Engine::Trader);
        let mut out: Vec<(usize, usize, Vec<&'static str>, bool)> = c::SHELVES
            .iter()
            .map(|&(rank, items)| {
                (
                    0,
                    rank,
                    items.to_vec(),
                    trader && self.rank_in(c::Engine::Trader, 0) >= rank,
                )
            })
            .collect();
        for r in 1..c::REGIONS.len() {
            out.push((
                r,
                1,
                c::region_goods(r),
                trader && self.region_open(r) && self.rank_in(c::Engine::Trader, r) >= 1,
            ));
        }
        out
    }

    /// Buys a pack of an item from an open shelf.
    pub fn shop_buy(&mut self, item: &str) -> bool {
        let on_sale = self
            .shelves()
            .iter()
            .any(|(_, _, items, open)| *open && items.contains(&item));
        let cost = c::shop_price(item) * c::SHOP_PACK;
        if !on_sale || self.gold < cost {
            return false;
        }
        self.gold -= cost;
        *self.bank.entry(item.to_string()).or_insert(0) += c::SHOP_PACK;
        true
    }

    // ---------- prestige (P9) ----------

    pub fn can_prestige(&self) -> bool {
        self.total_level() >= self.prestige_needs()
    }

    /// Back to level 1 with a bigger multiplier, and nothing else from the game
    /// kept: mastery and kill counts start over too (T6, IG-Q18 B), and so do
    /// the engines (IG-Q24 A). Recipes are kept: a recipe is something you
    /// learned (IG-Q26 A). The playtest log, flags and counts stay; they are
    /// about the player.
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
        s.recipes = keep.recipes;
        s.followers = keep.followers;
        s.offered = keep.offered;
        s.weapon_xp = keep.weapon_xp;
        s.view_sort = keep.view_sort;
        s.wield = keep.wield;
        s.asked = keep.asked;
        s.sound = keep.sound;
        s.plan = keep.plan;
        s.plan_advanced = keep.plan_advanced;
        s.rooms = vec![0; c::ROOMS.len()];
        s.upkeep_hour = keep.sim_ms / c::HOUR;
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
        // eighteen skills at 1, Hitpoints at 10
        assert_eq!(s.total_level(), 28);
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
    fn picking_an_action_by_hand_stops_the_plan_and_keeps_its_steps() {
        let mut s = State::new(3);
        s.plan_add("tree");
        s.plan_add("essence");
        s.plan_start();
        assert!(s.plan_on);
        s.start("copper", 0);
        assert!(!s.plan_on);
        assert_eq!(s.plan.len(), 2);
        assert_eq!(s.active.as_deref(), Some("copper"));
    }

    fn advanced() -> State {
        let mut s = State::new(3);
        s.plan_advanced = true;
        s
    }

    #[test]
    fn an_until_step_stops_at_its_amount() {
        // Chop until 5 logs (3 s each, 15 s), then burn until out (2 s each).
        let mut s = advanced();
        s.plan_add("tree");
        s.plan_add("burn_logs");
        assert!(s.plan_edit(0, "until", "item:10"));
        s.plan[0].until.as_mut().unwrap().n = 5;
        s.plan_start();
        run(&mut s, 15_000);
        assert_eq!(s.have("logs"), 5);
        assert_eq!(s.active.as_deref(), Some("burn_logs"));
        run(&mut s, 10_000);
        assert_eq!(s.have("logs"), 0);
        assert_eq!(s.xp[c::FIREMAKING], 5 * 40);
    }

    #[test]
    fn a_repeat_goes_back_its_number_of_times_and_the_plan_then_ends() {
        // Two chops, then back to the start twice: six chops, 18 s.
        let mut s = advanced();
        s.plan_add("tree");
        s.plan_count(0, 2);
        assert!(s.plan_add_control("repeat"));
        assert!(s.plan_edit(1, "times", "2"));
        s.plan_start();
        run(&mut s, 17_999);
        assert_eq!(s.have("logs"), 5);
        assert!(s.plan_on);
        run(&mut s, 1);
        assert_eq!(s.have("logs"), 6);
        assert!(!s.plan_on);
        assert_eq!(s.active, None);
        assert_eq!(s.plan.len(), 2);
    }

    #[test]
    fn an_if_jumps_while_its_test_holds() {
        // Chop one, and go back while there are fewer than 10 logs: ten chops
        // (30 s), then burn all ten (20 s), 400 Firemaking XP.
        let mut s = advanced();
        s.plan_add("tree");
        s.plan_count(0, 1);
        assert!(s.plan_add_control("if"));
        assert!(s.plan_edit(1, "on", "logs"));
        assert!(s.plan_edit(1, "amount", "10"));
        assert!(s.plan_edit(1, "to", "0"));
        s.plan_add("burn_logs");
        s.plan_start();
        run(&mut s, 30_000);
        assert_eq!(s.have("logs"), 10);
        assert_eq!(s.active.as_deref(), Some("burn_logs"));
        run(&mut s, 20_000);
        assert_eq!(s.xp[c::FIREMAKING], 400);
        assert!(!s.plan_on);
    }

    #[test]
    fn a_plan_that_goes_round_doing_nothing_stops() {
        // No logs to burn, and a repeat back to it forever.
        let mut s = advanced();
        s.plan_add("burn_logs");
        s.plan_add_control("repeat");
        s.plan_start();
        let r = run(&mut s, 1);
        assert!(r.plan_stuck.is_some());
        // It stops after passing each step about twice, not later.
        assert!(r.steps.len() <= 2 * 2 + 1, "{:?}", r.steps);
        assert!(!s.plan_on);
        assert_eq!(s.active, None);
        // The same plan stuck in the middle of a stretch of time ends too.
        let mut s = advanced();
        s.plan_add("tree");
        s.plan_count(0, 1);
        s.plan_add("burn_logs");
        s.plan_count(1, 1);
        s.plan_add("burn_logs");
        s.plan_add_control("repeat");
        assert!(s.plan_edit(3, "to", "1"));
        s.plan_start();
        let r = run(&mut s, 60_000);
        assert!(r.plan_stuck.is_some());
        assert!(!s.plan_on);
    }

    #[test]
    fn dragging_a_block_moves_it_and_jumps_follow_the_steps_they_name() {
        // 0 tree, 1 burn, 2 repeat back to 0 (tree).
        let mut s = advanced();
        s.plan_add("tree");
        s.plan_add("burn_logs");
        s.plan_add_control("repeat");
        // Drag burn to the top: burn, tree, repeat; the repeat still names tree, now 1.
        assert!(s.plan_move(1, 0));
        assert_eq!(s.plan[0].id, "burn_logs");
        assert_eq!(s.plan[2].to, 1);
        // Drop a new if at the top: if, burn, tree, repeat; the repeat moves to 2.
        assert!(s.plan_insert("if", 0));
        assert_eq!(s.plan[0].kind, "if");
        assert_eq!(s.plan[3].to, 2);
        assert!(!s.plan_move(0, 9), "past the end");
    }

    #[test]
    fn plan_edits_outside_the_offered_choices_are_refused() {
        let mut s = advanced();
        s.plan_add("tree");
        s.plan_add_control("repeat");
        assert!(!s.plan_edit(1, "to", "1"), "a repeat goes back, not to itself");
        assert!(!s.plan_edit(1, "times", "7"));
        assert!(!s.plan_edit(0, "count", "7"));
        assert!(!s.plan_edit(0, "until", "item:7"));
        assert!(!s.plan_edit(0, "until", "level:1"), "a level you already have");
        assert!(s.plan_edit(0, "until", "level:2"));
        s.plan_advanced = false;
        assert!(!s.plan_edit(0, "until", "none"));
        assert!(!s.plan_set_advanced(false));
        s.plan_advanced = true;
        s.plan_remove(1);
        s.plan_edit(0, "until", "none");
        assert!(s.plan_set_advanced(false));
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
    fn every_prestige_needs_900_and_keeps_only_the_multiplier() {
        let mut s = State::new(1);
        assert_eq!(s.prestige_needs(), 900);
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
        assert_eq!(s.prestige_needs(), 900);
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
    fn a_crop_grows_while_something_else_runs_and_waits_to_be_harvested() {
        // Potato: 10 minutes, 8 XP to plant, 9 XP each of three to six.
        let mut s = State::new(5);
        s.bank.insert("potato_seed".into(), 2);
        let mut r = Report::begin(&s, 0, 0);
        assert!(s.plant("potato", &mut r));
        assert_eq!(s.xp[c::FARMING], 8);
        assert_eq!(s.have("potato_seed"), 1);
        s.start("tree", 0);
        run(&mut s, 9 * 60_000);
        assert!(!s.ready(0));
        assert_eq!(s.grow_left(0), Some(60_000));
        run(&mut s, 5 * 60 * 60_000);
        assert!(s.ready(0));
        let n = s.harvest(0, &mut r);
        assert!((3..=6).contains(&n));
        assert_eq!(s.have("potato"), n);
        assert_eq!(s.xp[c::FARMING], 8 + 9 * n);
        assert_eq!(s.patch(0).crop, "");
    }

    #[test]
    fn three_patches_open_at_first_and_more_at_farming_40_and_70() {
        let mut s = State::new(5);
        assert_eq!(s.patches_open(), 3);
        s.bank.insert("potato_seed".into(), 9);
        let mut r = Report::begin(&s, 0, 0);
        assert!(s.plant("potato", &mut r) && s.plant("potato", &mut r) && s.plant("potato", &mut r));
        assert!(!s.plant("potato", &mut r));
        s.xp[c::FARMING] = xp::at(40);
        assert_eq!(s.patches_open(), 4);
        assert!(s.plant("potato", &mut r));
        s.xp[c::FARMING] = xp::at(70);
        assert_eq!(s.patches_open(), 5);
    }

    #[test]
    fn a_crop_above_your_level_cannot_be_planted() {
        let mut s = State::new(5);
        s.bank.insert("tomato_seed".into(), 1);
        let mut r = Report::begin(&s, 0, 0);
        assert!(!s.plant("tomato", &mut r));
        assert_eq!(s.have("tomato_seed"), 1);
    }

    #[test]
    fn armour_takes_ten_percent_a_tier_off_each_hit() {
        let mut s = State::new(1);
        assert_eq!(s.armour().1, 0);
        s.bank.insert("leather_body".into(), 1);
        s.bank.insert("silk_robe".into(), 1);
        assert_eq!(s.armour(), ("silk_robe", 3, 0));
    }

    #[test]
    fn armour_means_less_food_eaten_in_the_same_fight() {
        // Same seed, same swings: only the damage taken differs.
        let fight = |armour: bool| {
            let mut s = State::new(21);
            s.bank.insert("bronze_sword".into(), 1);
            s.bank.insert("shark".into(), 10_000);
            if armour {
                s.bank.insert("dragonhide_body".into(), 1);
            }
            s.start("hill_giant", 0);
            run(&mut s, 30 * 60_000).used.get("shark").copied().unwrap_or(0)
        };
        assert!(fight(true) < fight(false), "{} vs {}", fight(true), fight(false));
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

    // ---------- the mastery pool (T27) ----------

    #[test]
    fn a_skills_mastery_pool_opens_bonuses_for_every_action_in_it() {
        let mut s = State::new(3);
        let n = c::skill_actions(c::RUNECRAFTING).len() as u64;
        // A new game: every action at mastery 1.
        assert_eq!(s.pool(c::RUNECRAFTING), (n, n * 99));
        assert_eq!(s.pool_steps(c::RUNECRAFTING), 0);
        // death runes give 10 XP, enough to show +10%; on a 5 XP rune it rounds away (R18.2)
        let air = &c::ACTIONS[c::action_index("death_runes").unwrap()];
        let xp0 = s.xp_for(air);
        let ms0 = s.duration("air_runes");
        let xp_mastered = |s: &State| s.xp_for(air);
        // Master every Runecrafting action to 50: the pool is 50 × n of 99 × n, 50%.
        for &i in c::skill_actions(c::RUNECRAFTING) {
            s.mxp.insert(c::ACTIONS[i].id.to_string(), xp::mastery_at(50));
        }
        assert_eq!(s.pool_pct(c::RUNECRAFTING), 50);
        assert_eq!(s.pool_steps(c::RUNECRAFTING), 3);
        // +5% XP (on top of mastery 10's +5%), 5% faster, +5% keep
        assert_eq!(s.duration("air_runes"), (ms0 - c::MASTERY_FASTER_MS) * 95 / 100);
        assert!(xp_mastered(&s) > xp0);
        assert_eq!(s.keep_pct("air_runes"), c::KEEP_AT_50 + 5);
        // another skill is untouched
        assert_eq!(s.pool_steps(c::MINING), 0);
    }

    #[test]
    fn your_highest_mastery_in_a_skill_speeds_every_actions_mastery() {
        let mut s = State::new(3);
        s.mxp.insert("oak".into(), xp::mastery_at(60));
        assert_eq!(s.top_mastery(c::WOODCUTTING), 60);
        s.start("tree", 0);
        let mut r = Report::begin(&s, 3_000, 0);
        s.advance(3_000, &mut r);
        // a tree gives 10 mastery XP, 60% more: 16
        assert_eq!(s.mxp["tree"], 16);
    }

    #[test]
    fn cattle_raised_in_a_patch_give_cowhide() {
        // T32: Farming 12, a calf, an hour; 2 to 4 cowhide.
        let mut s = State::new(3);
        s.xp[c::FARMING] = xp::at(12);
        s.bank.insert("calf".into(), 1);
        let mut r = Report::begin(&s, 0, 0);
        assert!(s.plant("cattle", &mut r));
        assert_eq!(s.have("calf"), 0);
        run(&mut s, 3_600_000);
        let mut r = Report::begin(&s, 0, 0);
        s.harvest(0, &mut r);
        assert!((2..=4).contains(&s.have("cowhide")), "{}", s.have("cowhide"));
    }
}
