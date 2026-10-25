//! The engines' rules (T10 to T14): recipes from enemies, the Trader, the
//! estate, the Inventor's workshop and the turning sky. Each engine spends
//! what the skills make and gives back things they need; each has a ladder
//! of ranks and a timer that `advance` runs alongside your one action.

use crate::content::{self as c, Engine};
use crate::game::{Caravan, Report, Research, Servant, State};

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
fn mix(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn engine_slot(e: Engine) -> usize {
    match e {
        Engine::Trader => 0,
        Engine::Nobility => 1,
        Engine::Inventor => 2,
        Engine::Cosmic => 3,
    }
}

impl State {
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
            Engine::Inventor => reached && self.rooms[c::WORKSHOP] > 0,
            Engine::Cosmic => reached && self.rooms[c::OBSERVATORY] > 0,
            _ => reached,
        }
    }

    pub fn engine_reached(&self, e: Engine) -> bool {
        self.total_level() >= self.engine_at(e)
    }

    /// The rank on an engine's ladder.
    pub fn rank(&self, e: Engine) -> usize {
        self.ranks[engine_slot(e)]
    }

    /// Whether a skill can be trained: the old skills always, the engine
    /// skills once their engine is open.
    pub fn skill_open(&self, skill: usize) -> bool {
        match c::engine_of_skill(skill) {
            Some(e) => self.engine_open(e),
            None => true,
        }
    }

    /// How much time away counts (P4): longer at Cosmic ranks (IG-Q32 A).
    pub fn away_cap(&self) -> u64 {
        if self.engine_open(Engine::Cosmic) {
            c::AWAY_CAP_BY_COSMIC[self.rank(Engine::Cosmic)]
        } else {
            c::AWAY_CAP_MS
        }
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

    // ---------- the sky (T13) ----------

    /// The sign that is up now: six in turn, 80 minutes each, on game time.
    pub fn sign(&self) -> usize {
        self.sign_at(self.sim_ms)
    }

    pub fn sign_at(&self, at: u64) -> usize {
        (at / c::SIGN_MS % c::SIGNS.len() as u64) as usize
    }

    /// Game time until the sign changes.
    pub fn sign_left(&self) -> u64 {
        c::SIGN_MS - self.sim_ms % c::SIGN_MS
    }

    pub fn harvest_moon(&self) -> bool {
        self.engine_open(Engine::Cosmic) && self.sign() == c::SPROUT
    }

    /// Training a skill under its sign gathers starlight.
    pub(crate) fn starlight(&mut self, skill: usize, r: &mut Report) {
        if self.engine_open(Engine::Cosmic) && c::SIGNS[self.sign()].1.contains(&skill) {
            let n = if self.rank(Engine::Cosmic) >= c::DOUBLE_STARLIGHT_RANK {
                2
            } else {
                1
            };
            self.put("starlight", n, r, false);
        }
    }

    // ---------- timers alongside your action ----------

    /// How long servant `i` takes for one job, or `None` when it cannot work now.
    pub(crate) fn servant_need(&self, i: usize) -> Option<u64> {
        let sv = &self.servants[i];
        if sv.unpaid || sv.job.is_empty() || !self.servant_slots_ok(i) {
            return None;
        }
        let a = &c::ACTIONS[c::action_index(&sv.job)?];
        if !self.unlocked(a.id) || self.missing(a.id).is_some() {
            return None;
        }
        Some(self.duration(a.id) * c::SERVANT_SLOW)
    }

    fn servant_slots_ok(&self, i: usize) -> bool {
        (i as u64) < self.servant_slots()
    }

    /// Servant slots: from the rank (Squire on) and the Servants' quarters.
    pub fn servant_slots(&self) -> u64 {
        if !self.engine_open(Engine::Nobility) {
            return 0;
        }
        c::NOBLE_RANKS[self.rank(Engine::Nobility)].servants + self.rooms[c::QUARTERS]
    }

    /// A machine's cycle, or `None` when it is not built, off, or out of
    /// input or fuel.
    pub(crate) fn machine_need(&self, m: &c::Machine) -> Option<u64> {
        if self.have(m.id) == 0
            || self.machines_off.iter().any(|k| k == m.id)
            || !self.engine_open(Engine::Inventor)
        {
            return None;
        }
        if m.needs.iter().any(|(i, n)| self.have(i) < *n) {
            return None;
        }
        self.fuel_for(m)?;
        Some(self.machine_cycle(m))
    }

    pub fn machine_cycle(&self, m: &c::Machine) -> u64 {
        let w = self.rooms[c::WORKSHOP].max(1);
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

    /// Game time left on the research, or `None` when there is none.
    pub(crate) fn research_need(&self) -> Option<u64> {
        let rs = self.research.as_ref()?;
        let bp = &c::BLUEPRINTS[c::blueprint_index(&rs.id)?];
        if !self.engine_open(Engine::Inventor) {
            return None;
        }
        Some(bp.ms.saturating_sub(rs.progress))
    }

    /// The engines' part of `fire`: everything due now, in a fixed order.
    pub(crate) fn fire_engines(&mut self, r: &mut Report) {
        for i in 0..self.servants.len() {
            if let Some(need) = self.servant_need(i) {
                if self.servants[i].progress >= need {
                    self.servants[i].progress = 0;
                    if let Some(ai) = c::action_index(&self.servants[i].job) {
                        self.produce(ai, false, r);
                    }
                }
            }
        }
        for m in &c::MACHINES {
            if let Some(need) = self.machine_need(m) {
                if self.machine_ms.get(m.id).copied().unwrap_or(0) >= need {
                    self.machine_ms.insert(m.id.to_string(), 0);
                    let fuel = self.fuel_for(m).unwrap_or("logs");
                    for (i, n) in m.needs {
                        self.take(i, *n, r);
                    }
                    self.take(fuel, 1, r);
                    self.put(m.makes.0, m.makes.1, r, false);
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

    /// The estate's hour: its income, then each servant's wage and food.
    fn upkeep(&mut self, r: &mut Report) {
        let income = c::NOBLE_RANKS[self.rank(Engine::Nobility)].income;
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
    /// gold, Construction level), or `None` at its top level.
    pub fn room_cost(&self, k: usize) -> Option<(&'static str, u64, u64, u64, u64, u32)> {
        let room = &c::ROOMS[k];
        let l = self.rooms[k] + 1;
        if l > room.max {
            return None;
        }
        Some((
            c::ROOM_PLANKS[(l - 1) as usize],
            c::ROOM_PLANKS_PER * l,
            c::ROOM_NAILS_PER * l,
            c::ROOM_STONE_PER * l,
            c::ROOM_GOLD_PER * l * l,
            room.level + 10 * (l as u32 - 1),
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
        self.rooms[k] += 1;
        let l = self.rooms[k];
        let xp = c::ROOM_XP_PER * l * l * self.mult_bp() / 10_000;
        self.gain(c::CONSTRUCTION, xp, r);
        true
    }

    /// What stands between you and the next noble rank, if anything:
    /// (gold short, rooms short, the deed done by champion, the tribute held).
    pub fn noble_next(&self) -> Option<(u64, u64, bool, bool)> {
        let next = self.rank(Engine::Nobility) + 1;
        let rk = c::NOBLE_RANKS.get(next)?;
        let champion = next < c::DEED_FROM
            || self
                .kills
                .get(c::CHAMPIONS[next - c::DEED_FROM])
                .copied()
                .unwrap_or(0)
                > 0;
        let tribute = next < c::DEED_FROM
            || c::TRIBUTES[next - c::DEED_FROM]
                .iter()
                .all(|(i, n)| self.have(i) >= *n);
        Some((
            rk.gold.saturating_sub(self.gold),
            rk.rooms.saturating_sub(self.room_levels()),
            champion,
            tribute,
        ))
    }

    /// Rises a noble rank by a champion defeated or by tribute (IG-Q29 C).
    pub fn rise_noble(&mut self, tribute: bool, r: &mut Report) -> bool {
        if !self.engine_open(Engine::Nobility) {
            return false;
        }
        let next = self.rank(Engine::Nobility) + 1;
        let Some((gold, rooms, champion, held)) = self.noble_next() else {
            return false;
        };
        let deed = if tribute { held } else { champion };
        if gold > 0 || rooms > 0 || !deed {
            return false;
        }
        self.gold -= c::NOBLE_RANKS[next].gold;
        if tribute && next >= c::DEED_FROM {
            for (i, n) in c::TRIBUTES[next - c::DEED_FROM] {
                self.take(i, *n, r);
            }
        }
        self.ranks[1] = next;
        true
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
        self.engine_open(Engine::Trader)
            && self.level(c::TRADING) >= c::TOWNS[t].level
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

    /// Carts on the road and the most that can be: the carts you own, up to
    /// one plus the Stables' level.
    pub fn caravan_room(&self) -> (u64, u64) {
        let most = self.have("cart").min(1 + self.rooms[c::STABLES]);
        (self.caravans.len() as u64, most)
    }

    pub fn guard_cost(&self, t: usize) -> u64 {
        c::GUARD_GOLD * (t as u64 + 1)
    }

    /// Sends a caravan of up to a cart-load of `item` to town `t`.
    pub fn send_caravan(&mut self, t: usize, item: &str, guards: bool, r: &mut Report) -> bool {
        let (out, most) = self.caravan_room();
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
        let rank = 100 + c::TRADER_RANK_PCT * self.rank(Engine::Trader) as u64;
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

    /// Whether the next Trader rank's needs are met.
    pub fn trader_ready(&self) -> bool {
        let Some(rk) = c::TRADER_RANKS.get(self.rank(Engine::Trader) + 1) else {
            return false;
        };
        let orders = self.counts.get("orders").copied().unwrap_or(0);
        let each = c::TOWNS
            .iter()
            .all(|t| self.counts.get(&format!("orders:{}", t.id)).copied().unwrap_or(0) >= rk.each_town);
        self.engine_open(Engine::Trader)
            && self.level(c::TRADING) >= rk.level
            && orders >= rk.orders
            && self.gold >= rk.fee
            && each
            && (!rk.charter || self.rank(Engine::Nobility) >= c::CHARTER_RANK)
    }

    pub fn rise_trader(&mut self) -> bool {
        if !self.trader_ready() {
            return false;
        }
        let next = self.rank(Engine::Trader) + 1;
        self.gold -= c::TRADER_RANKS[next].fee;
        self.ranks[0] = next;
        true
    }

    // ---------- the Inventor (T14) ----------

    pub fn can_research(&self, id: &str) -> bool {
        let Some(bp) = c::blueprint_index(id).map(|b| &c::BLUEPRINTS[b]) else {
            return false;
        };
        self.engine_open(Engine::Inventor)
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

    pub fn inventor_ready(&self) -> bool {
        let Some(rk) = c::INVENTOR_RANKS.get(self.rank(Engine::Inventor) + 1) else {
            return false;
        };
        let machines = c::MACHINES.iter().filter(|m| self.have(m.id) > 0).count();
        self.engine_open(Engine::Inventor)
            && self.level(c::INVENTION) >= rk.level
            && self.rooms[c::WORKSHOP] >= rk.workshop
            && (!rk.machine || machines > 0)
            && (!rk.star_core || self.have("star_core") > 0)
            && (!rk.every_machine || machines == c::MACHINES.len())
    }

    pub fn rise_inventor(&mut self, r: &mut Report) -> bool {
        if !self.inventor_ready() {
            return false;
        }
        let next = self.rank(Engine::Inventor) + 1;
        if c::INVENTOR_RANKS[next].star_core {
            self.take("star_core", 1, r);
        }
        self.ranks[2] = next;
        true
    }

    // ---------- the Cosmic (T13) ----------

    pub fn cosmic_ready(&self) -> bool {
        let Some(rk) = c::COSMIC_RANKS.get(self.rank(Engine::Cosmic) + 1) else {
            return false;
        };
        self.engine_open(Engine::Cosmic)
            && self.level(c::ATTUNEMENT) >= rk.level
            && self.have("starlight") >= rk.starlight
            && self.have("stardust") >= rk.stardust
            && self.have("lens") >= rk.lenses
    }

    pub fn rise_cosmic(&mut self, r: &mut Report) -> bool {
        if !self.cosmic_ready() {
            return false;
        }
        let next = self.rank(Engine::Cosmic) + 1;
        let rk = &c::COSMIC_RANKS[next];
        self.take("starlight", rk.starlight, r);
        self.take("stardust", rk.stardust, r);
        self.take("lens", rk.lenses, r);
        self.ranks[3] = next;
        true
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
        assert_eq!(s.rooms, [0; 6]);
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
        // starlight, one a chop; mining does not.
        let mut s = opened(1);
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
        a.rooms = [1, 0, 1, 1, 1, 1];
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
