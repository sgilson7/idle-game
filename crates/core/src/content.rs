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
pub const SKILLS: [Skill; 18] = [
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
    Enemy { id: "chicken", level: 1, hp: 30, hit: 10, weak: MELEE, drops: &[("feathers", 5, 1), ("bones", 1, 1), ("potato_seed", 2, 3)], gold: 0 },
    Enemy { id: "cow", level: 1, hp: 80, hit: 10, weak: RANGED, drops: &[("cowhide", 1, 1), ("bones", 1, 1), ("sweetcorn_seed", 1, 6)], gold: 0 },
    Enemy { id: "goblin", level: 5, hp: 50, hit: 20, weak: MAGIC, drops: &[("bones", 1, 1), ("feathers", 10, 4), ("onion_seed", 2, 3), ("cabbage_seed", 1, 4)], gold: 12 },
    Enemy { id: "spider", level: 20, hp: 220, hit: 40, weak: MELEE, drops: &[("spider_silk", 1, 4)], gold: 30 },
    Enemy { id: "hill_giant", level: 30, hp: 350, hit: 60, weak: MAGIC, drops: &[("big_bones", 1, 1), ("iron_ore", 3, 3), ("coal", 2, 4), ("tomato_seed", 2, 4)], gold: 60 },
    Enemy { id: "black_knight", level: 45, hp: 420, hit: 80, weak: RANGED, drops: &[("steel_bar", 1, 3)], gold: 120 },
    Enemy { id: "moss_giant", level: 60, hp: 600, hit: 100, weak: MELEE, drops: &[("big_bones", 1, 1), ("mithril_ore", 2, 3), ("strawberry_seed", 1, 4)], gold: 200 },
    Enemy { id: "green_dragon", level: 80, hp: 1_200, hit: 180, weak: MAGIC, drops: &[("dragon_bones", 1, 1), ("dragon_scale", 1, 20), ("watermelon_seed", 2, 5)], gold: 600 },
    // Champions (T11): one boss an area, each a Nobility deed
    Enemy { id: "goblin_warlord", level: 15, hp: 400, hit: 35, weak: MAGIC, drops: &[("bones", 1, 1), ("onion_seed", 5, 1)], gold: 300 },
    Enemy { id: "broodmother", level: 30, hp: 900, hit: 60, weak: MELEE, drops: &[("spider_silk", 5, 1)], gold: 800 },
    Enemy { id: "giant_chieftain", level: 45, hp: 1_500, hit: 85, weak: MAGIC, drops: &[("big_bones", 3, 1), ("coal", 10, 1)], gold: 1_500 },
    Enemy { id: "black_knight_titan", level: 60, hp: 2_200, hit: 110, weak: RANGED, drops: &[("steel_bar", 5, 1)], gold: 3_000 },
    Enemy { id: "elder_dragon", level: 85, hp: 4_000, hit: 200, weak: MAGIC, drops: &[("dragon_bones", 3, 1), ("dragon_scale", 2, 1)], gold: 8_000 },
];

/// The champions, in deed order: Squire's deed is the first (T11).
pub const CHAMPIONS: [&str; 5] = [
    "goblin_warlord",
    "broodmother",
    "giant_chieftain",
    "black_knight_titan",
    "elder_dragon",
];

pub fn is_champion(id: &str) -> bool {
    CHAMPIONS.contains(&id)
}

pub fn enemy_index(id: &str) -> Option<usize> {
    ENEMIES.iter().position(|e| e.id == id)
}

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
    Style { skill: MELEE, ms: 2_400,
        held: &[("giants_maul", 5), ("mithril_sword", 4), ("steel_sword", 3), ("iron_sword", 2), ("bronze_sword", 1)], held_optional: false, spent: &[] },
    Style { skill: RANGED, ms: 2_000,
        held: &[("knights_longbow", 7), ("magic_shortbow", 6), ("yew_shortbow", 5), ("maple_shortbow", 4), ("willow_shortbow", 3), ("oak_shortbow", 2), ("shortbow", 1)],
        held_optional: false,
        spent: &[("mithril_arrows", 4), ("steel_arrows", 3), ("iron_arrows", 2), ("bronze_arrows", 1)] },
    Style { skill: MAGIC, ms: 3_000, held: &[("dragonfire_staff", 6)], held_optional: true,
        spent: &[("astral_rune", 6), ("death_rune", 5), ("chaos_rune", 4), ("fire_rune", 3), ("water_rune", 2), ("air_rune", 1)] },
];

/// Food, best first, and how much it heals.
pub const FOOD: [(&str, u64); 12] = [
    ("dragon_pepper_feast", 260),
    ("shark", 200),
    ("monkfish", 160),
    ("swordfish", 140),
    ("goblin_hot_pot", 120),
    ("lobster", 120),
    ("farmers_stew", 90),
    ("salmon", 90),
    ("trout", 70),
    ("cooked_sweetcorn", 60),
    ("baked_potato", 50),
    ("shrimp", 30),
];

/// Armour Crafting makes (T5), best first, by tier. Each tier takes this
/// many percent off every hit an enemy lands.
pub const ARMOUR: [(&str, u64); 7] = [
    ("mossplate", 4),
    ("dragonhide_body", 4),
    ("silkweave_vest", 3),
    ("silk_robe", 3),
    ("studded_leather", 2),
    ("hard_leather_body", 2),
    ("leather_body", 1),
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
    }
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
];

pub fn crop_index(id: &str) -> Option<usize> {
    CROPS.iter().position(|c| c.id == id)
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

/// A plan holds this many steps (P3, four since T9).
pub const PLAN_STEPS: usize = 4;
/// Step lengths offered on the plan screen; 0 means until the inputs run out
/// (or, for gathering, until you come back).
pub const PLAN_COUNTS: [u64; 4] = [0, 100, 500, 1_000];

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

// ---------- recipes (T10) ----------

/// A recipe: found once from an enemy, it unlocks the action of the same id.
pub struct Recipe {
    pub id: &'static str,
    pub enemy: &'static str,
    pub one_in: u64,
}

/// One in 64 to one in 512 by the enemy's level (IG-Q27 A).
#[rustfmt::skip]
pub const RECIPES: [Recipe; 9] = [
    Recipe { id: "farmers_stew", enemy: "chicken", one_in: 64 },
    Recipe { id: "studded_leather", enemy: "cow", one_in: 64 },
    Recipe { id: "goblin_hot_pot", enemy: "goblin", one_in: 128 },
    Recipe { id: "silkweave_vest", enemy: "spider", one_in: 128 },
    Recipe { id: "giants_maul", enemy: "hill_giant", one_in: 256 },
    Recipe { id: "knights_longbow", enemy: "black_knight", one_in: 256 },
    Recipe { id: "mossplate", enemy: "moss_giant", one_in: 384 },
    Recipe { id: "dragonfire_staff", enemy: "green_dragon", one_in: 512 },
    Recipe { id: "dragon_pepper_feast", enemy: "green_dragon", one_in: 512 },
];

pub fn recipe_index(id: &str) -> Option<usize> {
    RECIPES.iter().position(|r| r.id == id)
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
}

#[rustfmt::skip]
pub const TOWNS: [Town; 4] = [
    Town { id: "riverton", level: 1,
        wants: &["logs", "oak_logs", "willow_logs", "raw_shrimp", "raw_trout", "raw_salmon", "maple_logs", "raw_lobster"],
        sells: "spices", back_per: 10, trip_ms: HOUR, danger: 10 },
    Town { id: "saltmarsh", level: 15,
        wants: &["bronze_bar", "iron_bar", "steel_bar", "leather_body", "hard_leather_body", "gold_bar", "studded_leather"],
        sells: "dye", back_per: 10, trip_ms: 2 * HOUR, danger: 20 },
    Town { id: "highcliff", level: 30,
        wants: &["trout", "salmon", "lobster", "plank", "oak_plank", "baked_potato", "farmers_stew", "swordfish"],
        sells: "glass", back_per: 8, trip_ms: 3 * HOUR, danger: 30 },
    Town { id: "emberfall", level: 50,
        wants: &["gold_amulet", "silk_robe", "royal_robe", "dragonhide_body", "magic_shortbow", "mithril_sword", "yew_shortbow"],
        sells: "fine_parts", back_per: 4, trip_ms: 4 * HOUR, danger: 40 },
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
pub const TRADER_RANKS: [TraderRank; 5] = [
    TraderRank { id: "peddler", level: 1, orders: 0, fee: 0, each_town: 0, charter: false },
    TraderRank { id: "merchant", level: 20, orders: 10, fee: 0, each_town: 0, charter: false },
    TraderRank { id: "guild_member", level: 40, orders: 0, fee: 100_000, each_town: 0, charter: false },
    TraderRank { id: "master_trader", level: 60, orders: 0, fee: 0, each_town: 5, charter: false },
    TraderRank { id: "merchant_prince", level: 80, orders: 0, fee: 0, each_town: 0, charter: true },
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
pub const NOBLE_RANKS: [NobleRank; 7] = [
    NobleRank { id: "commoner", gold: 0, rooms: 0, income: 0, servants: 0 },
    NobleRank { id: "freeholder", gold: 5_000, rooms: 1, income: 60, servants: 0 },
    NobleRank { id: "squire", gold: 20_000, rooms: 2, income: 150, servants: 1 },
    NobleRank { id: "knight", gold: 50_000, rooms: 4, income: 400, servants: 1 },
    NobleRank { id: "baron", gold: 200_000, rooms: 6, income: 1_000, servants: 2 },
    NobleRank { id: "earl", gold: 1_000_000, rooms: 8, income: 2_500, servants: 2 },
    NobleRank { id: "duke", gold: 5_000_000, rooms: 10, income: 6_000, servants: 3 },
];
/// Ranks from Squire on need a deed: a champion (`CHAMPIONS`) or a tribute,
/// your choice each rank (IG-Q29 C). `TRIBUTES[k]` goes with `CHAMPIONS[k]`.
pub const DEED_FROM: usize = 2;
#[rustfmt::skip]
pub const TRIBUTES: [&[(&str, u64)]; 5] = [
    &[("gold_ring", 10), ("leather_body", 10)],
    &[("hard_leather_body", 10), ("salmon", 100)],
    &[("royal_robe", 5), ("lobster", 200)],
    &[("royal_robe", 10), ("gold_amulet", 20)],
    &[("royal_robe", 20), ("dragonhide_body", 5), ("shark", 100)],
];

pub struct Room {
    pub id: &'static str,
    /// The Construction level for the first level of the room; each level after needs ten more.
    pub level: u32,
    pub max: u64,
}

pub const ROOMS: [Room; 6] = [
    Room {
        id: "kitchen",
        level: 1,
        max: 5,
    },
    Room {
        id: "garden",
        level: 10,
        max: 2,
    },
    Room {
        id: "servants_quarters",
        level: 20,
        max: 3,
    },
    Room {
        id: "stables",
        level: 5,
        max: 3,
    },
    Room {
        id: "workshop",
        level: 30,
        max: 5,
    },
    Room {
        id: "observatory",
        level: 40,
        max: 5,
    },
];
pub const KITCHEN: usize = 0;
pub const GARDEN: usize = 1;
pub const QUARTERS: usize = 2;
pub const STABLES: usize = 3;
pub const WORKSHOP: usize = 4;
pub const OBSERVATORY: usize = 5;
/// The plank each room level is built from (levels 1 to 5).
pub const ROOM_PLANKS: [&str; 5] = ["plank", "plank", "oak_plank", "teak_plank", "mahogany_plank"];
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
}

#[rustfmt::skip]
pub const BLUEPRINTS: [Blueprint; 6] = [
    Blueprint { id: "axe_gadget", level: 10, needs: &[("wood_parts", 20)], gold: 2_000, ms: 20 * MINUTE, xp: 200, build: "build_axe_gadget" },
    Blueprint { id: "sawmill", level: 20, needs: &[("wood_parts", 40), ("metal_parts", 20)], gold: 8_000, ms: HOUR, xp: 500, build: "build_sawmill" },
    Blueprint { id: "pick_gadget", level: 25, needs: &[("metal_parts", 40)], gold: 10_000, ms: HOUR, xp: 600, build: "build_pick_gadget" },
    Blueprint { id: "rod_gadget", level: 35, needs: &[("cloth_parts", 40)], gold: 20_000, ms: 2 * HOUR, xp: 900, build: "build_rod_gadget" },
    Blueprint { id: "auto_smelter", level: 45, needs: &[("metal_parts", 80), ("magic_parts", 10)], gold: 50_000, ms: 3 * HOUR, xp: 1_500, build: "build_auto_smelter" },
    Blueprint { id: "fish_trap", level: 55, needs: &[("cloth_parts", 60), ("magic_parts", 20), ("fine_parts", 2)], gold: 100_000, ms: 4 * HOUR, xp: 2_500, build: "build_fish_trap" },
];

pub fn blueprint_index(id: &str) -> Option<usize> {
    BLUEPRINTS.iter().position(|b| b.id == id)
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
}

#[rustfmt::skip]
pub const MACHINES: [Machine; 3] = [
    Machine { id: "sawmill", needs: &[("logs", 1)], makes: ("plank", 1), ms: 8_000 },
    Machine { id: "auto_smelter", needs: &[("iron_ore", 1)], makes: ("iron_bar", 1), ms: 10_000 },
    Machine { id: "fish_trap", needs: &[], makes: ("raw_trout", 1), ms: 12_000 },
];
/// Fuel, in the order a machine reaches for it.
pub const FUEL: [&str; 2] = ["coal", "logs"];
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
pub const INVENTOR_RANKS: [InventorRank; 5] = [
    InventorRank { id: "tinkerer", level: 1, workshop: 1, machine: false, star_core: false, every_machine: false },
    InventorRank { id: "mechanic", level: 20, workshop: 1, machine: true, star_core: false, every_machine: false },
    InventorRank { id: "engineer", level: 40, workshop: 3, machine: false, star_core: false, every_machine: false },
    InventorRank { id: "inventor", level: 60, workshop: 3, machine: false, star_core: true, every_machine: false },
    InventorRank { id: "grand_inventor", level: 85, workshop: 3, machine: false, star_core: false, every_machine: true },
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
pub const COSMIC_RANKS: [CosmicRank; 5] = [
    CosmicRank { id: "stargazer", level: 1, starlight: 0, stardust: 0, lenses: 0 },
    CosmicRank { id: "seer", level: 20, starlight: 500, stardust: 0, lenses: 2 },
    CosmicRank { id: "astral_adept", level: 40, starlight: 0, stardust: 10, lenses: 4 },
    CosmicRank { id: "voidwalker", level: 60, starlight: 0, stardust: 50, lenses: 6 },
    CosmicRank { id: "one_with_the_wavelength", level: 90, starlight: 0, stardust: 200, lenses: 10 },
];
/// What each rank gives (IG-Q32 A, time): the harvest moon from Stargazer, a
/// longer away limit from Seer and Voidwalker, astral runes from Astral adept,
/// twice the starlight at the top.
pub const AWAY_CAP_BY_COSMIC: [u64; 5] = [24 * HOUR, 36 * HOUR, 36 * HOUR, 48 * HOUR, 48 * HOUR];
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
    const ENGINE_MADE: [&str; 3] = ["starlight", "stardust", "fine_parts"];

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
        for (item, _) in ARMOUR {
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
        for t in &TOWNS {
            for w in t.wants {
                assert!(makers(w), "{} wants {w}, which nothing makes", t.id);
            }
        }
        for m in &MACHINES {
            for (i, _) in m.needs {
                assert!(makers(i), "{} needs {i}", m.id);
            }
        }
        for b in &BLUEPRINTS {
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
        for tribute in TRIBUTES {
            for (i, _) in tribute {
                assert!(makers(i), "a tribute asks for {i}");
            }
        }
        for plank in ROOM_PLANKS {
            assert!(makers(plank));
        }
        for ch in CHAMPIONS {
            assert!(enemy_index(ch).is_some(), "{ch}");
        }
    }

    #[test]
    fn each_recipe_unlocks_an_action_that_needs_two_or_three_skills_goods() {
        for rc in &RECIPES {
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
