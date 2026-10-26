//! The game's content and every tuning number, in one place. Names a player
//! reads are not here: they live in `data/strings.json`, keyed by these ids.
//! When playtest files say something is too slow or too fast, this is the file
//! that changes (and the tests in `game.rs` say what each change does).

use std::collections::BTreeMap;
use std::sync::OnceLock;

/// How a skill trains.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// Takes nothing; makes a resource.
    Gather,
    /// Takes what other skills make.
    Make,
    /// Trained by fighting (Melee, Ranged, Magic) or alongside it (Hitpoints).
    Fight,
    /// Seeds grow in patches while other actions run (T5).
    Farm,
    /// Trained by an engine's own timer, not by an action (Trading, T12).
    Engine,
}

pub struct Skill {
    pub id: &'static str,
    pub kind: Kind,
}

/// The skills, in the order the save stores their experience (P1).
/// Crafting and Farming came later (T5), and the four engine skills later
/// still (T11 to T14); each goes at the end, so older saves still line up.
pub const SKILLS: [Skill; 19] = [
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
    Skill {
        id: "crafting",
        kind: Kind::Make,
    },
    Skill {
        id: "farming",
        kind: Kind::Farm,
    },
    Skill {
        id: "construction",
        kind: Kind::Make,
    },
    Skill {
        id: "trading",
        kind: Kind::Engine,
    },
    Skill {
        id: "invention",
        kind: Kind::Make,
    },
    Skill {
        id: "attunement",
        kind: Kind::Make,
    },
    // The bard (T18, IG-Q56 A): songs are actions that need an instrument
    // held, not spent, and make renown for the Fame engine.
    Skill {
        id: "performance",
        kind: Kind::Gather,
    },
];
pub const WOODCUTTING: usize = 0;
pub const MINING: usize = 1;
pub const FISHING: usize = 2;
pub const FIREMAKING: usize = 3;
pub const SMITHING: usize = 5;
pub const COOKING: usize = 6;
pub const RUNECRAFTING: usize = 7;
pub const MELEE: usize = 8;
pub const RANGED: usize = 9;
pub const MAGIC: usize = 10;
pub const HITPOINTS: usize = 11;
pub const CRAFTING: usize = 12;
pub const FARMING: usize = 13;
pub const CONSTRUCTION: usize = 14;
pub const TRADING: usize = 15;
pub const INVENTION: usize = 16;
pub const ATTUNEMENT: usize = 17;
pub const PERFORMANCE: usize = 18;
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
    /// The region it belongs to (T15): 0 is the Vale, today's game.
    pub region: usize,
}

/// A row of a later region.
const fn region(region: usize, a: Action) -> Action {
    Action { region, ..a }
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
        region: 0,
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
        region: 0,
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
    gather("limestone", 1, 10, 4_000, 18, "limestone"),
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
    // Smithing for the engines: nails for Construction, gears for Invention
    make("nails", 5, 18, 3_000, 20, &[("iron_bar", 1)], ("nails", 15)),
    make("gears", 5, 45, 4_000, 45, &[("steel_bar", 1)], ("gear", 2)),
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
    // Astral runes need stardust and the Astral adept rank (T13)
    make("astral_runes", 7, 60, 4_500, 60, &[("rune_essence", 10), ("stardust", 1)], ("astral_rune", 10)),
    // Cooking what Farming grows (T5)
    make("cook_potato", 6, 7, 2_500, 40, &[("potato", 1)], ("baked_potato", 1)),
    make("cook_sweetcorn", 6, 28, 2_800, 85, &[("sweetcorn", 1)], ("cooked_sweetcorn", 1)),
    // Crafting (T5): hides, silk, gold and scales into armour and jewellery
    make("tan_hide", 12, 1, 2_000, 5, &[("cowhide", 1)], ("leather", 1)),
    make("leather_gloves", 12, 1, 2_500, 14, &[("leather", 1)], ("leather_gloves", 1)),
    make("leather_body", 12, 14, 3_000, 25, &[("leather", 3)], ("leather_body", 1)),
    make("silk_cloth", 12, 20, 2_500, 15, &[("spider_silk", 1)], ("silk_cloth", 1)),
    make("gold_ring", 12, 25, 3_000, 40, &[("gold_bar", 1)], ("gold_ring", 1)),
    make("hard_leather_body", 12, 30, 3_500, 60, &[("leather", 5)], ("hard_leather_body", 1)),
    make("silk_robe", 12, 45, 4_000, 100, &[("silk_cloth", 4)], ("silk_robe", 1)),
    make("gold_amulet", 12, 50, 4_000, 80, &[("gold_bar", 2)], ("gold_amulet", 1)),
    make("dragonhide_body", 12, 70, 5_000, 250, &[("dragon_scale", 3), ("leather", 2)], ("dragonhide_body", 1)),
    // Crafting for the engines: springs (Invention), lenses from glass, royal robes from dye (T12)
    make("springs", 12, 35, 3_000, 30, &[("iron_bar", 1)], ("spring", 3)),
    make("lens", 12, 40, 3_000, 50, &[("glass", 1)], ("lens", 1)),
    make("royal_robe", 12, 55, 4_500, 140, &[("silk_cloth", 2), ("dye", 1)], ("royal_robe", 1)),
    // Recipes found from enemies (T10): each needs goods from two or three skills
    make("farmers_stew", 6, 12, 3_000, 60, &[("potato", 1), ("onion", 1), ("raw_shrimp", 1)], ("farmers_stew", 1)),
    make("goblin_hot_pot", 6, 30, 3_200, 110, &[("trout", 1), ("tomato", 1), ("onion", 1)], ("goblin_hot_pot", 1)),
    make("dragon_pepper_feast", 6, 75, 4_500, 320, &[("shark", 1), ("strawberry", 1), ("sweetcorn", 1)], ("dragon_pepper_feast", 1)),
    make("studded_leather", 12, 20, 3_500, 60, &[("leather", 3), ("iron_bar", 1)], ("studded_leather", 1)),
    make("silkweave_vest", 12, 40, 4_000, 130, &[("silk_cloth", 3), ("leather", 2), ("gold_bar", 1)], ("silkweave_vest", 1)),
    make("dragonfire_staff", 12, 70, 6_000, 400, &[("magic_logs", 1), ("dragon_scale", 1), ("fire_rune", 20)], ("dragonfire_staff", 1)),
    make("giants_maul", 5, 40, 5_000, 120, &[("steel_bar", 3), ("big_bones", 2), ("maple_logs", 1)], ("giants_maul", 1)),
    make("mossplate", 5, 60, 6_000, 220, &[("mithril_bar", 3), ("leather", 2)], ("mossplate", 1)),
    make("knights_longbow", 4, 55, 5_000, 180, &[("yew_logs", 1), ("steel_bar", 1), ("spider_silk", 1)], ("knights_longbow", 1)),
    // Construction (T11): planks, carts for the Trader, frames for the Inventor
    make("plank", 14, 1, 2_500, 12, &[("logs", 1)], ("plank", 1)),
    make("cart", 14, 5, 6_000, 60, &[("plank", 4), ("nails", 20)], ("cart", 1)),
    make("oak_plank", 14, 15, 3_000, 25, &[("oak_logs", 1)], ("oak_plank", 1)),
    make("frame", 14, 30, 5_000, 80, &[("oak_plank", 2), ("nails", 10)], ("frame", 1)),
    make("teak_plank", 14, 35, 3_500, 45, &[("teak_logs", 1)], ("teak_plank", 1)),
    make("mahogany_plank", 14, 50, 4_000, 70, &[("mahogany_logs", 1)], ("mahogany_plank", 1)),
    // Invention (T14): take apart what you made for parts (IG-Q35 A) ...
    make("take_apart_shortbow", 16, 1, 2_000, 8, &[("shortbow", 1)], ("wood_parts", 3)),
    make("take_apart_iron_sword", 16, 5, 2_200, 14, &[("iron_sword", 1)], ("metal_parts", 3)),
    make("take_apart_leather_body", 16, 10, 2_400, 20, &[("leather_body", 1)], ("cloth_parts", 4)),
    make("take_apart_oak_shortbow", 16, 15, 2_400, 26, &[("oak_shortbow", 1)], ("wood_parts", 5)),
    make("take_apart_steel_sword", 16, 25, 2_800, 40, &[("steel_sword", 1)], ("metal_parts", 6)),
    make("take_apart_silk_robe", 16, 35, 3_000, 60, &[("silk_robe", 1)], ("cloth_parts", 8)),
    make("take_apart_gold_amulet", 16, 40, 3_200, 75, &[("gold_amulet", 1)], ("magic_parts", 5)),
    make("take_apart_maple_shortbow", 16, 50, 3_400, 90, &[("maple_shortbow", 1)], ("wood_parts", 9)),
    make("take_apart_mithril_sword", 16, 55, 3_600, 110, &[("mithril_sword", 1)], ("metal_parts", 10)),
    // ... then build what a blueprint describes
    make("build_axe_gadget", 16, 10, 8_000, 150, &[("wood_parts", 30), ("metal_parts", 20), ("spring", 6)], ("axe_gadget", 1)),
    make("build_sawmill", 16, 20, 12_000, 300, &[("wood_parts", 40), ("gear", 4), ("frame", 2)], ("sawmill", 1)),
    make("build_pick_gadget", 16, 25, 8_000, 250, &[("metal_parts", 40), ("spring", 6), ("lens", 1)], ("pick_gadget", 1)),
    make("build_rod_gadget", 16, 35, 8_000, 350, &[("cloth_parts", 40), ("spring", 6), ("lens", 1)], ("rod_gadget", 1)),
    make("build_auto_smelter", 16, 45, 15_000, 600, &[("metal_parts", 80), ("gear", 10), ("frame", 4)], ("auto_smelter", 1)),
    make("build_fish_trap", 16, 55, 15_000, 800, &[("cloth_parts", 60), ("magic_parts", 20), ("gear", 6), ("frame", 4)], ("fish_trap", 1)),
    // Attunement (T13): meditate with runes and starlight; meditation makes nothing but XP
    make("meditate_air", 17, 1, 4_000, 30, &[("air_rune", 10), ("starlight", 5)], ("", 0)),
    make("meditate_water", 17, 10, 4_000, 45, &[("water_rune", 10), ("starlight", 8)], ("", 0)),
    make("meditate_fire", 17, 25, 4_500, 70, &[("fire_rune", 10), ("starlight", 12)], ("", 0)),
    make("meditate_chaos", 17, 45, 5_000, 120, &[("chaos_rune", 5), ("starlight", 20)], ("", 0)),
    make("forge_star_core", 17, 50, 20_000, 500, &[("stardust", 20), ("starlight", 200)], ("star_core", 1)),
    make("meditate_death", 17, 65, 6_000, 300, &[("death_rune", 5), ("starlight", 40), ("stardust", 1)], ("", 0)),
    // The Coast, the Peaks and the Wastes (T15), written by tools/content/regions.py
    // <gen:actions>
    region(1, gather("driftwood_tree", 0, 3, 3_085, 10, "driftwood_logs")),
    region(1, gather("palm_tree", 0, 12, 3_940, 14, "palm_logs")),
    region(1, gather("cork_tree", 0, 22, 4_890, 20, "cork_logs")),
    region(1, gather("mangrove_tree", 0, 32, 5_840, 29, "mangrove_logs")),
    region(1, gather("cypress_tree", 0, 42, 6_790, 43, "cypress_logs")),
    region(1, gather("sea_pine_tree", 0, 52, 7_740, 61, "sea_pine_logs")),
    region(1, gather("ironbark_tree", 0, 64, 8_880, 90, "ironbark_logs")),
    region(1, gather("salt_cedar_tree", 0, 78, 10_210, 137, "salt_cedar_logs")),
    region(1, gather("ghostwood_tree", 0, 92, 11_540, 201, "ghostwood_logs")),
    region(1, gather("rock_salt_rock", 1, 3, 3_085, 11, "rock_salt")),
    region(1, gather("sandstone_rock", 1, 12, 3_940, 15, "sandstone")),
    region(1, gather("zinc_rock", 1, 22, 4_890, 21, "zinc_ore")),
    region(1, gather("quartz_rock", 1, 32, 5_840, 29, "quartz")),
    region(1, gather("silver_rock", 1, 42, 6_790, 41, "silver_ore")),
    region(1, gather("cobalt_rock", 1, 52, 7_740, 56, "cobalt_ore")),
    region(1, gather("sea_iron_rock", 1, 64, 8_880, 81, "sea_iron_ore")),
    region(1, gather("orichalcum_rock", 1, 78, 10_210, 121, "orichalcum_ore")),
    region(1, gather("abyssal_rock", 1, 92, 11_540, 174, "abyssal_ore")),
    region(1, gather("crab_spot", 2, 3, 3_085, 13, "raw_crab")),
    region(1, gather("sardine_spot", 2, 12, 3_940, 17, "raw_sardine")),
    region(1, gather("mackerel_spot", 2, 22, 4_890, 24, "raw_mackerel")),
    region(1, gather("bass_spot", 2, 32, 5_840, 33, "raw_bass")),
    region(1, gather("squid_spot", 2, 42, 6_790, 46, "raw_squid")),
    region(1, gather("eel_spot", 2, 52, 7_740, 63, "raw_eel")),
    region(1, gather("tuna_spot", 2, 64, 8_880, 90, "raw_tuna")),
    region(1, gather("ray_spot", 2, 78, 10_210, 133, "raw_ray")),
    region(1, gather("angler_spot", 2, 92, 11_540, 190, "raw_angler")),
    region(1, make("burn_driftwood", 3, 3, 2_105, 40, &[("driftwood_logs", 1)], ("ashes", 1))),
    region(1, make("burn_palm", 3, 12, 2_420, 46, &[("palm_logs", 1)], ("ashes", 1))),
    region(1, make("burn_cork", 3, 22, 2_770, 59, &[("cork_logs", 1)], ("ashes", 1))),
    region(1, make("burn_mangrove", 3, 32, 3_120, 81, &[("mangrove_logs", 1)], ("ashes", 1))),
    region(1, make("burn_cypress", 3, 42, 3_470, 111, &[("cypress_logs", 1)], ("ashes", 1))),
    region(1, make("burn_sea_pine", 3, 52, 3_820, 148, &[("sea_pine_logs", 1)], ("ashes", 1))),
    region(1, make("burn_ironbark", 3, 64, 4_240, 204, &[("ironbark_logs", 1)], ("ashes", 1))),
    region(1, make("burn_salt_cedar", 3, 78, 4_730, 283, &[("salt_cedar_logs", 1)], ("ashes", 1))),
    region(1, make("burn_ghostwood", 3, 92, 5_220, 379, &[("ghostwood_logs", 1)], ("ashes", 1))),
    region(1, make("char_logs", 3, 10, 6_000, 25, &[("driftwood_logs", 1)], ("charcoal", 2))),
    region(1, make("brass_bar", 5, 24, 3_220, 9, &[("zinc_ore", 1), ("copper_ore", 1)], ("brass_bar", 1))),
    region(1, make("silver_bar", 5, 44, 3_820, 26, &[("silver_ore", 1)], ("silver_bar", 1))),
    region(1, make("cobalt_bar", 5, 54, 4_120, 38, &[("cobalt_ore", 1), ("coal", 2)], ("cobalt_bar", 1))),
    region(1, make("sea_iron_bar", 5, 66, 4_480, 56, &[("sea_iron_ore", 1), ("coal", 3)], ("sea_iron_bar", 1))),
    region(1, make("orichalcum_bar", 5, 80, 4_900, 82, &[("orichalcum_ore", 1), ("coal", 4)], ("orichalcum_bar", 1))),
    region(1, make("abyssal_bar", 5, 94, 5_320, 112, &[("abyssal_ore", 1), ("coal", 5)], ("abyssal_bar", 1))),
    region(1, make("brass_sword", 5, 26, 3_280, 21, &[("brass_bar", 2)], ("brass_sword", 1))),
    region(1, make("cobalt_sword", 5, 56, 4_180, 82, &[("cobalt_bar", 2)], ("cobalt_sword", 1))),
    region(1, make("sea_iron_sword", 5, 68, 4_540, 120, &[("sea_iron_bar", 2)], ("sea_iron_sword", 1))),
    region(1, make("orichalcum_sword", 5, 82, 4_960, 172, &[("orichalcum_bar", 2)], ("orichalcum_sword", 1))),
    region(1, make("abyssal_sword", 5, 96, 5_380, 234, &[("abyssal_bar", 2)], ("abyssal_sword", 1))),
    region(1, make("sea_iron_arrowheads", 5, 67, 2_255, 87, &[("sea_iron_bar", 1)], ("sea_iron_arrowheads", 15))),
    region(1, make("abyssal_arrowheads", 5, 95, 2_675, 172, &[("abyssal_bar", 1)], ("abyssal_arrowheads", 15))),
    region(1, make("cobalt_platebody", 5, 58, 5_240, 154, &[("cobalt_bar", 4)], ("cobalt_platebody", 1))),
    region(1, make("orichalcum_platebody", 5, 84, 6_020, 315, &[("orichalcum_bar", 4)], ("orichalcum_platebody", 1))),
    region(1, make("sea_iron_gears", 5, 67, 4_510, 102, &[("sea_iron_bar", 1)], ("gear", 4))),
    region(1, make("brass_nails", 5, 28, 3_340, 12, &[("brass_bar", 1)], ("nails", 30))),
    region(1, make("driftwood_shafts", 4, 4, 2_000, 5, &[("driftwood_logs", 1)], ("arrow_shafts", 20))),
    region(1, make("palm_shafts", 4, 13, 2_000, 5, &[("palm_logs", 1)], ("arrow_shafts", 25))),
    region(1, make("palm_bow", 4, 15, 2_950, 9, &[("palm_logs", 2)], ("palm_bow", 1))),
    region(1, make("mangrove_bow", 4, 35, 3_550, 30, &[("mangrove_logs", 2)], ("mangrove_bow", 1))),
    region(1, make("sea_pine_bow", 4, 55, 4_150, 70, &[("sea_pine_logs", 2)], ("sea_pine_bow", 1))),
    region(1, make("salt_cedar_bow", 4, 81, 4_930, 147, &[("salt_cedar_logs", 2)], ("salt_cedar_bow", 1))),
    region(1, make("ghostwood_bow", 4, 95, 5_350, 201, &[("ghostwood_logs", 2)], ("ghostwood_bow", 1))),
    region(1, make("brass_crossbow", 4, 27, 3_810, 25, &[("brass_bar", 2), ("cork_logs", 1)], ("brass_crossbow", 1))),
    region(1, make("sea_iron_crossbow", 4, 69, 5_070, 138, &[("sea_iron_bar", 2), ("ironbark_logs", 1)], ("sea_iron_crossbow", 1))),
    region(1, make("abyssal_crossbow", 4, 97, 5_910, 269, &[("abyssal_bar", 2), ("ghostwood_logs", 1)], ("abyssal_crossbow", 1))),
    region(1, make("sea_iron_arrows", 4, 69, 2_500, 123, &[("arrow_shafts", 15), ("feathers", 15), ("sea_iron_arrowheads", 15)], ("sea_iron_arrows", 15))),
    region(1, make("abyssal_arrows", 4, 97, 2_500, 239, &[("arrow_shafts", 15), ("feathers", 15), ("abyssal_arrowheads", 15)], ("abyssal_arrows", 15))),
    region(1, make("cook_crab", 6, 3, 2_075, 37, &[("raw_crab", 1)], ("cooked_crab", 1))),
    region(1, make("cook_sardine", 6, 12, 2_300, 56, &[("raw_sardine", 1)], ("cooked_sardine", 1))),
    region(1, make("cook_mackerel", 6, 22, 2_550, 78, &[("raw_mackerel", 1)], ("cooked_mackerel", 1))),
    region(1, make("cook_bass", 6, 32, 2_800, 100, &[("raw_bass", 1)], ("cooked_bass", 1))),
    region(1, make("cook_squid", 6, 42, 3_050, 122, &[("raw_squid", 1)], ("cooked_squid", 1))),
    region(1, make("cook_eel", 6, 52, 3_300, 144, &[("raw_eel", 1)], ("cooked_eel", 1))),
    region(1, make("cook_tuna", 6, 64, 3_600, 171, &[("raw_tuna", 1)], ("cooked_tuna", 1))),
    region(1, make("cook_ray", 6, 78, 3_950, 202, &[("raw_ray", 1)], ("cooked_ray", 1))),
    region(1, make("cook_angler", 6, 92, 4_300, 232, &[("raw_angler", 1)], ("cooked_angler", 1))),
    region(1, make("sea_beet_sardine_dish", 6, 19, 3_000, 89, &[("raw_sardine", 1), ("sea_beet", 2)], ("sea_beet_sardine_dish", 1))),
    region(1, make("rice_bass_dish", 6, 36, 3_000, 134, &[("raw_bass", 1), ("rice", 2)], ("rice_bass_dish", 1))),
    region(1, make("pepper_eel_dish", 6, 56, 3_000, 186, &[("raw_eel", 1), ("pepper", 2), ("sea_salt", 1)], ("pepper_eel_dish", 1))),
    region(1, make("tan_snakeskin", 12, 14, 2_000, 5, &[("sea_snake_skin", 1)], ("snakeskin", 1))),
    region(1, make("snakeskin_body", 12, 34, 3_520, 33, &[("snakeskin", 3)], ("snakeskin_body", 1))),
    region(1, make("snakeskin_chaps", 12, 54, 4_120, 77, &[("snakeskin", 2)], ("snakeskin_chaps", 1))),
    region(1, make("sea_silk_cloth", 12, 36, 2_500, 14, &[("sea_silk", 1)], ("sea_silk_cloth", 1))),
    region(1, make("sea_silk_robe", 12, 66, 4_480, 141, &[("sea_silk_cloth", 4)], ("sea_silk_robe", 1))),
    region(1, make("krakenhide_body", 12, 93, 6_790, 440, &[("kraken_hide", 3), ("snakeskin", 2)], ("krakenhide_body", 1))),
    region(1, make("pearl_ring", 12, 45, 3_850, 62, &[("silver_bar", 1), ("pearl", 1)], ("pearl_ring", 1))),
    region(1, make("pearl_amulet", 12, 67, 4_510, 131, &[("silver_bar", 2), ("pearl", 1)], ("pearl_amulet", 1))),
    region(1, make("quartz_glass", 12, 33, 2_500, 12, &[("quartz", 1)], ("glass", 1))),
    region(1, make("coral_bracelet", 12, 26, 3_280, 18, &[("silver_bar", 1), ("coral", 1)], ("coral_bracelet", 1))),
    region(1, make("ring_of_power_1", 12, 30, 4_400, 40, &[("silver_bar", 1), ("ruby", 1), ("fire_rune", 10)], ("ring_of_power_1", 1))),
    region(1, make("amulet_of_vigour_1", 12, 35, 4_550, 52, &[("silver_bar", 1), ("emerald", 1), ("water_rune", 10)], ("amulet_of_vigour_1", 1))),
    region(1, make("ring_of_wealth_1", 12, 40, 4_700, 66, &[("silver_bar", 1), ("sapphire", 1), ("air_rune", 10)], ("ring_of_wealth_1", 1))),
    region(1, make("amulet_of_warding_1", 12, 45, 4_850, 82, &[("silver_bar", 1), ("diamond", 1), ("earth_rune", 10)], ("amulet_of_warding_1", 1))),
    region(1, make("earth_runes", 7, 9, 2_680, 9, &[("rune_essence", 1)], ("earth_rune", 1))),
    region(1, make("mind_runes", 7, 20, 2_900, 10, &[("rune_essence", 1)], ("mind_rune", 1))),
    region(1, make("mist_runes", 7, 30, 3_100, 11, &[("rune_essence", 1), ("air_rune", 1), ("water_rune", 1)], ("mist_rune", 2))),
    region(1, make("body_runes", 7, 40, 3_300, 13, &[("rune_essence", 1)], ("body_rune", 1))),
    region(1, make("nature_runes", 7, 55, 3_600, 14, &[("rune_essence", 1)], ("nature_rune", 1))),
    region(1, make("law_runes", 7, 70, 3_900, 16, &[("rune_essence", 1)], ("law_rune", 1))),
    region(1, make("cork_plank", 14, 23, 3_260, 8, &[("cork_logs", 1)], ("cork_plank", 1))),
    region(1, make("cypress_plank", 14, 43, 3_660, 22, &[("cypress_logs", 1)], ("cypress_plank", 1))),
    region(1, make("build_ship", 14, 10, 8_000, 10, &[("cork_plank", 8), ("nails", 40), ("sea_silk_cloth", 4)], ("ship", 1))),
    region(1, make("build_fish_rack", 14, 30, 7_000, 37, &[("cypress_plank", 6), ("nails", 30), ("sea_salt", 2)], ("fish_rack", 1))),
    region(1, make("build_cork_bed", 14, 38, 7_000, 55, &[("cypress_plank", 6), ("nails", 30), ("sea_salt", 2)], ("cork_bed", 1))),
    region(1, make("build_ship_in_a_bottle", 14, 46, 7_000, 78, &[("cypress_plank", 6), ("nails", 30), ("coral", 2)], ("ship_in_a_bottle", 1))),
    region(1, make("take_apart_palm_bow", 16, 8, 2_400, 5, &[("palm_bow", 1)], ("wood_parts", 6))),
    region(1, make("take_apart_brass_sword", 16, 18, 2_650, 6, &[("brass_sword", 1)], ("metal_parts", 6))),
    region(1, make("take_apart_snakeskin_body", 16, 28, 2_900, 12, &[("snakeskin_body", 1)], ("cloth_parts", 7))),
    region(1, make("take_apart_mangrove_bow", 16, 38, 3_150, 20, &[("mangrove_bow", 1)], ("wood_parts", 7))),
    region(1, make("take_apart_sea_iron_sword", 16, 48, 3_400, 31, &[("sea_iron_sword", 1)], ("metal_parts", 8))),
    region(1, make("take_apart_sea_silk_robe", 16, 58, 3_650, 44, &[("sea_silk_robe", 1)], ("cloth_parts", 8))),
    region(1, make("take_apart_pearl_amulet", 16, 68, 3_900, 60, &[("pearl_amulet", 1)], ("magic_parts", 9))),
    region(1, make("take_apart_brass_crossbow", 16, 78, 4_150, 78, &[("brass_crossbow", 1)], ("metal_parts", 9))),
    region(1, make("take_apart_orichalcum_sword", 16, 88, 4_400, 99, &[("orichalcum_sword", 1)], ("metal_parts", 10))),
    region(1, make("build_tide_mill", 16, 20, 15_000, 450, &[("metal_parts", 60), ("gear", 6), ("frame", 3)], ("tide_mill", 1))),
    region(1, make("build_salt_pan", 16, 35, 15_000, 600, &[("metal_parts", 60), ("gear", 6), ("frame", 3)], ("salt_pan", 1))),
    region(1, make("build_brass_works", 16, 50, 15_000, 750, &[("metal_parts", 60), ("gear", 6), ("frame", 3)], ("brass_works", 1))),
    region(1, make("build_sawmill_gearbox", 16, 30, 12_000, 370, &[("metal_parts", 50), ("spring", 8), ("lens", 1)], ("sawmill_gearbox", 1))),
    region(1, make("build_smelter_bellows", 16, 45, 12_000, 490, &[("metal_parts", 50), ("spring", 8), ("lens", 1)], ("smelter_bellows", 1))),
    region(1, make("build_trap_nets", 16, 60, 12_000, 610, &[("metal_parts", 50), ("spring", 8), ("lens", 1)], ("trap_nets", 1))),
    region(1, make("meditate_earth", 17, 5, 4_200, 26, &[("earth_rune", 10), ("starlight", 12)], ("", 0))),
    region(1, make("meditate_mind", 17, 19, 4_760, 36, &[("mind_rune", 10), ("starlight", 18)], ("", 0))),
    region(1, make("meditate_mist", 17, 33, 5_320, 58, &[("mist_rune", 10), ("starlight", 24)], ("", 0))),
    region(1, make("meditate_body", 17, 47, 5_880, 91, &[("body_rune", 10), ("starlight", 30)], ("", 0))),
    region(1, make("meditate_nature", 17, 61, 6_440, 137, &[("nature_rune", 10), ("starlight", 36)], ("", 0))),
    region(1, make("meditate_law", 17, 75, 7_000, 194, &[("law_rune", 10), ("starlight", 42), ("stardust", 1)], ("", 0))),
    region(1, make("crab_cakes", 6, 20, 4_600, 32, &[("cooked_mackerel", 1), ("rice", 1), ("pepper", 1)], ("crab_cakes", 1))),
    region(1, make("fishermans_pie", 6, 45, 5_350, 123, &[("cooked_squid", 1), ("sea_beet", 2), ("sea_salt", 1)], ("fishermans_pie", 1))),
    region(1, make("ink_risotto", 6, 80, 6_400, 369, &[("cooked_angler", 1), ("sea_grape", 1), ("saffron", 1)], ("ink_risotto", 1))),
    region(1, make("tidecutter", 5, 60, 5_800, 212, &[("orichalcum_bar", 3), ("coral", 1), ("sea_pine_logs", 1)], ("tidecutter", 1))),
    region(1, make("harpoon_bow", 4, 70, 6_100, 285, &[("sea_iron_bar", 2), ("salt_cedar_logs", 1), ("sea_silk_cloth", 2)], ("harpoon_bow", 1))),
    region(1, make("coral_staff", 12, 75, 6_250, 325, &[("ghostwood_logs", 1), ("coral", 2), ("law_rune", 20)], ("coral_staff", 1))),
    region(1, make("sharkskin_vest", 12, 50, 5_500, 150, &[("snakeskin", 4), ("pearl", 1), ("cobalt_bar", 1)], ("sharkskin_vest", 1))),
    region(1, make("barnacle_plate", 5, 85, 6_550, 415, &[("abyssal_bar", 4), ("coral", 2), ("snakeskin", 2)], ("barnacle_plate", 1))),
    region(1, make("pearl_mantle", 12, 90, 6_700, 464, &[("sea_silk_cloth", 4), ("pearl", 2), ("nature_rune", 30)], ("pearl_mantle", 1))),
    region(2, gather("larch_tree", 0, 5, 3_275, 11, "larch_logs")),
    region(2, gather("spruce_tree", 0, 14, 4_130, 16, "spruce_logs")),
    region(2, gather("alpine_ash_tree", 0, 24, 5_080, 23, "alpine_ash_logs")),
    region(2, gather("birch_tree", 0, 34, 6_030, 33, "birch_logs")),
    region(2, gather("juniper_tree", 0, 44, 6_980, 47, "juniper_logs")),
    region(2, gather("stone_pine_tree", 0, 54, 7_930, 67, "stone_pine_logs")),
    region(2, gather("frostwood_tree", 0, 66, 9_070, 100, "frostwood_logs")),
    region(2, gather("cloudwood_tree", 0, 80, 10_400, 151, "cloudwood_logs")),
    region(2, gather("skyroot_tree", 0, 94, 11_730, 220, "skyroot_logs")),
    region(2, gather("granite_rock", 1, 5, 3_275, 12, "granite")),
    region(2, gather("flint_rock", 1, 14, 4_130, 17, "flint")),
    region(2, gather("nickel_rock", 1, 24, 5_080, 23, "nickel_ore")),
    region(2, gather("rock_crystal_rock", 1, 34, 6_030, 33, "rock_crystal")),
    region(2, gather("platinum_rock", 1, 44, 6_980, 45, "platinum_ore")),
    region(2, gather("frostiron_rock", 1, 54, 7_930, 62, "frostiron_ore")),
    region(2, gather("titanium_rock", 1, 66, 9_070, 90, "titanium_ore")),
    region(2, gather("skysteel_rock", 1, 80, 10_400, 133, "skysteel_ore")),
    region(2, gather("aetherium_rock", 1, 94, 11_730, 190, "aetherium_ore")),
    region(2, gather("char_spot", 2, 5, 3_275, 14, "raw_char")),
    region(2, gather("ice_cod_spot", 2, 14, 4_130, 19, "raw_ice_cod")),
    region(2, gather("glacier_trout_spot", 2, 24, 5_080, 26, "raw_glacier_trout")),
    region(2, gather("grayling_spot", 2, 34, 6_030, 37, "raw_grayling")),
    region(2, gather("pike_spot", 2, 44, 6_980, 51, "raw_pike")),
    region(2, gather("sturgeon_spot", 2, 54, 7_930, 69, "raw_sturgeon")),
    region(2, gather("snow_crab_spot", 2, 66, 9_070, 99, "raw_snow_crab")),
    region(2, gather("ice_eel_spot", 2, 80, 10_400, 146, "raw_ice_eel")),
    region(2, gather("frostfin_spot", 2, 94, 11_730, 207, "raw_frostfin")),
    region(2, make("burn_larch", 3, 5, 2_175, 41, &[("larch_logs", 1)], ("ashes", 1))),
    region(2, make("burn_spruce", 3, 14, 2_490, 48, &[("spruce_logs", 1)], ("ashes", 1))),
    region(2, make("burn_alpine_ash", 3, 24, 2_840, 63, &[("alpine_ash_logs", 1)], ("ashes", 1))),
    region(2, make("burn_birch", 3, 34, 3_190, 86, &[("birch_logs", 1)], ("ashes", 1))),
    region(2, make("burn_juniper", 3, 44, 3_540, 117, &[("juniper_logs", 1)], ("ashes", 1))),
    region(2, make("burn_stone_pine", 3, 54, 3_890, 157, &[("stone_pine_logs", 1)], ("ashes", 1))),
    region(2, make("burn_frostwood", 3, 66, 4_310, 214, &[("frostwood_logs", 1)], ("ashes", 1))),
    region(2, make("burn_cloudwood", 3, 80, 4_800, 296, &[("cloudwood_logs", 1)], ("ashes", 1))),
    region(2, make("burn_skyroot", 3, 94, 5_290, 393, &[("skyroot_logs", 1)], ("ashes", 1))),
    region(2, make("nickel_steel_bar", 5, 26, 3_280, 11, &[("nickel_ore", 1), ("iron_ore", 1)], ("nickel_steel_bar", 1))),
    region(2, make("platinum_bar", 5, 46, 3_880, 30, &[("platinum_ore", 1)], ("platinum_bar", 1))),
    region(2, make("frostiron_bar", 5, 56, 4_180, 43, &[("frostiron_ore", 1), ("coal", 2)], ("frostiron_bar", 1))),
    region(2, make("titanium_bar", 5, 68, 4_540, 62, &[("titanium_ore", 1), ("coal", 3)], ("titanium_bar", 1))),
    region(2, make("skysteel_bar", 5, 82, 4_960, 89, &[("skysteel_ore", 1), ("coal", 4)], ("skysteel_bar", 1))),
    region(2, make("aetherium_bar", 5, 96, 5_380, 122, &[("aetherium_ore", 1), ("coal", 5)], ("aetherium_bar", 1))),
    region(2, make("nickel_steel_sword", 5, 28, 3_340, 25, &[("nickel_steel_bar", 2)], ("nickel_steel_sword", 1))),
    region(2, make("frostiron_sword", 5, 58, 4_240, 92, &[("frostiron_bar", 2)], ("frostiron_sword", 1))),
    region(2, make("titanium_sword", 5, 70, 4_600, 131, &[("titanium_bar", 2)], ("titanium_sword", 1))),
    region(2, make("skysteel_sword", 5, 84, 5_020, 187, &[("skysteel_bar", 2)], ("skysteel_sword", 1))),
    region(2, make("aetherium_sword", 5, 98, 5_440, 253, &[("aetherium_bar", 2)], ("aetherium_sword", 1))),
    region(2, make("titanium_arrowheads", 5, 69, 2_285, 96, &[("titanium_bar", 1)], ("titanium_arrowheads", 15))),
    region(2, make("aetherium_arrowheads", 5, 97, 2_705, 186, &[("aetherium_bar", 1)], ("aetherium_arrowheads", 15))),
    region(2, make("frostiron_platebody", 5, 60, 5_300, 171, &[("frostiron_bar", 4)], ("frostiron_platebody", 1))),
    region(2, make("skysteel_platebody", 5, 86, 6_080, 343, &[("skysteel_bar", 4)], ("skysteel_platebody", 1))),
    region(2, make("titanium_gears", 5, 69, 4_570, 112, &[("titanium_bar", 1)], ("gear", 4))),
    region(2, make("nickel_steel_nails", 5, 30, 3_400, 14, &[("nickel_steel_bar", 1)], ("nails", 30))),
    region(2, make("larch_shafts", 4, 6, 2_000, 5, &[("larch_logs", 1)], ("arrow_shafts", 20))),
    region(2, make("spruce_shafts", 4, 15, 2_000, 5, &[("spruce_logs", 1)], ("arrow_shafts", 25))),
    region(2, make("spruce_bow", 4, 17, 3_010, 10, &[("spruce_logs", 2)], ("spruce_bow", 1))),
    region(2, make("birch_bow", 4, 37, 3_610, 35, &[("birch_logs", 2)], ("birch_bow", 1))),
    region(2, make("stone_pine_bow", 4, 57, 4_210, 77, &[("stone_pine_logs", 2)], ("stone_pine_bow", 1))),
    region(2, make("cloudwood_bow", 4, 83, 4_990, 160, &[("cloudwood_logs", 2)], ("cloudwood_bow", 1))),
    region(2, make("skyroot_bow", 4, 97, 5_410, 217, &[("skyroot_logs", 2)], ("skyroot_bow", 1))),
    region(2, make("nickel_steel_crossbow", 4, 29, 3_870, 29, &[("nickel_steel_bar", 2), ("alpine_ash_logs", 1)], ("nickel_steel_crossbow", 1))),
    region(2, make("titanium_crossbow", 4, 71, 5_130, 152, &[("titanium_bar", 2), ("frostwood_logs", 1)], ("titanium_crossbow", 1))),
    region(2, make("aetherium_crossbow", 4, 99, 5_970, 291, &[("aetherium_bar", 2), ("skyroot_logs", 1)], ("aetherium_crossbow", 1))),
    region(2, make("titanium_arrows", 4, 71, 2_500, 135, &[("arrow_shafts", 15), ("feathers", 15), ("titanium_arrowheads", 15)], ("titanium_arrows", 15))),
    region(2, make("aetherium_arrows", 4, 99, 2_500, 258, &[("arrow_shafts", 15), ("feathers", 15), ("aetherium_arrowheads", 15)], ("aetherium_arrows", 15))),
    region(2, make("cook_char", 6, 5, 2_125, 41, &[("raw_char", 1)], ("cooked_char", 1))),
    region(2, make("cook_ice_cod", 6, 14, 2_350, 61, &[("raw_ice_cod", 1)], ("cooked_ice_cod", 1))),
    region(2, make("cook_glacier_trout", 6, 24, 2_600, 83, &[("raw_glacier_trout", 1)], ("cooked_glacier_trout", 1))),
    region(2, make("cook_grayling", 6, 34, 2_850, 105, &[("raw_grayling", 1)], ("cooked_grayling", 1))),
    region(2, make("cook_pike", 6, 44, 3_100, 127, &[("raw_pike", 1)], ("cooked_pike", 1))),
    region(2, make("cook_sturgeon", 6, 54, 3_350, 149, &[("raw_sturgeon", 1)], ("cooked_sturgeon", 1))),
    region(2, make("cook_snow_crab", 6, 66, 3_650, 175, &[("raw_snow_crab", 1)], ("cooked_snow_crab", 1))),
    region(2, make("cook_ice_eel", 6, 80, 4_000, 206, &[("raw_ice_eel", 1)], ("cooked_ice_eel", 1))),
    region(2, make("cook_frostfin", 6, 94, 4_350, 237, &[("raw_frostfin", 1)], ("cooked_frostfin", 1))),
    region(2, make("hops_ice_cod_dish", 6, 21, 3_000, 95, &[("raw_ice_cod", 1), ("hops", 2)], ("hops_ice_cod_dish", 1))),
    region(2, make("buckwheat_grayling_dish", 6, 38, 3_000, 139, &[("raw_grayling", 1), ("buckwheat", 2)], ("buckwheat_grayling_dish", 1))),
    region(2, make("alpine_herb_sturgeon_dish", 6, 58, 3_000, 191, &[("raw_sturgeon", 1), ("alpine_herb", 2), ("mountain_tea", 1)], ("alpine_herb_sturgeon_dish", 1))),
    region(2, make("tan_wolf_leather", 12, 16, 2_000, 5, &[("wolf_pelt", 1)], ("wolf_leather", 1))),
    region(2, make("wolf_leather_body", 12, 36, 3_580, 38, &[("wolf_leather", 3)], ("wolf_leather_body", 1))),
    region(2, make("wolf_leather_chaps", 12, 56, 4_180, 86, &[("wolf_leather", 2)], ("wolf_leather_chaps", 1))),
    region(2, make("fur_cloth", 12, 38, 2_500, 16, &[("yeti_fur", 1)], ("fur_cloth", 1))),
    region(2, make("fur_robe", 12, 68, 4_540, 155, &[("fur_cloth", 4)], ("fur_robe", 1))),
    region(2, make("griffinhide_body", 12, 95, 6_850, 476, &[("griffin_hide", 3), ("wolf_leather", 2)], ("griffinhide_body", 1))),
    region(2, make("jade_ring", 12, 47, 3_910, 69, &[("platinum_bar", 1), ("jade", 1)], ("jade_ring", 1))),
    region(2, make("jade_amulet", 12, 69, 4_570, 144, &[("platinum_bar", 2), ("jade", 1)], ("jade_amulet", 1))),
    region(2, make("rock_crystal_glass", 12, 35, 2_500, 14, &[("rock_crystal", 1)], ("glass", 1))),
    region(2, make("wool_bracelet", 12, 28, 3_340, 22, &[("platinum_bar", 1), ("wool", 1)], ("wool_bracelet", 1))),
    region(2, make("ring_of_power_2", 12, 50, 5_000, 104, &[("platinum_bar", 1), ("ruby", 1), ("fire_rune", 20)], ("ring_of_power_2", 1))),
    region(2, make("amulet_of_vigour_2", 12, 55, 5_150, 124, &[("platinum_bar", 1), ("emerald", 1), ("water_rune", 20)], ("amulet_of_vigour_2", 1))),
    region(2, make("ring_of_wealth_2", 12, 60, 5_300, 146, &[("platinum_bar", 1), ("sapphire", 1), ("air_rune", 20)], ("ring_of_wealth_2", 1))),
    region(2, make("amulet_of_warding_2", 12, 65, 5_450, 171, &[("platinum_bar", 1), ("diamond", 1), ("earth_rune", 20)], ("amulet_of_warding_2", 1))),
    region(2, make("cosmic_runes", 7, 11, 2_720, 9, &[("rune_essence", 1)], ("cosmic_rune", 1))),
    region(2, make("dust_runes", 7, 22, 2_940, 10, &[("rune_essence", 1), ("earth_rune", 1), ("air_rune", 1)], ("dust_rune", 2))),
    region(2, make("blood_runes", 7, 32, 3_140, 12, &[("rune_essence", 1)], ("blood_rune", 1))),
    region(2, make("frost_runes", 7, 42, 3_340, 13, &[("rune_essence", 1)], ("frost_rune", 1))),
    region(2, make("storm_runes", 7, 57, 3_640, 15, &[("rune_essence", 1)], ("storm_rune", 1))),
    region(2, make("spirit_runes", 7, 72, 3_940, 17, &[("rune_essence", 1)], ("spirit_rune", 1))),
    region(2, make("alpine_ash_plank", 14, 25, 3_300, 9, &[("alpine_ash_logs", 1)], ("alpine_ash_plank", 1))),
    region(2, make("juniper_plank", 14, 45, 3_700, 25, &[("juniper_logs", 1)], ("juniper_plank", 1))),
    region(2, make("build_mule_train", 14, 25, 8_000, 31, &[("alpine_ash_plank", 8), ("nails", 40), ("wolf_leather", 4)], ("mule_train", 1))),
    region(2, make("build_anvil_stand", 14, 50, 7_000, 95, &[("juniper_plank", 6), ("nails", 30), ("mountain_tea", 2)], ("anvil_stand", 1))),
    region(2, make("build_bookcase", 14, 58, 7_000, 126, &[("juniper_plank", 6), ("nails", 30), ("mountain_tea", 2)], ("bookcase", 1))),
    region(2, make("build_tea_set", 14, 66, 7_000, 161, &[("juniper_plank", 6), ("nails", 30), ("wool", 2)], ("tea_set", 1))),
    region(2, make("take_apart_spruce_bow", 16, 11, 2_475, 5, &[("spruce_bow", 1)], ("wood_parts", 8))),
    region(2, make("take_apart_nickel_steel_sword", 16, 21, 2_725, 8, &[("nickel_steel_sword", 1)], ("metal_parts", 8))),
    region(2, make("take_apart_wolf_leather_body", 16, 31, 2_975, 15, &[("wolf_leather_body", 1)], ("cloth_parts", 9))),
    region(2, make("take_apart_birch_bow", 16, 41, 3_225, 24, &[("birch_bow", 1)], ("wood_parts", 9))),
    region(2, make("take_apart_titanium_sword", 16, 51, 3_475, 36, &[("titanium_sword", 1)], ("metal_parts", 10))),
    region(2, make("take_apart_fur_robe", 16, 61, 3_725, 50, &[("fur_robe", 1)], ("cloth_parts", 10))),
    region(2, make("take_apart_jade_amulet", 16, 71, 3_975, 67, &[("jade_amulet", 1)], ("magic_parts", 11))),
    region(2, make("take_apart_nickel_steel_crossbow", 16, 81, 4_225, 87, &[("nickel_steel_crossbow", 1)], ("metal_parts", 11))),
    region(2, make("take_apart_skysteel_sword", 16, 91, 4_475, 109, &[("skysteel_sword", 1)], ("metal_parts", 12))),
    region(2, make("build_ore_crusher", 16, 30, 15_000, 600, &[("metal_parts", 80), ("gear", 8), ("frame", 4)], ("ore_crusher", 1))),
    region(2, make("build_lumber_flume", 16, 45, 15_000, 900, &[("metal_parts", 80), ("gear", 8), ("frame", 4)], ("lumber_flume", 1))),
    region(2, make("build_tea_press", 16, 60, 15_000, 1200, &[("metal_parts", 80), ("gear", 8), ("frame", 4)], ("tea_press", 1))),
    region(2, make("build_tide_mill_wheel", 16, 40, 12_000, 490, &[("metal_parts", 70), ("spring", 10), ("lens", 2)], ("tide_mill_wheel", 1))),
    region(2, make("build_salt_pan_rakes", 16, 55, 12_000, 730, &[("metal_parts", 70), ("spring", 10), ("lens", 2)], ("salt_pan_rakes", 1))),
    region(2, make("build_brass_works_crucible", 16, 70, 12_000, 970, &[("metal_parts", 70), ("spring", 10), ("lens", 2)], ("brass_works_crucible", 1))),
    region(2, make("meditate_cosmic", 17, 10, 4_400, 28, &[("cosmic_rune", 10), ("starlight", 16)], ("", 0))),
    region(2, make("meditate_dust", 17, 24, 4_960, 42, &[("dust_rune", 10), ("starlight", 22)], ("", 0))),
    region(2, make("meditate_blood", 17, 38, 5_520, 68, &[("blood_rune", 10), ("starlight", 28)], ("", 0))),
    region(2, make("meditate_frost", 17, 52, 6_080, 106, &[("frost_rune", 10), ("starlight", 34)], ("", 0))),
    region(2, make("meditate_storm", 17, 66, 6_640, 156, &[("storm_rune", 10), ("starlight", 40)], ("", 0))),
    region(2, make("meditate_spirit", 17, 80, 7_200, 217, &[("spirit_rune", 10), ("starlight", 46), ("stardust", 1)], ("", 0))),
    region(2, make("hunters_stew", 6, 22, 4_660, 38, &[("cooked_glacier_trout", 1), ("buckwheat", 1), ("alpine_herb", 1)], ("hunters_stew", 1))),
    region(2, make("barley_bread", 6, 47, 5_410, 139, &[("cooked_pike", 1), ("hops", 2), ("mountain_tea", 1)], ("barley_bread", 1))),
    region(2, make("yeti_broth", 6, 82, 6_460, 402, &[("cooked_frostfin", 1), ("edelweiss", 1), ("snow_pea", 1)], ("yeti_broth", 1))),
    region(2, make("frostbrand", 5, 62, 5_860, 234, &[("skysteel_bar", 3), ("wool", 1), ("stone_pine_logs", 1)], ("frostbrand", 1))),
    region(2, make("eagle_bow", 4, 72, 6_160, 312, &[("titanium_bar", 2), ("cloudwood_logs", 1), ("fur_cloth", 2)], ("eagle_bow", 1))),
    region(2, make("storm_staff", 12, 77, 6_310, 355, &[("skyroot_logs", 1), ("wool", 2), ("spirit_rune", 20)], ("storm_staff", 1))),
    region(2, make("wolfskin_coat", 12, 52, 5_560, 167, &[("wolf_leather", 4), ("jade", 1), ("frostiron_bar", 1)], ("wolfskin_coat", 1))),
    region(2, make("titan_plate", 5, 87, 6_610, 451, &[("aetherium_bar", 4), ("wool", 2), ("wolf_leather", 2)], ("titan_plate", 1))),
    region(2, make("aurora_robe", 12, 92, 6_760, 503, &[("fur_cloth", 4), ("jade", 2), ("storm_rune", 30)], ("aurora_robe", 1))),
    region(3, gather("charwood_tree", 0, 7, 3_465, 13, "charwood_logs")),
    region(3, gather("ember_pine_tree", 0, 16, 4_320, 17, "ember_pine_logs")),
    region(3, gather("ashbark_tree", 0, 26, 5_270, 25, "ashbark_logs")),
    region(3, gather("sunwood_tree", 0, 36, 6_220, 37, "sunwood_logs")),
    region(3, gather("cinderoak_tree", 0, 46, 7_170, 53, "cinderoak_logs")),
    region(3, gather("glasswood_tree", 0, 56, 8_120, 75, "glasswood_logs")),
    region(3, gather("thornwood_tree", 0, 68, 9_260, 110, "thornwood_logs")),
    region(3, gather("emberheart_tree", 0, 82, 10_590, 166, "emberheart_logs")),
    region(3, gather("phoenixwood_tree", 0, 96, 11_920, 240, "phoenixwood_logs")),
    region(3, gather("basalt_rock", 1, 7, 3_465, 14, "basalt")),
    region(3, gather("black_sand_rock", 1, 16, 4_320, 19, "black_sand")),
    region(3, gather("brimstone_rock", 1, 26, 5_270, 26, "brimstone_ore")),
    region(3, gather("obsidian_rock", 1, 36, 6_220, 36, "obsidian")),
    region(3, gather("aurum_rock", 1, 46, 7_170, 50, "aurum_ore")),
    region(3, gather("starmetal_rock", 1, 56, 8_120, 69, "starmetal_ore")),
    region(3, gather("emberite_rock", 1, 68, 9_260, 99, "emberite_ore")),
    region(3, gather("dragonite_rock", 1, 82, 10_590, 145, "dragonite_ore")),
    region(3, gather("voidsteel_rock", 1, 96, 11_920, 207, "voidsteel_ore")),
    region(3, gather("lava_eel_spot", 2, 7, 3_465, 16, "raw_lava_eel")),
    region(3, gather("cinderfish_spot", 2, 16, 4_320, 21, "raw_cinderfish")),
    region(3, gather("ash_carp_spot", 2, 26, 5_270, 29, "raw_ash_carp")),
    region(3, gather("sandfish_spot", 2, 36, 6_220, 41, "raw_sandfish")),
    region(3, gather("fire_crab_spot", 2, 46, 7_170, 56, "raw_fire_crab")),
    region(3, gather("magma_ray_spot", 2, 56, 8_120, 76, "raw_magma_ray")),
    region(3, gather("sunfish_spot", 2, 68, 9_260, 109, "raw_sunfish")),
    region(3, gather("salamanderfish_spot", 2, 82, 10_590, 159, "raw_salamanderfish")),
    region(3, gather("phoenix_koi_spot", 2, 96, 11_920, 226, "raw_phoenix_koi")),
    region(3, make("burn_charwood", 3, 7, 2_245, 42, &[("charwood_logs", 1)], ("ashes", 1))),
    region(3, make("burn_ember_pine", 3, 16, 2_560, 50, &[("ember_pine_logs", 1)], ("ashes", 1))),
    region(3, make("burn_ashbark", 3, 26, 2_910, 67, &[("ashbark_logs", 1)], ("ashes", 1))),
    region(3, make("burn_sunwood", 3, 36, 3_260, 92, &[("sunwood_logs", 1)], ("ashes", 1))),
    region(3, make("burn_cinderoak", 3, 46, 3_610, 125, &[("cinderoak_logs", 1)], ("ashes", 1))),
    region(3, make("burn_glasswood", 3, 56, 3_960, 165, &[("glasswood_logs", 1)], ("ashes", 1))),
    region(3, make("burn_thornwood", 3, 68, 4_380, 225, &[("thornwood_logs", 1)], ("ashes", 1))),
    region(3, make("burn_emberheart", 3, 82, 4_870, 309, &[("emberheart_logs", 1)], ("ashes", 1))),
    region(3, make("burn_phoenixwood", 3, 96, 5_360, 409, &[("phoenixwood_logs", 1)], ("ashes", 1))),
    region(3, make("brimsteel_bar", 5, 28, 3_340, 13, &[("brimstone_ore", 1), ("iron_ore", 1)], ("brimsteel_bar", 1))),
    region(3, make("aurum_bar", 5, 48, 3_940, 33, &[("aurum_ore", 1)], ("aurum_bar", 1))),
    region(3, make("starmetal_bar", 5, 58, 4_240, 47, &[("starmetal_ore", 1), ("coal", 2)], ("starmetal_bar", 1))),
    region(3, make("emberite_bar", 5, 70, 4_600, 68, &[("emberite_ore", 1), ("coal", 3)], ("emberite_bar", 1))),
    region(3, make("dragonite_bar", 5, 84, 5_020, 97, &[("dragonite_ore", 1), ("coal", 4)], ("dragonite_bar", 1))),
    region(3, make("voidsteel_bar", 5, 98, 5_440, 131, &[("voidsteel_ore", 1), ("coal", 5)], ("voidsteel_bar", 1))),
    region(3, make("brimsteel_sword", 5, 30, 3_400, 29, &[("brimsteel_bar", 2)], ("brimsteel_sword", 1))),
    region(3, make("starmetal_sword", 5, 60, 4_300, 101, &[("starmetal_bar", 2)], ("starmetal_sword", 1))),
    region(3, make("emberite_sword", 5, 72, 4_660, 144, &[("emberite_bar", 2)], ("emberite_sword", 1))),
    region(3, make("dragonite_sword", 5, 86, 5_080, 203, &[("dragonite_bar", 2)], ("dragonite_sword", 1))),
    region(3, make("voidsteel_sword", 5, 99, 5_470, 268, &[("voidsteel_bar", 2)], ("voidsteel_sword", 1))),
    region(3, make("emberite_arrowheads", 5, 71, 2_315, 105, &[("emberite_bar", 1)], ("emberite_arrowheads", 15))),
    region(3, make("voidsteel_arrowheads", 5, 99, 2_735, 201, &[("voidsteel_bar", 1)], ("voidsteel_arrowheads", 15))),
    region(3, make("starmetal_platebody", 5, 62, 5_360, 189, &[("starmetal_bar", 4)], ("starmetal_platebody", 1))),
    region(3, make("dragonite_platebody", 5, 88, 6_140, 372, &[("dragonite_bar", 4)], ("dragonite_platebody", 1))),
    region(3, make("emberite_gears", 5, 71, 4_630, 122, &[("emberite_bar", 1)], ("gear", 4))),
    region(3, make("brimsteel_nails", 5, 32, 3_460, 16, &[("brimsteel_bar", 1)], ("nails", 30))),
    region(3, make("charwood_shafts", 4, 8, 2_000, 5, &[("charwood_logs", 1)], ("arrow_shafts", 20))),
    region(3, make("ember_pine_shafts", 4, 17, 2_000, 5, &[("ember_pine_logs", 1)], ("arrow_shafts", 25))),
    region(3, make("ember_pine_bow", 4, 19, 3_070, 12, &[("ember_pine_logs", 2)], ("ember_pine_bow", 1))),
    region(3, make("sunwood_bow", 4, 39, 3_670, 40, &[("sunwood_logs", 2)], ("sunwood_bow", 1))),
    region(3, make("glasswood_bow", 4, 59, 4_270, 86, &[("glasswood_logs", 2)], ("glasswood_bow", 1))),
    region(3, make("emberheart_bow", 4, 85, 5_050, 174, &[("emberheart_logs", 2)], ("emberheart_bow", 1))),
    region(3, make("phoenixwood_bow", 4, 99, 5_470, 234, &[("phoenixwood_logs", 2)], ("phoenixwood_bow", 1))),
    region(3, make("brimsteel_crossbow", 4, 31, 3_930, 34, &[("brimsteel_bar", 2), ("ashbark_logs", 1)], ("brimsteel_crossbow", 1))),
    region(3, make("emberite_crossbow", 4, 73, 5_190, 166, &[("emberite_bar", 2), ("thornwood_logs", 1)], ("emberite_crossbow", 1))),
    region(3, make("voidsteel_crossbow", 4, 99, 5_970, 301, &[("voidsteel_bar", 2), ("phoenixwood_logs", 1)], ("voidsteel_crossbow", 1))),
    region(3, make("emberite_arrows", 4, 73, 2_500, 148, &[("arrow_shafts", 15), ("feathers", 15), ("emberite_arrowheads", 15)], ("emberite_arrows", 15))),
    region(3, make("voidsteel_arrows", 4, 99, 2_500, 268, &[("arrow_shafts", 15), ("feathers", 15), ("voidsteel_arrowheads", 15)], ("voidsteel_arrows", 15))),
    region(3, make("cook_lava_eel", 6, 7, 2_175, 45, &[("raw_lava_eel", 1)], ("cooked_lava_eel", 1))),
    region(3, make("cook_cinderfish", 6, 16, 2_400, 65, &[("raw_cinderfish", 1)], ("cooked_cinderfish", 1))),
    region(3, make("cook_ash_carp", 6, 26, 2_650, 87, &[("raw_ash_carp", 1)], ("cooked_ash_carp", 1))),
    region(3, make("cook_sandfish", 6, 36, 2_900, 109, &[("raw_sandfish", 1)], ("cooked_sandfish", 1))),
    region(3, make("cook_fire_crab", 6, 46, 3_150, 131, &[("raw_fire_crab", 1)], ("cooked_fire_crab", 1))),
    region(3, make("cook_magma_ray", 6, 56, 3_400, 153, &[("raw_magma_ray", 1)], ("cooked_magma_ray", 1))),
    region(3, make("cook_sunfish", 6, 68, 3_700, 180, &[("raw_sunfish", 1)], ("cooked_sunfish", 1))),
    region(3, make("cook_salamanderfish", 6, 82, 4_050, 210, &[("raw_salamanderfish", 1)], ("cooked_salamanderfish", 1))),
    region(3, make("cook_phoenix_koi", 6, 96, 4_400, 241, &[("raw_phoenix_koi", 1)], ("cooked_phoenix_koi", 1))),
    region(3, make("sun_melon_cinderfish_dish", 6, 23, 3_000, 100, &[("raw_cinderfish", 1), ("sun_melon", 2)], ("sun_melon_cinderfish_dish", 1))),
    region(3, make("cactus_fruit_sandfish_dish", 6, 40, 3_000, 144, &[("raw_sandfish", 1), ("cactus_fruit", 2)], ("cactus_fruit_sandfish_dish", 1))),
    region(3, make("ember_bean_magma_ray_dish", 6, 60, 3_000, 196, &[("raw_magma_ray", 1), ("ember_bean", 2), ("fire_oil", 1)], ("ember_bean_magma_ray_dish", 1))),
    region(3, make("tan_salamander_leather", 12, 18, 2_000, 5, &[("salamander_skin", 1)], ("salamander_leather", 1))),
    region(3, make("salamander_leather_body", 12, 38, 3_640, 43, &[("salamander_leather", 3)], ("salamander_leather_body", 1))),
    region(3, make("salamander_leather_chaps", 12, 58, 4_240, 95, &[("salamander_leather", 2)], ("salamander_leather_chaps", 1))),
    region(3, make("ember_cloth", 12, 40, 2_500, 18, &[("ember_silk", 1)], ("ember_cloth", 1))),
    region(3, make("ember_robe", 12, 70, 4_600, 170, &[("ember_cloth", 4)], ("ember_robe", 1))),
    region(3, make("drakehide_body", 12, 97, 6_910, 515, &[("drake_hide", 3), ("salamander_leather", 2)], ("drakehide_body", 1))),
    region(3, make("fire_opal_ring", 12, 49, 3_970, 78, &[("aurum_bar", 1), ("fire_opal", 1)], ("fire_opal_ring", 1))),
    region(3, make("fire_opal_amulet", 12, 71, 4_630, 157, &[("aurum_bar", 2), ("fire_opal", 1)], ("fire_opal_amulet", 1))),
    region(3, make("obsidian_glass", 12, 37, 2_500, 15, &[("obsidian", 1)], ("glass", 1))),
    region(3, make("sunstone_bracelet", 12, 30, 3_400, 25, &[("aurum_bar", 1), ("sunstone", 1)], ("sunstone_bracelet", 1))),
    region(3, make("ring_of_power_3", 12, 70, 5_600, 204, &[("aurum_bar", 1), ("ruby", 1), ("fire_rune", 30)], ("ring_of_power_3", 1))),
    region(3, make("amulet_of_vigour_3", 12, 75, 5_750, 234, &[("aurum_bar", 1), ("emerald", 1), ("water_rune", 30)], ("amulet_of_vigour_3", 1))),
    region(3, make("ring_of_wealth_3", 12, 80, 5_900, 265, &[("aurum_bar", 1), ("sapphire", 1), ("air_rune", 30)], ("ring_of_wealth_3", 1))),
    region(3, make("amulet_of_warding_3", 12, 85, 6_050, 298, &[("aurum_bar", 1), ("diamond", 1), ("earth_rune", 30)], ("amulet_of_warding_3", 1))),
    region(3, make("soul_runes", 7, 13, 2_760, 9, &[("rune_essence", 1)], ("soul_rune", 1))),
    region(3, make("smoke_runes", 7, 24, 2_980, 11, &[("rune_essence", 1), ("fire_rune", 1), ("air_rune", 1)], ("smoke_rune", 2))),
    region(3, make("wrath_runes", 7, 34, 3_180, 12, &[("rune_essence", 1)], ("wrath_rune", 1))),
    region(3, make("ash_runes", 7, 44, 3_380, 13, &[("rune_essence", 1)], ("ash_rune", 1))),
    region(3, make("sun_runes", 7, 59, 3_680, 15, &[("rune_essence", 1)], ("sun_rune", 1))),
    region(3, make("void_runes", 7, 74, 3_980, 17, &[("rune_essence", 1)], ("void_rune", 1))),
    region(3, make("ashbark_plank", 14, 27, 3_340, 11, &[("ashbark_logs", 1)], ("ashbark_plank", 1))),
    region(3, make("cinderoak_plank", 14, 47, 3_740, 28, &[("cinderoak_logs", 1)], ("cinderoak_plank", 1))),
    region(3, make("build_sandship", 14, 40, 8_000, 71, &[("ashbark_plank", 8), ("nails", 40), ("salamander_leather", 4)], ("sandship", 1))),
    region(3, make("build_gold_scales", 14, 70, 7_000, 187, &[("cinderoak_plank", 6), ("nails", 30), ("fire_oil", 2)], ("gold_scales", 1))),
    region(3, make("build_sun_lamp", 14, 78, 7_000, 231, &[("cinderoak_plank", 6), ("nails", 30), ("fire_oil", 2)], ("sun_lamp", 1))),
    region(3, make("build_star_chart", 14, 86, 7_000, 280, &[("cinderoak_plank", 6), ("nails", 30), ("sunstone", 2)], ("star_chart", 1))),
    region(3, make("take_apart_ember_pine_bow", 16, 14, 2_550, 5, &[("ember_pine_bow", 1)], ("wood_parts", 10))),
    region(3, make("take_apart_brimsteel_sword", 16, 24, 2_800, 10, &[("brimsteel_sword", 1)], ("metal_parts", 10))),
    region(3, make("take_apart_salamander_leather_body", 16, 34, 3_050, 18, &[("salamander_leather_body", 1)], ("cloth_parts", 11))),
    region(3, make("take_apart_sunwood_bow", 16, 44, 3_300, 28, &[("sunwood_bow", 1)], ("wood_parts", 11))),
    region(3, make("take_apart_emberite_sword", 16, 54, 3_550, 41, &[("emberite_sword", 1)], ("metal_parts", 12))),
    region(3, make("take_apart_ember_robe", 16, 64, 3_800, 57, &[("ember_robe", 1)], ("cloth_parts", 12))),
    region(3, make("take_apart_fire_opal_amulet", 16, 74, 4_050, 76, &[("fire_opal_amulet", 1)], ("magic_parts", 13))),
    region(3, make("take_apart_brimsteel_crossbow", 16, 84, 4_300, 97, &[("brimsteel_crossbow", 1)], ("metal_parts", 13))),
    region(3, make("take_apart_dragonite_sword", 16, 94, 4_550, 121, &[("dragonite_sword", 1)], ("metal_parts", 14))),
    region(3, make("build_sun_furnace", 16, 40, 15_000, 750, &[("metal_parts", 100), ("gear", 10), ("frame", 5)], ("sun_furnace", 1))),
    region(3, make("build_glassworks", 16, 55, 15_000, 1200, &[("metal_parts", 100), ("gear", 10), ("frame", 5)], ("glassworks", 1))),
    region(3, make("build_ember_forge", 16, 70, 15_000, 1650, &[("metal_parts", 100), ("gear", 10), ("frame", 5)], ("ember_forge", 1))),
    region(3, make("build_crusher_hammers", 16, 50, 12_000, 610, &[("metal_parts", 90), ("spring", 12), ("lens", 3)], ("crusher_hammers", 1))),
    region(3, make("build_flume_gates", 16, 65, 12_000, 970, &[("metal_parts", 90), ("spring", 12), ("lens", 3)], ("flume_gates", 1))),
    region(3, make("build_press_screws", 16, 80, 12_000, 1330, &[("metal_parts", 90), ("spring", 12), ("lens", 3)], ("press_screws", 1))),
    region(3, make("meditate_soul", 17, 15, 4_600, 32, &[("soul_rune", 10), ("starlight", 20)], ("", 0))),
    region(3, make("meditate_smoke", 17, 29, 5_160, 50, &[("smoke_rune", 10), ("starlight", 26)], ("", 0))),
    region(3, make("meditate_wrath", 17, 43, 5_720, 80, &[("wrath_rune", 10), ("starlight", 32)], ("", 0))),
    region(3, make("meditate_ash", 17, 57, 6_280, 122, &[("ash_rune", 10), ("starlight", 38)], ("", 0))),
    region(3, make("meditate_sun", 17, 71, 6_840, 176, &[("sun_rune", 10), ("starlight", 44)], ("", 0))),
    region(3, make("meditate_void", 17, 85, 7_400, 242, &[("void_rune", 10), ("starlight", 50), ("stardust", 1)], ("", 0))),
    region(3, make("fire_curry", 6, 24, 4_720, 45, &[("cooked_ash_carp", 1), ("cactus_fruit", 1), ("ember_bean", 1)], ("fire_curry", 1))),
    region(3, make("sun_feast", 6, 49, 5_470, 155, &[("cooked_fire_crab", 1), ("sun_melon", 2), ("fire_oil", 1)], ("sun_feast", 1))),
    region(3, make("phoenix_roast", 6, 84, 6_520, 437, &[("cooked_phoenix_koi", 1), ("sunflower", 1), ("ash_wheat", 1)], ("phoenix_roast", 1))),
    region(3, make("sunblade", 5, 64, 5_920, 258, &[("dragonite_bar", 3), ("sunstone", 1), ("glasswood_logs", 1)], ("sunblade", 1))),
    region(3, make("drakebone_bow", 4, 74, 6_220, 341, &[("emberite_bar", 2), ("emberheart_logs", 1), ("ember_cloth", 2)], ("drakebone_bow", 1))),
    region(3, make("void_staff", 12, 79, 6_370, 388, &[("phoenixwood_logs", 1), ("sunstone", 2), ("void_rune", 20)], ("void_staff", 1))),
    region(3, make("drakescale_coat", 12, 54, 5_620, 186, &[("salamander_leather", 4), ("fire_opal", 1), ("starmetal_bar", 1)], ("drakescale_coat", 1))),
    region(3, make("voidsteel_plate", 5, 89, 6_670, 489, &[("voidsteel_bar", 4), ("sunstone", 2), ("salamander_leather", 2)], ("voidsteel_plate", 1))),
    region(3, make("ember_silk_robe", 12, 94, 6_820, 544, &[("ember_cloth", 4), ("fire_opal", 2), ("sun_rune", 30)], ("ember_silk_robe", 1))),
    // </gen:actions>
    // Performance (T18, IG-Q56 A): each song needs its instrument held (`SONG_INSTRUMENTS`).
    make("wooden_flute", 4, 3, 2_500, 12, &[("logs", 4)], ("wooden_flute", 1)),
    make("lute", 12, 18, 3_500, 45, &[("oak_plank", 2), ("flax", 4)], ("lute", 1)),
    make("iron_horn", 5, 28, 4_000, 70, &[("iron_bar", 3)], ("iron_horn", 1)),
    make("golden_harp", 12, 45, 5_000, 150, &[("maple_logs", 3), ("flax", 10), ("gold_bar", 1)], ("golden_harp", 1)),
    region(1, make("conch_horn", 12, 30, 4_400, 90, &[("coral", 2), ("brass_bar", 2)], ("conch_horn", 1))),
    region(2, make("alpine_horn", 4, 40, 4_600, 120, &[("alpine_ash_logs", 6), ("nickel_steel_bar", 1)], ("alpine_horn", 1))),
    region(3, make("war_drum", 12, 50, 5_200, 170, &[("salamander_leather", 3), ("ashbark_logs", 4)], ("war_drum", 1))),
    make("jig", PERFORMANCE, 1, 3_000, 10, &[], ("renown", 1)),
    make("ballad", PERFORMANCE, 10, 4_000, 15, &[], ("renown", 1)),
    make("lay_of_the_vale", PERFORMANCE, 20, 5_000, 22, &[], ("renown", 2)),
    make("drinking_song", PERFORMANCE, 30, 5_500, 30, &[], ("renown", 2)),
    make("war_march", PERFORMANCE, 40, 6_500, 42, &[], ("renown", 3)),
    make("harvest_hymn", PERFORMANCE, 50, 7_000, 52, &[], ("renown", 3)),
    make("court_air", PERFORMANCE, 60, 8_000, 75, &[], ("renown", 4)),
    make("song_of_the_hunt", PERFORMANCE, 75, 10_000, 125, &[], ("renown", 5)),
    make("legend_of_the_vale", PERFORMANCE, 90, 12_000, 190, &[], ("renown", 6)),
    region(1, make("sea_shanty", PERFORMANCE, 3, 3_090, 11, &[], ("renown", 2))),
    region(1, make("tide_song", PERFORMANCE, 12, 4_120, 16, &[], ("renown", 2))),
    region(1, make("gull_reel", PERFORMANCE, 22, 5_150, 24, &[], ("renown", 4))),
    region(1, make("shanty_of_the_deep", PERFORMANCE, 32, 5_660, 32, &[], ("renown", 4))),
    region(1, make("mermaids_air", PERFORMANCE, 42, 6_700, 45, &[], ("renown", 6))),
    region(1, make("storm_chorus", PERFORMANCE, 52, 7_210, 56, &[], ("renown", 6))),
    region(1, make("harbour_hymn", PERFORMANCE, 64, 8_240, 81, &[], ("renown", 8))),
    region(1, make("kraken_saga", PERFORMANCE, 78, 10_300, 135, &[], ("renown", 10))),
    region(1, make("song_of_the_coast", PERFORMANCE, 92, 12_360, 205, &[], ("renown", 12))),
    region(2, make("yodel", PERFORMANCE, 5, 3_180, 12, &[], ("renown", 3))),
    region(2, make("shepherds_pipe", PERFORMANCE, 14, 4_240, 17, &[], ("renown", 3))),
    region(2, make("snowfall_ballad", PERFORMANCE, 24, 5_300, 26, &[], ("renown", 6))),
    region(2, make("echo_song", PERFORMANCE, 34, 5_830, 35, &[], ("renown", 6))),
    region(2, make("alpine_call", PERFORMANCE, 44, 6_890, 49, &[], ("renown", 9))),
    region(2, make("avalanche_march", PERFORMANCE, 54, 7_420, 60, &[], ("renown", 9))),
    region(2, make("frost_hymn", PERFORMANCE, 66, 8_480, 87, &[], ("renown", 12))),
    region(2, make("griffin_saga", PERFORMANCE, 80, 10_600, 145, &[], ("renown", 15))),
    region(2, make("song_of_the_peaks", PERFORMANCE, 94, 12_720, 220, &[], ("renown", 18))),
    region(3, make("sand_chant", PERFORMANCE, 7, 3_270, 12, &[], ("renown", 4))),
    region(3, make("caravan_song", PERFORMANCE, 16, 4_360, 19, &[], ("renown", 4))),
    region(3, make("dune_ballad", PERFORMANCE, 26, 5_450, 27, &[], ("renown", 8))),
    region(3, make("fire_dance", PERFORMANCE, 36, 6_000, 37, &[], ("renown", 8))),
    region(3, make("mirage_air", PERFORMANCE, 46, 7_090, 52, &[], ("renown", 12))),
    region(3, make("drum_of_embers", PERFORMANCE, 56, 7_630, 64, &[], ("renown", 12))),
    region(3, make("sun_hymn", PERFORMANCE, 68, 8_720, 93, &[], ("renown", 16))),
    region(3, make("wyrm_saga", PERFORMANCE, 82, 10_900, 155, &[], ("renown", 20))),
    region(3, make("song_of_the_wastes", PERFORMANCE, 96, 13_080, 236, &[], ("renown", 24))),
];

/// Ids to table positions, built once: the full game has about 500 actions
/// and the time loop looks them up on every event.
fn index_of(ids: impl Iterator<Item = &'static str>) -> BTreeMap<&'static str, usize> {
    ids.enumerate().map(|(i, id)| (id, i)).collect()
}

pub fn action_index(id: &str) -> Option<usize> {
    static INDEX: OnceLock<BTreeMap<&'static str, usize>> = OnceLock::new();
    INDEX
        .get_or_init(|| index_of(ACTIONS.iter().map(|a| a.id)))
        .get(id)
        .copied()
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
    pub region: usize,
}

#[rustfmt::skip]
pub const ENEMIES: &[Enemy] = &[
    Enemy { id: "chicken", level: 1, hp: 30, hit: 10, weak: MELEE, drops: &[("feathers", 5, 1), ("bones", 1, 1), ("potato_seed", 2, 3)], gold: 0, region: 0 },
    Enemy { id: "cow", level: 1, hp: 80, hit: 10, weak: RANGED, drops: &[("cowhide", 1, 1), ("bones", 1, 1), ("sweetcorn_seed", 1, 6)], gold: 0, region: 0 },
    Enemy { id: "goblin", level: 5, hp: 50, hit: 20, weak: MAGIC, drops: &[("bones", 1, 1), ("feathers", 10, 4), ("onion_seed", 2, 3), ("cabbage_seed", 1, 4), ("flax_seed", 2, 4)], gold: 12, region: 0 },
    Enemy { id: "spider", level: 20, hp: 220, hit: 40, weak: MELEE, drops: &[("spider_silk", 1, 4)], gold: 30, region: 0 },
    Enemy { id: "hill_giant", level: 30, hp: 350, hit: 60, weak: MAGIC, drops: &[("big_bones", 1, 1), ("iron_ore", 3, 3), ("coal", 2, 4), ("tomato_seed", 2, 4)], gold: 60, region: 0 },
    Enemy { id: "black_knight", level: 45, hp: 420, hit: 80, weak: RANGED, drops: &[("steel_bar", 1, 3)], gold: 120, region: 0 },
    Enemy { id: "moss_giant", level: 60, hp: 600, hit: 100, weak: MELEE, drops: &[("big_bones", 1, 1), ("mithril_ore", 2, 3), ("strawberry_seed", 1, 4)], gold: 200, region: 0 },
    Enemy { id: "green_dragon", level: 80, hp: 1_200, hit: 180, weak: MAGIC, drops: &[("dragon_bones", 1, 1), ("dragon_scale", 1, 20), ("watermelon_seed", 2, 5)], gold: 600, region: 0 },
    // Champions (T11): one boss an area, each a Nobility deed
    Enemy { id: "goblin_warlord", level: 15, hp: 400, hit: 35, weak: MAGIC, drops: &[("bones", 1, 1), ("onion_seed", 5, 1)], gold: 300, region: 0 },
    Enemy { id: "broodmother", level: 30, hp: 900, hit: 60, weak: MELEE, drops: &[("spider_silk", 5, 1)], gold: 800, region: 0 },
    Enemy { id: "giant_chieftain", level: 45, hp: 1_500, hit: 85, weak: MAGIC, drops: &[("big_bones", 3, 1), ("coal", 10, 1)], gold: 1_500, region: 0 },
    Enemy { id: "black_knight_titan", level: 60, hp: 2_200, hit: 110, weak: RANGED, drops: &[("steel_bar", 5, 1)], gold: 3_000, region: 0 },
    Enemy { id: "elder_dragon", level: 85, hp: 4_000, hit: 200, weak: MAGIC, drops: &[("dragon_bones", 3, 1), ("dragon_scale", 2, 1)], gold: 8_000, region: 0 },
    // <gen:enemies>
    Enemy { id: "shore_crab", level: 8, hp: 143, hit: 27, weak: RANGED, drops: &[("bones", 1, 1), ("kelp_seed", 2, 3), ("feathers", 10, 2)], gold: 14, region: 1 },
    Enemy { id: "gull", level: 18, hp: 293, hit: 48, weak: MAGIC, drops: &[("bones", 1, 1), ("sea_beet_seed", 2, 3), ("sea_snake_skin", 1, 2)], gold: 50, region: 1 },
    Enemy { id: "smuggler", level: 28, hp: 457, hit: 69, weak: MELEE, drops: &[("bones", 1, 1), ("rice_seed", 2, 3)], gold: 106, region: 1 },
    Enemy { id: "sea_snake", level: 38, hp: 634, hit: 90, weak: RANGED, drops: &[("bones", 1, 1), ("samphire_seed", 2, 3), ("sea_snake_skin", 1, 2)], gold: 182, region: 1 },
    Enemy { id: "pirate", level: 50, hp: 865, hit: 116, weak: MAGIC, drops: &[("bones", 1, 1), ("pepper_seed", 2, 3), ("sea_silk", 1, 3)], gold: 300, region: 1 },
    Enemy { id: "drowned_knight", level: 62, hp: 1_116, hit: 141, weak: MELEE, drops: &[("bones", 1, 1), ("saffron_seed", 2, 3), ("sea_iron_ore", 3, 3)], gold: 446, region: 1 },
    Enemy { id: "reef_troll", level: 74, hp: 1_386, hit: 166, weak: RANGED, drops: &[("bones", 1, 1), ("sea_grape_seed", 2, 3)], gold: 621, region: 1 },
    Enemy { id: "sea_serpent", level: 88, hp: 1_727, hit: 195, weak: MAGIC, drops: &[("bones", 1, 1), ("sea_grape_seed", 2, 3), ("kraken_hide", 1, 25)], gold: 862, region: 1 },
    Enemy { id: "pirate_captain", level: 20, hp: 974, hit: 68, weak: MELEE, drops: &[("big_bones", 2, 1), ("kraken_hide", 1, 10), ("sea_grape_seed", 3, 2)], gold: 300, region: 1 },
    Enemy { id: "siren", level: 40, hp: 2_013, hit: 123, weak: RANGED, drops: &[("big_bones", 2, 1), ("kraken_hide", 1, 10), ("sea_grape_seed", 3, 2)], gold: 1_000, region: 1 },
    Enemy { id: "drowned_king", level: 55, hp: 2_901, hit: 164, weak: MAGIC, drops: &[("big_bones", 2, 1), ("kraken_hide", 1, 10), ("sea_grape_seed", 3, 2)], gold: 1_785, region: 1 },
    Enemy { id: "reef_titan", level: 70, hp: 3_882, hit: 205, weak: MELEE, drops: &[("big_bones", 2, 1), ("kraken_hide", 1, 10), ("sea_grape_seed", 3, 2)], gold: 2_800, region: 1 },
    Enemy { id: "kraken", level: 90, hp: 5_334, hit: 259, weak: RANGED, drops: &[("big_bones", 2, 1), ("kraken_hide", 1, 4), ("sea_grape_seed", 3, 2)], gold: 4_500, region: 1 },
    Enemy { id: "grey_wolf", level: 10, hp: 188, hit: 33, weak: MAGIC, drops: &[("bones", 1, 1), ("barley_seed", 2, 3), ("feathers", 10, 2)], gold: 20, region: 2 },
    Enemy { id: "mountain_goat", level: 20, hp: 354, hit: 55, weak: MELEE, drops: &[("bones", 1, 1), ("hops_seed", 2, 3), ("wolf_pelt", 1, 2)], gold: 60, region: 2 },
    Enemy { id: "highwayman", level: 30, hp: 536, hit: 77, weak: RANGED, drops: &[("bones", 1, 1), ("buckwheat_seed", 2, 3)], gold: 120, region: 2 },
    Enemy { id: "yeti", level: 40, hp: 732, hit: 99, weak: MAGIC, drops: &[("bones", 1, 1), ("lingonberry_seed", 2, 3), ("wolf_pelt", 1, 2)], gold: 200, region: 2 },
    Enemy { id: "harpy", level: 52, hp: 988, hit: 125, weak: MELEE, drops: &[("bones", 1, 1), ("alpine_herb_seed", 2, 3), ("yeti_fur", 1, 3)], gold: 322, region: 2 },
    Enemy { id: "stone_golem", level: 64, hp: 1_265, hit: 152, weak: RANGED, drops: &[("bones", 1, 1), ("snow_pea_seed", 2, 3), ("titanium_ore", 3, 3)], gold: 473, region: 2 },
    Enemy { id: "frost_troll", level: 76, hp: 1_564, hit: 178, weak: MAGIC, drops: &[("bones", 1, 1), ("edelweiss_seed", 2, 3)], gold: 653, region: 2 },
    Enemy { id: "griffin", level: 90, hp: 1_940, hit: 209, weak: MELEE, drops: &[("bones", 1, 1), ("edelweiss_seed", 2, 3), ("griffin_hide", 1, 25)], gold: 900, region: 2 },
    Enemy { id: "wolf_king", level: 22, hp: 1_167, hit: 77, weak: RANGED, drops: &[("big_bones", 2, 1), ("griffin_hide", 1, 10), ("edelweiss_seed", 3, 2)], gold: 350, region: 2 },
    Enemy { id: "yeti_chief", level: 42, hp: 2_319, hit: 134, weak: MAGIC, drops: &[("big_bones", 2, 1), ("griffin_hide", 1, 10), ("edelweiss_seed", 3, 2)], gold: 1_090, region: 2 },
    Enemy { id: "stone_colossus", level: 57, hp: 3_301, hit: 177, weak: MELEE, drops: &[("big_bones", 2, 1), ("griffin_hide", 1, 10), ("edelweiss_seed", 3, 2)], gold: 1_905, region: 2 },
    Enemy { id: "frost_giant", level: 72, hp: 4_385, hit: 220, weak: RANGED, drops: &[("big_bones", 2, 1), ("griffin_hide", 1, 10), ("edelweiss_seed", 3, 2)], gold: 2_950, region: 2 },
    Enemy { id: "griffin_queen", level: 92, hp: 5_987, hit: 277, weak: MAGIC, drops: &[("big_bones", 2, 1), ("griffin_hide", 1, 4), ("edelweiss_seed", 3, 2)], gold: 4_690, region: 2 },
    Enemy { id: "fire_imp", level: 12, hp: 238, hit: 39, weak: MELEE, drops: &[("bones", 1, 1), ("fire_pepper_seed", 2, 3), ("feathers", 10, 2)], gold: 26, region: 3 },
    Enemy { id: "salamander", level: 22, hp: 422, hit: 62, weak: RANGED, drops: &[("bones", 1, 1), ("sun_melon_seed", 2, 3), ("salamander_skin", 1, 2)], gold: 70, region: 3 },
    Enemy { id: "scorpion", level: 32, hp: 621, hit: 85, weak: MAGIC, drops: &[("bones", 1, 1), ("cactus_fruit_seed", 2, 3)], gold: 134, region: 3 },
    Enemy { id: "ash_wraith", level: 42, hp: 838, hit: 108, weak: MELEE, drops: &[("bones", 1, 1), ("dates_seed", 2, 3), ("salamander_skin", 1, 2)], gold: 218, region: 3 },
    Enemy { id: "dune_raider", level: 54, hp: 1_118, hit: 136, weak: RANGED, drops: &[("bones", 1, 1), ("ember_bean_seed", 2, 3), ("ember_silk", 1, 3)], gold: 345, region: 3 },
    Enemy { id: "lava_golem", level: 66, hp: 1_423, hit: 163, weak: MAGIC, drops: &[("bones", 1, 1), ("ash_wheat_seed", 2, 3), ("emberite_ore", 3, 3)], gold: 501, region: 3 },
    Enemy { id: "fire_elemental", level: 78, hp: 1_750, hit: 191, weak: MELEE, drops: &[("bones", 1, 1), ("sunflower_seed", 2, 3)], gold: 686, region: 3 },
    Enemy { id: "fire_drake", level: 92, hp: 2_162, hit: 223, weak: RANGED, drops: &[("bones", 1, 1), ("sunflower_seed", 2, 3), ("drake_hide", 1, 25)], gold: 938, region: 3 },
    Enemy { id: "imp_lord", level: 24, hp: 1_381, hit: 87, weak: MAGIC, drops: &[("big_bones", 2, 1), ("drake_hide", 1, 10), ("sunflower_seed", 3, 2)], gold: 405, region: 3 },
    Enemy { id: "sand_sphinx", level: 44, hp: 2_648, hit: 147, weak: MELEE, drops: &[("big_bones", 2, 1), ("drake_hide", 1, 10), ("sunflower_seed", 3, 2)], gold: 1_185, region: 3 },
    Enemy { id: "ash_titan", level: 59, hp: 3_727, hit: 191, weak: RANGED, drops: &[("big_bones", 2, 1), ("drake_hide", 1, 10), ("sunflower_seed", 3, 2)], gold: 2_035, region: 3 },
    Enemy { id: "lava_king", level: 74, hp: 4_915, hit: 236, weak: MAGIC, drops: &[("big_bones", 2, 1), ("drake_hide", 1, 10), ("sunflower_seed", 3, 2)], gold: 3_105, region: 3 },
    Enemy { id: "ember_wyrm", level: 94, hp: 6_670, hit: 296, weak: MELEE, drops: &[("big_bones", 2, 1), ("drake_hide", 1, 4), ("sunflower_seed", 3, 2)], gold: 4_885, region: 3 },
    // </gen:enemies>
];

/// The champions of each region, in deed order: the region ladder's third
/// rank asks for the first (T11, T15).
#[rustfmt::skip]
pub const CHAMPIONS: [[&str; 5]; 4] = [
    ["goblin_warlord", "broodmother", "giant_chieftain", "black_knight_titan", "elder_dragon"],
    // <gen:champions>
    ["pirate_captain", "siren", "drowned_king", "reef_titan", "kraken"],
    ["wolf_king", "yeti_chief", "stone_colossus", "frost_giant", "griffin_queen"],
    ["imp_lord", "sand_sphinx", "ash_titan", "lava_king", "ember_wyrm"],
    // </gen:champions>
];

pub fn is_champion(id: &str) -> bool {
    CHAMPIONS.iter().any(|row| row.contains(&id))
}

pub fn enemy_index(id: &str) -> Option<usize> {
    static INDEX: OnceLock<BTreeMap<&'static str, usize>> = OnceLock::new();
    INDEX
        .get_or_init(|| index_of(ENEMIES.iter().map(|e| e.id)))
        .get(id)
        .copied()
}

/// A dungeon (T15, IG-Q42 A): a region's enemies in a fixed order, ending at
/// its strongest champion, with one chest at the end.
pub struct Dungeon {
    pub id: &'static str,
    pub region: usize,
    pub stages: &'static [&'static str],
}

#[rustfmt::skip]
pub const DUNGEONS: &[Dungeon] = &[
    // <gen:dungeons>
    Dungeon { id: "sunken_ship", region: 1, stages: &["shore_crab", "gull", "smuggler", "sea_snake", "pirate", "drowned_knight", "reef_troll", "sea_serpent", "kraken"] },
    Dungeon { id: "frozen_halls", region: 2, stages: &["grey_wolf", "mountain_goat", "highwayman", "yeti", "harpy", "stone_golem", "frost_troll", "griffin", "griffin_queen"] },
    Dungeon { id: "ember_depths", region: 3, stages: &["fire_imp", "salamander", "scorpion", "ash_wraith", "dune_raider", "lava_golem", "fire_elemental", "fire_drake", "ember_wyrm"] },
    // </gen:dungeons>
];

pub fn dungeon_index(id: &str) -> Option<usize> {
    DUNGEONS.iter().position(|d| d.id == id)
}
/// A chest pays this many times the gold of the run's enemies, fine parts,
/// and one in this many a recipe of the region.
pub const CHEST_GOLD_TIMES: u64 = 2;
pub const CHEST_FINE_PARTS: u64 = 3;
pub const CHEST_RECIPE_ONE_IN: u64 = 4;

/// A fighting style's swing time, and the items it holds (`held`, kept) or
/// uses up (`spent`, one per swing). Tiers raise the most a swing can hit.
pub struct Style {
    pub skill: usize,
    pub ms: u64,
    pub held: &'static [(&'static str, u64)],
    /// A held item helps but is not needed (Magic's staff, T10).
    pub held_optional: bool,
    pub spent: &'static [(&'static str, u64)],
}

#[rustfmt::skip]
pub const STYLES: [Style; 3] = [
    // Every list is best first; the region rows are written by tools/content/regions.py.
    Style { skill: MELEE, ms: 2_400, held_optional: false, spent: &[], held: &[
        // <gen:melee_held>
        ("voidsteel_sword", 16), ("aetherium_sword", 15), ("sunblade", 15), ("abyssal_sword", 14), ("frostbrand", 14), ("dragonite_sword", 14),
        ("tidecutter", 13), ("skysteel_sword", 13), ("orichalcum_sword", 12), ("emberite_sword", 12), ("titanium_sword", 11), ("starmetal_sword", 11),
        ("sea_iron_sword", 10), ("frostiron_sword", 10), ("cobalt_sword", 8), ("brimsteel_sword", 7), ("nickel_steel_sword", 6), ("giants_maul", 5),
        ("mithril_sword", 4), ("brass_sword", 4), ("steel_sword", 3), ("iron_sword", 2), ("bronze_sword", 1),
        // </gen:melee_held>
    ] },
    Style { skill: RANGED, ms: 2_000, held_optional: false, held: &[
        // <gen:ranged_held>
        ("voidsteel_crossbow", 17), ("aetherium_crossbow", 16), ("phoenixwood_bow", 16), ("drakebone_bow", 16), ("abyssal_crossbow", 15), ("skyroot_bow", 15),
        ("eagle_bow", 15), ("harpoon_bow", 14), ("emberheart_bow", 14), ("emberite_crossbow", 14), ("ghostwood_bow", 13), ("cloudwood_bow", 13),
        ("salt_cedar_bow", 12), ("titanium_crossbow", 12), ("sea_iron_crossbow", 11), ("glasswood_bow", 11), ("stone_pine_bow", 9), ("sea_pine_bow", 8),
        ("sunwood_bow", 8), ("brimsteel_crossbow", 8), ("knights_longbow", 7), ("birch_bow", 7), ("nickel_steel_crossbow", 7), ("magic_shortbow", 6),
        ("mangrove_bow", 6), ("brass_crossbow", 6), ("yew_shortbow", 5), ("ember_pine_bow", 5), ("maple_shortbow", 4), ("spruce_bow", 4),
        ("willow_shortbow", 3), ("palm_bow", 3), ("oak_shortbow", 2), ("shortbow", 1),
        // </gen:ranged_held>
    ], spent: &[
        // <gen:ranged_spent>
        ("voidsteel_arrows", 16), ("aetherium_arrows", 15), ("abyssal_arrows", 14), ("emberite_arrows", 13), ("titanium_arrows", 11), ("sea_iron_arrows", 10),
        ("mithril_arrows", 4), ("steel_arrows", 3), ("iron_arrows", 2), ("bronze_arrows", 1),
        // </gen:ranged_spent>
    ] },
    Style { skill: MAGIC, ms: 3_000, held_optional: true, held: &[
        // <gen:magic_held>
        ("void_staff", 16), ("storm_staff", 15), ("coral_staff", 14), ("dragonfire_staff", 6),
        // </gen:magic_held>
    ], spent: &[
        // <gen:magic_spent>
        ("void_rune", 13), ("spirit_rune", 11), ("sun_rune", 11), ("law_rune", 10), ("storm_rune", 9), ("ash_rune", 9),
        ("nature_rune", 8), ("frost_rune", 8), ("smoke_rune", 7), ("wrath_rune", 7), ("astral_rune", 6), ("mist_rune", 6),
        ("body_rune", 6), ("dust_rune", 6), ("blood_rune", 6), ("death_rune", 5), ("soul_rune", 5), ("chaos_rune", 4),
        ("mind_rune", 4), ("fire_rune", 3), ("cosmic_rune", 3), ("water_rune", 2), ("earth_rune", 2), ("air_rune", 1),
        // </gen:magic_spent>
    ] },
];

/// Food, best first, and how much it heals.
#[rustfmt::skip]
pub const FOOD: &[(&str, u64)] = &[
    // <gen:food>
    ("phoenix_roast", 500), ("yeti_broth", 440), ("sun_feast", 400), ("ink_risotto", 380), ("cooked_phoenix_koi", 378), ("cooked_frostfin", 352),
    ("barley_bread", 340), ("fire_curry", 340), ("cooked_salamanderfish", 336), ("cooked_angler", 326), ("ember_bean_magma_ray_dish", 320), ("cooked_ice_eel", 310),
    ("cooked_sunfish", 294), ("cooked_ray", 284), ("alpine_herb_sturgeon_dish", 284), ("fishermans_pie", 280), ("hunters_stew", 280), ("cooked_snow_crab", 268),
    ("dragon_pepper_feast", 260), ("cactus_fruit_sandfish_dish", 260), ("cooked_magma_ray", 258), ("pepper_eel_dish", 248), ("cooked_tuna", 242), ("cooked_sturgeon", 232),
    ("cooked_fire_crab", 228), ("buckwheat_grayling_dish", 224), ("crab_cakes", 220), ("sun_melon_cinderfish_dish", 209), ("cooked_eel", 206), ("cooked_pike", 202),
    ("shark", 200), ("cooked_sandfish", 198), ("rice_bass_dish", 188), ("cooked_squid", 176), ("hops_ice_cod_dish", 173), ("cooked_grayling", 172),
    ("cooked_ash_carp", 168), ("monkfish", 160), ("cooked_bass", 146), ("cooked_glacier_trout", 142), ("swordfish", 140), ("cooked_cinderfish", 138),
    ("sea_beet_sardine_dish", 137), ("goblin_hot_pot", 120), ("lobster", 120), ("cooked_mackerel", 116), ("cooked_ice_cod", 112), ("cooked_lava_eel", 111),
    ("farmers_stew", 90), ("salmon", 90), ("cooked_sardine", 86), ("cooked_char", 85), ("trout", 70), ("cooked_sweetcorn", 60),
    ("cooked_crab", 59), ("baked_potato", 50), ("shrimp", 30),
    // </gen:food>
];

/// Armour, best first: (item, tier, extra hitpoints). Each tier takes this
/// many percent off every hit an enemy lands, up to tier 4 (IG-Q23 A); the
/// regions' armour stays at tier 4 and adds hitpoints instead (R9.5).
#[rustfmt::skip]
pub const ARMOUR: &[(&str, u64, u64)] = &[
    // <gen:armour>
    ("voidsteel_plate", 4, 180), ("ember_silk_robe", 4, 150), ("titan_plate", 4, 120), ("drakehide_body", 4, 120), ("aurora_robe", 4, 100),
    ("griffinhide_body", 4, 80), ("dragonite_platebody", 4, 70), ("barnacle_plate", 4, 60), ("starmetal_platebody", 4, 60), ("pearl_mantle", 4, 50),
    ("skysteel_platebody", 4, 50), ("krakenhide_body", 4, 40), ("frostiron_platebody", 4, 40), ("orichalcum_platebody", 4, 30), ("cobalt_platebody", 4, 20),
    ("mossplate", 4, 0), ("dragonhide_body", 4, 0), ("drakescale_coat", 3, 90), ("ember_robe", 3, 75), ("wolfskin_coat", 3, 60),
    ("fur_robe", 3, 50), ("sharkskin_vest", 3, 30), ("salamander_leather_body", 3, 30), ("sea_silk_robe", 3, 25), ("wolf_leather_body", 3, 20),
    ("snakeskin_body", 3, 10), ("silkweave_vest", 3, 0), ("silk_robe", 3, 0), ("salamander_leather_chaps", 2, 30), ("wolf_leather_chaps", 2, 20),
    ("snakeskin_chaps", 2, 10), ("studded_leather", 2, 0), ("hard_leather_body", 2, 0), ("leather_body", 1, 0),
    // </gen:armour>
];
pub const ARMOUR_PCT_PER_TIER: u64 = 10;

/// A crop (T5): a seed planted in a patch grows for `grow_ms` of game time,
/// whatever else is running, and waits until it is harvested.
pub struct Crop {
    pub id: &'static str,
    pub level: u32,
    pub grow_ms: u64,
    pub seed: &'static str,
    pub produce: &'static str,
    pub xp_plant: u64,
    pub xp_each: u64,
    pub yield_min: u64,
    pub yield_max: u64,
    /// Gold for one seed in the shop.
    pub seed_cost: u64,
    pub region: usize,
}

const fn crop(
    id: &'static str,
    level: u32,
    minutes: u64,
    seed: &'static str,
    xp_plant: u64,
    xp_each: u64,
    cost: u64,
) -> Crop {
    Crop {
        id,
        level,
        grow_ms: minutes * 60_000,
        seed,
        produce: id,
        xp_plant,
        xp_each,
        yield_min: 3,
        yield_max: 6,
        seed_cost: cost,
        region: 0,
    }
}

const fn region_crop(region: usize, c: Crop) -> Crop {
    Crop { region, ..c }
}

#[rustfmt::skip]
pub const CROPS: &[Crop] = &[
    crop("potato", 1, 10, "potato_seed", 8, 9, 2),
    crop("onion", 5, 15, "onion_seed", 10, 11, 4),
    crop("cabbage", 9, 20, "cabbage_seed", 12, 14, 6),
    crop("tomato", 20, 30, "tomato_seed", 15, 18, 12),
    crop("sweetcorn", 28, 45, "sweetcorn_seed", 20, 24, 20),
    crop("strawberry", 40, 60, "strawberry_seed", 26, 31, 40),
    crop("watermelon", 55, 90, "watermelon_seed", 48, 55, 90),
    // Strings for the bard's lute and harp (T18).
    crop("flax", 3, 15, "flax_seed", 9, 10, 3),
    // <gen:crops>
    region_crop(1, crop("kelp", 5, 20, "kelp_seed", 10, 12, 2)),
    region_crop(1, crop("sea_beet", 15, 30, "sea_beet_seed", 15, 19, 7)),
    region_crop(1, crop("rice", 25, 45, "rice_seed", 20, 25, 20)),
    region_crop(1, crop("samphire", 35, 60, "samphire_seed", 25, 32, 40)),
    region_crop(1, crop("pepper", 45, 90, "pepper_seed", 30, 39, 67)),
    region_crop(1, crop("saffron", 60, 120, "saffron_seed", 38, 49, 120)),
    region_crop(1, crop("sea_grape", 75, 180, "sea_grape_seed", 45, 59, 187)),
    region_crop(2, crop("barley", 7, 20, "barley_seed", 11, 13, 2)),
    region_crop(2, crop("hops", 17, 30, "hops_seed", 16, 20, 9)),
    region_crop(2, crop("buckwheat", 27, 45, "buckwheat_seed", 21, 27, 24)),
    region_crop(2, crop("lingonberry", 37, 60, "lingonberry_seed", 26, 33, 45)),
    region_crop(2, crop("alpine_herb", 47, 90, "alpine_herb_seed", 31, 40, 73)),
    region_crop(2, crop("snow_pea", 62, 120, "snow_pea_seed", 39, 50, 128)),
    region_crop(2, crop("edelweiss", 77, 180, "edelweiss_seed", 46, 60, 197)),
    region_crop(3, crop("fire_pepper", 9, 20, "fire_pepper_seed", 12, 15, 2)),
    region_crop(3, crop("sun_melon", 19, 30, "sun_melon_seed", 17, 21, 12)),
    region_crop(3, crop("cactus_fruit", 29, 45, "cactus_fruit_seed", 22, 28, 28)),
    region_crop(3, crop("dates", 39, 60, "dates_seed", 27, 35, 50)),
    region_crop(3, crop("ember_bean", 49, 90, "ember_bean_seed", 32, 41, 80)),
    region_crop(3, crop("ash_wheat", 64, 120, "ash_wheat_seed", 40, 51, 136)),
    region_crop(3, crop("sunflower", 79, 180, "sunflower_seed", 47, 61, 208)),
    // </gen:crops>
];

pub fn crop_index(id: &str) -> Option<usize> {
    static INDEX: OnceLock<BTreeMap<&'static str, usize>> = OnceLock::new();
    INDEX
        .get_or_init(|| index_of(CROPS.iter().map(|c| c.id)))
        .get(id)
        .copied()
}

/// Ashes are compost (T8, IG-Q21 B): planting uses one if you have any, and
/// that crop grows in this many percent of its time.
pub const COMPOST_GROW_PCT: u64 = 75;

/// The Farming level that opens each patch.
pub const PATCH_LEVELS: [u32; 5] = [1, 1, 1, 40, 70];
/// The shop sells seeds in packs of this many.
pub const SEED_PACK: u64 = 10;

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
/// The combat log keeps this many newest lines (T19).
pub const COMBAT_LOG: usize = 12;
/// The encyclopedia shows this many search results at once (T19).
pub const BOOK_ROWS: usize = 40;
/// The app asks for a rating the first time a skill reaches this level.
pub const REVIEW_LEVEL: u32 = 25;
/// Experience per point of damage, in tenths (style) and thousandths (Hitpoints).
pub const STYLE_XP_TENTHS: u64 = 4;
pub const HP_XP_THOUSANDTHS: u64 = 133;

/// Tools (P10): one line per gathering skill, six tiers each.
pub const TOOL_SKILLS: [usize; 3] = [0, 1, 2];
/// (cost in gold, percent faster)
pub const TOOL_TIERS: [(u64, u64); 8] = [
    (0, 0),
    (500, 5),
    (4_000, 10),
    (25_000, 15),
    (120_000, 20),
    (600_000, 25),
    // Opened by the Trader (T18, IG-Q55 A).
    (3_000_000, 30),
    (15_000_000, 35),
];
/// Tool tiers past this one need the Trader's rank `TOOL_TIER_RANK` in the Vale.
pub const TOOL_TIERS_FREE: usize = 5;
pub const TOOL_TIER_RANK: usize = 2;

/// The Trader's shelves in the shop (T18, IG-Q55 A): a Vale rank and what
/// it puts on sale. Each later region's first rank adds that region's
/// goods: what its actions need that no action makes (`region_goods`).
#[rustfmt::skip]
pub const SHELVES: &[(usize, &[&str])] = &[
    (1, &["feathers", "cowhide", "coal", "raw_shrimp", "potato"]),
    (2, &["spider_silk", "gold_ore", "onion", "tomato"]),
    (3, &["big_bones", "magic_parts", "raw_lobster", "sweetcorn"]),
    (4, &["dragon_scale", "yew_logs", "strawberry"]),
];
/// The shop sells at this many times what it pays, in packs.
pub const SHOP_MARKUP: u64 = 5;
pub const SHOP_PACK: u64 = 10;

pub fn shop_price(item: &str) -> u64 {
    (value(item) * SHOP_MARKUP).max(SHOP_MARKUP)
}

/// What a later region's actions need that no action makes but an enemy
/// drops or a crop grows, cheapest first.
pub fn region_goods(r: usize) -> Vec<&'static str> {
    let dropped = |i: &str| {
        ENEMIES.iter().any(|e| e.drops.iter().any(|d| d.0 == i)) || CROPS.iter().any(|cr| cr.produce == i)
    };
    let mut out: Vec<&'static str> = Vec::new();
    for a in ACTIONS.iter().filter(|a| a.region == r) {
        for &(i, _) in a.needs {
            if value(i) > 0 && !ACTIONS.iter().any(|b| b.makes.0 == i) && dropped(i) && !out.contains(&i) {
                out.push(i);
            }
        }
    }
    out.sort_by_key(|i| (value(i), *i));
    out
}

/// What the shop pays for one of each item.
#[rustfmt::skip]
pub const VALUES: &[(&str, u64)] = &[
    // The bard (T18): instruments, flax; renown is the Fame engine's and does not sell.
    ("wooden_flute", 30), ("lute", 120), ("iron_horn", 90), ("golden_harp", 600), ("conch_horn", 300),
    ("alpine_horn", 500), ("war_drum", 900), ("flax", 3), ("flax_seed", 2), ("renown", 0),
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
    ("leather", 20), ("leather_gloves", 30), ("leather_body", 80), ("hard_leather_body", 160), ("silk_cloth", 40),
    ("silk_robe", 220), ("gold_ring", 150), ("gold_amulet", 320), ("dragonhide_body", 2_000),
    ("potato", 2), ("onion", 3), ("cabbage", 4), ("tomato", 8), ("sweetcorn", 12), ("strawberry", 25), ("watermelon", 60),
    ("baked_potato", 6), ("cooked_sweetcorn", 25),
    ("potato_seed", 1), ("onion_seed", 2), ("cabbage_seed", 3), ("tomato_seed", 6), ("sweetcorn_seed", 10),
    ("strawberry_seed", 20), ("watermelon_seed", 45),
    // the engines (T10 to T14)
    ("limestone", 4), ("nails", 1), ("gear", 60), ("spring", 10), ("lens", 90), ("royal_robe", 600),
    ("farmers_stew", 20), ("goblin_hot_pot", 45), ("dragon_pepper_feast", 220),
    ("studded_leather", 120), ("silkweave_vest", 400), ("dragonfire_staff", 3_000), ("giants_maul", 700),
    ("mossplate", 1_500), ("knights_longbow", 1_200),
    ("plank", 4), ("cart", 150), ("oak_plank", 10), ("frame", 60), ("teak_plank", 25), ("mahogany_plank", 50),
    ("wood_parts", 3), ("metal_parts", 4), ("cloth_parts", 4), ("magic_parts", 12), ("fine_parts", 150),
    ("axe_gadget", 0), ("pick_gadget", 0), ("rod_gadget", 0), ("sawmill", 0), ("auto_smelter", 0), ("fish_trap", 0),
    ("spices", 30), ("dye", 40), ("glass", 35),
    ("starlight", 0), ("stardust", 0), ("star_core", 0), ("astral_rune", 20),
    // <gen:values>
    ("driftwood_logs", 2), ("palm_logs", 5), ("cork_logs", 14), ("mangrove_logs", 27), ("cypress_logs", 46), ("sea_pine_logs", 69),
    ("ironbark_logs", 104), ("salt_cedar_logs", 154), ("ghostwood_logs", 213), ("rock_salt", 2), ("sandstone", 5), ("zinc_ore", 14),
    ("quartz", 27), ("silver_ore", 46), ("cobalt_ore", 69), ("sea_iron_ore", 104), ("orichalcum_ore", 154), ("abyssal_ore", 213),
    ("raw_crab", 3), ("raw_sardine", 6), ("raw_mackerel", 15), ("raw_bass", 28), ("raw_squid", 47), ("raw_eel", 70),
    ("raw_tuna", 105), ("raw_ray", 155), ("raw_angler", 214), ("kelp", 2), ("kelp_seed", 1), ("sea_beet", 5),
    ("sea_beet_seed", 2), ("rice", 12), ("rice_seed", 7), ("samphire", 22), ("samphire_seed", 15), ("pepper", 35),
    ("pepper_seed", 25), ("saffron", 62), ("saffron_seed", 45), ("sea_grape", 95), ("sea_grape_seed", 70), ("charcoal", 6),
    ("brass_bar", 21), ("silver_bar", 73), ("cobalt_bar", 99), ("sea_iron_bar", 149), ("orichalcum_bar", 221), ("abyssal_bar", 305),
    ("brass_sword", 54), ("cobalt_sword", 257), ("sea_iron_sword", 387), ("orichalcum_sword", 574), ("abyssal_sword", 793), ("sea_iron_arrowheads", 15),
    ("abyssal_arrowheads", 31), ("cobalt_platebody", 544), ("orichalcum_platebody", 1_215), ("palm_bow", 25), ("mangrove_bow", 91), ("sea_pine_bow", 217),
    ("salt_cedar_bow", 472), ("ghostwood_bow", 649), ("brass_crossbow", 89), ("sea_iron_crossbow", 643), ("abyssal_crossbow", 1_316), ("sea_iron_arrows", 17),
    ("abyssal_arrows", 33), ("cooked_crab", 7), ("cooked_sardine", 12), ("cooked_mackerel", 29), ("cooked_bass", 52), ("cooked_squid", 86),
    ("cooked_eel", 128), ("cooked_tuna", 191), ("cooked_ray", 281), ("cooked_angler", 387), ("sea_beet_sardine_dish", 24), ("rice_bass_dish", 78),
    ("pepper_eel_dish", 211), ("sea_snake_skin", 15), ("snakeskin", 20), ("snakeskin_body", 96), ("snakeskin_chaps", 64), ("sea_silk", 48),
    ("sea_silk_cloth", 54), ("sea_silk_robe", 324), ("kraken_hide", 400), ("krakenhide_body", 1_600), ("pearl_ring", 206), ("pearl_amulet", 352),
    ("coral_bracelet", 186), ("ring_of_power_1", 300), ("amulet_of_vigour_1", 300), ("ring_of_wealth_1", 300), ("amulet_of_warding_1", 300), ("earth_rune", 5),
    ("mind_rune", 8), ("mist_rune", 10), ("body_rune", 13), ("nature_rune", 16), ("law_rune", 20), ("cork_plank", 18),
    ("cypress_plank", 50), ("ship", 480), ("fish_rack", 0), ("cork_bed", 0), ("ship_in_a_bottle", 0), ("tide_mill", 0),
    ("salt_pan", 0), ("brass_works", 0), ("sawmill_gearbox", 0), ("smelter_bellows", 0), ("trap_nets", 0), ("crab_cakes", 152),
    ("fishermans_pie", 194), ("ink_risotto", 1_088), ("tidecutter", 1_466), ("harpoon_bow", 1_120), ("coral_staff", 1_230), ("sharkskin_vest", 360),
    ("barnacle_plate", 2_524), ("pearl_mantle", 1_396), ("sea_salt", 55), ("pearl", 55), ("coral", 55), ("larch_logs", 2),
    ("spruce_logs", 6), ("alpine_ash_logs", 16), ("birch_logs", 30), ("juniper_logs", 50), ("stone_pine_logs", 74), ("frostwood_logs", 110),
    ("cloudwood_logs", 162), ("skyroot_logs", 222), ("granite", 2), ("flint", 6), ("nickel_ore", 16), ("rock_crystal", 30),
    ("platinum_ore", 50), ("frostiron_ore", 74), ("titanium_ore", 110), ("skysteel_ore", 162), ("aetherium_ore", 222), ("raw_char", 3),
    ("raw_ice_cod", 7), ("raw_glacier_trout", 17), ("raw_grayling", 31), ("raw_pike", 51), ("raw_sturgeon", 75), ("raw_snow_crab", 111),
    ("raw_ice_eel", 163), ("raw_frostfin", 223), ("barley", 2), ("barley_seed", 1), ("hops", 6), ("hops_seed", 3),
    ("buckwheat", 14), ("buckwheat_seed", 9), ("lingonberry", 24), ("lingonberry_seed", 17), ("alpine_herb", 38), ("alpine_herb_seed", 27),
    ("snow_pea", 66), ("snow_pea_seed", 48), ("edelweiss", 100), ("edelweiss_seed", 74), ("nickel_steel_bar", 23), ("platinum_bar", 80),
    ("frostiron_bar", 106), ("titanium_bar", 158), ("skysteel_bar", 232), ("aetherium_bar", 317), ("nickel_steel_sword", 59), ("frostiron_sword", 275),
    ("titanium_sword", 410), ("skysteel_sword", 603), ("aetherium_sword", 824), ("titanium_arrowheads", 16), ("aetherium_arrowheads", 32), ("frostiron_platebody", 583),
    ("skysteel_platebody", 1_276), ("spruce_bow", 28), ("birch_bow", 100), ("stone_pine_bow", 232), ("cloudwood_bow", 496), ("skyroot_bow", 676),
    ("nickel_steel_crossbow", 99), ("titanium_crossbow", 681), ("aetherium_crossbow", 1_369), ("titanium_arrows", 18), ("aetherium_arrows", 34), ("cooked_char", 7),
    ("cooked_ice_cod", 14), ("cooked_glacier_trout", 32), ("cooked_grayling", 57), ("cooked_pike", 93), ("cooked_sturgeon", 137), ("cooked_snow_crab", 201),
    ("cooked_ice_eel", 295), ("cooked_frostfin", 403), ("hops_ice_cod_dish", 28), ("buckwheat_grayling_dish", 88), ("alpine_herb_sturgeon_dish", 228), ("wolf_pelt", 17),
    ("wolf_leather", 22), ("wolf_leather_body", 105), ("wolf_leather_chaps", 70), ("yeti_fur", 52), ("fur_cloth", 58), ("fur_robe", 348),
    ("griffin_hide", 800), ("griffinhide_body", 3_200), ("jade_ring", 280), ("jade_amulet", 440), ("wool_bracelet", 240), ("ring_of_power_2", 1_200),
    ("amulet_of_vigour_2", 1_200), ("ring_of_wealth_2", 1_200), ("amulet_of_warding_2", 1_200), ("cosmic_rune", 5), ("dust_rune", 8), ("blood_rune", 11),
    ("frost_rune", 13), ("storm_rune", 17), ("spirit_rune", 21), ("alpine_ash_plank", 20), ("juniper_plank", 54), ("mule_train", 345),
    ("anvil_stand", 0), ("bookcase", 0), ("tea_set", 0), ("ore_crusher", 0), ("lumber_flume", 0), ("tea_press", 0),
    ("tide_mill_wheel", 0), ("salt_pan_rakes", 0), ("brass_works_crucible", 0), ("hunters_stew", 168), ("barley_bread", 212), ("yeti_broth", 1_138),
    ("frostbrand", 1_542), ("eagle_bow", 1_188), ("storm_staff", 1_288), ("wolfskin_coat", 390), ("titan_plate", 2_628), ("aurora_robe", 1_488),
    ("mountain_tea", 80), ("jade", 80), ("wool", 80), ("charwood_logs", 3), ("ember_pine_logs", 8), ("ashbark_logs", 18),
    ("sunwood_logs", 34), ("cinderoak_logs", 54), ("glasswood_logs", 80), ("thornwood_logs", 117), ("emberheart_logs", 170), ("phoenixwood_logs", 232),
    ("basalt", 3), ("black_sand", 8), ("brimstone_ore", 18), ("obsidian", 34), ("aurum_ore", 54), ("starmetal_ore", 80),
    ("emberite_ore", 117), ("dragonite_ore", 170), ("voidsteel_ore", 232), ("raw_lava_eel", 4), ("raw_cinderfish", 9), ("raw_ash_carp", 19),
    ("raw_sandfish", 35), ("raw_fire_crab", 55), ("raw_magma_ray", 81), ("raw_sunfish", 118), ("raw_salamanderfish", 171), ("raw_phoenix_koi", 233),
    ("fire_pepper", 3), ("fire_pepper_seed", 1), ("sun_melon", 8), ("sun_melon_seed", 4), ("cactus_fruit", 16), ("cactus_fruit_seed", 10),
    ("dates", 27), ("dates_seed", 19), ("ember_bean", 42), ("ember_bean_seed", 30), ("ash_wheat", 70), ("ash_wheat_seed", 51),
    ("sunflower", 106), ("sunflower_seed", 78), ("brimsteel_bar", 26), ("aurum_bar", 86), ("starmetal_bar", 114), ("emberite_bar", 168),
    ("dragonite_bar", 243), ("voidsteel_bar", 331), ("brimsteel_sword", 67), ("starmetal_sword", 296), ("emberite_sword", 436), ("dragonite_sword", 631),
    ("voidsteel_sword", 860), ("emberite_arrowheads", 17), ("voidsteel_arrowheads", 34), ("starmetal_platebody", 627), ("dragonite_platebody", 1_336), ("ember_pine_bow", 34),
    ("sunwood_bow", 112), ("glasswood_bow", 250), ("emberheart_bow", 520), ("phoenixwood_bow", 706), ("brimsteel_crossbow", 112), ("emberite_crossbow", 724),
    ("voidsteel_crossbow", 1_430), ("emberite_arrows", 19), ("voidsteel_arrows", 36), ("cooked_lava_eel", 9), ("cooked_cinderfish", 18), ("cooked_ash_carp", 36),
    ("cooked_sandfish", 65), ("cooked_fire_crab", 101), ("cooked_magma_ray", 147), ("cooked_sunfish", 214), ("cooked_salamanderfish", 309), ("cooked_phoenix_koi", 421),
    ("sun_melon_cinderfish_dish", 37), ("cactus_fruit_sandfish_dish", 100), ("ember_bean_magma_ray_dish", 249), ("salamander_skin", 19), ("salamander_leather", 24), ("salamander_leather_body", 115),
    ("salamander_leather_chaps", 76), ("ember_silk", 56), ("ember_cloth", 62), ("ember_robe", 372), ("drake_hide", 1_200), ("drakehide_body", 4_800),
    ("fire_opal_ring", 352), ("fire_opal_amulet", 524), ("sunstone_bracelet", 292), ("ring_of_power_3", 2_700), ("amulet_of_vigour_3", 2_700), ("ring_of_wealth_3", 2_700),
    ("amulet_of_warding_3", 2_700), ("soul_rune", 6), ("smoke_rune", 9), ("wrath_rune", 11), ("ash_rune", 14), ("sun_rune", 17),
    ("void_rune", 21), ("ashbark_plank", 22), ("cinderoak_plank", 58), ("sandship", 374), ("gold_scales", 0), ("sun_lamp", 0),
    ("star_chart", 0), ("sun_furnace", 0), ("glassworks", 0), ("ember_forge", 0), ("crusher_hammers", 0), ("flume_gates", 0),
    ("press_screws", 0), ("fire_curry", 188), ("sun_feast", 236), ("phoenix_roast", 1_194), ("sunblade", 1_620), ("drakebone_bow", 1_260),
    ("void_staff", 1_308), ("drakescale_coat", 422), ("voidsteel_plate", 2_748), ("ember_silk_robe", 1_520), ("fire_oil", 105), ("fire_opal", 105),
    ("sunstone", 105), ("sapphire", 60), ("emerald", 120), ("ruby", 220), ("diamond", 400), ("fine_ashes", 4),
    // </gen:values>
];

pub fn value(item: &str) -> u64 {
    static INDEX: OnceLock<BTreeMap<&'static str, u64>> = OnceLock::new();
    INDEX
        .get_or_init(|| VALUES.iter().copied().collect())
        .get(item)
        .copied()
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

/// Prestige (P9): +25% skill XP per prestige; every prestige needs the same
/// total level (T7, IG-Q19 B), 900 in the full game so a run is about four
/// times as long (T15, IG-Q36 A and IG-Q37 A).
pub const PRESTIGE_BASE: u32 = 900;
pub const PRESTIGE_STEP_BP: u64 = 2_500;

/// A plan holds this many steps (P3, four since T9).
pub const PLAN_STEPS: usize = 4;
/// In advanced mode (T18, IG-Q53 A).
pub const PLAN_STEPS_ADVANCED: usize = 12;
/// The amounts an "until" or an "if" can test, for items and gold.
pub const PLAN_AMOUNTS: [u64; 7] = [10, 50, 100, 500, 1_000, 5_000, 10_000];
/// How many times a "repeat" can go back; 0 is forever.
pub const PLAN_TIMES: [u64; 5] = [0, 2, 3, 5, 10];
/// Step lengths offered on the plan screen; 0 means until the inputs run out
/// (or, for gathering, until you come back).
pub const PLAN_COUNTS: [u64; 4] = [0, 100, 500, 1_000];

// ====================================================================
// The four regions (T15, IG-Q38 A). Each opens at a total level and adds a
// band of every skill, its enemies, champions, recipes, crops and towns, a
// dungeon, and a tier of every engine. They close again at prestige, when
// the total level starts over (IG-Q44 A).
// ====================================================================

#[rustfmt::skip]
pub const REGIONS: [(&str, u32); 4] = [("vale", 0), ("coast", 300), ("peaks", 500), ("wastes", 700)];
/// Each region opened past the first also opens one more farming patch.
pub const PATCH_PER_REGION: usize = 1;

// ---------- each skill's own new thing, opened with the Coast (Q40 A) ----------

/// Woodcutting: one tree each game hour is rare and gives twice the logs.
pub const RARE_TREE_MS: u64 = HOUR;
/// Mining: one rock in this many also gives a gem, by the rock's level.
pub const GEM_ONE_IN: u64 = 40;
#[rustfmt::skip]
pub const GEMS: [(&str, u32); 4] = [("sapphire", 0), ("emerald", 30), ("ruby", 55), ("diamond", 80)];
/// Fishing: one catch in this many also brings up treasure.
pub const TREASURE_ONE_IN: u64 = 80;
/// Farming: fine ashes, from logs of level `FINE_ASH_LEVEL` and up, make a
/// crop grow in this many percent of its time.
pub const FINE_ASH_LEVEL: u32 = 45;
pub const FINE_COMPOST_GROW_PCT: u64 = 60;
/// Smithing: one item in this many is a masterwork, sold at once for this many times its value.
pub const MASTERWORK_ONE_IN: u64 = 20;
pub const MASTERWORK_TIMES: u64 = 2;
/// Cooking: one dish in this many is perfect and makes one more.
pub const PERFECT_ONE_IN: u64 = 10;

/// Enchanted jewellery (Crafting, Q40 A): the best of each kind you hold
/// counts. Power: percent more max hit. Vigour: percent more healing.
/// Wealth: percent more gold from kills. Warding: more hitpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trinket {
    Power,
    Vigour,
    Wealth,
    Warding,
}

#[rustfmt::skip]
pub const TRINKETS: &[(&str, Trinket, u64)] = &[
    // <gen:trinkets>
    ("ring_of_power_1", Trinket::Power, 10),
    ("amulet_of_vigour_1", Trinket::Vigour, 10),
    ("ring_of_wealth_1", Trinket::Wealth, 10),
    ("amulet_of_warding_1", Trinket::Warding, 50),
    ("ring_of_power_2", Trinket::Power, 20),
    ("amulet_of_vigour_2", Trinket::Vigour, 20),
    ("ring_of_wealth_2", Trinket::Wealth, 20),
    ("amulet_of_warding_2", Trinket::Warding, 100),
    ("ring_of_power_3", Trinket::Power, 30),
    ("amulet_of_vigour_3", Trinket::Vigour, 30),
    ("ring_of_wealth_3", Trinket::Wealth, 30),
    ("amulet_of_warding_3", Trinket::Warding, 150),
    // </gen:trinkets>
];

// ====================================================================
// The engines (T10 to T14). Each has a ladder of ranks, a timer that runs
// alongside your one action, and something to collect when you come back.
// ====================================================================

pub const HOUR: u64 = 3_600_000;
pub const MINUTE: u64 = 60_000;

/// Which engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Engine {
    Trader,
    Nobility,
    Inventor,
    Cosmic,
}

/// The total level that opens each engine, one at a time and all before the
/// first prestige (R6.2). Construction opens with the Trader, because carts
/// need it (R7.2).
pub const ENGINE_AT: [(Engine, u32); 4] = [
    (Engine::Trader, 150),
    (Engine::Nobility, 250),
    (Engine::Inventor, 350),
    (Engine::Cosmic, 450),
];

/// The engine a skill belongs to; the old skills belong to none.
pub fn engine_of_skill(skill: usize) -> Option<Engine> {
    match skill {
        CONSTRUCTION | TRADING => Some(Engine::Trader),
        INVENTION => Some(Engine::Inventor),
        ATTUNEMENT => Some(Engine::Cosmic),
        _ => None,
    }
}

// ---------- the bard: Performance and Fame (T18, IG-Q56 A, IG-Q57 A) ----------

/// Fame opens at this total level, before the Trader.
pub const FAME_AT: u32 = 50;

/// The instrument each song needs held; it is not spent.
#[rustfmt::skip]
pub const SONG_INSTRUMENTS: &[(&str, &str)] = &[
    ("jig", "wooden_flute"),
    ("ballad", "wooden_flute"),
    ("lay_of_the_vale", "lute"),
    ("drinking_song", "iron_horn"),
    ("war_march", "iron_horn"),
    ("harvest_hymn", "lute"),
    ("court_air", "golden_harp"),
    ("song_of_the_hunt", "golden_harp"),
    ("legend_of_the_vale", "golden_harp"),
    ("sea_shanty", "wooden_flute"),
    ("tide_song", "wooden_flute"),
    ("gull_reel", "lute"),
    ("shanty_of_the_deep", "conch_horn"),
    ("mermaids_air", "lute"),
    ("storm_chorus", "conch_horn"),
    ("harbour_hymn", "golden_harp"),
    ("kraken_saga", "conch_horn"),
    ("song_of_the_coast", "golden_harp"),
    ("yodel", "wooden_flute"),
    ("shepherds_pipe", "wooden_flute"),
    ("snowfall_ballad", "lute"),
    ("echo_song", "iron_horn"),
    ("alpine_call", "alpine_horn"),
    ("avalanche_march", "alpine_horn"),
    ("frost_hymn", "golden_harp"),
    ("griffin_saga", "alpine_horn"),
    ("song_of_the_peaks", "golden_harp"),
    ("sand_chant", "wooden_flute"),
    ("caravan_song", "lute"),
    ("dune_ballad", "lute"),
    ("fire_dance", "iron_horn"),
    ("mirage_air", "golden_harp"),
    ("drum_of_embers", "war_drum"),
    ("sun_hymn", "golden_harp"),
    ("wyrm_saga", "war_drum"),
    ("song_of_the_wastes", "golden_harp"),
];

/// Songs that strengthen other work for an hour of game time after each
/// is played (IG-Q57 A): fights, crops planted and gathering go 10% faster.
pub const SONG_BUFFS: &[(&str, &str)] = &[
    ("war_march", "fight"),
    ("harvest_hymn", "farm"),
    ("song_of_the_hunt", "gather"),
];
pub const BUFF_MS: u64 = 3_600_000;
pub const BUFF_PCT: u64 = 10;

/// A rank on a Fame ladder: the Performance level, the renown spent and the gold.
pub struct FameRank {
    pub id: &'static str,
    pub level: u32,
    pub renown: u64,
    pub gold: u64,
}

const fn fame(id: &'static str, level: u32, renown: u64, gold: u64) -> FameRank {
    FameRank {
        id,
        level,
        renown,
        gold,
    }
}

/// One ladder for each region; the first rank of each is where you start.
#[rustfmt::skip]
pub const FAME_RANKS: [[FameRank; 5]; 4] = [
    [fame("busker", 1, 0, 0), fame("tavern_player", 10, 50, 0), fame("town_favourite", 25, 300, 5_000), fame("court_bard", 45, 1_500, 50_000), fame("legend_of_the_vale", 70, 6_000, 500_000)],
    [fame("dockside_singer", 3, 0, 0), fame("shanty_master", 15, 200, 10_000), fame("harbour_favourite", 35, 1_000, 100_000), fame("admirals_bard", 55, 4_000, 1_000_000), fame("legend_of_the_coast", 80, 15_000, 5_000_000)],
    [fame("hill_piper", 5, 0, 0), fame("lodge_singer", 18, 400, 25_000), fame("valley_favourite", 38, 2_000, 250_000), fame("mountain_kings_bard", 58, 8_000, 2_500_000), fame("legend_of_the_peaks", 85, 30_000, 12_000_000)],
    [fame("camp_drummer", 7, 0, 0), fame("caravan_singer", 20, 800, 60_000), fame("oasis_favourite", 40, 4_000, 600_000), fame("sultans_bard", 60, 16_000, 6_000_000), fame("legend_of_the_wastes", 90, 60_000, 25_000_000)],
];

/// Towns pay this many percent more for orders, and the estate this many
/// percent more income, for each Fame rank above the first in every open region.
pub const FAME_PCT: u64 = 5;

/// A venue for a booked show: the Fame rank it needs in its region, how
/// long the show runs, what the crowd eats, and what it pays at the end.
pub struct Venue {
    pub id: &'static str,
    pub region: usize,
    pub rank: usize,
    pub ms: u64,
    pub needs: &'static [(&'static str, u64)],
    pub gold: u64,
    pub renown: u64,
}

#[rustfmt::skip]
pub const VENUES: &[Venue] = &[
    Venue { id: "tavern", region: 0, rank: 0, ms: 1_800_000, needs: &[("shrimp", 10)], gold: 300, renown: 20 },
    Venue { id: "market_square", region: 0, rank: 1, ms: 3_600_000, needs: &[("trout", 20)], gold: 1_500, renown: 60 },
    Venue { id: "great_hall", region: 0, rank: 2, ms: 7_200_000, needs: &[("salmon", 20)], gold: 8_000, renown: 200 },
    Venue { id: "royal_court", region: 0, rank: 3, ms: 14_400_000, needs: &[("shark", 10)], gold: 40_000, renown: 600 },
    Venue { id: "harbour_inn", region: 1, rank: 1, ms: 7_200_000, needs: &[("rice", 20)], gold: 20_000, renown: 300 },
    Venue { id: "mountain_lodge", region: 2, rank: 1, ms: 10_800_000, needs: &[("hops", 20)], gold: 60_000, renown: 700 },
    Venue { id: "oasis_court", region: 3, rank: 1, ms: 14_400_000, needs: &[("dates", 20)], gold: 150_000, renown: 1_500 },
];

pub fn venue_index(id: &str) -> Option<usize> {
    VENUES.iter().position(|v| v.id == id)
}

/// Followers (IG-Q54 A): a fan joins at each of these Vale Fame ranks, three
/// at most ever, kept through prestige. Each gathers at a quarter of your
/// speed at its level 1, rising to your full speed at `FOLLOWER_FULL`.
pub const FOLLOWER_RANKS: [usize; 3] = [1, 2, 3];
pub const FOLLOWER_START_PCT: u64 = 25;
pub const FOLLOWER_FULL: u32 = 50;
/// Performance XP a show gives at its end, for each renown it pays.
pub const SHOW_XP_PER_RENOWN: u64 = 2;

// ---------- recipes (T10) ----------

/// A recipe: found once from an enemy, it unlocks the action of the same id.
pub struct Recipe {
    pub id: &'static str,
    pub enemy: &'static str,
    pub one_in: u64,
    pub region: usize,
}

/// One in 64 to one in 512 by the enemy's level (IG-Q27 A).
#[rustfmt::skip]
pub const RECIPES: &[Recipe] = &[
    Recipe { id: "farmers_stew", enemy: "chicken", one_in: 64, region: 0 },
    Recipe { id: "studded_leather", enemy: "cow", one_in: 64, region: 0 },
    Recipe { id: "goblin_hot_pot", enemy: "goblin", one_in: 128, region: 0 },
    Recipe { id: "silkweave_vest", enemy: "spider", one_in: 128, region: 0 },
    Recipe { id: "giants_maul", enemy: "hill_giant", one_in: 256, region: 0 },
    Recipe { id: "knights_longbow", enemy: "black_knight", one_in: 256, region: 0 },
    Recipe { id: "mossplate", enemy: "moss_giant", one_in: 384, region: 0 },
    Recipe { id: "dragonfire_staff", enemy: "green_dragon", one_in: 512, region: 0 },
    Recipe { id: "dragon_pepper_feast", enemy: "green_dragon", one_in: 512, region: 0 },
    // <gen:recipes>
    Recipe { id: "crab_cakes", enemy: "shore_crab", one_in: 64, region: 1 },
    Recipe { id: "fishermans_pie", enemy: "gull", one_in: 64, region: 1 },
    Recipe { id: "ink_risotto", enemy: "smuggler", one_in: 128, region: 1 },
    Recipe { id: "tidecutter", enemy: "sea_snake", one_in: 128, region: 1 },
    Recipe { id: "harpoon_bow", enemy: "pirate", one_in: 256, region: 1 },
    Recipe { id: "coral_staff", enemy: "drowned_knight", one_in: 256, region: 1 },
    Recipe { id: "sharkskin_vest", enemy: "reef_troll", one_in: 384, region: 1 },
    Recipe { id: "barnacle_plate", enemy: "sea_serpent", one_in: 512, region: 1 },
    Recipe { id: "pearl_mantle", enemy: "sea_serpent", one_in: 512, region: 1 },
    Recipe { id: "hunters_stew", enemy: "grey_wolf", one_in: 64, region: 2 },
    Recipe { id: "barley_bread", enemy: "mountain_goat", one_in: 64, region: 2 },
    Recipe { id: "yeti_broth", enemy: "highwayman", one_in: 128, region: 2 },
    Recipe { id: "frostbrand", enemy: "yeti", one_in: 128, region: 2 },
    Recipe { id: "eagle_bow", enemy: "harpy", one_in: 256, region: 2 },
    Recipe { id: "storm_staff", enemy: "stone_golem", one_in: 256, region: 2 },
    Recipe { id: "wolfskin_coat", enemy: "frost_troll", one_in: 384, region: 2 },
    Recipe { id: "titan_plate", enemy: "griffin", one_in: 512, region: 2 },
    Recipe { id: "aurora_robe", enemy: "griffin", one_in: 512, region: 2 },
    Recipe { id: "fire_curry", enemy: "fire_imp", one_in: 64, region: 3 },
    Recipe { id: "sun_feast", enemy: "salamander", one_in: 64, region: 3 },
    Recipe { id: "phoenix_roast", enemy: "scorpion", one_in: 128, region: 3 },
    Recipe { id: "sunblade", enemy: "ash_wraith", one_in: 128, region: 3 },
    Recipe { id: "drakebone_bow", enemy: "dune_raider", one_in: 256, region: 3 },
    Recipe { id: "void_staff", enemy: "lava_golem", one_in: 256, region: 3 },
    Recipe { id: "drakescale_coat", enemy: "fire_elemental", one_in: 384, region: 3 },
    Recipe { id: "voidsteel_plate", enemy: "fire_drake", one_in: 512, region: 3 },
    Recipe { id: "ember_silk_robe", enemy: "fire_drake", one_in: 512, region: 3 },
    // </gen:recipes>
];

pub fn recipe_index(id: &str) -> Option<usize> {
    static INDEX: OnceLock<BTreeMap<&'static str, usize>> = OnceLock::new();
    INDEX
        .get_or_init(|| index_of(RECIPES.iter().map(|r| r.id)))
        .get(id)
        .copied()
}

/// A second copy of a recipe you know sells for this much gold for each
/// "one in" of its rarity.
pub const RECIPE_COPY_GOLD_PER_ONE_IN: u64 = 10;
/// The Goblin hot pot: kills give this many percent more gold for ten minutes.
pub const HOT_POT_GOLD_PCT: u64 = 10;
pub const HOT_POT_MS: u64 = 10 * MINUTE;
/// Spices from the Trader: food eaten with one heals this many percent more.
pub const SPICE_HEAL_PCT: u64 = 25;

// ---------- the Trader (T12) ----------

pub struct Town {
    pub id: &'static str,
    /// The Trading level that opens it.
    pub level: u32,
    /// What its orders ask for and its caravans buy.
    pub wants: &'static [&'static str],
    /// What a caravan brings back: one for every `back_per` items carried.
    pub sells: &'static str,
    pub back_per: u64,
    pub trip_ms: u64,
    /// Percent chance that bandits meet a caravan, before your guards and combat level.
    pub danger: u64,
    pub region: usize,
    /// What carries the goods: carts in the Vale, a ship to the Coast (T15).
    pub vehicle: &'static str,
}

#[rustfmt::skip]
pub const TOWNS: &[Town] = &[
    Town { id: "riverton", level: 1,
        wants: &["logs", "oak_logs", "willow_logs", "raw_shrimp", "raw_trout", "raw_salmon", "maple_logs", "raw_lobster"],
        sells: "spices", back_per: 10, trip_ms: HOUR, danger: 10, region: 0, vehicle: "cart" },
    Town { id: "saltmarsh", level: 15,
        wants: &["bronze_bar", "iron_bar", "steel_bar", "leather_body", "hard_leather_body", "gold_bar", "studded_leather"],
        sells: "dye", back_per: 10, trip_ms: 2 * HOUR, danger: 20, region: 0, vehicle: "cart" },
    Town { id: "highcliff", level: 30,
        wants: &["trout", "salmon", "lobster", "plank", "oak_plank", "baked_potato", "farmers_stew", "swordfish"],
        sells: "glass", back_per: 8, trip_ms: 3 * HOUR, danger: 30, region: 0, vehicle: "cart" },
    Town { id: "emberfall", level: 50,
        wants: &["gold_amulet", "silk_robe", "royal_robe", "dragonhide_body", "magic_shortbow", "mithril_sword", "yew_shortbow"],
        sells: "fine_parts", back_per: 4, trip_ms: 4 * HOUR, danger: 40, region: 0, vehicle: "cart" },
    // <gen:towns>
    Town { id: "brinewick", level: 10, wants: &["palm_logs", "cypress_logs", "raw_mackerel", "cobalt_ore", "snakeskin_body", "cooked_bass"], sells: "sea_salt", back_per: 6, trip_ms: 3_600_000, danger: 20, region: 1, vehicle: "ship" },
    Town { id: "gullport", level: 25, wants: &["brass_bar", "snakeskin_body", "cooked_bass", "cork_plank", "pearl_ring", "sea_silk_robe"], sells: "pearl", back_per: 5, trip_ms: 7_200_000, danger: 30, region: 1, vehicle: "ship" },
    Town { id: "coral_bay", level: 45, wants: &["sea_iron_bar", "pearl_ring", "sea_silk_robe", "cooked_tuna", "samphire", "salt_cedar_bow"], sells: "coral", back_per: 4, trip_ms: 10_800_000, danger: 40, region: 1, vehicle: "ship" },
    Town { id: "tidewatch", level: 65, wants: &["abyssal_bar", "samphire", "salt_cedar_bow", "pearl_amulet", "cypress_logs", "raw_mackerel"], sells: "glass", back_per: 3, trip_ms: 14_400_000, danger: 50, region: 1, vehicle: "ship" },
    Town { id: "frostford", level: 20, wants: &["spruce_logs", "juniper_logs", "raw_glacier_trout", "frostiron_ore", "wolf_leather_body", "cooked_grayling"], sells: "mountain_tea", back_per: 6, trip_ms: 3_600_000, danger: 25, region: 2, vehicle: "mule_train" },
    Town { id: "eaglecrest", level: 35, wants: &["nickel_steel_bar", "wolf_leather_body", "cooked_grayling", "alpine_ash_plank", "jade_ring", "fur_robe"], sells: "jade", back_per: 5, trip_ms: 7_200_000, danger: 35, region: 2, vehicle: "mule_train" },
    Town { id: "stonegate", level: 55, wants: &["titanium_bar", "jade_ring", "fur_robe", "cooked_snow_crab", "lingonberry", "cloudwood_bow"], sells: "wool", back_per: 4, trip_ms: 10_800_000, danger: 45, region: 2, vehicle: "mule_train" },
    Town { id: "cloudhold", level: 75, wants: &["aetherium_bar", "lingonberry", "cloudwood_bow", "jade_amulet", "juniper_logs", "raw_glacier_trout"], sells: "spices", back_per: 3, trip_ms: 14_400_000, danger: 55, region: 2, vehicle: "mule_train" },
    Town { id: "cinderhold", level: 30, wants: &["ember_pine_logs", "cinderoak_logs", "raw_ash_carp", "starmetal_ore", "salamander_leather_body", "cooked_sandfish"], sells: "fire_oil", back_per: 6, trip_ms: 3_600_000, danger: 30, region: 3, vehicle: "sandship" },
    Town { id: "sunreach", level: 50, wants: &["brimsteel_bar", "salamander_leather_body", "cooked_sandfish", "ashbark_plank", "fire_opal_ring", "ember_robe"], sells: "fire_opal", back_per: 5, trip_ms: 7_200_000, danger: 40, region: 3, vehicle: "sandship" },
    Town { id: "ashport", level: 70, wants: &["emberite_bar", "fire_opal_ring", "ember_robe", "cooked_sunfish", "dates", "emberheart_bow"], sells: "sunstone", back_per: 4, trip_ms: 10_800_000, danger: 50, region: 3, vehicle: "sandship" },
    Town { id: "duneholm", level: 90, wants: &["voidsteel_bar", "dates", "emberheart_bow", "fire_opal_amulet", "cinderoak_logs", "raw_ash_carp"], sells: "fine_parts", back_per: 3, trip_ms: 14_400_000, danger: 60, region: 3, vehicle: "sandship" },
    // </gen:towns>
];
/// Emberfall also needs an Earl's charter (Nobility rank 5).
pub const EMBERFALL: usize = 3;
pub const CHARTER_RANK: usize = 5;

pub fn town_index(id: &str) -> Option<usize> {
    TOWNS.iter().position(|t| t.id == id)
}

/// Each town posts this many orders, new every `ORDER_MS` of game time.
pub const ORDER_SLOTS: u64 = 3;
pub const ORDER_MS: u64 = 6 * HOUR;
/// An order asks for 10 to 50 of one thing and pays this many percent of its value.
pub const ORDER_PAY_PCT: u64 = 160;
/// A caravan carries up to this many of one item.
pub const CART_LOAD: u64 = 100;
/// Prices move in a twelve-hour cycle you can see ahead (IG-Q30 A): from this
/// many percent at the bottom, up by `PRICE_STEP_PCT` an hour for six hours.
pub const PRICE_LOW_PCT: u64 = 80;
pub const PRICE_STEP_PCT: u64 = 8;
pub const PRICE_CYCLE_HOURS: u64 = 12;
/// Guards cost this much gold a trip for each town along the road, and take
/// this many points off the bandits' chance.
pub const GUARD_GOLD: u64 = 200;
pub const GUARD_PCT: u64 = 25;
/// When bandits win, the caravan earns this many percent (IG-Q31 A): nothing is lost.
pub const BANDIT_PAY_PCT: u64 = 50;

/// Trader ranks: (Trading level, orders filled, gold fee, orders in every town, needs the charter).
pub struct TraderRank {
    pub id: &'static str,
    pub level: u32,
    pub orders: u64,
    pub fee: u64,
    pub each_town: u64,
    pub charter: bool,
}

#[rustfmt::skip]
/// One ladder for each region: the Vale's, then a tier for each later region (T15, IG-Q43 A).
pub const TRADER_RANKS: [[TraderRank; 5]; 4] = [
    [
        TraderRank { id: "peddler", level: 1, orders: 0, fee: 0, each_town: 0, charter: false },
        TraderRank { id: "merchant", level: 20, orders: 10, fee: 0, each_town: 0, charter: false },
        TraderRank { id: "guild_member", level: 40, orders: 0, fee: 100_000, each_town: 0, charter: false },
        TraderRank { id: "master_trader", level: 60, orders: 0, fee: 0, each_town: 5, charter: false },
        TraderRank { id: "merchant_prince", level: 80, orders: 0, fee: 0, each_town: 0, charter: true },
    ],
    // <gen:trader_ranks>
    [TraderRank { id: "deckhand", level: 5, orders: 0, fee: 0, each_town: 0, charter: false }, TraderRank { id: "shipper", level: 25, orders: 10, fee: 0, each_town: 0, charter: false }, TraderRank { id: "sea_merchant", level: 45, orders: 0, fee: 250_000, each_town: 0, charter: false }, TraderRank { id: "harbour_guild_member", level: 65, orders: 0, fee: 0, each_town: 5, charter: false }, TraderRank { id: "sea_prince", level: 85, orders: 0, fee: 0, each_town: 0, charter: true },],
    [TraderRank { id: "mule_driver", level: 8, orders: 0, fee: 0, each_town: 0, charter: false }, TraderRank { id: "pass_trader", level: 28, orders: 20, fee: 0, each_town: 0, charter: false }, TraderRank { id: "mountain_merchant", level: 48, orders: 0, fee: 1_000_000, each_town: 0, charter: false }, TraderRank { id: "peak_guild_member", level: 68, orders: 0, fee: 0, each_town: 5, charter: false }, TraderRank { id: "prince_of_passes", level: 88, orders: 0, fee: 0, each_town: 0, charter: true },],
    [TraderRank { id: "sand_trader", level: 11, orders: 0, fee: 0, each_town: 0, charter: false }, TraderRank { id: "dune_merchant", level: 31, orders: 30, fee: 0, each_town: 0, charter: false }, TraderRank { id: "ember_guild_member", level: 51, orders: 0, fee: 2_250_000, each_town: 0, charter: false }, TraderRank { id: "sun_merchant", level: 71, orders: 0, fee: 0, each_town: 5, charter: false }, TraderRank { id: "emperor_of_trade", level: 91, orders: 0, fee: 0, each_town: 0, charter: true },],
    // </gen:trader_ranks>
];
/// Each Trader rank above Peddler pays this many percent more on caravans.
pub const TRADER_RANK_PCT: u64 = 5;

// ---------- Nobility (T11) ----------

pub struct NobleRank {
    pub id: &'static str,
    pub gold: u64,
    /// Room levels built across the estate.
    pub rooms: u64,
    /// Gold the estate pays each hour of game time at this rank.
    pub income: u64,
    pub servants: u64,
}

#[rustfmt::skip]
/// One ladder for each region: the Vale's, then a tier for each later region (T15, IG-Q43 A).
pub const NOBLE_RANKS: [[NobleRank; 7]; 4] = [
    [
        NobleRank { id: "commoner", gold: 0, rooms: 0, income: 0, servants: 0 },
        NobleRank { id: "freeholder", gold: 5_000, rooms: 1, income: 60, servants: 0 },
        NobleRank { id: "squire", gold: 20_000, rooms: 2, income: 150, servants: 1 },
        NobleRank { id: "knight", gold: 50_000, rooms: 4, income: 400, servants: 1 },
        NobleRank { id: "baron", gold: 200_000, rooms: 6, income: 1_000, servants: 2 },
        NobleRank { id: "earl", gold: 1_000_000, rooms: 8, income: 2_500, servants: 2 },
        NobleRank { id: "duke", gold: 5_000_000, rooms: 10, income: 6_000, servants: 3 },
    ],
    // <gen:noble_ranks>
    [NobleRank { id: "landholder", gold: 0, rooms: 0, income: 0, servants: 0 }, NobleRank { id: "harbour_master", gold: 50_000, rooms: 16, income: 300, servants: 1 }, NobleRank { id: "sea_knight", gold: 150_000, rooms: 20, income: 700, servants: 1 }, NobleRank { id: "commodore", gold: 400_000, rooms: 24, income: 1_500, servants: 2 }, NobleRank { id: "sea_baron", gold: 1_000_000, rooms: 28, income: 3_000, servants: 2 }, NobleRank { id: "admiral", gold: 3_000_000, rooms: 32, income: 6_000, servants: 3 }, NobleRank { id: "lord_of_the_coast", gold: 10_000_000, rooms: 36, income: 12_000, servants: 3 },],
    [NobleRank { id: "highlander", gold: 0, rooms: 0, income: 0, servants: 0 }, NobleRank { id: "thane", gold: 200_000, rooms: 32, income: 600, servants: 1 }, NobleRank { id: "mountain_knight", gold: 600_000, rooms: 40, income: 1_400, servants: 1 }, NobleRank { id: "warden", gold: 1_600_000, rooms: 48, income: 3_000, servants: 2 }, NobleRank { id: "jarl", gold: 4_000_000, rooms: 56, income: 6_000, servants: 2 }, NobleRank { id: "high_lord", gold: 12_000_000, rooms: 64, income: 12_000, servants: 3 }, NobleRank { id: "lord_of_the_peaks", gold: 40_000_000, rooms: 72, income: 24_000, servants: 3 },],
    [NobleRank { id: "settler", gold: 0, rooms: 0, income: 0, servants: 0 }, NobleRank { id: "ember_squire", gold: 450_000, rooms: 48, income: 900, servants: 1 }, NobleRank { id: "dune_knight", gold: 1_350_000, rooms: 60, income: 2_100, servants: 1 }, NobleRank { id: "sun_baron", gold: 3_600_000, rooms: 72, income: 4_500, servants: 2 }, NobleRank { id: "ash_count", gold: 9_000_000, rooms: 84, income: 9_000, servants: 2 }, NobleRank { id: "flame_duke", gold: 27_000_000, rooms: 96, income: 18_000, servants: 3 }, NobleRank { id: "sovereign_of_the_wastes", gold: 90_000_000, rooms: 108, income: 36_000, servants: 3 },],
    // </gen:noble_ranks>
];
/// Ranks from Squire on need a deed: a champion (`CHAMPIONS`) or a tribute,
/// your choice each rank (IG-Q29 C). `TRIBUTES[k]` goes with `CHAMPIONS[k]`.
pub const DEED_FROM: usize = 2;
#[rustfmt::skip]
pub const TRIBUTES: [[&[(&str, u64)]; 5]; 4] = [
    [
        &[("gold_ring", 10), ("leather_body", 10)],
        &[("hard_leather_body", 10), ("salmon", 100)],
        &[("royal_robe", 5), ("lobster", 200)],
        &[("royal_robe", 10), ("gold_amulet", 20)],
        &[("royal_robe", 20), ("dragonhide_body", 5), ("shark", 100)],
    ],
    // <gen:tributes>
    [&[("sea_salt", 50), ("pearl_ring", 5)], &[("sea_iron_bar", 40), ("sea_silk_robe", 4)], &[("sea_salt", 150), ("pearl_ring", 15)], &[("abyssal_bar", 80), ("sea_silk_robe", 8)], &[("sea_salt", 250), ("pearl_ring", 25)]],
    [&[("mountain_tea", 50), ("jade_ring", 5)], &[("titanium_bar", 40), ("fur_robe", 4)], &[("mountain_tea", 150), ("jade_ring", 15)], &[("aetherium_bar", 80), ("fur_robe", 8)], &[("mountain_tea", 250), ("jade_ring", 25)]],
    [&[("fire_oil", 50), ("fire_opal_ring", 5)], &[("emberite_bar", 40), ("ember_robe", 4)], &[("fire_oil", 150), ("fire_opal_ring", 15)], &[("voidsteel_bar", 80), ("ember_robe", 8)], &[("fire_oil", 250), ("fire_opal_ring", 25)]],
    // </gen:tributes>
];

/// What a room does for each of its levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomEffect {
    /// A skill this many percent faster a level (the kitchen: 10% Cooking).
    Faster(usize),
    Patch,
    Servant,
    Caravan,
    Workshop,
    Observatory,
    /// Research this many percent faster a level.
    Research,
    /// Estate income this many percent more a level.
    Income,
}

pub struct Room {
    pub id: &'static str,
    /// The Construction level for the first level of the room; each level after needs eight more.
    pub level: u32,
    /// The top level while only the Vale is open; each later region raises
    /// every room's top level (T15), to `ROOM_MAX`.
    pub max: u64,
    pub region: usize,
    pub effect: RoomEffect,
}

#[rustfmt::skip]
pub const ROOMS: &[Room] = &[
    Room { id: "kitchen", level: 1, max: 5, region: 0, effect: RoomEffect::Faster(COOKING) },
    Room { id: "garden", level: 10, max: 2, region: 0, effect: RoomEffect::Patch },
    Room { id: "servants_quarters", level: 20, max: 3, region: 0, effect: RoomEffect::Servant },
    Room { id: "stables", level: 5, max: 3, region: 0, effect: RoomEffect::Caravan },
    Room { id: "workshop", level: 30, max: 5, region: 0, effect: RoomEffect::Workshop },
    Room { id: "observatory", level: 40, max: 5, region: 0, effect: RoomEffect::Observatory },
    // <gen:rooms>
    Room { id: "harbour", level: 30, max: 5, region: 1, effect: RoomEffect::Caravan },
    Room { id: "smokehouse", level: 25, max: 5, region: 1, effect: RoomEffect::Faster(FISHING) },
    Room { id: "forge", level: 45, max: 5, region: 2, effect: RoomEffect::Faster(SMITHING) },
    Room { id: "library", level: 55, max: 5, region: 2, effect: RoomEffect::Research },
    Room { id: "treasury", level: 60, max: 5, region: 3, effect: RoomEffect::Income },
    Room { id: "sun_garden", level: 50, max: 5, region: 3, effect: RoomEffect::Patch },
    // </gen:rooms>
];
/// Each region opened past the first adds this many levels to every room's top, up to `ROOM_MAX`.
pub const ROOM_MAX_PER_REGION: u64 = 2;
pub const ROOM_MAX: u64 = 10;
/// Percent a level for each kind of room.
pub const FASTER_PCT: u64 = 5;
pub const RESEARCH_PCT: u64 = 10;
pub const INCOME_PCT: u64 = 10;
pub const KITCHEN: usize = 0;
pub const GARDEN: usize = 1;
pub const QUARTERS: usize = 2;
pub const STABLES: usize = 3;
pub const WORKSHOP: usize = 4;
pub const OBSERVATORY: usize = 5;
/// The plank each room level is built from (levels 1 to 10).
#[rustfmt::skip]
pub const ROOM_PLANKS: [&str; 10] = [
    "plank", "plank", "oak_plank", "teak_plank", "mahogany_plank",
    // <gen:room_planks>
    "cork_plank", "cypress_plank", "alpine_ash_plank", "juniper_plank", "cinderoak_plank",
    // </gen:room_planks>
];
/// Furniture (T15, Q40 A): each piece counts as one more level of its room's effect.
#[rustfmt::skip]
pub const FURNITURE: &[(&str, &str)] = &[
    // <gen:furniture>
    ("fish_rack", "smokehouse"),
    ("cork_bed", "servants_quarters"),
    ("ship_in_a_bottle", "harbour"),
    ("anvil_stand", "forge"),
    ("bookcase", "library"),
    ("tea_set", "kitchen"),
    ("gold_scales", "treasury"),
    ("sun_lamp", "sun_garden"),
    ("star_chart", "observatory"),
    // </gen:furniture>
];

/// A room level L takes 10·L planks, 20·L nails, 10·L limestone and 500·L² gold,
/// and gives 150·L² Construction XP.
pub const ROOM_PLANKS_PER: u64 = 10;
pub const ROOM_NAILS_PER: u64 = 20;
pub const ROOM_STONE_PER: u64 = 10;
pub const ROOM_GOLD_PER: u64 = 500;
pub const ROOM_XP_PER: u64 = 150;
/// The kitchen: Cooking this many percent faster a level.
pub const KITCHEN_PCT: u64 = 10;

/// A servant runs one of your actions at half speed, alongside your own
/// (IG-Q28 A). Each eats a food and takes a wage every hour of game time.
pub const SERVANT_SLOW: u64 = 2;
pub const SERVANT_WAGE: u64 = 25;

// ---------- the Inventor (T14) ----------

/// A blueprint: researched with parts and gold over game time, it unlocks
/// the build action of the same name.
pub struct Blueprint {
    pub id: &'static str,
    pub level: u32,
    pub needs: &'static [(&'static str, u64)],
    pub gold: u64,
    pub ms: u64,
    pub xp: u64,
    pub build: &'static str,
    pub region: usize,
}

#[rustfmt::skip]
pub const BLUEPRINTS: &[Blueprint] = &[
    Blueprint { id: "axe_gadget", level: 10, needs: &[("wood_parts", 20)], gold: 2_000, ms: 20 * MINUTE, xp: 200, build: "build_axe_gadget", region: 0 },
    Blueprint { id: "sawmill", level: 20, needs: &[("wood_parts", 40), ("metal_parts", 20)], gold: 8_000, ms: HOUR, xp: 500, build: "build_sawmill", region: 0 },
    Blueprint { id: "pick_gadget", level: 25, needs: &[("metal_parts", 40)], gold: 10_000, ms: HOUR, xp: 600, build: "build_pick_gadget", region: 0 },
    Blueprint { id: "rod_gadget", level: 35, needs: &[("cloth_parts", 40)], gold: 20_000, ms: 2 * HOUR, xp: 900, build: "build_rod_gadget", region: 0 },
    Blueprint { id: "auto_smelter", level: 45, needs: &[("metal_parts", 80), ("magic_parts", 10)], gold: 50_000, ms: 3 * HOUR, xp: 1_500, build: "build_auto_smelter", region: 0 },
    Blueprint { id: "fish_trap", level: 55, needs: &[("cloth_parts", 60), ("magic_parts", 20), ("fine_parts", 2)], gold: 100_000, ms: 4 * HOUR, xp: 2_500, build: "build_fish_trap", region: 0 },
    // <gen:blueprints>
    Blueprint { id: "tide_mill", level: 18, needs: &[("wood_parts", 30), ("metal_parts", 30)], gold: 20_000, ms: 60 * MINUTE, xp: 800, build: "build_tide_mill", region: 1 },
    Blueprint { id: "salt_pan", level: 33, needs: &[("wood_parts", 30), ("metal_parts", 30)], gold: 40_000, ms: 90 * MINUTE, xp: 1_600, build: "build_salt_pan", region: 1 },
    Blueprint { id: "brass_works", level: 48, needs: &[("wood_parts", 30), ("metal_parts", 30)], gold: 60_000, ms: 120 * MINUTE, xp: 2_400, build: "build_brass_works", region: 1 },
    Blueprint { id: "sawmill_gearbox", level: 28, needs: &[("magic_parts", 10), ("fine_parts", 1)], gold: 15_000, ms: 45 * MINUTE, xp: 600, build: "build_sawmill_gearbox", region: 1 },
    Blueprint { id: "smelter_bellows", level: 43, needs: &[("magic_parts", 10), ("fine_parts", 1)], gold: 30_000, ms: 75 * MINUTE, xp: 1_200, build: "build_smelter_bellows", region: 1 },
    Blueprint { id: "trap_nets", level: 58, needs: &[("magic_parts", 10), ("fine_parts", 1)], gold: 45_000, ms: 105 * MINUTE, xp: 1_800, build: "build_trap_nets", region: 1 },
    Blueprint { id: "ore_crusher", level: 28, needs: &[("wood_parts", 60), ("metal_parts", 60)], gold: 40_000, ms: 120 * MINUTE, xp: 1_600, build: "build_ore_crusher", region: 2 },
    Blueprint { id: "lumber_flume", level: 43, needs: &[("wood_parts", 60), ("metal_parts", 60)], gold: 80_000, ms: 180 * MINUTE, xp: 3_200, build: "build_lumber_flume", region: 2 },
    Blueprint { id: "tea_press", level: 58, needs: &[("wood_parts", 60), ("metal_parts", 60)], gold: 120_000, ms: 240 * MINUTE, xp: 4_800, build: "build_tea_press", region: 2 },
    Blueprint { id: "tide_mill_wheel", level: 38, needs: &[("magic_parts", 20), ("fine_parts", 2)], gold: 30_000, ms: 90 * MINUTE, xp: 1_200, build: "build_tide_mill_wheel", region: 2 },
    Blueprint { id: "salt_pan_rakes", level: 53, needs: &[("magic_parts", 20), ("fine_parts", 2)], gold: 60_000, ms: 150 * MINUTE, xp: 2_400, build: "build_salt_pan_rakes", region: 2 },
    Blueprint { id: "brass_works_crucible", level: 68, needs: &[("magic_parts", 20), ("fine_parts", 2)], gold: 90_000, ms: 210 * MINUTE, xp: 3_600, build: "build_brass_works_crucible", region: 2 },
    Blueprint { id: "sun_furnace", level: 38, needs: &[("wood_parts", 90), ("metal_parts", 90)], gold: 60_000, ms: 180 * MINUTE, xp: 2_400, build: "build_sun_furnace", region: 3 },
    Blueprint { id: "glassworks", level: 53, needs: &[("wood_parts", 90), ("metal_parts", 90)], gold: 120_000, ms: 270 * MINUTE, xp: 4_800, build: "build_glassworks", region: 3 },
    Blueprint { id: "ember_forge", level: 68, needs: &[("wood_parts", 90), ("metal_parts", 90)], gold: 180_000, ms: 360 * MINUTE, xp: 7_200, build: "build_ember_forge", region: 3 },
    Blueprint { id: "crusher_hammers", level: 48, needs: &[("magic_parts", 30), ("fine_parts", 3)], gold: 45_000, ms: 135 * MINUTE, xp: 1_800, build: "build_crusher_hammers", region: 3 },
    Blueprint { id: "flume_gates", level: 63, needs: &[("magic_parts", 30), ("fine_parts", 3)], gold: 90_000, ms: 225 * MINUTE, xp: 3_600, build: "build_flume_gates", region: 3 },
    Blueprint { id: "press_screws", level: 78, needs: &[("magic_parts", 30), ("fine_parts", 3)], gold: 135_000, ms: 315 * MINUTE, xp: 5_400, build: "build_press_screws", region: 3 },
    // </gen:blueprints>
];

pub fn blueprint_index(id: &str) -> Option<usize> {
    static INDEX: OnceLock<BTreeMap<&'static str, usize>> = OnceLock::new();
    INDEX
        .get_or_init(|| index_of(BLUEPRINTS.iter().map(|b| b.id)))
        .get(id)
        .copied()
}

/// The blueprint whose build action is `id`.
pub fn blueprint_of_build(id: &str) -> Option<usize> {
    static INDEX: OnceLock<BTreeMap<&'static str, usize>> = OnceLock::new();
    INDEX
        .get_or_init(|| index_of(BLUEPRINTS.iter().map(|b| b.build)))
        .get(id)
        .copied()
}

/// Every tree, as (region, action id), for the rare tree.
pub fn trees() -> &'static [(usize, &'static str)] {
    static TREES: OnceLock<Vec<(usize, &'static str)>> = OnceLock::new();
    TREES.get_or_init(|| {
        ACTIONS
            .iter()
            .filter(|a| a.skill == WOODCUTTING)
            .map(|a| (a.region, a.id))
            .collect()
    })
}

/// Gadgets on your tools: each makes one gathering skill this many percent faster.
pub const GADGETS: [(&str, usize); 3] = [
    ("axe_gadget", WOODCUTTING),
    ("pick_gadget", MINING),
    ("rod_gadget", FISHING),
];
pub const GADGET_PCT: u64 = 10;

/// A machine turns one item into another alongside you and burns one fuel a
/// cycle (IG-Q34 A). The Workshop's level makes it faster.
pub struct Machine {
    pub id: &'static str,
    pub needs: &'static [(&'static str, u64)],
    pub makes: (&'static str, u64),
    pub ms: u64,
    /// Burns fuel each cycle; the tide mill and the sun furnace do not.
    pub fuel: bool,
    pub region: usize,
}

#[rustfmt::skip]
pub const MACHINES: &[Machine] = &[
    Machine { id: "sawmill", needs: &[("logs", 1)], makes: ("plank", 1), ms: 8_000, fuel: true, region: 0 },
    Machine { id: "auto_smelter", needs: &[("iron_ore", 1)], makes: ("iron_bar", 1), ms: 10_000, fuel: true, region: 0 },
    Machine { id: "fish_trap", needs: &[], makes: ("raw_trout", 1), ms: 12_000, fuel: true, region: 0 },
    // <gen:machines>
    Machine { id: "tide_mill", needs: &[("cork_logs", 1)], makes: ("cork_plank", 1), ms: 9_000, fuel: false, region: 1 },
    Machine { id: "salt_pan", needs: &[], makes: ("sea_salt", 1), ms: 11_000, fuel: true, region: 1 },
    Machine { id: "brass_works", needs: &[("zinc_ore", 1)], makes: ("brass_bar", 1), ms: 13_000, fuel: true, region: 1 },
    Machine { id: "ore_crusher", needs: &[("frostiron_ore", 1)], makes: ("frostiron_bar", 1), ms: 9_000, fuel: true, region: 2 },
    Machine { id: "lumber_flume", needs: &[("alpine_ash_logs", 1)], makes: ("alpine_ash_plank", 1), ms: 11_000, fuel: false, region: 2 },
    Machine { id: "tea_press", needs: &[("alpine_herb", 1)], makes: ("mountain_tea", 2), ms: 13_000, fuel: true, region: 2 },
    Machine { id: "sun_furnace", needs: &[("starmetal_ore", 1)], makes: ("starmetal_bar", 1), ms: 9_000, fuel: false, region: 3 },
    Machine { id: "glassworks", needs: &[("black_sand", 2)], makes: ("glass", 1), ms: 11_000, fuel: true, region: 3 },
    Machine { id: "ember_forge", needs: &[("emberite_ore", 1)], makes: ("emberite_bar", 1), ms: 13_000, fuel: true, region: 3 },
    // </gen:machines>
];
/// Fuel, in the order a machine reaches for it: charcoal first (T15).
pub const FUEL: [&str; 3] = ["charcoal", "coal", "logs"];
/// Machine upgrades (T15, Q40 A): owning one makes its machine make twice as much.
#[rustfmt::skip]
pub const UPGRADES: &[(&str, &str)] = &[
    // <gen:upgrades>
    ("sawmill_gearbox", "sawmill"),
    ("smelter_bellows", "auto_smelter"),
    ("trap_nets", "fish_trap"),
    ("tide_mill_wheel", "tide_mill"),
    ("salt_pan_rakes", "salt_pan"),
    ("brass_works_crucible", "brass_works"),
    ("crusher_hammers", "ore_crusher"),
    ("flume_gates", "lumber_flume"),
    ("press_screws", "tea_press"),
    // </gen:upgrades>
];
/// Each Workshop level above the first takes this many percent off a machine's cycle.
pub const WORKSHOP_PCT: u64 = 10;
/// One take-apart in this many also gives fine parts.
pub const FINE_PARTS_ONE_IN: u64 = 20;

/// Inventor ranks: Invention level, Workshop level, a star core, every machine.
pub struct InventorRank {
    pub id: &'static str,
    pub level: u32,
    pub workshop: u64,
    pub machine: bool,
    pub star_core: bool,
    pub every_machine: bool,
}

#[rustfmt::skip]
/// One ladder for each region: the Vale's, then a tier for each later region (T15, IG-Q43 A).
pub const INVENTOR_RANKS: [[InventorRank; 5]; 4] = [
    [
        InventorRank { id: "tinkerer", level: 1, workshop: 1, machine: false, star_core: false, every_machine: false },
        InventorRank { id: "mechanic", level: 20, workshop: 1, machine: true, star_core: false, every_machine: false },
        InventorRank { id: "engineer", level: 40, workshop: 3, machine: false, star_core: false, every_machine: false },
        InventorRank { id: "inventor", level: 60, workshop: 3, machine: false, star_core: true, every_machine: false },
        InventorRank { id: "grand_inventor", level: 85, workshop: 3, machine: false, star_core: false, every_machine: true },
    ],
    // <gen:inventor_ranks>
    [InventorRank { id: "tide_tinkerer", level: 5, workshop: 5, machine: false, star_core: false, every_machine: false }, InventorRank { id: "millwright", level: 25, workshop: 5, machine: true, star_core: false, every_machine: false }, InventorRank { id: "harbour_engineer", level: 45, workshop: 6, machine: false, star_core: false, every_machine: false }, InventorRank { id: "tide_inventor", level: 65, workshop: 6, machine: false, star_core: true, every_machine: false }, InventorRank { id: "master_of_tides", level: 88, workshop: 6, machine: false, star_core: false, every_machine: true },],
    [InventorRank { id: "forge_tinkerer", level: 8, workshop: 6, machine: false, star_core: false, every_machine: false }, InventorRank { id: "forgewright", level: 28, workshop: 6, machine: true, star_core: false, every_machine: false }, InventorRank { id: "mountain_engineer", level: 48, workshop: 7, machine: false, star_core: false, every_machine: false }, InventorRank { id: "forge_inventor", level: 68, workshop: 7, machine: false, star_core: true, every_machine: false }, InventorRank { id: "master_of_forges", level: 91, workshop: 7, machine: false, star_core: false, every_machine: true },],
    [InventorRank { id: "ember_tinkerer", level: 11, workshop: 7, machine: false, star_core: false, every_machine: false }, InventorRank { id: "furnace_wright", level: 31, workshop: 7, machine: true, star_core: false, every_machine: false }, InventorRank { id: "sun_engineer", level: 51, workshop: 8, machine: false, star_core: false, every_machine: false }, InventorRank { id: "ember_inventor", level: 71, workshop: 8, machine: false, star_core: true, every_machine: false }, InventorRank { id: "grand_artificer", level: 94, workshop: 8, machine: false, star_core: false, every_machine: true },],
    // </gen:inventor_ranks>
];
/// Each Inventor rank above Tinkerer adds one part to every take-apart.
pub const INVENTOR_RANK_PARTS: u64 = 1;

// ---------- the Cosmic (T13) ----------

/// Six signs rise in turn on game time (IG-Q33 A), each for 80 minutes.
/// Training a skill under its sign gathers starlight.
pub const SIGN_MS: u64 = 80 * MINUTE;
#[rustfmt::skip]
pub const SIGNS: [(&str, [usize; 2]); 6] = [
    ("axe", [WOODCUTTING, CONSTRUCTION]),
    ("anvil", [SMITHING, INVENTION]),
    ("sprout", [FARMING, COOKING]),
    ("blade", [MELEE, RANGED]),
    ("scroll", [TRADING, CRAFTING]),
    ("moon", [MAGIC, RUNECRAFTING]),
];
pub const SPROUT: usize = 2;

/// A sky: its own six signs and how long each is up. The Vale's sky is
/// `SIGNS`; each later region adds one, and all the open skies turn at once.
pub struct Sky {
    pub sign_ms: u64,
    pub signs: [(&'static str, [usize; 2]); 6],
}

#[rustfmt::skip]
pub const SKIES: [Sky; 4] = [
    Sky { sign_ms: SIGN_MS, signs: SIGNS },
    // <gen:skies>
    Sky { sign_ms: 60 * MINUTE, signs: [("wave", [2, 15]), ("shell", [12, 6]), ("gull", [9, 4]), ("anchor", [14, 5]), ("pearl", [7, 17]), ("tide", [13, 1])] },
    Sky { sign_ms: 100 * MINUTE, signs: [("summit", [1, 14]), ("wolf", [8, 10]), ("hearth", [6, 3]), ("eagle", [9, 0]), ("forge", [5, 16]), ("aurora", [17, 13])] },
    Sky { sign_ms: 120 * MINUTE, signs: [("sun", [3, 13]), ("dune", [15, 1]), ("flame", [5, 6]), ("scorpion", [9, 8]), ("phoenix", [10, 17]), ("eclipse", [16, 7])] },
    // </gen:skies>
];
/// Shooting stars: one mining action in this many leaves stardust.
pub const STARDUST_ONE_IN: u64 = 200;
/// The harvest moon: a crop planted under The Sprout grows in this many percent of its time.
pub const HARVEST_MOON_PCT: u64 = 75;

/// Cosmic ranks: Attunement level, starlight, stardust and lenses spent to rise.
pub struct CosmicRank {
    pub id: &'static str,
    pub level: u32,
    pub starlight: u64,
    pub stardust: u64,
    pub lenses: u64,
}

#[rustfmt::skip]
/// One ladder for each region: the Vale's, then a tier for each later region (T15, IG-Q43 A).
pub const COSMIC_RANKS: [[CosmicRank; 5]; 4] = [
    [
        CosmicRank { id: "stargazer", level: 1, starlight: 0, stardust: 0, lenses: 0 },
        CosmicRank { id: "seer", level: 20, starlight: 500, stardust: 0, lenses: 2 },
        CosmicRank { id: "astral_adept", level: 40, starlight: 0, stardust: 10, lenses: 4 },
        CosmicRank { id: "voidwalker", level: 60, starlight: 0, stardust: 50, lenses: 6 },
        CosmicRank { id: "one_with_the_wavelength", level: 90, starlight: 0, stardust: 200, lenses: 10 },
    ],
    // <gen:cosmic_ranks>
    [CosmicRank { id: "tide_watcher", level: 5, starlight: 0, stardust: 0, lenses: 2 }, CosmicRank { id: "sea_seer", level: 25, starlight: 1_500, stardust: 0, lenses: 5 }, CosmicRank { id: "pearl_adept", level: 45, starlight: 0, stardust: 20, lenses: 7 }, CosmicRank { id: "deepwalker", level: 65, starlight: 0, stardust: 80, lenses: 10 }, CosmicRank { id: "one_with_the_tide", level: 95, starlight: 0, stardust: 300, lenses: 14 },],
    [CosmicRank { id: "snow_watcher", level: 6, starlight: 0, stardust: 0, lenses: 4 }, CosmicRank { id: "peak_seer", level: 26, starlight: 3_000, stardust: 0, lenses: 7 }, CosmicRank { id: "frost_adept", level: 46, starlight: 0, stardust: 40, lenses: 9 }, CosmicRank { id: "stormwalker", level: 66, starlight: 0, stardust: 160, lenses: 12 }, CosmicRank { id: "one_with_the_storm", level: 96, starlight: 0, stardust: 600, lenses: 16 },],
    [CosmicRank { id: "sun_watcher", level: 7, starlight: 0, stardust: 0, lenses: 6 }, CosmicRank { id: "eclipse_seer", level: 27, starlight: 4_500, stardust: 0, lenses: 9 }, CosmicRank { id: "flame_adept", level: 47, starlight: 0, stardust: 60, lenses: 11 }, CosmicRank { id: "sunwalker", level: 67, starlight: 0, stardust: 240, lenses: 14 }, CosmicRank { id: "one_with_the_sun", level: 97, starlight: 0, stardust: 900, lenses: 18 },],
    // </gen:cosmic_ranks>
];
/// What each rank gives (IG-Q32 A, time): the harvest moon from Stargazer, a
/// longer away limit from Seer and Voidwalker, astral runes from Astral adept,
/// twice the starlight at the top.
#[rustfmt::skip]
pub const AWAY_CAP_BY_COSMIC: [[u64; 5]; 4] = [
    [24 * HOUR, 36 * HOUR, 36 * HOUR, 48 * HOUR, 48 * HOUR],
    [48 * HOUR, 52 * HOUR, 54 * HOUR, 56 * HOUR, 60 * HOUR],
    [60 * HOUR, 62 * HOUR, 64 * HOUR, 66 * HOUR, 66 * HOUR],
    [66 * HOUR, 68 * HOUR, 70 * HOUR, 72 * HOUR, 72 * HOUR],
];
pub const ASTRAL_RANK: usize = 2;
pub const DOUBLE_STARLIGHT_RANK: usize = 4;

/// The playtest log keeps this many events, newest last (P12).
pub const LOG_CAP: usize = 4_000;
pub const NOTE_MAX_CHARS: usize = 600;

#[cfg(test)]
mod tests {
    use super::*;

    /// Things the engines make rather than an action: starlight under a
    /// sign, stardust from shooting stars, fine parts from taking apart.
    const ENGINE_MADE: [&str; 8] = [
        "starlight",
        "stardust",
        "fine_parts",
        "fine_ashes",
        "sapphire",
        "emerald",
        "ruby",
        "diamond",
    ];

    fn makers(item: &str) -> bool {
        ACTIONS.iter().any(|a| a.makes.0 == item)
            || ENEMIES.iter().any(|e| e.drops.iter().any(|d| d.0 == item))
            || CROPS.iter().any(|c| c.produce == item)
            || TOWNS.iter().any(|t| t.sells == item)
            || MACHINES.iter().any(|m| m.makes.0 == item)
            || ENGINE_MADE.contains(&item)
    }

    /// The skills whose actions make `item`.
    fn made_by(item: &str) -> Vec<usize> {
        let mut out: Vec<usize> = ACTIONS
            .iter()
            .filter(|a| a.makes.0 == item)
            .map(|a| a.skill)
            .collect();
        if CROPS.iter().any(|c| c.produce == item) {
            out.push(FARMING);
        }
        out.sort();
        out.dedup();
        out
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
        for (item, _, _) in ARMOUR {
            assert!(makers(item), "armour {item} is made by nothing");
        }
    }

    #[test]
    fn every_item_has_a_value_entry() {
        for a in ACTIONS.iter().filter(|a| !a.makes.0.is_empty()) {
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
    fn each_farm_crop_has_a_seed_some_enemy_drops_and_a_value() {
        for c in CROPS {
            assert!(
                ENEMIES.iter().any(|e| e.drops.iter().any(|d| d.0 == c.seed)),
                "{}",
                c.seed
            );
            assert!(value(c.produce) > 0 && value(c.seed) > 0, "{}", c.id);
        }
        assert_eq!(CROPS[0].level, 1);
    }

    #[test]
    fn each_skill_has_a_level_one_action_unless_it_fights_or_farms() {
        for (i, s) in SKILLS.iter().enumerate() {
            if s.kind != Kind::Fight && s.kind != Kind::Farm && s.kind != Kind::Engine {
                assert!(ACTIONS.iter().any(|a| a.skill == i && a.level == 1), "{}", s.id);
            }
        }
    }

    #[test]
    fn the_engines_inputs_are_all_made_somewhere() {
        for t in TOWNS {
            for w in t.wants {
                assert!(makers(w), "{} wants {w}, which nothing makes", t.id);
            }
        }
        for m in MACHINES {
            for (i, _) in m.needs {
                assert!(makers(i), "{} needs {i}", m.id);
            }
        }
        for b in BLUEPRINTS {
            for (i, _) in b.needs {
                assert!(makers(i), "{} needs {i}", b.id);
            }
            assert!(
                action_index(b.build).is_some(),
                "{} builds with {}",
                b.id,
                b.build
            );
        }
        for tribute in TRIBUTES.iter().flatten() {
            for (i, _) in *tribute {
                assert!(makers(i), "a tribute asks for {i}");
            }
        }
        for plank in ROOM_PLANKS {
            assert!(makers(plank));
        }
        for ch in CHAMPIONS.iter().flatten() {
            assert!(enemy_index(ch).is_some(), "{ch}");
        }
    }

    #[test]
    fn each_recipe_unlocks_an_action_that_needs_two_or_three_skills_goods() {
        for rc in RECIPES {
            let a = &ACTIONS[action_index(rc.id).expect(rc.id)];
            assert!(enemy_index(rc.enemy).is_some(), "{}", rc.enemy);
            let mut from: Vec<usize> = a.needs.iter().flat_map(|(i, _)| made_by(i)).collect();
            from.sort();
            from.dedup();
            assert!(
                (2..=3).contains(&from.len()),
                "{} needs goods from {} skills",
                rc.id,
                from.len()
            );
            assert!((64..=512).contains(&rc.one_in), "{}", rc.id);
        }
    }

    #[test]
    fn no_new_skill_takes_from_or_feeds_more_than_three() {
        // R6.2's rule, for the four engine skills. Taking apart accepts
        // anything you made, and Trading buys goods from any skill, by design
        // (R7.3); those are left out of the count.
        for skill in [CONSTRUCTION, INVENTION, ATTUNEMENT] {
            let acts: Vec<&Action> = ACTIONS
                .iter()
                .filter(|a| a.skill == skill && !a.id.starts_with("take_apart"))
                .collect();
            let mut takes: Vec<usize> = acts
                .iter()
                .flat_map(|a| a.needs.iter().flat_map(|(i, _)| made_by(i)))
                .filter(|&s| s != skill)
                .collect();
            takes.sort();
            takes.dedup();
            let outs: Vec<&str> = acts.iter().map(|a| a.makes.0).collect();
            let mut feeds: Vec<usize> = ACTIONS
                .iter()
                .filter(|a| a.skill != skill && a.needs.iter().any(|(i, _)| outs.contains(i)))
                .map(|a| a.skill)
                .collect();
            feeds.sort();
            feeds.dedup();
            assert!(takes.len() <= 3, "{} takes from {takes:?}", SKILLS[skill].id);
            assert!(feeds.len() <= 3, "{} feeds {feeds:?}", SKILLS[skill].id);
        }
    }

    #[test]
    fn each_style_is_weak_point_of_some_enemy() {
        for s in &STYLES {
            assert!(ENEMIES.iter().any(|e| e.weak == s.skill));
        }
    }
}
