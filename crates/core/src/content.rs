//! The game's content and every tuning number, in one place. Names a player
//! reads are not here: they live in `data/strings.json`, keyed by these ids.
//! When playtest files say something is too slow or too fast, this is the file
//! that changes (and the tests in `game.rs` say what each change does).

/// How a skill trains.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Takes nothing; makes a resource.
    Gather,
    /// Takes what other skills make.
    Make,
    /// Trained by fighting (Melee, Ranged, Magic) or alongside it (Hitpoints).
    Fight,
}

pub struct Skill {
    pub id: &'static str,
    pub kind: Kind,
}

/// The twelve skills, in the order the save stores their experience (P1).
pub const SKILLS: [Skill; 12] = [
    Skill {
        id: "woodcutting",
        kind: Kind::Gather,
    },
    Skill {
        id: "mining",
        kind: Kind::Gather,
    },
    Skill {
        id: "fishing",
        kind: Kind::Gather,
    },
    Skill {
        id: "firemaking",
        kind: Kind::Make,
    },
    Skill {
        id: "fletching",
        kind: Kind::Make,
    },
    Skill {
        id: "smithing",
        kind: Kind::Make,
    },
    Skill {
        id: "cooking",
        kind: Kind::Make,
    },
    Skill {
        id: "runecrafting",
        kind: Kind::Make,
    },
    Skill {
        id: "melee",
        kind: Kind::Fight,
    },
    Skill {
        id: "ranged",
        kind: Kind::Fight,
    },
    Skill {
        id: "magic",
        kind: Kind::Fight,
    },
    Skill {
        id: "hitpoints",
        kind: Kind::Fight,
    },
];
pub const WOODCUTTING: usize = 0;
pub const FIREMAKING: usize = 3;
pub const MELEE: usize = 8;
pub const RANGED: usize = 9;
pub const MAGIC: usize = 10;
pub const HITPOINTS: usize = 11;
/// Hitpoints starts here, as in RuneScape.
pub const HITPOINTS_START_LEVEL: u32 = 10;

pub fn skill_index(id: &str) -> Option<usize> {
    SKILLS.iter().position(|s| s.id == id)
}

/// One thing a skill can do: needs inputs (or none), takes `ms`, gives `xp`
/// skill experience and the same in mastery, and makes `makes`.
pub struct Action {
    pub id: &'static str,
    pub skill: usize,
    pub level: u32,
    pub ms: u64,
    pub xp: u64,
    pub needs: &'static [(&'static str, u64)],
    pub makes: (&'static str, u64),
}

const fn gather(id: &'static str, skill: usize, level: u32, ms: u64, xp: u64, item: &'static str) -> Action {
    Action {
        id,
        skill,
        level,
        ms,
        xp,
        needs: &[],
        makes: (item, 1),
    }
}

const fn make(
    id: &'static str,
    skill: usize,
    level: u32,
    ms: u64,
    xp: u64,
    needs: &'static [(&'static str, u64)],
    makes: (&'static str, u64),
) -> Action {
    Action {
        id,
        skill,
        level,
        ms,
        xp,
        needs,
        makes,
    }
}

#[rustfmt::skip]
pub const ACTIONS: &[Action] = &[
    // Woodcutting
    gather("tree", 0, 1, 3_000, 10, "logs"),
    gather("oak", 0, 15, 4_000, 15, "oak_logs"),
    gather("willow", 0, 30, 5_000, 22, "willow_logs"),
    gather("teak", 0, 35, 5_500, 30, "teak_logs"),
    gather("maple", 0, 45, 6_500, 42, "maple_logs"),
    gather("mahogany", 0, 50, 7_000, 52, "mahogany_logs"),
    gather("yew", 0, 60, 8_000, 75, "yew_logs"),
    gather("magic_tree", 0, 75, 10_000, 125, "magic_logs"),
    gather("redwood", 0, 90, 12_000, 190, "redwood_logs"),
    // Mining
    gather("essence", 1, 1, 2_500, 5, "rune_essence"),
    gather("copper", 1, 1, 3_000, 10, "copper_ore"),
    gather("tin", 1, 1, 3_000, 10, "tin_ore"),
    gather("iron", 1, 15, 4_000, 22, "iron_ore"),
    gather("coal", 1, 30, 5_000, 32, "coal"),
    gather("gold_rock", 1, 40, 6_000, 45, "gold_ore"),
    gather("mithril", 1, 55, 8_000, 80, "mithril_ore"),
    gather("adamantite", 1, 70, 10_000, 110, "adamantite_ore"),
    gather("runite", 1, 85, 12_000, 160, "runite_ore"),
    // Fishing
    gather("shrimp_spot", 2, 1, 3_000, 10, "raw_shrimp"),
    gather("trout_spot", 2, 20, 4_500, 25, "raw_trout"),
    gather("salmon_spot", 2, 30, 5_000, 35, "raw_salmon"),
    gather("lobster_spot", 2, 40, 6_500, 50, "raw_lobster"),
    gather("swordfish_spot", 2, 50, 7_500, 70, "raw_swordfish"),
    gather("monkfish_spot", 2, 62, 8_500, 90, "raw_monkfish"),
    gather("shark_spot", 2, 76, 10_000, 115, "raw_shark"),
    // Firemaking: XP only (T4)
    make("burn_logs", 3, 1, 2_000, 40, &[("logs", 1)], ("ashes", 1)),
    make("burn_oak", 3, 15, 2_500, 60, &[("oak_logs", 1)], ("ashes", 1)),
    make("burn_willow", 3, 30, 3_000, 90, &[("willow_logs", 1)], ("ashes", 1)),
    make("burn_teak", 3, 35, 3_200, 105, &[("teak_logs", 1)], ("ashes", 1)),
    make("burn_maple", 3, 45, 3_500, 135, &[("maple_logs", 1)], ("ashes", 1)),
    make("burn_mahogany", 3, 50, 3_800, 157, &[("mahogany_logs", 1)], ("ashes", 1)),
    make("burn_yew", 3, 60, 4_000, 202, &[("yew_logs", 1)], ("ashes", 1)),
    make("burn_magic", 3, 75, 4_500, 303, &[("magic_logs", 1)], ("ashes", 1)),
    make("burn_redwood", 3, 90, 5_000, 350, &[("redwood_logs", 1)], ("ashes", 1)),
    // Fletching
    make("shafts", 4, 1, 2_000, 5, &[("logs", 1)], ("arrow_shafts", 15)),
    make("bronze_arrows", 4, 1, 2_500, 20, &[("arrow_shafts", 15), ("feathers", 15), ("bronze_arrowheads", 15)], ("bronze_arrows", 15)),
    make("shortbow", 4, 5, 3_000, 10, &[("logs", 1)], ("shortbow", 1)),
    make("iron_arrows", 4, 15, 2_500, 38, &[("arrow_shafts", 15), ("feathers", 15), ("iron_arrowheads", 15)], ("iron_arrows", 15)),
    make("oak_shortbow", 4, 20, 3_000, 33, &[("oak_logs", 1)], ("oak_shortbow", 1)),
    make("steel_arrows", 4, 30, 2_500, 75, &[("arrow_shafts", 15), ("feathers", 15), ("steel_arrowheads", 15)], ("steel_arrows", 15)),
    make("willow_shortbow", 4, 35, 3_500, 66, &[("willow_logs", 1)], ("willow_shortbow", 1)),
    make("mithril_arrows", 4, 45, 2_500, 113, &[("arrow_shafts", 15), ("feathers", 15), ("mithril_arrowheads", 15)], ("mithril_arrows", 15)),
    make("maple_shortbow", 4, 50, 3_800, 100, &[("maple_logs", 1)], ("maple_shortbow", 1)),
    make("yew_shortbow", 4, 65, 4_200, 135, &[("yew_logs", 1)], ("yew_shortbow", 1)),
    make("magic_shortbow", 4, 80, 4_800, 167, &[("magic_logs", 1)], ("magic_shortbow", 1)),
    // Smithing
    make("bronze_bar", 5, 1, 3_000, 6, &[("copper_ore", 1), ("tin_ore", 1)], ("bronze_bar", 1)),
    make("bronze_sword", 5, 4, 3_000, 12, &[("bronze_bar", 1)], ("bronze_sword", 1)),
    make("bronze_arrowheads", 5, 5, 2_500, 12, &[("bronze_bar", 1)], ("bronze_arrowheads", 15)),
    make("iron_bar", 5, 15, 3_500, 12, &[("iron_ore", 1)], ("iron_bar", 1)),
    make("iron_sword", 5, 19, 3_500, 25, &[("iron_bar", 1)], ("iron_sword", 1)),
    make("iron_arrowheads", 5, 20, 2_500, 25, &[("iron_bar", 1)], ("iron_arrowheads", 15)),
    make("steel_bar", 5, 30, 4_000, 17, &[("iron_ore", 1), ("coal", 2)], ("steel_bar", 1)),
    make("steel_sword", 5, 34, 4_000, 37, &[("steel_bar", 1)], ("steel_sword", 1)),
    make("steel_arrowheads", 5, 35, 2_500, 37, &[("steel_bar", 1)], ("steel_arrowheads", 15)),
    make("gold_bar", 5, 40, 4_000, 22, &[("gold_ore", 1)], ("gold_bar", 1)),
    make("mithril_bar", 5, 50, 5_000, 30, &[("mithril_ore", 1), ("coal", 4)], ("mithril_bar", 1)),
    make("mithril_sword", 5, 54, 5_000, 50, &[("mithril_bar", 1)], ("mithril_sword", 1)),
    make("mithril_arrowheads", 5, 55, 2_500, 50, &[("mithril_bar", 1)], ("mithril_arrowheads", 15)),
    // Cooking
    make("cook_shrimp", 6, 1, 2_000, 30, &[("raw_shrimp", 1)], ("shrimp", 1)),
    make("cook_trout", 6, 15, 2_500, 70, &[("raw_trout", 1)], ("trout", 1)),
    make("cook_salmon", 6, 25, 2_600, 90, &[("raw_salmon", 1)], ("salmon", 1)),
    make("cook_lobster", 6, 40, 3_000, 120, &[("raw_lobster", 1)], ("lobster", 1)),
    make("cook_swordfish", 6, 45, 3_200, 140, &[("raw_swordfish", 1)], ("swordfish", 1)),
    make("cook_monkfish", 6, 62, 3_400, 150, &[("raw_monkfish", 1)], ("monkfish", 1)),
    make("cook_shark", 6, 80, 4_000, 210, &[("raw_shark", 1)], ("shark", 1)),
    // Runecrafting
    make("air_runes", 7, 1, 2_500, 5, &[("rune_essence", 1)], ("air_rune", 1)),
    make("water_runes", 7, 5, 2_500, 6, &[("rune_essence", 1)], ("water_rune", 1)),
    make("fire_runes", 7, 14, 3_000, 7, &[("rune_essence", 1)], ("fire_rune", 1)),
    make("chaos_runes", 7, 35, 3_500, 9, &[("rune_essence", 1)], ("chaos_rune", 1)),
    make("death_runes", 7, 65, 4_000, 10, &[("rune_essence", 1)], ("death_rune", 1)),
];

pub fn action_index(id: &str) -> Option<usize> {
    ACTIONS.iter().position(|a| a.id == id)
}

/// An enemy. Each is weak to one style (IG-Q17 default A).
pub struct Enemy {
    pub id: &'static str,
    pub level: u32,
    pub hp: u64,
    pub hit: u64,
    pub weak: usize,
    /// (item, how many, one in how many kills)
    pub drops: &'static [(&'static str, u64, u64)],
    pub gold: u64,
}

#[rustfmt::skip]
pub const ENEMIES: &[Enemy] = &[
    Enemy { id: "chicken", level: 1, hp: 30, hit: 10, weak: MELEE, drops: &[("feathers", 5, 1), ("bones", 1, 1)], gold: 0 },
    Enemy { id: "cow", level: 1, hp: 80, hit: 10, weak: RANGED, drops: &[("cowhide", 1, 1), ("bones", 1, 1)], gold: 0 },
    Enemy { id: "goblin", level: 5, hp: 50, hit: 20, weak: MAGIC, drops: &[("bones", 1, 1), ("feathers", 10, 4)], gold: 12 },
    Enemy { id: "spider", level: 20, hp: 220, hit: 40, weak: MELEE, drops: &[("spider_silk", 1, 4)], gold: 30 },
    Enemy { id: "hill_giant", level: 30, hp: 350, hit: 60, weak: MAGIC, drops: &[("big_bones", 1, 1), ("iron_ore", 3, 3), ("coal", 2, 4)], gold: 60 },
    Enemy { id: "black_knight", level: 45, hp: 420, hit: 80, weak: RANGED, drops: &[("steel_bar", 1, 3)], gold: 120 },
    Enemy { id: "moss_giant", level: 60, hp: 600, hit: 100, weak: MELEE, drops: &[("big_bones", 1, 1), ("mithril_ore", 2, 3)], gold: 200 },
    Enemy { id: "green_dragon", level: 80, hp: 1_200, hit: 180, weak: MAGIC, drops: &[("dragon_bones", 1, 1), ("dragon_scale", 1, 20)], gold: 600 },
];

pub fn enemy_index(id: &str) -> Option<usize> {
    ENEMIES.iter().position(|e| e.id == id)
}

/// A fighting style's swing time, and the items it holds (`held`, kept) or
/// uses up (`spent`, one per swing). Tiers raise the most a swing can hit.
pub struct Style {
    pub skill: usize,
    pub ms: u64,
    pub held: &'static [(&'static str, u64)],
    pub spent: &'static [(&'static str, u64)],
}

#[rustfmt::skip]
pub const STYLES: [Style; 3] = [
    Style { skill: MELEE, ms: 2_400,
        held: &[("mithril_sword", 4), ("steel_sword", 3), ("iron_sword", 2), ("bronze_sword", 1)], spent: &[] },
    Style { skill: RANGED, ms: 2_000,
        held: &[("magic_shortbow", 6), ("yew_shortbow", 5), ("maple_shortbow", 4), ("willow_shortbow", 3), ("oak_shortbow", 2), ("shortbow", 1)],
        spent: &[("mithril_arrows", 4), ("steel_arrows", 3), ("iron_arrows", 2), ("bronze_arrows", 1)] },
    Style { skill: MAGIC, ms: 3_000, held: &[],
        spent: &[("death_rune", 5), ("chaos_rune", 4), ("fire_rune", 3), ("water_rune", 2), ("air_rune", 1)] },
];

/// Food, best first, and how much it heals.
pub const FOOD: [(&str, u64); 7] = [
    ("shark", 200),
    ("monkfish", 160),
    ("swordfish", 140),
    ("lobster", 120),
    ("salmon", 90),
    ("trout", 70),
    ("shrimp", 30),
];

/// Combat tuning (P8).
pub const HP_PER_LEVEL: u64 = 10;
pub const EAT_BELOW_PCT: u64 = 45;
pub const HIT_PCT: u64 = 65;
pub const HIT_PCT_STRONG: u64 = 85;
pub const MAX_HIT_BASE: u64 = 20;
pub const MAX_HIT_PER_LEVEL: u64 = 3;
pub const MAX_HIT_PER_TIER: u64 = 10;
pub const MAX_HIT_STRONG: u64 = 15;
pub const ENEMY_HIT_PCT: u64 = 50;
/// Experience per point of damage, in tenths (style) and thousandths (Hitpoints).
pub const STYLE_XP_TENTHS: u64 = 4;
pub const HP_XP_THOUSANDTHS: u64 = 133;

/// Tools (P10): one line per gathering skill, six tiers each.
pub const TOOL_SKILLS: [usize; 3] = [0, 1, 2];
/// (cost in gold, percent faster)
pub const TOOL_TIERS: [(u64, u64); 6] = [
    (0, 0),
    (500, 5),
    (4_000, 10),
    (25_000, 15),
    (120_000, 20),
    (600_000, 25),
];

/// What the shop pays for one of each item.
#[rustfmt::skip]
pub const VALUES: &[(&str, u64)] = &[
    ("logs", 1), ("oak_logs", 3), ("willow_logs", 6), ("teak_logs", 10), ("maple_logs", 15), ("mahogany_logs", 22),
    ("yew_logs", 40), ("magic_logs", 90), ("redwood_logs", 160),
    ("rune_essence", 1), ("copper_ore", 2), ("tin_ore", 2), ("iron_ore", 6), ("coal", 9), ("gold_ore", 28),
    ("mithril_ore", 45), ("adamantite_ore", 85), ("runite_ore", 150),
    ("raw_shrimp", 1), ("raw_trout", 5), ("raw_salmon", 8), ("raw_lobster", 18), ("raw_swordfish", 30), ("raw_monkfish", 45), ("raw_shark", 80),
    ("shrimp", 3), ("trout", 9), ("salmon", 14), ("lobster", 30), ("swordfish", 45), ("monkfish", 60), ("shark", 110),
    ("ashes", 1), ("arrow_shafts", 0), ("feathers", 1), ("bones", 1), ("big_bones", 8), ("dragon_bones", 60),
    ("cowhide", 15), ("spider_silk", 25), ("dragon_scale", 500), ("bird_nest", 50),
    ("bronze_bar", 8), ("iron_bar", 15), ("steel_bar", 40), ("gold_bar", 70), ("mithril_bar", 120),
    ("bronze_sword", 20), ("iron_sword", 45), ("steel_sword", 110), ("mithril_sword", 300),
    ("bronze_arrowheads", 1), ("iron_arrowheads", 2), ("steel_arrowheads", 4), ("mithril_arrowheads", 8),
    ("bronze_arrows", 1), ("iron_arrows", 3), ("steel_arrows", 6), ("mithril_arrows", 12),
    ("shortbow", 12), ("oak_shortbow", 30), ("willow_shortbow", 70), ("maple_shortbow", 150), ("yew_shortbow", 320), ("magic_shortbow", 700),
    ("air_rune", 3), ("water_rune", 3), ("fire_rune", 4), ("chaos_rune", 12), ("death_rune", 30),
];

pub fn value(item: &str) -> u64 {
    VALUES
        .iter()
        .find(|(i, _)| *i == item)
        .map(|(_, v)| *v)
        .unwrap_or(0)
}

/// Mastery checkpoints (P6): the levels where an action's numbers change.
pub const CHECKPOINTS: [u32; 4] = [10, 25, 50, 99];
pub const MASTERY_FASTER_MS: u64 = 200;
/// Gathering: chance of a double, in percent, at 10 and 50, plus one per ten levels.
pub const DOUBLE_AT_10: u64 = 5;
pub const DOUBLE_AT_50: u64 = 15;
/// Making: percent more XP at 10; chance to keep the inputs at 50 and 99.
pub const MAKE_XP_AT_10: u64 = 5;
pub const KEEP_AT_50: u64 = 10;
pub const KEEP_AT_99: u64 = 20;

/// A bird nest falls once in this many woodcutting actions.
pub const NEST_ONE_IN: u64 = 256;

/// Time away (P4, P5).
pub const AWAY_CAP_MS: u64 = 24 * 3_600_000;
pub const RETURN_AFTER_MS: u64 = 10 * 60_000;
pub const FORECAST_MS: u64 = 20 * 60_000;
pub const EXPORT_REMIND_MS: u64 = 7 * 24 * 3_600_000;
pub const AUTOSAVE_MS: u64 = 10_000;

/// Prestige (P9): +25% skill XP per prestige; every prestige needs total
/// level 600 (T7, IG-Q19 B).
pub const PRESTIGE_BASE: u32 = 600;
pub const PRESTIGE_STEP_BP: u64 = 2_500;

/// A plan holds this many steps (P3).
pub const PLAN_STEPS: usize = 3;
/// Step lengths offered on the plan screen; 0 means until the inputs run out
/// (or, for gathering, until you come back).
pub const PLAN_COUNTS: [u64; 4] = [0, 100, 500, 1_000];

/// The playtest log keeps this many events, newest last (P12).
pub const LOG_CAP: usize = 4_000;
pub const NOTE_MAX_CHARS: usize = 600;

#[cfg(test)]
mod tests {
    use super::*;

    fn makers(item: &str) -> bool {
        ACTIONS.iter().any(|a| a.makes.0 == item)
            || ENEMIES.iter().any(|e| e.drops.iter().any(|d| d.0 == item))
    }

    #[test]
    fn ids_are_unique() {
        let mut ids: Vec<&str> = ACTIONS
            .iter()
            .map(|a| a.id)
            .chain(ENEMIES.iter().map(|e| e.id))
            .collect();
        let n = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), n);
    }

    #[test]
    fn every_input_is_made_by_some_action_or_dropped_by_some_enemy() {
        for a in ACTIONS {
            for (item, _) in a.needs {
                assert!(makers(item), "{} needs {item}, which nothing makes", a.id);
            }
        }
        for s in &STYLES {
            for (item, _) in s.held.iter().chain(s.spent.iter()) {
                assert!(makers(item), "a style uses {item}, which nothing makes");
            }
        }
        for (item, _) in FOOD {
            assert!(makers(item), "food {item} is made by nothing");
        }
    }

    #[test]
    fn every_item_has_a_value_entry() {
        for a in ACTIONS {
            assert!(
                VALUES.iter().any(|(i, _)| *i == a.makes.0),
                "{} has no value",
                a.makes.0
            );
        }
        for e in ENEMIES {
            for d in e.drops {
                assert!(VALUES.iter().any(|(i, _)| *i == d.0), "{} has no value", d.0);
            }
        }
    }

    #[test]
    fn each_skill_has_a_level_one_action_unless_it_fights() {
        for (i, s) in SKILLS.iter().enumerate() {
            if s.kind != Kind::Fight {
                assert!(ACTIONS.iter().any(|a| a.skill == i && a.level == 1), "{}", s.id);
            }
        }
    }

    #[test]
    fn each_style_is_weak_point_of_some_enemy() {
        for s in &STYLES {
            assert!(ENEMIES.iter().any(|e| e.weak == s.skill));
        }
    }
}
