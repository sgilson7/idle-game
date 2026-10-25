//! The engines' rules (T10 to T14) and the regions (T15): recipes from
//! enemies, the Trader, the estate, the Inventor's workshop, the turning
//! skies, and which region is open. Each engine spends what the skills make
//! and gives back things they need; each has a ladder of ranks in every open
//! region and a timer that `advance` runs alongside your one action.

use crate::content::{self as c, Engine, RoomEffect};
use crate::game::{Caravan, Report, Research, Servant, State};

/// What is open at one moment, for the time loop's many checks.
pub(crate) struct Open {
    pub regions: usize,
    /// Trader, Nobility, Inventor, Cosmic.
    pub engines: [bool; 4],
    pub slots: u64,
}

/// What the timers need now: each servant's and machine's cycle, if it can run.
pub(crate) struct Timers {
    pub servants: Vec<Option<u64>>,
    pub machines: Vec<Option<u64>>,
}

/// An order a town has posted: what it wants, how many, and what it pays.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub town: usize,
    pub slot: u64,
    pub item: &'static str,
    pub n: u64,
    pub gold: u64,
    pub xp: u64,
    pub key: String,
}

/// A fixed mix of whole numbers, so a town's orders are the same on every
/// machine and can be shown before they are posted (IG-Q30 A).
pub(crate) fn mix(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

pub fn engine_slot(e: Engine) -> usize {
    match e {
        Engine::Trader => 0,
        Engine::Nobility => 1,
        Engine::Inventor => 2,
        Engine::Cosmic => 3,
    }
}

impl State {
    // ---------- regions (T15) ----------

    /// A region is open while the total level is at its gate; prestige
    /// closes it again (IG-Q44 A).
    /// Reached by total level, and in the app paid for past the Vale
    /// (IG-Q46 B).
    pub fn region_open(&self, r: usize) -> bool {
        (r == 0 || !self.sealed) && self.region_reached(r)
    }

    /// Reached by total level, whether or not the purchase has opened it.
    pub fn region_reached(&self, r: usize) -> bool {
        c::REGIONS.get(r).is_some_and(|&(_, at)| self.total_level() >= at)
    }

    pub fn regions_open(&self) -> usize {
        (0..c::REGIONS.len()).filter(|&r| self.region_open(r)).count()
    }

    /// The skills' new mechanics open with the Coast (Q40 A).
    pub fn mechanics_open(&self) -> bool {
        self.region_open(1)
    }

    // ---------- which engines are open ----------

    /// The total level an engine needs.
    pub fn engine_at(&self, e: Engine) -> u32 {
        c::ENGINE_AT[engine_slot(e)].1
    }

    /// Reached by total level. The Inventor also needs the Workshop room and
    /// the Cosmic the Observatory (R6.2).
    pub fn engine_open(&self, e: Engine) -> bool {
        let reached = self.total_level() >= self.engine_at(e);
        match e {
            Engine::Inventor => reached && self.room(c::WORKSHOP) > 0,
            Engine::Cosmic => reached && self.room(c::OBSERVATORY) > 0,
            _ => reached,
        }
    }

    pub fn engine_reached(&self, e: Engine) -> bool {
        self.total_level() >= self.engine_at(e)
    }

    /// The rank on an engine's ladder in the Vale.
    pub fn rank(&self, e: Engine) -> usize {
        self.rank_in(e, 0)
    }

    /// The rank on an engine's ladder in region `r` (T15, IG-Q43 A).
    pub fn rank_in(&self, e: Engine, r: usize) -> usize {
        if r == 0 {
            self.ranks[engine_slot(e)]
        } else {
            self.region_ranks
                .get(r - 1)
                .map(|t| t[engine_slot(e)])
                .unwrap_or(0)
        }
    }

    fn set_rank(&mut self, e: Engine, r: usize, k: usize) {
        if r == 0 {
            self.ranks[engine_slot(e)] = k;
        } else if let Some(t) = self.region_ranks.get_mut(r - 1) {
            t[engine_slot(e)] = k;
        }
    }

    /// An engine's tier in region `r` is open with the engine and the region.
    pub fn tier_open(&self, e: Engine, r: usize) -> bool {
        self.engine_open(e) && self.region_open(r)
    }

    /// The sum of an engine's ranks over its open tiers, for effects that add up.
    pub fn rank_sum(&self, e: Engine) -> usize {
        (0..c::REGIONS.len())
            .filter(|&r| self.tier_open(e, r))
            .map(|r| self.rank_in(e, r))
            .sum()
    }

    /// Whether a skill can be trained: the old skills always, the engine
    /// skills once their engine is open.
    pub fn skill_open(&self, skill: usize) -> bool {
        match c::engine_of_skill(skill) {
            Some(e) => self.engine_open(e),
            None => true,
        }
    }

    /// How much time away counts (P4): longer at Cosmic ranks (IG-Q32 A),
    /// longest in the last sky.
    pub fn away_cap(&self) -> u64 {
        if !self.engine_open(Engine::Cosmic) {
            return c::AWAY_CAP_MS;
        }
        let mut cap = c::AWAY_CAP_BY_COSMIC[0][self.rank(Engine::Cosmic)];
        for r in 1..c::REGIONS.len() {
            let k = self.rank_in(Engine::Cosmic, r);
            if self.region_open(r) && k > 0 {
                cap = cap.max(c::AWAY_CAP_BY_COSMIC[r][k]);
            }
        }
        cap
    }

    // ---------- rooms and furniture ----------

    /// A room's built level.
    pub fn room(&self, k: usize) -> u64 {
        self.rooms.get(k).copied().unwrap_or(0)
    }

    /// A room's level for its effect: built levels and each piece of its furniture.
    pub fn room_effect(&self, k: usize) -> u64 {
        let id = c::ROOMS[k].id;
        let pieces = c::FURNITURE
            .iter()
            .filter(|(f, room)| *room == id && self.have(f) > 0)
            .count() as u64;
        if self.room(k) == 0 {
            0
        } else {
            self.room(k) + pieces
        }
    }

    /// The summed effect level of every room of one kind.
    pub fn rooms_of(&self, effect: RoomEffect) -> u64 {
        (0..c::ROOMS.len())
            .filter(|&k| c::ROOMS[k].effect == effect)
            .map(|k| self.room_effect(k))
            .sum()
    }

    /// A room's top level now: each region opened past its own adds two (T15).
    pub fn room_max(&self, k: usize) -> u64 {
        let room = &c::ROOMS[k];
        let later = self.regions_open().saturating_sub(room.region + 1) as u64;
        let cap = if room.effect == RoomEffect::Patch {
            4
        } else {
            c::ROOM_MAX
        };
        (room.max + c::ROOM_MAX_PER_REGION * later).min(cap)
    }

    /// Percent off a skill's action time from its rooms (the kitchen 10% a level, the rest 5%).
    pub fn room_speed_pct(&self, skill: usize) -> u64 {
        (0..c::ROOMS.len())
            .filter(|&k| c::ROOMS[k].effect == RoomEffect::Faster(skill))
            .map(|k| {
                let pct = if k == c::KITCHEN {
                    c::KITCHEN_PCT
                } else {
                    c::FASTER_PCT
                };
                pct * self.room_effect(k)
            })
            .sum::<u64>()
            .min(50)
    }

    // ---------- recipes (T10) ----------

    pub(crate) fn find_recipe(&mut self, rc: &c::Recipe, r: &mut Report) {
        if self.recipes.iter().any(|k| k == rc.id) {
            let gold = rc.one_in * c::RECIPE_COPY_GOLD_PER_ONE_IN;
            self.gold += gold;
            r.gold += gold;
        } else {
            self.recipes.push(rc.id.to_string());
            r.recipes.push(rc.id.to_string());
        }
    }

    // ---------- the skies (T13, T15) ----------

    /// The Vale's sign that is up now: six in turn, 80 minutes each, on game time.
    pub fn sign(&self) -> usize {
        self.sign_at(self.sim_ms)
    }

    pub fn sign_at(&self, at: u64) -> usize {
        self.sky_sign_at(0, at)
    }

    pub fn sky_sign_at(&self, sky: usize, at: u64) -> usize {
        (at / c::SKIES[sky].sign_ms % 6) as usize
    }

    /// Game time until the Vale's sign changes.
    pub fn sign_left(&self) -> u64 {
        self.sky_left(0)
    }

    pub fn sky_left(&self, sky: usize) -> u64 {
        let ms = c::SKIES[sky].sign_ms;
        ms - self.sim_ms % ms
    }

    /// A region's sky turns once the Cosmic is open and the region is.
    pub fn sky_open(&self, sky: usize) -> bool {
        self.engine_open(Engine::Cosmic) && self.region_open(sky)
    }

    /// How many open skies have a sign up for `skill`: a constellation when
    /// more than one (Q40 A).
    pub fn signs_for(&self, skill: usize) -> u64 {
        let open = self.open_now();
        if !open.engines[3] {
            return 0;
        }
        (0..open.regions)
            .filter(|&k| {
                c::SKIES[k].signs[self.sky_sign_at(k, self.sim_ms)]
                    .1
                    .contains(&skill)
            })
            .count() as u64
    }

    pub fn harvest_moon(&self) -> bool {
        self.signs_for(c::FARMING) > 0
    }

    /// Training a skill under its sign gathers starlight, one for each sky
    /// that shows it.
    pub(crate) fn starlight(&mut self, skill: usize, r: &mut Report) {
        let mut n = self.signs_for(skill);
        if n > 0 && self.rank(Engine::Cosmic) >= c::DOUBLE_STARLIGHT_RANK {
            n *= 2;
        }
        self.put("starlight", n, r, false);
    }

    /// The tree that is rare this game hour (Q40 A): one of the open trees.
    pub fn rare_tree(&self) -> Option<&'static str> {
        let open = self.regions_open();
        if open < 2 {
            return None;
        }
        let n = c::trees().iter().filter(|(r, _)| *r < open).count() as u64;
        let hour = self.sim_ms / c::RARE_TREE_MS;
        c::trees()
            .iter()
            .filter(|(r, _)| *r < open)
            .nth((mix(hour ^ 0x7EE5) % n.max(1)) as usize)
            .map(|(_, id)| *id)
    }

    // ---------- timers alongside your action ----------

    /// What is open now, worked out once for each step of the time loop:
    /// the total level sums eighteen skills, and every timer asks.
    pub(crate) fn open_now(&self) -> Open {
        let total = self.total_level();
        let regions = c::REGIONS.iter().filter(|&&(_, at)| total >= at).count();
        let reached = |e: Engine| total >= self.engine_at(e);
        let engines = [
            reached(Engine::Trader),
            reached(Engine::Nobility),
            reached(Engine::Inventor) && self.room(c::WORKSHOP) > 0,
            reached(Engine::Cosmic) && self.room(c::OBSERVATORY) > 0,
        ];
        let slots = if engines[1] {
            (0..regions)
                .map(|r| c::NOBLE_RANKS[r][self.rank_in(Engine::Nobility, r)].servants)
                .sum::<u64>()
                + self.rooms_of(RoomEffect::Servant)
        } else {
            0
        };
        Open {
            regions,
            engines,
            slots,
        }
    }

    /// How long servant `i` takes for one job, or `None` when it cannot work now.
    pub(crate) fn servant_need(&self, i: usize) -> Option<u64> {
        self.servant_need_in(i, &self.open_now())
    }

    pub(crate) fn servant_need_in(&self, i: usize, open: &Open) -> Option<u64> {
        let sv = &self.servants[i];
        if sv.unpaid || sv.job.is_empty() || (i as u64) >= open.slots {
            return None;
        }
        let a = &c::ACTIONS[c::action_index(&sv.job)?];
        let skill_open = c::engine_of_skill(a.skill).is_none_or(|e| open.engines[engine_slot(e)]);
        if self.level(a.skill) < a.level
            || a.region >= open.regions
            || !skill_open
            || !self.gate_open(a.id)
            || self.missing(a.id).is_some()
        {
            return None;
        }
        Some(self.duration(a.id) * c::SERVANT_SLOW)
    }

    fn servant_slots_ok(&self, i: usize) -> bool {
        (i as u64) < self.servant_slots()
    }

    /// Servant slots: from the ranks of every open tier and the Servants' quarters.
    pub fn servant_slots(&self) -> u64 {
        if !self.engine_open(Engine::Nobility) {
            return 0;
        }
        (0..c::REGIONS.len())
            .filter(|&r| self.region_open(r))
            .map(|r| c::NOBLE_RANKS[r][self.rank_in(Engine::Nobility, r)].servants)
            .sum::<u64>()
            + self.rooms_of(RoomEffect::Servant)
    }

    /// A machine's cycle, or `None` when it is not built, off, or out of
    /// input or fuel.
    pub(crate) fn machine_need_in(&self, m: &c::Machine, open: &Open) -> Option<u64> {
        if self.have(m.id) == 0 || self.machines_off.iter().any(|k| k == m.id) || !open.engines[2] {
            return None;
        }
        if m.needs.iter().any(|(i, n)| self.have(i) < *n) {
            return None;
        }
        if m.fuel {
            self.fuel_for(m)?;
        }
        Some(self.machine_cycle(m))
    }

    pub fn machine_cycle(&self, m: &c::Machine) -> u64 {
        let w = self.rooms_of(RoomEffect::Workshop).clamp(1, 10);
        m.ms * (100 - c::WORKSHOP_PCT * (w - 1)) / 100
    }

    /// The fuel a machine would burn: the first in `FUEL` left over after its input.
    pub fn fuel_for(&self, m: &c::Machine) -> Option<&'static str> {
        c::FUEL.iter().copied().find(|f| {
            let used = m
                .needs
                .iter()
                .filter(|(i, _)| i == f)
                .map(|(_, n)| *n)
                .sum::<u64>();
            self.have(f) > used
        })
    }

    /// How many a machine makes a cycle: twice as many with its upgrade (Q40 A).
    pub fn machine_makes(&self, m: &c::Machine) -> u64 {
        let up = c::UPGRADES.iter().any(|(u, id)| *id == m.id && self.have(u) > 0);
        m.makes.1 * if up { 2 } else { 1 }
    }

    /// Game time left on the research, or `None` when there is none. The
    /// library makes research faster.
    pub(crate) fn research_need(&self) -> Option<u64> {
        let rs = self.research.as_ref()?;
        if !self.engine_open(Engine::Inventor) {
            return None;
        }
        Some(self.research_ms(&rs.id)?.saturating_sub(rs.progress))
    }

    pub fn research_ms(&self, id: &str) -> Option<u64> {
        let bp = &c::BLUEPRINTS[c::blueprint_index(id)?];
        let pct = (c::RESEARCH_PCT * self.rooms_of(RoomEffect::Research)).min(50);
        Some(bp.ms * (100 - pct) / 100)
    }

    /// The engines' part of `fire`: everything due now, in a fixed order.
    pub(crate) fn fire_engines(&mut self, r: &mut Report) {
        let open = self.open_now();
        for i in 0..self.servants.len() {
            if let Some(need) = self.servant_need_in(i, &open) {
                if self.servants[i].progress >= need {
                    self.servants[i].progress = 0;
                    if let Some(ai) = c::action_index(&self.servants[i].job) {
                        self.produce(ai, false, r);
                    }
                }
            }
        }
        for m in c::MACHINES {
            if let Some(need) = self.machine_need_in(m, &open) {
                if self.machine_ms.get(m.id).copied().unwrap_or(0) >= need {
                    self.machine_ms.insert(m.id.to_string(), 0);
                    if m.fuel {
                        let fuel = self.fuel_for(m).unwrap_or("logs");
                        self.take(fuel, 1, r);
                    }
                    for (i, n) in m.needs {
                        self.take(i, *n, r);
                    }
                    let n = self.machine_makes(m);
                    self.put(m.makes.0, n, r, false);
                }
            }
        }
        if self.research_need() == Some(0) {
            if let Some(rs) = self.research.take() {
                if let Some(bp) = c::blueprint_index(&rs.id).map(|b| &c::BLUEPRINTS[b]) {
                    self.blueprints.push(bp.id.to_string());
                    r.research.push(bp.id.to_string());
                    let xp = bp.xp * self.mult_bp() / 10_000;
                    self.gain(c::INVENTION, xp, r);
                }
            }
        }
        let now = self.sim_ms;
        let (home, away): (Vec<Caravan>, Vec<Caravan>) =
            self.caravans.drain(..).partition(|cv| cv.back <= now);
        self.caravans = away;
        for cv in home {
            self.caravan_home(&cv, r);
        }
        let hour = self.sim_ms / c::HOUR;
        if hour > self.upkeep_hour {
            if self.engine_open(Engine::Nobility) {
                self.upkeep(r);
            }
            self.upkeep_hour = hour;
        }
    }

    /// Gold the estate pays an hour: every open tier's rank, and the treasury.
    pub fn income(&self) -> u64 {
        let base: u64 = (0..c::REGIONS.len())
            .filter(|&r| self.region_open(r))
            .map(|r| c::NOBLE_RANKS[r][self.rank_in(Engine::Nobility, r)].income)
            .sum();
        base * (100 + c::INCOME_PCT * self.rooms_of(RoomEffect::Income)) / 100
    }

    /// The estate's hour: its income, then each servant's wage and food.
    fn upkeep(&mut self, r: &mut Report) {
        let income = self.income();
        self.gold += income;
        r.income += income;
        r.gold += income;
        for i in 0..self.servants.len() {
            if self.servants[i].job.is_empty() || !self.servant_slots_ok(i) {
                continue;
            }
            let food = c::FOOD.iter().rev().map(|(f, _)| *f).find(|f| self.have(f) > 0);
            match food {
                Some(f) if self.gold >= c::SERVANT_WAGE => {
                    self.gold -= c::SERVANT_WAGE;
                    r.wages += c::SERVANT_WAGE;
                    self.take(f, 1, r);
                    self.servants[i].unpaid = false;
                }
                _ => {
                    self.servants[i].unpaid = true;
                    r.unpaid = true;
                }
            }
        }
    }

    // ---------- Nobility (T11) ----------

    pub fn room_levels(&self) -> u64 {
        self.rooms.iter().sum()
    }

    /// What the next level of room `k` costs: (plank, planks, nails, limestone,
    /// gold, Construction level), or `None` at its top level now.
    pub fn room_cost(&self, k: usize) -> Option<(&'static str, u64, u64, u64, u64, u32)> {
        let room = &c::ROOMS[k];
        let l = self.room(k) + 1;
        if l > self.room_max(k) || !self.region_open(room.region) {
            return None;
        }
        Some((
            c::ROOM_PLANKS[(l - 1) as usize],
            c::ROOM_PLANKS_PER * l,
            c::ROOM_NAILS_PER * l,
            c::ROOM_STONE_PER * l,
            c::ROOM_GOLD_PER * l * l,
            (room.level + 8 * (l as u32 - 1)).min(99),
        ))
    }

    pub fn can_build(&self, k: usize) -> bool {
        let Some((plank, p, n, st, g, lvl)) = self.room_cost(k) else {
            return false;
        };
        self.engine_open(Engine::Nobility)
            && self.level(c::CONSTRUCTION) >= lvl
            && self.have(plank) >= p
            && self.have("nails") >= n
            && self.have("limestone") >= st
            && self.gold >= g
    }

    pub fn build_room(&mut self, k: usize, r: &mut Report) -> bool {
        if !self.can_build(k) {
            return false;
        }
        let Some((plank, p, n, st, g, _)) = self.room_cost(k) else {
            return false;
        };
        self.take(plank, p, r);
        self.take("nails", n, r);
        self.take("limestone", st, r);
        self.gold -= g;
        if self.rooms.len() < c::ROOMS.len() {
            self.rooms.resize(c::ROOMS.len(), 0);
        }
        self.rooms[k] += 1;
        let l = self.rooms[k];
        let xp = c::ROOM_XP_PER * l * l * self.mult_bp() / 10_000;
        self.gain(c::CONSTRUCTION, xp, r);
        true
    }

    /// What stands between you and the next noble rank in region `reg`, if
    /// anything: (gold short, rooms short, the deed done by champion, the tribute held).
    pub fn noble_next_in(&self, reg: usize) -> Option<(u64, u64, bool, bool)> {
        let next = self.rank_in(Engine::Nobility, reg) + 1;
        let rk = c::NOBLE_RANKS.get(reg)?.get(next)?;
        let champion = next < c::DEED_FROM
            || self
                .kills
                .get(c::CHAMPIONS[reg][next - c::DEED_FROM])
                .copied()
                .unwrap_or(0)
                > 0;
        let tribute = next < c::DEED_FROM
            || c::TRIBUTES[reg][next - c::DEED_FROM]
                .iter()
                .all(|(i, n)| self.have(i) >= *n);
        Some((
            rk.gold.saturating_sub(self.gold),
            rk.rooms.saturating_sub(self.room_levels()),
            champion,
            tribute,
        ))
    }

    pub fn noble_next(&self) -> Option<(u64, u64, bool, bool)> {
        self.noble_next_in(0)
    }

    /// Rises a noble rank in region `reg` by a champion defeated or by tribute (IG-Q29 C).
    pub fn rise_noble_in(&mut self, reg: usize, tribute: bool, r: &mut Report) -> bool {
        if !self.tier_open(Engine::Nobility, reg) {
            return false;
        }
        let next = self.rank_in(Engine::Nobility, reg) + 1;
        let Some((gold, rooms, champion, held)) = self.noble_next_in(reg) else {
            return false;
        };
        let deed = if tribute { held } else { champion };
        if gold > 0 || rooms > 0 || !deed {
            return false;
        }
        self.gold -= c::NOBLE_RANKS[reg][next].gold;
        if tribute && next >= c::DEED_FROM {
            for (i, n) in c::TRIBUTES[reg][next - c::DEED_FROM] {
                self.take(i, *n, r);
            }
        }
        self.set_rank(Engine::Nobility, reg, next);
        true
    }

    pub fn rise_noble(&mut self, tribute: bool, r: &mut Report) -> bool {
        self.rise_noble_in(0, tribute, r)
    }

    /// Puts servant slot `i` to work on `job`, a gathering or making action.
    pub fn set_servant(&mut self, i: usize, job: &str) -> bool {
        let ok = c::action_index(job)
            .map(|a| c::SKILLS[c::ACTIONS[a].skill].kind)
            .is_some_and(|k| k == c::Kind::Gather || k == c::Kind::Make);
        if !ok || (i as u64) >= self.servant_slots() || !self.unlocked(job) {
            return false;
        }
        while self.servants.len() <= i {
            self.servants.push(Servant::default());
        }
        self.servants[i] = Servant {
            job: job.to_string(),
            progress: 0,
            unpaid: false,
        };
        true
    }

    pub fn dismiss_servant(&mut self, i: usize) {
        if let Some(sv) = self.servants.get_mut(i) {
            *sv = Servant::default();
        }
    }

    // ---------- the Trader (T12) ----------

    pub fn town_open(&self, t: usize) -> bool {
        let town = &c::TOWNS[t];
        self.engine_open(Engine::Trader)
            && self.region_open(town.region)
            && self.level(c::TRADING) >= town.level
            && (t != c::EMBERFALL || self.rank(Engine::Nobility) >= c::CHARTER_RANK)
    }

    /// A town's price, in percent, at game time `at`: a twelve-hour cycle,
    /// each town three hours behind the one before.
    pub fn price_pct(&self, t: usize, at: u64) -> u64 {
        let phase = (at / c::HOUR + 3 * t as u64) % c::PRICE_CYCLE_HOURS;
        let half = c::PRICE_CYCLE_HOURS / 2;
        let up = if phase < half {
            phase
        } else {
            c::PRICE_CYCLE_HOURS - phase
        };
        c::PRICE_LOW_PCT + c::PRICE_STEP_PCT * up
    }

    /// The order town `t` posts in slot `slot` of window `w` (each `ORDER_MS` long).
    pub fn order(&self, t: usize, w: u64, slot: u64) -> Order {
        let town = &c::TOWNS[t];
        let h = mix(w * 1_000 + t as u64 * 10 + slot);
        let item = town.wants[(h % town.wants.len() as u64) as usize];
        let n = 10 * (1 + (h >> 16) % 5);
        let gold = c::value(item) * n * c::ORDER_PAY_PCT / 100;
        Order {
            town: t,
            slot,
            item,
            n,
            gold,
            xp: 40 + 3 * n + gold / 4,
            key: format!("{w}:{t}:{slot}"),
        }
    }

    pub fn orders_now(&self, t: usize) -> Vec<Order> {
        let w = self.sim_ms / c::ORDER_MS;
        (0..c::ORDER_SLOTS).map(|k| self.order(t, w, k)).collect()
    }

    pub fn order_filled(&self, o: &Order) -> bool {
        self.filled.contains(&o.key)
    }

    pub fn fill_order(&mut self, t: usize, slot: u64, r: &mut Report) -> bool {
        if !self.town_open(t) || slot >= c::ORDER_SLOTS {
            return false;
        }
        let o = self.order(t, self.sim_ms / c::ORDER_MS, slot);
        if self.order_filled(&o) || self.have(o.item) < o.n {
            return false;
        }
        self.take(o.item, o.n, r);
        self.gold += o.gold;
        r.gold += o.gold;
        let xp = o.xp * self.mult_bp() / 10_000;
        self.gain(c::TRADING, xp, r);
        let window = self.sim_ms / c::ORDER_MS;
        self.filled
            .retain(|k| k.split(':').next().and_then(|w| w.parse::<u64>().ok()) == Some(window));
        self.filled.push(o.key);
        self.count("orders", 1);
        self.count(&format!("orders:{}", c::TOWNS[t].id), 1);
        true
    }

    /// Vehicles of one kind on the road and the most that can be: those you
    /// own, up to one plus the stables' and harbours' levels.
    pub fn caravan_room_for(&self, vehicle: &str) -> (u64, u64) {
        let out = self
            .caravans
            .iter()
            .filter(|cv| c::TOWNS[cv.town].vehicle == vehicle)
            .count() as u64;
        let most = self.have(vehicle).min(1 + self.rooms_of(RoomEffect::Caravan));
        (out, most)
    }

    /// Carts on the road and the most that can be.
    pub fn caravan_room(&self) -> (u64, u64) {
        self.caravan_room_for("cart")
    }

    pub fn guard_cost(&self, t: usize) -> u64 {
        c::GUARD_GOLD * (t as u64 + 1)
    }

    /// Sends a caravan of up to a load of `item` to town `t`.
    pub fn send_caravan(&mut self, t: usize, item: &str, guards: bool, r: &mut Report) -> bool {
        if t >= c::TOWNS.len() {
            return false;
        }
        let (out, most) = self.caravan_room_for(c::TOWNS[t].vehicle);
        if !self.town_open(t)
            || out >= most
            || !c::TOWNS[t].wants.contains(&item)
            || self.have(item) == 0
            || (guards && self.gold < self.guard_cost(t))
        {
            return false;
        }
        let n = self.have(item).min(c::CART_LOAD);
        self.take(item, n, r);
        if guards {
            self.gold -= self.guard_cost(t);
        }
        self.caravans.push(Caravan {
            town: t,
            item: item.to_string(),
            n,
            back: self.sim_ms + c::TOWNS[t].trip_ms,
            guards,
        });
        true
    }

    /// The bandits' chance against a caravan to town `t`, in percent.
    pub fn bandit_pct(&self, t: usize, guards: bool) -> u64 {
        let best = [c::MELEE, c::RANGED, c::MAGIC]
            .iter()
            .map(|&s| self.level(s) as u64)
            .max()
            .unwrap_or(1);
        let mut pct = c::TOWNS[t].danger.saturating_sub(best / 4);
        if guards {
            pct = pct.saturating_sub(c::GUARD_PCT);
        }
        pct
    }

    /// What a caravan earns at the price on its return.
    pub fn caravan_gold(&self, cv: &Caravan, at: u64) -> u64 {
        let rank = 100 + c::TRADER_RANK_PCT * self.rank_sum(Engine::Trader) as u64;
        c::value(&cv.item) * cv.n * self.price_pct(cv.town, at) / 100 * rank / 100
    }

    fn caravan_home(&mut self, cv: &Caravan, r: &mut Report) {
        let bandits = self.rng.percent(self.bandit_pct(cv.town, cv.guards));
        let mut gold = self.caravan_gold(cv, cv.back);
        if bandits {
            gold = gold * c::BANDIT_PAY_PCT / 100;
        }
        self.gold += gold;
        r.gold += gold;
        let town = &c::TOWNS[cv.town];
        self.put(town.sells, (cv.n / town.back_per).max(1), r, false);
        let xp = (60 + 2 * cv.n + gold / 4) * self.mult_bp() / 10_000;
        self.gain(c::TRADING, xp, r);
        self.count("caravans", 1);
        r.caravans.push((cv.town, gold, bandits));
    }

    /// Orders filled in region `reg`'s towns (the Vale counts every order, as before T15).
    pub fn orders_in(&self, reg: usize) -> u64 {
        if reg == 0 {
            return self.counts.get("orders").copied().unwrap_or(0);
        }
        c::TOWNS
            .iter()
            .filter(|t| t.region == reg)
            .map(|t| self.counts.get(&format!("orders:{}", t.id)).copied().unwrap_or(0))
            .sum()
    }

    /// Whether the next Trader rank's needs are met in region `reg`.
    pub fn trader_ready_in(&self, reg: usize) -> bool {
        let Some(rk) = c::TRADER_RANKS[reg].get(self.rank_in(Engine::Trader, reg) + 1) else {
            return false;
        };
        let each = c::TOWNS
            .iter()
            .filter(|t| t.region == reg)
            .all(|t| self.counts.get(&format!("orders:{}", t.id)).copied().unwrap_or(0) >= rk.each_town);
        self.tier_open(Engine::Trader, reg)
            && self.level(c::TRADING) >= rk.level
            && self.orders_in(reg) >= rk.orders
            && self.gold >= rk.fee
            && each
            && (!rk.charter || self.rank_in(Engine::Nobility, reg) >= c::CHARTER_RANK)
    }

    pub fn trader_ready(&self) -> bool {
        self.trader_ready_in(0)
    }

    pub fn rise_trader_in(&mut self, reg: usize) -> bool {
        if !self.trader_ready_in(reg) {
            return false;
        }
        let next = self.rank_in(Engine::Trader, reg) + 1;
        self.gold -= c::TRADER_RANKS[reg][next].fee;
        self.set_rank(Engine::Trader, reg, next);
        true
    }

    pub fn rise_trader(&mut self) -> bool {
        self.rise_trader_in(0)
    }

    // ---------- the Inventor (T14) ----------

    pub fn can_research(&self, id: &str) -> bool {
        let Some(bp) = c::blueprint_index(id).map(|b| &c::BLUEPRINTS[b]) else {
            return false;
        };
        self.engine_open(Engine::Inventor)
            && self.region_open(bp.region)
            && self.research.is_none()
            && !self.blueprints.iter().any(|k| k == id)
            && self.level(c::INVENTION) >= bp.level
            && self.gold >= bp.gold
            && bp.needs.iter().all(|(i, n)| self.have(i) >= *n)
    }

    pub fn start_research(&mut self, id: &str, r: &mut Report) -> bool {
        if !self.can_research(id) {
            return false;
        }
        let bp = &c::BLUEPRINTS[c::blueprint_index(id).unwrap_or(0)];
        self.gold -= bp.gold;
        for (i, n) in bp.needs {
            self.take(i, *n, r);
        }
        self.research = Some(Research {
            id: id.to_string(),
            progress: 0,
        });
        true
    }

    pub fn toggle_machine(&mut self, id: &str) {
        if self.machines_off.iter().any(|k| k == id) {
            self.machines_off.retain(|k| k != id);
        } else if c::MACHINES.iter().any(|m| m.id == id) {
            self.machines_off.push(id.to_string());
        }
    }

    pub fn inventor_ready_in(&self, reg: usize) -> bool {
        let Some(rk) = c::INVENTOR_RANKS[reg].get(self.rank_in(Engine::Inventor, reg) + 1) else {
            return false;
        };
        let mine: Vec<&c::Machine> = c::MACHINES.iter().filter(|m| m.region == reg).collect();
        let built = mine.iter().filter(|m| self.have(m.id) > 0).count();
        self.tier_open(Engine::Inventor, reg)
            && self.level(c::INVENTION) >= rk.level
            && self.room(c::WORKSHOP) >= rk.workshop
            && (!rk.machine || built > 0)
            && (!rk.star_core || self.have("star_core") > 0)
            && (!rk.every_machine || built == mine.len())
    }

    pub fn inventor_ready(&self) -> bool {
        self.inventor_ready_in(0)
    }

    pub fn rise_inventor_in(&mut self, reg: usize, r: &mut Report) -> bool {
        if !self.inventor_ready_in(reg) {
            return false;
        }
        let next = self.rank_in(Engine::Inventor, reg) + 1;
        if c::INVENTOR_RANKS[reg][next].star_core {
            self.take("star_core", 1, r);
        }
        self.set_rank(Engine::Inventor, reg, next);
        true
    }

    pub fn rise_inventor(&mut self, r: &mut Report) -> bool {
        self.rise_inventor_in(0, r)
    }

    // ---------- the Cosmic (T13) ----------

    pub fn cosmic_ready_in(&self, reg: usize) -> bool {
        let Some(rk) = c::COSMIC_RANKS[reg].get(self.rank_in(Engine::Cosmic, reg) + 1) else {
            return false;
        };
        self.tier_open(Engine::Cosmic, reg)
            && self.level(c::ATTUNEMENT) >= rk.level
            && self.have("starlight") >= rk.starlight
            && self.have("stardust") >= rk.stardust
            && self.have("lens") >= rk.lenses
    }

    pub fn cosmic_ready(&self) -> bool {
        self.cosmic_ready_in(0)
    }

    pub fn rise_cosmic_in(&mut self, reg: usize, r: &mut Report) -> bool {
        if !self.cosmic_ready_in(reg) {
            return false;
        }
        let next = self.rank_in(Engine::Cosmic, reg) + 1;
        let rk = &c::COSMIC_RANKS[reg][next];
        self.take("starlight", rk.starlight, r);
        self.take("stardust", rk.stardust, r);
        self.take("lens", rk.lenses, r);
        self.set_rank(Engine::Cosmic, reg, next);
        true
    }

    pub fn rise_cosmic(&mut self, r: &mut Report) -> bool {
        self.rise_cosmic_in(0, r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xp;

    fn run(s: &mut State, ms: u64) -> Report {
        let mut r = Report::begin(s, ms, 0);
        s.advance(ms, &mut r);
        r
    }

    fn r0(s: &State) -> Report {
        Report::begin(s, 0, 0)
    }

    /// Every skill at 30: total level 540, past every engine's gate.
    fn opened(seed: u64) -> State {
        let mut s = State::new(seed);
        for i in 0..c::SKILLS.len() {
            s.xp[i] = xp::at(30);
        }
        s
    }

    #[test]
    fn engines_open_one_at_a_time_by_total_level() {
        let mut s = State::new(1);
        assert!(!s.engine_open(Engine::Trader));
        assert!(!s.unlocked("plank"));
        // 18 skills: 17 at level 8 (136) and Hitpoints at 14 make 150.
        for i in 0..c::SKILLS.len() {
            s.xp[i] = xp::at(8);
        }
        s.xp[c::HITPOINTS] = xp::at(14);
        assert_eq!(s.total_level(), 150);
        assert!(s.engine_open(Engine::Trader));
        assert!(s.unlocked("plank"));
        assert!(!s.engine_open(Engine::Nobility));
        let mut s = opened(1);
        assert!(s.engine_open(Engine::Nobility));
        // the Inventor and the Cosmic also need their rooms
        assert!(!s.engine_open(Engine::Inventor) && !s.engine_open(Engine::Cosmic));
        s.rooms[c::WORKSHOP] = 1;
        s.rooms[c::OBSERVATORY] = 1;
        assert!(s.engine_open(Engine::Inventor) && s.engine_open(Engine::Cosmic));
    }

    #[test]
    fn ashes_are_compost_and_a_composted_crop_grows_in_three_quarters_of_its_time() {
        // Potato: 10 minutes; with compost 7 m 30 s.
        let mut s = State::new(1);
        s.bank.insert("potato_seed".into(), 2);
        s.bank.insert("ashes".into(), 1);
        let mut r = r0(&s);
        assert!(s.plant("potato", &mut r));
        assert_eq!(s.have("ashes"), 0);
        assert_eq!(s.grow_left(0), Some(450_000));
        assert!(s.plant("potato", &mut r));
        assert_eq!(s.grow_left(1), Some(600_000));
    }

    #[test]
    fn a_plan_holds_four_steps() {
        let mut s = State::new(1);
        for _ in 0..4 {
            assert!(s.plan_add("tree"));
        }
        assert!(!s.plan_add("tree"));
        assert_eq!(s.plan.len(), 4);
    }

    #[test]
    fn a_recipe_drops_from_its_enemy_and_unlocks_its_action() {
        let mut s = State::new(7);
        s.xp[c::COOKING] = xp::at(12);
        assert!(!s.unlocked("farmers_stew"));
        s.bank.insert("bronze_sword".into(), 1);
        s.bank.insert("shrimp".into(), 100_000);
        s.start("chicken", 0);
        // One in 64 kills; ten hours is well over a thousand kills.
        let r = run(&mut s, 10 * c::HOUR);
        assert!(s.kills["chicken"] > 1_000, "{}", s.kills["chicken"]);
        assert_eq!(s.recipes, vec!["farmers_stew".to_string()]);
        assert_eq!(r.recipes.len(), 1);
        assert!(s.unlocked("farmers_stew"));
    }

    #[test]
    fn a_second_copy_of_a_recipe_sells_for_gold() {
        let mut s = State::new(1);
        let mut r = r0(&s);
        let rc = &c::RECIPES[0];
        s.find_recipe(rc, &mut r);
        assert_eq!(s.gold, 0);
        s.find_recipe(rc, &mut r);
        // one in 64, ten gold for each
        assert_eq!(s.gold, 640);
        assert_eq!(s.recipes.len(), 1);
    }

    #[test]
    fn recipes_are_kept_through_prestige_and_engines_are_not() {
        let mut s = opened(1);
        s.recipes.push("mossplate".into());
        s.ranks = [1, 2, 0, 0];
        s.rooms[c::KITCHEN] = 3;
        s.do_prestige(5);
        assert_eq!(s.recipes, vec!["mossplate".to_string()]);
        assert_eq!(s.ranks, [0; 4]);
        assert_eq!(s.rooms, vec![0; c::ROOMS.len()]);
    }

    #[test]
    fn prices_move_in_a_twelve_hour_cycle_each_town_three_hours_behind() {
        let s = State::new(1);
        assert_eq!(s.price_pct(0, 0), 80);
        assert_eq!(s.price_pct(0, c::HOUR), 88);
        assert_eq!(s.price_pct(0, 6 * c::HOUR), 128);
        assert_eq!(s.price_pct(0, 9 * c::HOUR), 104);
        assert_eq!(s.price_pct(0, 12 * c::HOUR), 80);
        assert_eq!(s.price_pct(1, 0), 104);
    }

    #[test]
    fn a_guarded_caravan_comes_back_with_gold_spices_and_trading_xp() {
        // 100 logs to Riverton: an hour there and back, sold at the price an
        // hour from now (88%): 100 · 1 gold · 88% = 88 gold. Guards (200 gold)
        // take the bandits' 10% to nothing. Back: one spice for every ten logs.
        // Trading XP: 60 + 2·100 + 88/4 = 282.
        let mut s = opened(1);
        s.bank.insert("cart".into(), 1);
        s.bank.insert("logs".into(), 150);
        s.gold = 200;
        let mut r = r0(&s);
        assert!(s.send_caravan(0, "logs", true, &mut r));
        assert_eq!((s.have("logs"), s.gold), (50, 0));
        assert!(!s.send_caravan(0, "logs", false, &mut r), "one cart, one caravan");
        let xp0 = s.xp[c::TRADING];
        let r = run(&mut s, c::HOUR);
        assert_eq!(s.gold, 88);
        assert_eq!(s.have("spices"), 10);
        assert_eq!(s.xp[c::TRADING] - xp0, 282);
        assert_eq!(r.caravans, vec![(0, 88, false)]);
        assert!(s.caravans.is_empty());
    }

    #[test]
    fn bandits_halve_what_a_caravan_earns_and_take_nothing() {
        let mut s = opened(3);
        let cv = Caravan {
            town: 3,
            item: "gold_amulet".into(),
            n: 10,
            back: 0,
            guards: false,
        };
        // Emberfall: danger 40, less a quarter of your best combat level (30 → 7).
        assert_eq!(s.bandit_pct(3, false), 33);
        assert_eq!(s.bandit_pct(3, true), 8);
        let full = s.caravan_gold(&cv, 0);
        let mut met = false;
        for _ in 0..200 {
            let mut r = r0(&s);
            s.caravan_home(&cv, &mut r);
            if r.caravans[0].2 {
                assert_eq!(r.caravans[0].1, full / 2);
                met = true;
            } else {
                assert_eq!(r.caravans[0].1, full);
            }
        }
        assert!(met);
    }

    #[test]
    fn an_order_is_filled_once_and_new_ones_come_every_six_hours() {
        let mut s = opened(1);
        let o = s.orders_now(0)[0].clone();
        s.bank.insert(o.item.into(), o.n);
        let mut r = r0(&s);
        assert!(s.fill_order(0, 0, &mut r));
        assert_eq!(s.gold, o.gold);
        assert_eq!(s.have(o.item), 0);
        s.bank.insert(o.item.into(), o.n);
        assert!(!s.fill_order(0, 0, &mut r), "the same order twice");
        assert_eq!(s.counts["orders"], 1);
        // Orders are the same for the same window, so they can be shown ahead.
        assert_eq!(s.order(0, 0, 0), o);
        let later = s.order(0, 1, 0).key;
        assert_ne!(later, o.key);
    }

    #[test]
    fn a_room_costs_planks_nails_limestone_and_gold_and_the_kitchen_cooks_faster() {
        // Kitchen level 1: 10 planks, 20 nails, 10 limestone, 500 gold; 150 XP.
        let mut s = opened(1);
        assert!(!s.can_build(c::KITCHEN));
        s.bank.insert("plank".into(), 10);
        s.bank.insert("nails".into(), 20);
        s.bank.insert("limestone".into(), 10);
        s.gold = 500;
        let xp0 = s.xp[c::CONSTRUCTION];
        assert_eq!(s.duration("cook_shrimp"), 2_000);
        let mut r = r0(&s);
        assert!(s.build_room(c::KITCHEN, &mut r));
        assert_eq!(s.rooms[c::KITCHEN], 1);
        assert_eq!(
            (s.gold, s.have("plank"), s.have("nails"), s.have("limestone")),
            (0, 0, 0, 0)
        );
        assert_eq!(s.xp[c::CONSTRUCTION] - xp0, 150);
        assert_eq!(s.duration("cook_shrimp"), 1_800);
    }

    #[test]
    fn a_noble_rank_from_squire_on_needs_a_deed_by_champion_or_tribute() {
        let mut s = opened(1);
        s.rooms[c::KITCHEN] = 2;
        s.gold = 25_000;
        let mut r = r0(&s);
        assert!(
            s.rise_noble(false, &mut r),
            "Freeholder: gold and a room, no deed"
        );
        assert_eq!(s.gold, 20_000);
        assert!(!s.rise_noble(false, &mut r), "Squire wants the goblin warlord");
        assert!(
            !s.rise_noble(true, &mut r),
            "or 10 gold rings and 10 leather bodies"
        );
        s.bank.insert("gold_ring".into(), 10);
        s.bank.insert("leather_body".into(), 10);
        assert!(s.rise_noble(true, &mut r));
        assert_eq!(s.rank(Engine::Nobility), 2);
        assert_eq!((s.gold, s.have("gold_ring")), (0, 0));
        // the next deed by champion
        s.rooms[c::KITCHEN] = 4;
        s.gold = 50_000;
        s.kills.insert("broodmother".into(), 1);
        assert!(s.rise_noble(false, &mut r));
        assert_eq!(s.rank(Engine::Nobility), 3);
    }

    #[test]
    fn a_servant_works_at_half_speed_and_the_estate_pays_by_the_hour() {
        // Squire: one servant, 150 gold an hour. Trees: 3 s, so 6 s for a
        // servant: 600 logs in an hour. At the hour: +150 income, −25 wage, one
        // shrimp eaten.
        let mut s = opened(1);
        s.ranks[1] = 2;
        s.bank.insert("shrimp".into(), 5);
        assert!(s.set_servant(0, "tree"));
        assert!(!s.set_servant(1, "tree"), "one slot at Squire");
        run(&mut s, c::HOUR);
        assert_eq!(s.have("logs"), 600);
        assert_eq!(s.gold, 125);
        assert_eq!(s.have("shrimp"), 4);
        assert_eq!(s.mastery("tree"), 1, "a servant's work trains no mastery");
    }

    #[test]
    fn an_unpaid_servant_waits_for_the_next_hour() {
        let mut s = opened(1);
        s.ranks[1] = 2;
        assert!(s.set_servant(0, "tree"));
        // No food: at the first hour the servant goes unpaid and stops.
        run(&mut s, c::HOUR);
        assert_eq!(s.have("logs"), 600);
        assert!(s.servants[0].unpaid);
        run(&mut s, c::HOUR);
        assert_eq!(s.have("logs"), 600);
        // Food and gold at the next hour: back to work.
        s.bank.insert("shrimp".into(), 1);
        run(&mut s, c::HOUR);
        assert!(!s.servants[0].unpaid);
        run(&mut s, c::HOUR);
        assert_eq!(s.have("logs"), 1_200);
    }

    #[test]
    fn a_machine_works_alongside_you_and_burns_fuel() {
        // The sawmill: a log into a plank every 8 s at Workshop 1, burning a
        // log as fuel when there is no coal. Ten logs make five planks.
        let mut s = opened(1);
        s.rooms[c::WORKSHOP] = 1;
        s.bank.insert("sawmill".into(), 1);
        s.bank.insert("logs".into(), 10);
        run(&mut s, 60_000);
        assert_eq!(s.have("plank"), 5);
        assert_eq!(s.have("logs"), 0);
        // With coal, coal burns first; Workshop 3 is 20% faster: 6.4 s.
        s.rooms[c::WORKSHOP] = 3;
        s.bank.insert("logs".into(), 3);
        s.bank.insert("coal".into(), 10);
        assert_eq!(s.machine_cycle(&c::MACHINES[0]), 6_400);
        run(&mut s, 3 * 6_400);
        assert_eq!((s.have("plank"), s.have("logs"), s.have("coal")), (8, 0, 7));
    }

    #[test]
    fn a_switched_off_machine_does_nothing() {
        let mut s = opened(1);
        s.rooms[c::WORKSHOP] = 1;
        s.bank.insert("sawmill".into(), 1);
        s.bank.insert("logs".into(), 10);
        s.toggle_machine("sawmill");
        run(&mut s, 60_000);
        assert_eq!(s.have("plank"), 0);
    }

    #[test]
    fn research_finishes_a_blueprint_and_its_gadget_makes_a_skill_faster() {
        // The axe gadget: Invention 10, 20 wood parts and 2,000 gold, 20 minutes, 200 XP.
        let mut s = opened(1);
        s.rooms[c::WORKSHOP] = 1;
        s.bank.insert("wood_parts".into(), 50);
        s.gold = 2_000;
        assert!(!s.unlocked("build_axe_gadget"));
        let mut r = r0(&s);
        assert!(s.start_research("axe_gadget", &mut r));
        assert_eq!((s.gold, s.have("wood_parts")), (0, 30));
        let xp0 = s.xp[c::INVENTION];
        run(&mut s, 20 * c::MINUTE - 1);
        assert!(s.blueprints.is_empty());
        let r = run(&mut s, 1);
        assert_eq!(r.research, vec!["axe_gadget".to_string()]);
        assert_eq!(s.xp[c::INVENTION] - xp0, 200);
        assert!(s.unlocked("build_axe_gadget"));
        assert_eq!(s.duration("tree"), 3_000);
        s.bank.insert("axe_gadget".into(), 1);
        assert!(!s.unlocked("build_axe_gadget"), "built once");
        assert_eq!(s.duration("tree"), 2_700);
    }

    #[test]
    fn the_sky_turns_and_a_skill_under_its_sign_gathers_starlight() {
        // The Axe is up for the first 80 minutes: woodcutting gathers
        // starlight, one a chop; mining does not. Total level 450: the Coast's
        // sky turns too (its first sign is for Fishing and Trading), the
        // Peaks' does not.
        let mut s = State::new(1);
        for i in 0..c::SKILLS.len() {
            s.xp[i] = xp::at(25);
        }
        s.rooms[c::OBSERVATORY] = 1;
        assert_eq!(s.sign(), 0);
        s.start("tree", 0);
        run(&mut s, 30_000);
        assert_eq!(s.have("starlight"), 10);
        s.start("copper", 0);
        run(&mut s, 30_000);
        assert_eq!(s.have("starlight"), 10);
        assert_eq!(s.sign_at(c::SIGN_MS), 1);
        assert_eq!(s.sign_at(6 * c::SIGN_MS), 0);
    }

    #[test]
    fn the_harvest_moon_and_compost_together_grow_a_crop_in_nine_sixteenths() {
        // Potato 10 min under The Sprout with compost: 600 s · ¾ · ¾ = 337.5 s.
        let mut s = opened(1);
        s.rooms[c::OBSERVATORY] = 1;
        s.sim_ms = 2 * c::SIGN_MS;
        s.bank.insert("potato_seed".into(), 1);
        s.bank.insert("ashes".into(), 1);
        let mut r = r0(&s);
        assert!(s.plant("potato", &mut r));
        assert_eq!(s.grow_left(0), Some(337_500));
    }

    #[test]
    fn cosmic_ranks_lengthen_the_away_limit() {
        let mut s = opened(1);
        assert_eq!(s.away_cap(), 24 * c::HOUR);
        s.rooms[c::OBSERVATORY] = 1;
        s.ranks[3] = 1;
        assert_eq!(s.away_cap(), 36 * c::HOUR);
        s.ranks[3] = 3;
        assert_eq!(s.away_cap(), 48 * c::HOUR);
    }

    #[test]
    fn a_cosmic_rank_spends_starlight_and_lenses() {
        // Seer: Attunement 20, 500 starlight, 2 lenses.
        let mut s = opened(1);
        s.rooms[c::OBSERVATORY] = 1;
        s.bank.insert("starlight".into(), 600);
        s.bank.insert("lens".into(), 1);
        let mut r = r0(&s);
        assert!(!s.rise_cosmic(&mut r));
        s.bank.insert("lens".into(), 2);
        assert!(s.rise_cosmic(&mut r));
        assert_eq!((s.have("starlight"), s.have("lens")), (100, 0));
        assert_eq!(s.rank(Engine::Cosmic), 1);
    }

    #[test]
    fn astral_runes_wait_for_the_astral_adept() {
        let mut s = opened(1);
        s.xp[c::RUNECRAFTING] = xp::at(60);
        s.rooms[c::OBSERVATORY] = 1;
        assert!(!s.unlocked("astral_runes"));
        s.ranks[3] = c::ASTRAL_RANK;
        assert!(s.unlocked("astral_runes"));
    }

    #[test]
    fn a_trader_rank_needs_its_level_and_its_orders() {
        // Merchant: Trading 20 and ten orders filled.
        let mut s = opened(1);
        s.xp[c::TRADING] = xp::at(20);
        s.counts.insert("orders".into(), 9);
        assert!(!s.rise_trader());
        s.counts.insert("orders".into(), 10);
        assert!(s.rise_trader());
        assert_eq!(s.rank(Engine::Trader), 1);
        // Guild member: Trading 40 and a 100,000 gold fee.
        s.xp[c::TRADING] = xp::at(40);
        s.gold = 100_000;
        assert!(s.rise_trader());
        assert_eq!(s.gold, 0);
    }

    #[test]
    fn emberfall_needs_an_earls_charter() {
        let mut s = opened(1);
        s.xp[c::TRADING] = xp::at(50);
        assert!(!s.town_open(c::EMBERFALL));
        s.ranks[1] = c::CHARTER_RANK;
        assert!(s.town_open(c::EMBERFALL));
    }

    #[test]
    fn spices_make_food_heal_more_and_are_used_up() {
        let fight = |spices: u64| {
            let mut s = State::new(21);
            s.bank.insert("bronze_sword".into(), 1);
            // shrimp heal 30, or 37 with a spice: less than a full bar, so none is wasted
            s.bank.insert("shrimp".into(), 10_000);
            s.bank.insert("spices".into(), spices);
            s.start("hill_giant", 0);
            let r = run(&mut s, 30 * c::MINUTE);
            (r.used.get("shrimp").copied().unwrap_or(0), s.have("spices"))
        };
        let (plain, _) = fight(0);
        let (spiced, left) = fight(10_000);
        assert!(spiced < plain, "{spiced} vs {plain}");
        assert_eq!(10_000 - left, spiced);
    }

    #[test]
    fn engines_running_together_give_the_same_result_however_time_is_split() {
        // IG-3 with every timer at once: your action, a servant, a machine,
        // research, a caravan and the estate's hours.
        let mut a = opened(5);
        a.rooms = vec![1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0];
        a.ranks[1] = 2;
        a.bank.insert("shrimp".into(), 50);
        a.bank.insert("cart".into(), 2);
        a.bank.insert("logs".into(), 400);
        a.bank.insert("iron_ore".into(), 30);
        a.bank.insert("coal".into(), 20);
        a.bank.insert("sawmill".into(), 1);
        a.bank.insert("auto_smelter".into(), 1);
        a.bank.insert("wood_parts".into(), 40);
        a.gold = 3_000;
        let mut r = r0(&a);
        assert!(a.set_servant(0, "copper"));
        assert!(a.send_caravan(0, "logs", false, &mut r));
        assert!(a.start_research("axe_gadget", &mut r));
        a.plan_add("tree");
        a.plan_add("burn_logs");
        a.plan_start();
        let mut b = a.clone();
        run(&mut a, 3 * c::HOUR);
        for _ in 0..(3 * 3_600) {
            run(&mut b, 1_000);
        }
        assert_eq!(a, b);
        assert_eq!(a.blueprints, vec!["axe_gadget".to_string()]);
        assert!(a.have("copper_ore") > 0 && a.have("iron_bar") > 0);
    }
}

/// The full game (T15): regions, their tiers and each skill's new mechanic.
#[cfg(test)]
mod region_tests {
    use super::*;
    use crate::xp;

    fn run(s: &mut State, ms: u64) -> Report {
        let mut r = Report::begin(s, ms, 0);
        s.advance(ms, &mut r);
        r
    }

    /// Every skill at `level`: total level 18 × level, with Hitpoints at the same.
    fn at(level: u32) -> State {
        let mut s = State::new(3);
        for i in 0..c::SKILLS.len() {
            s.xp[i] = xp::at(level);
        }
        s
    }

    #[test]
    fn regions_open_at_300_500_and_700_and_close_at_prestige() {
        // 18 skills: level 16 is 288, 17 is 306; 27 is 486, 28 is 504; 38 is 684, 39 is 702.
        assert_eq!(at(16).regions_open(), 1);
        assert_eq!(at(17).regions_open(), 2);
        assert_eq!(at(27).regions_open(), 2);
        assert_eq!(at(28).regions_open(), 3);
        assert_eq!(at(38).regions_open(), 3);
        assert_eq!(at(39).regions_open(), 4);
        let mut s = at(50);
        assert_eq!(s.total_level(), 900);
        assert!(s.can_prestige());
        s.do_prestige(1);
        assert_eq!(s.regions_open(), 1);
    }

    #[test]
    fn a_region_row_waits_for_its_region() {
        // Driftwood is a Coast tree at Woodcutting 3.
        let s = at(16);
        assert!(!s.unlocked("driftwood_tree"));
        assert!(at(17).unlocked("driftwood_tree"));
        assert!(!s.unlocked("shore_crab"));
        assert!(!s.unlocked("sunken_ship"));
        assert!(at(17).unlocked("sunken_ship"));
    }

    #[test]
    fn the_rare_tree_gives_twice_the_logs() {
        let mut s = at(99);
        let rare = s.rare_tree().expect("the Coast is open");
        let logs = c::ACTIONS[c::action_index(rare).unwrap()].makes.0;
        s.start(rare, 0);
        let ms = s.duration(rare);
        run(&mut s, ms);
        // one chop at mastery 1: no double from mastery, so the 2 is the rare tree
        assert_eq!(s.have(logs), 2);
        // without the Coast there is no rare tree
        assert_eq!(at(16).rare_tree(), None);
    }

    #[test]
    fn mining_finds_gems_once_the_coast_is_open() {
        let count = |level: u32| {
            let mut s = at(level);
            s.start("copper", 0);
            run(&mut s, 2_000 * 3_000);
            c::GEMS.iter().map(|(g, _)| s.have(g)).sum::<u64>()
        };
        // 2,000 copper ores at one in 40: about 50 gems; none before the Coast
        // (from total level 18, the mining raises it to about 45)
        assert_eq!(count(1), 0);
        assert!((25..=80).contains(&count(17)), "{}", count(17));
    }

    #[test]
    fn better_logs_leave_fine_ashes_which_grow_a_crop_in_three_fifths() {
        // Maple logs are level 45: fine ashes once the Coast is open.
        let mut s = at(50);
        s.bank.insert("maple_logs".into(), 1);
        s.start("burn_maple", 0);
        run(&mut s, 5_000);
        assert_eq!((s.have("fine_ashes"), s.have("ashes")), (1, 0));
        let mut t = at(10);
        t.bank.insert("maple_logs".into(), 1);
        t.xp[c::FIREMAKING] = xp::at(50);
        t.start("burn_maple", 0);
        run(&mut t, 5_000);
        assert_eq!(t.have("fine_ashes"), 0);
        // Potato, 10 minutes, with fine compost: 6 minutes.
        s.bank.insert("potato_seed".into(), 1);
        s.sim_ms = 80 * c::MINUTE; // past the first sign of every sky
        let mut r = Report::begin(&s, 0, 0);
        let moons = s.signs_for(c::FARMING);
        assert!(s.plant("potato", &mut r));
        let mut ms = 360_000;
        for _ in 0..moons {
            ms = ms * c::HARVEST_MOON_PCT / 100;
        }
        assert_eq!(s.grow_left(0), Some(ms));
    }

    #[test]
    fn smithing_sells_masterworks_and_cooking_makes_perfect_dishes() {
        let mut s = at(50);
        s.bank.insert("bronze_bar".into(), 400);
        s.start("bronze_sword", 0);
        let r = run(&mut s, 400 * 3_000);
        // 400 swords at one in 20: about 20 masterworks of 2 × 20 gold
        assert!(
            r.masterworks > 0 && r.masterworks.is_multiple_of(40),
            "{}",
            r.masterworks
        );
        let mut s = at(50);
        s.bank.insert("raw_shrimp".into(), 200);
        s.start("cook_shrimp", 0);
        run(&mut s, 200 * 2_000);
        // one in ten perfect: about 220 shrimp from 200
        assert!(s.have("shrimp") > 200, "{}", s.have("shrimp"));
    }

    #[test]
    fn charcoal_burns_first_and_an_upgrade_doubles_a_machine() {
        let mut s = at(30);
        s.rooms[c::WORKSHOP] = 1;
        s.bank.insert("sawmill".into(), 1);
        s.bank.insert("sawmill_gearbox".into(), 1);
        s.bank.insert("logs".into(), 3);
        s.bank.insert("charcoal".into(), 2);
        s.bank.insert("coal".into(), 5);
        // two 8 s cycles: two logs into four planks, two charcoal burned, no coal
        run(&mut s, 16_000);
        assert_eq!(
            (
                s.have("plank"),
                s.have("charcoal"),
                s.have("coal"),
                s.have("logs")
            ),
            (4, 0, 5, 1)
        );
    }

    #[test]
    fn the_tide_mill_burns_no_fuel() {
        // cork logs into cork planks every 9 s, with no coal or logs to burn
        let mut s = at(30);
        s.rooms[c::WORKSHOP] = 1;
        s.bank.insert("tide_mill".into(), 1);
        s.bank.insert("cork_logs".into(), 3);
        run(&mut s, 27_000);
        assert_eq!(s.have("cork_plank"), 3);
    }

    #[test]
    fn furniture_counts_as_a_level_of_its_room() {
        // The Kitchen at 1 is 10%; the tea set adds a level: 20%. Shrimp: 2 s → 1.6 s.
        let mut s = at(30);
        s.rooms[c::KITCHEN] = 1;
        assert_eq!(s.duration("cook_shrimp"), 1_800);
        s.bank.insert("tea_set".into(), 1);
        assert_eq!(s.duration("cook_shrimp"), 1_600);
    }

    #[test]
    fn each_later_region_raises_every_rooms_top_level() {
        // The Kitchen: 5 in the Vale, then 7, 9, 10 (two a region, at most 10).
        assert_eq!(at(16).room_max(c::KITCHEN), 5);
        assert_eq!(at(17).room_max(c::KITCHEN), 7);
        assert_eq!(at(28).room_max(c::KITCHEN), 9);
        assert_eq!(at(39).room_max(c::KITCHEN), 10);
        // the Garden stops at 4
        assert_eq!(at(39).room_max(c::GARDEN), 4);
    }

    #[test]
    fn jewellery_and_region_armour_make_you_stronger() {
        let mut s = at(10);
        let chicken = &c::ENEMIES[c::enemy_index("chicken").unwrap()];
        s.bank.insert("bronze_sword".into(), 1);
        // 20 + 10·3 + bronze tier 1·10 + 15 against the weak style = 75; +10% = 82
        assert_eq!(s.max_hit(chicken), 75);
        s.bank.insert("ring_of_power_1".into(), 1);
        assert_eq!(s.max_hit(chicken), 82);
        // Hitpoints 10: 100, +50 warding, +20 for a cobalt platebody
        assert_eq!(s.max_hp(), 100);
        s.bank.insert("amulet_of_warding_1".into(), 1);
        s.bank.insert("cobalt_platebody".into(), 1);
        assert_eq!(s.max_hp(), 170);
        assert_eq!(s.armour().1, 4);
    }

    #[test]
    fn a_dungeon_runs_its_stages_and_pays_a_chest() {
        let mut s = at(99);
        s.bank.insert("abyssal_sword".into(), 1);
        s.bank.insert("shark".into(), 100_000);
        s.start("sunken_ship", 0);
        assert_eq!(s.foe("sunken_ship"), c::enemy_index("shore_crab"));
        let r = run(&mut s, 6 * c::HOUR);
        assert!(!r.chests.is_empty());
        assert!(s.have("fine_parts") >= c::CHEST_FINE_PARTS * r.chests.len() as u64);
        // nine stages: each chest comes after a kraken
        assert!(s.kills["kraken"] >= r.chests.len() as u64);
        assert!(s.kills["shore_crab"] >= s.kills["kraken"]);
    }

    #[test]
    fn a_dungeon_retreat_starts_it_over() {
        let mut s = at(99);
        s.bank.insert("abyssal_sword".into(), 1);
        s.bank.insert("shark".into(), 3);
        s.start("sunken_ship", 0);
        let r = run(&mut s, 6 * c::HOUR);
        assert_eq!(r.stop, Some(crate::game::Stop::Food));
        assert_eq!(s.dungeon_stage, 0);
    }

    #[test]
    fn each_region_has_its_own_tier_of_every_ladder_and_they_add_up() {
        let mut s = at(30);
        s.ranks[1] = 2; // Squire in the Vale: 150 an hour
        assert_eq!(s.income(), 150);
        s.rooms[c::KITCHEN] = 4;
        s.gold = 50_000;
        let mut r = Report::begin(&s, 0, 0);
        // The Coast's second rank: 50,000 gold and 16 room levels
        assert!(!s.rise_noble_in(1, false, &mut r));
        s.rooms[c::GARDEN] = 2;
        s.rooms[c::STABLES] = 3;
        s.rooms[c::QUARTERS] = 3;
        s.rooms[c::WORKSHOP] = 4;
        assert!(s.rise_noble_in(1, false, &mut r));
        assert_eq!(s.rank_in(Engine::Nobility, 1), 1);
        assert_eq!(s.rank(Engine::Nobility), 2);
        // income: 150 + the Coast's 300
        assert_eq!(s.income(), 450);
    }

    #[test]
    fn a_coast_town_needs_a_ship() {
        let mut s = at(30);
        s.bank.insert("cart".into(), 1);
        let t = c::town_index("brinewick").unwrap();
        let item = c::TOWNS[t].wants[0];
        s.bank.insert(item.into(), 50);
        let mut r = Report::begin(&s, 0, 0);
        assert!(!s.send_caravan(t, item, false, &mut r), "a cart does not sail");
        s.bank.insert("ship".into(), 1);
        assert!(s.send_caravan(t, item, false, &mut r));
    }

    #[test]
    fn a_skill_under_signs_in_two_skies_gathers_twice_the_starlight() {
        // Mining is under the Coast's Tide and the Peaks' Summit; find a
        // moment, on a 20-minute step, when both are up, and mine once.
        let mut s = at(30);
        s.rooms[c::OBSERVATORY] = 1;
        let mut t = 0;
        while s.signs_for(c::MINING) < 2 {
            t += 20 * c::MINUTE;
            s.sim_ms = t;
            assert!(t < 200 * c::HOUR, "no constellation for Mining in 200 hours");
        }
        let n = s.signs_for(c::MINING);
        s.start("copper", 0);
        run(&mut s, 3_000);
        assert_eq!(s.have("starlight"), n);
    }

    #[test]
    fn a_later_skys_rank_lengthens_the_away_limit_further() {
        let mut s = at(30);
        s.rooms[c::OBSERVATORY] = 1;
        s.ranks[3] = 3;
        assert_eq!(s.away_cap(), 48 * c::HOUR);
        s.region_ranks[0][3] = 4; // the Coast's tier: 60 hours
        assert_eq!(s.away_cap(), 60 * c::HOUR);
    }

    #[test]
    fn the_library_speeds_research_and_the_treasury_income() {
        let mut s = at(39);
        let lib = c::ROOMS.iter().position(|r| r.id == "library").unwrap();
        let tre = c::ROOMS.iter().position(|r| r.id == "treasury").unwrap();
        assert_eq!(s.research_ms("axe_gadget"), Some(20 * c::MINUTE));
        s.rooms[lib] = 2;
        assert_eq!(s.research_ms("axe_gadget"), Some(16 * c::MINUTE));
        s.ranks[1] = 2;
        assert_eq!(s.income(), 150);
        s.rooms[tre] = 1;
        assert_eq!(s.income(), 165);
    }

    #[test]
    fn each_region_opens_one_more_farming_patch() {
        // Farming 30: three patches by level, plus one for each later region open.
        let mut s = at(30);
        assert_eq!(s.patches_open(), 3 + 2);
        s.xp[c::FARMING] = xp::at(30);
        assert_eq!(at(16).patches_open(), 3);
    }
}
