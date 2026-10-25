#!/usr/bin/env python3
"""Write the full game's region content (T15, Plan 3) into the core.

The Vale is today's game and stays hand-written in content.rs. The Coast, the
Peaks and the Wastes are written here, each as a few name lists and the same
template, so every region has the same shape: nine trees, nine rocks, nine
fishing spots, seven crops, and the making rows, enemies, champions, recipes,
towns, rooms, machines, signs and engine ranks that go with them.

The script rewrites the blocks between `// <gen:NAME>` and `// </gen:NAME>` in
crates/core/src/content.rs, and adds each new name to data/strings.json (names
the person may change there, IG-Q39 A; ids stay). Numbers come from a few
formulas below; they are first guesses for the playtests to correct (R7.6).

    python3 tools/content/regions.py           write
    python3 tools/content/regions.py --check   fail if a write would change anything
"""
import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
CONTENT = ROOT / "crates" / "core" / "src" / "content.rs"
STRINGS = ROOT / "data" / "strings.json"

SK = {"woodcutting": 0, "mining": 1, "fishing": 2, "firemaking": 3, "fletching": 4, "smithing": 5, "cooking": 6,
      "runecrafting": 7, "melee": 8, "ranged": 9, "magic": 10, "hitpoints": 11, "crafting": 12, "farming": 13,
      "construction": 14, "trading": 15, "invention": 16, "attunement": 17}
SKC = {v: "c" for v in SK.values()}
CONST = {0: "WOODCUTTING", 1: "MINING", 2: "FISHING", 3: "FIREMAKING", 4: "FLETCHING", 5: "SMITHING", 6: "COOKING",
         7: "RUNECRAFTING", 8: "MELEE", 9: "RANGED", 10: "MAGIC", 12: "CRAFTING", 13: "FARMING", 14: "CONSTRUCTION",
         15: "TRADING", 16: "INVENTION", 17: "ATTUNEMENT"}

# The nine gathering levels of each region, shifted so the regions interleave.
GATHER_LEVELS = [3, 12, 22, 32, 42, 52, 64, 78, 92]
SHIFT = {1: 0, 2: 2, 3: 4}
CROP_LEVELS = [5, 15, 25, 35, 45, 60, 75]
CROP_MINUTES = [20, 30, 45, 60, 90, 120, 180]
RUNE_LEVELS = [9, 20, 30, 40, 55, 70]

REGIONS = [
    {"r": 1, "id": "coast", "name": "The Coast",
     "trees": ["driftwood", "palm", "cork", "mangrove", "cypress", "sea_pine", "ironbark", "salt_cedar", "ghostwood"],
     "stone": "rock_salt", "sand": "sandstone", "alloy": ("zinc", "copper_ore", "brass"), "crystal": "quartz",
     "precious": ("silver", "silver"), "metals": ["cobalt", "sea_iron", "orichalcum", "abyssal"],
     "fish": ["crab", "sardine", "mackerel", "bass", "squid", "eel", "tuna", "ray", "angler"],
     "crops": ["kelp", "sea_beet", "rice", "samphire", "pepper", "saffron", "sea_grape"],
     "hide": ("sea_snake_skin", "snakeskin"), "silk": ("sea_silk", "sea_silk"), "top_hide": ("kraken_hide", "krakenhide"),
     "goods": ("sea_salt", "pearl", "coral", "glass"),
     "runes": ["earth", "mind", "mist", "body", "nature", "law"], "combo": ("mist", "air_rune", "water_rune"),
     "vehicle": "ship", "rooms": [("harbour", 30, "caravan", 0), ("smokehouse", 25, "faster", 2)],
     "furniture": [("fish_rack", "smokehouse"), ("cork_bed", "servants_quarters"), ("ship_in_a_bottle", "harbour")],
     "machines": [("tide_mill", "cork_logs", 1, "cork_plank", 1, False), ("salt_pan", None, 0, "sea_salt", 1, True),
                  ("brass_works", "zinc_ore", 1, "brass_bar", 1, True)],
     "upgrades": [("sawmill_gearbox", "sawmill"), ("smelter_bellows", "auto_smelter"), ("trap_nets", "fish_trap")],
     "enemies": ["shore_crab", "gull", "smuggler", "sea_snake", "pirate", "drowned_knight", "reef_troll", "sea_serpent"],
     "champions": ["pirate_captain", "siren", "drowned_king", "reef_titan", "kraken"],
     "towns": ["brinewick", "gullport", "coral_bay", "tidewatch"], "town_levels": [10, 25, 45, 65],
     "noble": ["landholder", "harbour_master", "sea_knight", "commodore", "sea_baron", "admiral", "lord_of_the_coast"],
     "trader": ["deckhand", "shipper", "sea_merchant", "harbour_guild_member", "sea_prince"],
     "inventor": ["tide_tinkerer", "millwright", "harbour_engineer", "tide_inventor", "master_of_tides"],
     "cosmic": ["tide_watcher", "sea_seer", "pearl_adept", "deepwalker", "one_with_the_tide"],
     "signs": [("wave", "fishing", "trading"), ("shell", "crafting", "cooking"), ("gull", "ranged", "fletching"),
               ("anchor", "construction", "smithing"), ("pearl", "runecrafting", "attunement"), ("tide", "farming", "mining")],
     "sign_minutes": 60, "dungeon": "sunken_ship",
     "recipes": {"food": ["crab_cakes", "fishermans_pie", "ink_risotto"], "melee": "tidecutter", "ranged": "harpoon_bow",
                 "magic": "coral_staff", "armour": ["sharkskin_vest", "barnacle_plate", "pearl_mantle"]},
     "trinkets": 1},
    {"r": 2, "id": "peaks", "name": "The Peaks",
     "trees": ["larch", "spruce", "alpine_ash", "birch", "juniper", "stone_pine", "frostwood", "cloudwood", "skyroot"],
     "stone": "granite", "sand": "flint", "alloy": ("nickel", "iron_ore", "nickel_steel"), "crystal": "rock_crystal",
     "precious": ("platinum", "platinum"), "metals": ["frostiron", "titanium", "skysteel", "aetherium"],
     "fish": ["char", "ice_cod", "glacier_trout", "grayling", "pike", "sturgeon", "snow_crab", "ice_eel", "frostfin"],
     "crops": ["barley", "hops", "buckwheat", "lingonberry", "alpine_herb", "snow_pea", "edelweiss"],
     "hide": ("wolf_pelt", "wolf_leather"), "silk": ("yeti_fur", "fur"), "top_hide": ("griffin_hide", "griffinhide"),
     "goods": ("mountain_tea", "jade", "wool", "spices"),
     "runes": ["cosmic", "dust", "blood", "frost", "storm", "spirit"], "combo": ("dust", "earth_rune", "air_rune"),
     "vehicle": "mule_train", "rooms": [("forge", 45, "faster", 5), ("library", 55, "research", 0)],
     "furniture": [("anvil_stand", "forge"), ("bookcase", "library"), ("tea_set", "kitchen")],
     "machines": [("ore_crusher", "frostiron_ore", 1, "frostiron_bar", 1, True), ("lumber_flume", "alpine_ash_logs", 1, "alpine_ash_plank", 1, False),
                  ("tea_press", "alpine_herb", 1, "mountain_tea", 2, True)],
     "upgrades": [("tide_mill_wheel", "tide_mill"), ("salt_pan_rakes", "salt_pan"), ("brass_works_crucible", "brass_works")],
     "enemies": ["grey_wolf", "mountain_goat", "highwayman", "yeti", "harpy", "stone_golem", "frost_troll", "griffin"],
     "champions": ["wolf_king", "yeti_chief", "stone_colossus", "frost_giant", "griffin_queen"],
     "towns": ["frostford", "eaglecrest", "stonegate", "cloudhold"], "town_levels": [20, 35, 55, 75],
     "noble": ["highlander", "thane", "mountain_knight", "warden", "jarl", "high_lord", "lord_of_the_peaks"],
     "trader": ["mule_driver", "pass_trader", "mountain_merchant", "peak_guild_member", "prince_of_passes"],
     "inventor": ["forge_tinkerer", "forgewright", "mountain_engineer", "forge_inventor", "master_of_forges"],
     "cosmic": ["snow_watcher", "peak_seer", "frost_adept", "stormwalker", "one_with_the_storm"],
     "signs": [("summit", "mining", "construction"), ("wolf", "melee", "magic"), ("hearth", "cooking", "firemaking"),
               ("eagle", "ranged", "woodcutting"), ("forge", "smithing", "invention"), ("aurora", "attunement", "farming")],
     "sign_minutes": 100, "dungeon": "frozen_halls",
     "recipes": {"food": ["hunters_stew", "barley_bread", "yeti_broth"], "melee": "frostbrand", "ranged": "eagle_bow",
                 "magic": "storm_staff", "armour": ["wolfskin_coat", "titan_plate", "aurora_robe"]},
     "trinkets": 2},
    {"r": 3, "id": "wastes", "name": "The Wastes",
     "trees": ["charwood", "ember_pine", "ashbark", "sunwood", "cinderoak", "glasswood", "thornwood", "emberheart", "phoenixwood"],
     "stone": "basalt", "sand": "black_sand", "alloy": ("brimstone", "iron_ore", "brimsteel"), "crystal": "obsidian",
     "precious": ("aurum", "aurum"), "metals": ["starmetal", "emberite", "dragonite", "voidsteel"],
     "fish": ["lava_eel", "cinderfish", "ash_carp", "sandfish", "fire_crab", "magma_ray", "sunfish", "salamanderfish", "phoenix_koi"],
     "crops": ["fire_pepper", "sun_melon", "cactus_fruit", "dates", "ember_bean", "ash_wheat", "sunflower"],
     "hide": ("salamander_skin", "salamander_leather"), "silk": ("ember_silk", "ember"), "top_hide": ("drake_hide", "drakehide"),
     "goods": ("fire_oil", "fire_opal", "sunstone", "fine_parts"),
     "runes": ["soul", "smoke", "wrath", "ash", "sun", "void"], "combo": ("smoke", "fire_rune", "air_rune"),
     "vehicle": "sandship", "rooms": [("treasury", 60, "income", 0), ("sun_garden", 50, "patch", 0)],
     "furniture": [("gold_scales", "treasury"), ("sun_lamp", "sun_garden"), ("star_chart", "observatory")],
     "machines": [("sun_furnace", "starmetal_ore", 1, "starmetal_bar", 1, False), ("glassworks", "black_sand", 2, "glass", 1, True),
                  ("ember_forge", "emberite_ore", 1, "emberite_bar", 1, True)],
     "upgrades": [("crusher_hammers", "ore_crusher"), ("flume_gates", "lumber_flume"), ("press_screws", "tea_press")],
     "enemies": ["fire_imp", "salamander", "scorpion", "ash_wraith", "dune_raider", "lava_golem", "fire_elemental", "fire_drake"],
     "champions": ["imp_lord", "sand_sphinx", "ash_titan", "lava_king", "ember_wyrm"],
     "towns": ["cinderhold", "sunreach", "ashport", "duneholm"], "town_levels": [30, 50, 70, 90],
     "noble": ["settler", "ember_squire", "dune_knight", "sun_baron", "ash_count", "flame_duke", "sovereign_of_the_wastes"],
     "trader": ["sand_trader", "dune_merchant", "ember_guild_member", "sun_merchant", "emperor_of_trade"],
     "inventor": ["ember_tinkerer", "furnace_wright", "sun_engineer", "ember_inventor", "grand_artificer"],
     "cosmic": ["sun_watcher", "eclipse_seer", "flame_adept", "sunwalker", "one_with_the_sun"],
     "signs": [("sun", "firemaking", "farming"), ("dune", "trading", "mining"), ("flame", "smithing", "cooking"),
               ("scorpion", "ranged", "melee"), ("phoenix", "magic", "attunement"), ("eclipse", "invention", "runecrafting")],
     "sign_minutes": 120, "dungeon": "ember_depths",
     "recipes": {"food": ["fire_curry", "sun_feast", "phoenix_roast"], "melee": "sunblade", "ranged": "drakebone_bow",
                 "magic": "void_staff", "armour": ["drakescale_coat", "voidsteel_plate", "ember_silk_robe"]},
     "trinkets": 3},
]

# ---------------------------------------------------------------- numbers
def lvl(i, r):
    return min(99, GATHER_LEVELS[i] + SHIFT[r])

def gather_ms(L):
    return 2_800 + 95 * L

def gather_xp(skill, L, r):
    a, b = {0: (3.2, 0.0016), 1: (3.5, 0.0013), 2: (4.0, 0.0014)}[skill]
    return round((a + b * L * L) * (1 + 0.04 * r) * gather_ms(L) / 1000)

def make_ms(L):
    return 2_500 + 30 * L

def make_xp(L, r, scale=1.0):
    return max(5, round((5 + 0.03 * L * L) * (1 + 0.04 * r) * scale))

def raw_value(L):
    return 1 + L * L // 40

def tier(L, r):
    """Weapon and ammunition tiers for region rows: later regions are better at the same level."""
    return round(L / 99 * 13) + r

# ---------------------------------------------------------------- output tables
acts, values, enemies, crops, recipes, towns, blueprints, machines = [], {}, [], [], [], [], [], []
food, armour, held = [], [], {"melee": [], "ranged": [], "magic": []}
spent = {"ranged": [], "magic": []}
names = {"actions": {}, "items": {}, "enemies": {}, "ui": {}}
noble, trader, inventor, cosmic, champions, tributes, skies, rooms, dungeons = [], [], [], [], [], [], [], [], []
trinkets, furniture, upgrades, room_planks = [], [], [], []

def title(s):
    s = s.replace("_", " ")
    return s[:1].upper() + s[1:]

def act(id_, skill, L, ms, xp, needs, makes, r, name=None, kind="make"):
    acts.append((id_, skill, L, ms, xp, needs, makes, r, kind))
    names["actions"][id_] = name or title(id_)

def item(i, v, name=None):
    values.setdefault(i, max(0, int(v)))
    names["items"].setdefault(i, name or title(i))

def worth(needs):
    return sum(values.get(i, 1) * n for i, n in needs)

for R in REGIONS:
    r = R["r"]; rid = R["id"]
    names["ui"][f"region_{rid}"] = R["name"]
    logs = [f"{t}_logs" for t in R["trees"]]
    # --- Woodcutting
    for i, t in enumerate(R["trees"]):
        L = lvl(i, r)
        act(f"{t}_tree", 0, L, gather_ms(L), gather_xp(0, L, r), [], (logs[i], 1), r, title(t), "gather")
        item(logs[i], raw_value(L) + 1)
    # --- Mining: stone, sand, alloy ore, crystal, precious ore, four metals
    al, al_partner, al_bar = R["alloy"]
    ores = [R["stone"], R["sand"], f"{al}_ore", R["crystal"], f"{R['precious'][0]}_ore"] + [f"{m}_ore" for m in R["metals"]]
    rock_ids = [R["stone"], R["sand"], al, R["crystal"], R["precious"][0]] + R["metals"]
    for i, (rk, ore) in enumerate(zip(rock_ids, ores)):
        L = lvl(i, r)
        act(f"{rk}_rock", 1, L, gather_ms(L), gather_xp(1, L, r), [], (ore, 1), r, title(rk), "gather")
        item(ore, raw_value(L) + 1)
    # --- Fishing
    for i, f in enumerate(R["fish"]):
        L = lvl(i, r)
        act(f"{f}_spot", 2, L, gather_ms(L), gather_xp(2, L, r), [], (f"raw_{f}", 1), r, title(f), "gather")
        item(f"raw_{f}", raw_value(L) + 2)
    # --- Farming: seven crops, their seeds dropped by the region's enemies
    for i, cr in enumerate(R["crops"]):
        L = min(99, CROP_LEVELS[i] + SHIFT[r])
        crops.append((cr, L, CROP_MINUTES[i], f"{cr}_seed", 8 + L // 2, 9 + L * 2 // 3, max(2, L * L // 30), r))
        item(cr, 2 + L * L // 60); item(f"{cr}_seed", max(1, L * L // 80))
        names["actions"][cr] = title(cr)
    # --- Firemaking: one burn for each log; better logs leave fine ashes (Q40 A)
    for i, t in enumerate(R["trees"]):
        L = lvl(i, r)
        act(f"burn_{t}", 3, L, 2_000 + 35 * L, round(40 + 0.04 * L * L), [(logs[i], 1)], ("ashes", 1), r, title(t) + " logs")
    if r == 1:
        act("char_logs", 3, 10, 6_000, 25, [(logs[0], 1)], ("charcoal", 2), r, "Charcoal")
        item("charcoal", 6)
    # --- Smithing
    bars = {}
    bars["alloy"] = f"{al_bar}_bar"
    L = lvl(2, r) + 2
    act(f"{al_bar}_bar", 5, L, make_ms(L), make_xp(L, r, .4), [(f"{al}_ore", 1), (al_partner, 1)], (bars["alloy"], 1), r)
    item(bars["alloy"], worth([(f"{al}_ore", 1), (al_partner, 1)]) * 1.4)
    pbar = f"{R['precious'][1]}_bar"
    L = lvl(4, r) + 2
    act(pbar, 5, L, make_ms(L), make_xp(L, r, .4), [(f"{R['precious'][0]}_ore", 1)], (pbar, 1), r)
    item(pbar, values[f"{R['precious'][0]}_ore"] * 1.6)
    metal_bars = []
    for k, m in enumerate(R["metals"]):
        L = lvl(5 + k, r) + 2
        b = f"{m}_bar"; metal_bars.append(b)
        needs = [(f"{m}_ore", 1), ("coal", 2 + k)]
        act(b, 5, L, make_ms(L), make_xp(L, r, .4), needs, (b, 1), r)
        item(b, worth(needs) * 1.4)
    for k, b in enumerate([bars["alloy"]] + metal_bars):
        L = min(99, (lvl(2, r) if k == 0 else lvl(4 + k, r)) + 4)
        sw = b.replace("_bar", "_sword")
        act(sw, 5, L, make_ms(L), make_xp(L, r, .8), [(b, 2)], (sw, 1), r)
        item(sw, values[b] * 2.6)
        held["melee"].append((sw, tier(L, r)))
    for k in (1, 3):
        b = metal_bars[k]; L = min(99, lvl(5 + k, r) + 3)
        ah = b.replace("_bar", "_arrowheads")
        act(ah, 5, L, make_ms(L) // 2, make_xp(L, r, .6), [(b, 1)], (ah, 15), r)
        item(ah, values[b] / 10 + 1)
    for k in (0, 2):
        b = metal_bars[k]; L = min(99, lvl(5 + k, r) + 6)
        pb = b.replace("_bar", "_platebody")
        act(pb, 5, L, make_ms(L) + 1_000, make_xp(L, r, 1.4), [(b, 4)], (pb, 1), r)
        item(pb, values[b] * 5.5)
        armour.append((pb, 4, 20 * r + (10 if k else 0)))
    L = lvl(6, r) + 3
    act(f"{metal_bars[1].replace('_bar', '')}_gears", 5, L, make_ms(L), make_xp(L, r, .7), [(metal_bars[1], 1)], ("gear", 4), r)
    L = lvl(2, r) + 6
    act(f"{al_bar}_nails", 5, L, make_ms(L), make_xp(L, r, .4), [(bars["alloy"], 1)], ("nails", 30), r, title(al_bar) + " nails")
    # --- Fletching: shafts, bows, crossbows (Q40 A) and arrows
    L = lvl(0, r) + 1
    act(f"{R['trees'][0]}_shafts", 4, L, 2_000, make_xp(L, r, .3), [(logs[0], 1)], ("arrow_shafts", 20), r, title(R["trees"][0]) + " shafts")
    L = lvl(1, r) + 1
    act(f"{R['trees'][1]}_shafts", 4, L, 2_000, make_xp(L, r, .3), [(logs[1], 1)], ("arrow_shafts", 25), r, title(R["trees"][1]) + " shafts")
    for i in (1, 3, 5, 7, 8):
        L = min(99, lvl(i, r) + 3)
        bow = f"{R['trees'][i]}_bow"
        act(bow, 4, L, make_ms(L), make_xp(L, r, .7), [(logs[i], 2)], (bow, 1), r)
        item(bow, values[logs[i]] * 3 + 10)
        held["ranged"].append((bow, tier(L, r)))
    for b, i in ((bars["alloy"], 2), (metal_bars[1], 6), (metal_bars[3], 8)):
        L = min(99, lvl(i, r) + 5)
        xb = b.replace("_bar", "_crossbow")
        needs = [(b, 2), (logs[i], 1)]
        act(xb, 4, L, make_ms(L) + 500, make_xp(L, r, .9), needs, (xb, 1), r)
        item(xb, worth(needs) * 1.6)
        held["ranged"].append((xb, tier(L, r) + 1))
    for k in (1, 3):
        b = metal_bars[k]; L = min(99, lvl(5 + k, r) + 5)
        ah = b.replace("_bar", "_arrowheads"); ar = b.replace("_bar", "_arrows")
        needs = [("arrow_shafts", 15), ("feathers", 15), (ah, 15)]
        act(ar, 4, L, 2_500, make_xp(L, r, .8), needs, (ar, 15), r)
        item(ar, values[ah] + 2)
        spent["ranged"].append((ar, tier(L, r)))
    # --- Cooking: every fish, and three dishes of fish and crops
    for i, f in enumerate(R["fish"]):
        L = lvl(i, r)
        act(f"cook_{f}", 6, L, 2_000 + 25 * L, round(30 + 2.2 * L), [(f"raw_{f}", 1)], (f"cooked_{f}", 1), r, title(f))
        item(f"cooked_{f}", values[f"raw_{f}"] * 1.8 + 2, title(f))
        food.append((f"cooked_{f}", 30 + 3 * L + 20 * r))
    g_food, g_jewel, g_craft, _ = R["goods"]
    for d, (fi, ci, extra) in enumerate([(1, 1, None), (3, 2, None), (5, 4, g_food)]):
        L = min(99, max(lvl(fi, r), CROP_LEVELS[ci] + SHIFT[r]) + 4)
        dish = f"{R['crops'][ci]}_{R['fish'][fi]}_dish"
        needs = [(f"raw_{R['fish'][fi]}", 1), (R["crops"][ci], 2)] + ([(extra, 1)] if extra else [])
        act(dish, 6, L, 3_000, round(40 + 2.6 * L), needs, (dish, 1), r, title(R["crops"][ci]) + " and " + R["fish"][fi].replace("_", " "))
        item(dish, worth(needs) * 1.5, title(R["crops"][ci]) + " and " + R["fish"][fi].replace("_", " "))
        food.append((dish, 50 + 3 * L + 30 * r))
    # --- Crafting: leather, cloth, jewellery, glass, trinkets
    raw_hide, leather = R["hide"]
    L = lvl(1, r) + 2
    act(f"tan_{leather}", 12, L, 2_000, make_xp(L, r, .3), [(raw_hide, 1)], (leather, 1), r, "Tan " + title(leather).lower())
    item(raw_hide, raw_value(L) + 10); item(leather, values[raw_hide] + 5)
    for k, (part, n, extra) in enumerate([("body", 3, 0), ("chaps", 2, 0)]):
        L = lvl(3 + 2 * k, r) + 2
        it = f"{leather}_{part}"
        act(it, 12, L, make_ms(L), make_xp(L, r, .8), [(leather, n)], (it, 1), r)
        item(it, values[leather] * n * 1.6)
        armour.append((it, 3 if part == "body" else 2, 10 * r))
    raw_silk, silk = R["silk"]
    L = lvl(3, r) + 4
    act(f"{silk}_cloth", 12, L, 2_500, make_xp(L, r, .3), [(raw_silk, 1)], (f"{silk}_cloth", 1), r)
    item(raw_silk, raw_value(L) + 15); item(f"{silk}_cloth", values[raw_silk] + 6)
    L = lvl(6, r) + 2
    act(f"{silk}_robe", 12, L, make_ms(L), make_xp(L, r, 1.0), [(f"{silk}_cloth", 4)], (f"{silk}_robe", 1), r)
    item(f"{silk}_robe", values[f"{silk}_cloth"] * 6)
    armour.append((f"{silk}_robe", 3, 25 * r))
    raw_top, top = R["top_hide"]
    L = min(99, lvl(8, r) + 1)
    act(f"{top}_body", 12, L, make_ms(L) + 1_500, make_xp(L, r, 1.6), [(raw_top, 3), (leather, 2)], (f"{top}_body", 1), r)
    item(raw_top, 400 * r); item(f"{top}_body", values[raw_top] * 4)
    armour.append((f"{top}_body", 4, 40 * r))
    for k, (kind, n) in enumerate([("ring", 1), ("amulet", 2)]):
        L = lvl(4 + 2 * k, r) + 3
        it = f"{g_jewel}_{kind}"
        needs = [(pbar, n), (g_jewel, 1)]
        act(it, 12, L, make_ms(L), make_xp(L, r, .9), needs, (it, 1), r)
        item(it, values[pbar] * n * 2 + 60 * r)
    L = lvl(3, r) + 1
    act(f"{R['crystal']}_glass", 12, L, 2_500, make_xp(L, r, .3), [(R["crystal"], 1)], ("glass", 1), r, title(R["crystal"]) + " glass")
    L = lvl(2, r) + 4
    act(f"{g_craft}_bracelet", 12, L, make_ms(L), make_xp(L, r, .7), [(pbar, 1), (g_craft, 1)], (f"{g_craft}_bracelet", 1), r)
    item(f"{g_craft}_bracelet", values[pbar] * 2 + 40 * r)
    t = R["trinkets"]; roman = {1: "I", 2: "II", 3: "III"}[t]
    for k, (kind, gem, rune) in enumerate([("power", "ruby", "fire_rune"), ("vigour", "emerald", "water_rune"),
                                           ("wealth", "sapphire", "air_rune"), ("warding", "diamond", "earth_rune")]):
        L = min(99, 30 + 20 * (t - 1) + 5 * k)
        it = f"{'ring' if k % 2 == 0 else 'amulet'}_of_{kind}_{t}"
        needs = [(pbar, 1), (gem, 1), (rune, 10 * t)]
        act(it, 12, L, make_ms(L) + 1_000, make_xp(L, r, 1.2), needs, (it, 1), r,
            f"{'Ring' if k % 2 == 0 else 'Amulet'} of {kind} {roman}")
        item(it, 300 * t * t, f"{'Ring' if k % 2 == 0 else 'Amulet'} of {kind} {roman}")
        trinkets.append((it, kind, {"warding": 50}.get(kind, 10) * t))
    # --- Runecrafting: five runes and a combination rune (one essence makes two)
    cname, ca, cb = R["combo"]
    for k, rn in enumerate(R["runes"]):
        L = min(99, RUNE_LEVELS[k] + SHIFT[r])
        needs = [("rune_essence", 1)] + ([(ca, 1), (cb, 1)] if rn == cname else [])
        n = 2 if rn == cname else 1
        act(f"{rn}_runes", 7, L, 2_500 + 20 * L, 8 + L // 8, needs, (f"{rn}_rune", n), r)
        item(f"{rn}_rune", 3 + L // 4)
        spent["magic"].append((f"{rn}_rune", tier(L, r) + (1 if rn == cname else 0)))
    # --- Construction: two planks, a vehicle, three pieces of furniture
    for i in (2, 4):
        L = lvl(i, r) + 1
        pl = f"{R['trees'][i]}_plank"
        act(pl, 14, L, 2_800 + 20 * L, make_xp(L, r, .35), [(logs[i], 1)], (pl, 1), r)
        item(pl, values[logs[i]] + 4)
    room_planks.extend([f"{R['trees'][2]}_plank", f"{R['trees'][4]}_plank"] if r < 3 else [f"{R['trees'][4]}_plank"])
    L = 10 + 15 * (r - 1)
    veh = R["vehicle"]
    needs = [(f"{R['trees'][2]}_plank", 8), ("nails", 40), (f"{silk}_cloth" if r == 1 else leather, 4)]
    act(f"build_{veh}", 14, L, 8_000, make_xp(L, r, 1.2), needs, (veh, 1), r, title(veh))
    item(veh, worth(needs) * 1.2)
    for k, (fu, room) in enumerate(R["furniture"]):
        L = min(99, 30 + 20 * (r - 1) + 8 * k)
        needs = [(f"{R['trees'][4]}_plank", 6), ("nails", 30), (g_craft if k == 2 else g_food, 2)]
        act(f"build_{fu}", 14, L, 7_000, make_xp(L, r, 1.1), needs, (fu, 1), r, title(fu))
        item(fu, 0)
        furniture.append((fu, room))
    # --- Invention: take apart what the region makes, build its machines and upgrades
    for k, it in enumerate([f"{R['trees'][1]}_bow", f"{al_bar}_sword", f"{leather}_body", f"{R['trees'][3]}_bow",
                            f"{R['metals'][1]}_sword", f"{silk}_robe", f"{g_jewel}_amulet", f"{al_bar}_crossbow",
                            f"{R['metals'][2]}_sword"]):
        L = min(99, 5 + 10 * k + 3 * r)
        part = {0: "wood_parts", 3: "wood_parts", 2: "cloth_parts", 5: "cloth_parts", 6: "magic_parts"}.get(k, "metal_parts")
        act(f"take_apart_{it}", 16, L, 2_200 + 25 * L, make_xp(L, r, .4), [(it, 1)], (part, 4 + 2 * r + k // 2), r,
            title(it) + " parts")
    for k, (m, inp, ni, out, no, fuel) in enumerate(R["machines"]):
        L = min(99, 20 + 15 * k + 10 * (r - 1))
        needs = [("metal_parts", 40 + 20 * r), ("gear", 4 + 2 * r), ("frame", 2 + r)]
        act(f"build_{m}", 16, L, 15_000, 300 + 150 * r * (k + 1), needs, (m, 1), r, title(m))
        item(m, 0)
        machines.append((m, inp, ni, out, no, 9_000 + 2_000 * k, fuel, r))
        blueprints.append((m, L - 2, [("wood_parts", 30 * r), ("metal_parts", 30 * r)], 20_000 * r * (k + 1), (60 + 30 * k) * r, 800 * r * (k + 1), f"build_{m}", r))
    for k, (u, m) in enumerate(R["upgrades"]):
        L = min(99, 30 + 15 * k + 10 * (r - 1))
        needs = [("metal_parts", 30 + 20 * r), ("spring", 6 + 2 * r), ("lens", r)]
        act(f"build_{u}", 16, L, 12_000, 250 + 120 * r * (k + 1), needs, (u, 1), r, title(u))
        item(u, 0)
        upgrades.append((u, m))
        blueprints.append((u, L - 2, [("magic_parts", 10 * r), ("fine_parts", r)], 15_000 * r * (k + 1), (45 + 30 * k) * r, 600 * r * (k + 1), f"build_{u}", r))
    # --- Attunement: meditate on the region's runes
    for k, rn in enumerate(R["runes"]):
        L = min(99, 5 + 14 * k + 5 * (r - 1))
        needs = [(f"{rn}_rune", 10), ("starlight", 8 + 6 * k + 4 * r)] + ([("stardust", 1)] if k == 5 else [])
        act(f"meditate_{rn}", 17, L, 4_000 + 40 * L, round(25 + 0.03 * L * L), needs, ("", 0), r, title(rn) + " runes")
    # --- Combat: eight enemies, five champions, nine recipes, a dungeon
    elevels = [8, 18, 28, 38, 50, 62, 74, 88]
    seeds = [f"{c}_seed" for c in R["crops"]]
    for k, en in enumerate(R["enemies"]):
        L = min(99, elevels[k] + SHIFT[r])
        hp = round((30 + 12 * L + L * L / 16) * (1 + 0.1 * r))
        hit = round((10 + 2 * L) * (1 + 0.05 * r))
        gold = L * L // 10 + L
        weak = ["MELEE", "RANGED", "MAGIC"][(k + r) % 3]
        drops = [("bones", 1, 1), (seeds[min(k, 6)], 2, 3)]
        if k in (1, 3): drops.append((raw_hide, 1, 2))
        if k == 4: drops.append((raw_silk, 1, 3))
        if k == 5: drops.append((ores[6], 3, 3))
        if k == 0: drops.append(("feathers", 10, 2))
        if k == 7: drops.append((raw_top, 1, 25))
        enemies.append((en, L, hp, hit, weak, drops, gold, r, False))
        names["enemies"][en] = title(en)
    clevels = [20, 40, 55, 70, 90]
    for k, ch in enumerate(R["champions"]):
        L = min(99, clevels[k] + SHIFT[r])
        hp = round((30 + 12 * L + L * L / 16) * (1 + 0.1 * r) * 3)
        hit = round((10 + 2 * L) * (1 + 0.05 * r) * 1.3)
        gold = (L * L // 10 + L) * 5
        weak = ["MAGIC", "MELEE", "RANGED"][(k + r) % 3]
        drops = [("big_bones", 2, 1), (raw_top, 1, 4 if k == 4 else 10), (seeds[6], 3, 2)]
        enemies.append((ch, L, hp, hit, weak, drops, gold, r, True))
        names["enemies"][ch] = title(ch)
    champions.append(R["champions"])
    # recipes: three foods, three weapons, three armour
    rc = R["recipes"]
    fishes = [f"cooked_{f}" for f in R["fish"]]
    rows = [
        (rc["food"][0], 6, 20, [(fishes[2], 1), (R["crops"][2], 1), (R["crops"][4], 1)], "food", 160 + 60 * r),
        (rc["food"][1], 6, 45, [(fishes[4], 1), (R["crops"][1], 2), (g_food, 1)], "food", 220 + 60 * r),
        (rc["food"][2], 6, 80, [(fishes[8], 1), (R["crops"][6], 1), (R["crops"][5], 1)], "food", 320 + 60 * r),
        (rc["melee"], 5, 60, [(metal_bars[2], 3), (g_craft, 1), (logs[5], 1)], "melee", tier(99, r) - 1),
        (rc["ranged"], 4, 70, [(metal_bars[1], 2), (logs[7], 1), (f"{silk}_cloth", 2)], "ranged", tier(99, r)),
        (rc["magic"], 12, 75, [(logs[8], 1), (g_craft, 2), (f"{R['runes'][5]}_rune", 20)], "magic", tier(99, r)),
        (rc["armour"][0], 12, 50, [(leather, 4), (g_jewel, 1), (metal_bars[0], 1)], "armour", (3, 30 * r)),
        (rc["armour"][1], 5, 85, [(metal_bars[3], 4), (g_craft, 2), (leather, 2)], "armour", (4, 60 * r)),
        (rc["armour"][2], 12, 90, [(f"{silk}_cloth", 4), (g_jewel, 2), (f"{R['runes'][4]}_rune", 30)], "armour", (4, 50 * r)),
    ]
    for k, (rid_, skill, L, needs, kind, power) in enumerate(rows):
        L = min(99, L + SHIFT[r])
        act(rid_, skill, L, make_ms(L) + 1_500, make_xp(L, r, 1.8), needs, (rid_, 1), r)
        item(rid_, worth(needs) * 2)
        if kind == "food": food.append((rid_, power))
        elif kind == "armour": armour.append((rid_, power[0], power[1]))
        elif kind == "magic": held["magic"].append((rid_, power))
        else: held[kind].append((rid_, power))
        en = R["enemies"][min(k, 7)] if k < 8 else R["enemies"][7]
        recipes.append((rid_, en, [64, 64, 128, 128, 256, 256, 384, 512, 512][k], r))
        names["ui"][f"does_{rid_}"] = {"food": f"heals {power}", "melee": f"melee weapon tier {power}",
                                        "ranged": f"ranged weapon tier {power}", "magic": f"magic staff tier {power}"}.get(
            kind, f"armour, and {power[1] if kind == 'armour' else 0} more hitpoints")
        names["ui"][f"gate_{rid_}"] = "Needs its recipe, from an enemy"
    dungeons.append((R["dungeon"], r, R["enemies"] + [R["champions"][4]]))
    names["ui"][f"dungeon_{R['dungeon']}"] = title(R["dungeon"])
    # trade goods
    for g in R["goods"]:
        if g not in values: item(g, 30 + 25 * r)
    # towns: each wants the region's raw and made goods, and sells one of its goods
    wants_pool = [logs[1], logs[4], f"raw_{R['fish'][2]}", ores[5], bars["alloy"], f"{leather}_body", fishes[3],
                  f"{R['trees'][2]}_plank", metal_bars[1], f"{g_jewel}_ring", f"{silk}_robe", fishes[6], metal_bars[3],
                  f"{R['crops'][3]}", f"{R['trees'][7]}_bow", f"{g_jewel}_amulet"]
    for k, tn in enumerate(R["towns"]):
        wants = wants_pool[k * 4:(k + 1) * 4] + wants_pool[(k * 4 + 5) % 16:(k * 4 + 5) % 16 + 2]
        towns.append((tn, R["town_levels"][k], wants, R["goods"][k], 6 - k, (k + 1) * 3_600_000, 15 + 10 * k + 5 * r, r, veh))
        names["ui"][f"town_{tn}"] = title(tn)
    # ranks
    g0 = [0, 50_000, 150_000, 400_000, 1_000_000, 3_000_000, 10_000_000]
    for k, nm in enumerate(R["noble"]):
        noble.append((nm, g0[k] * r * r, (12 + 4 * k) * r if k else 0, [0, 300, 700, 1_500, 3_000, 6_000, 12_000][k] * r, [0, 1, 1, 2, 2, 3, 3][k], r))
        names["ui"][f"noble_{nm}"] = title(nm)
    tributes.append([[(R["goods"][0], 50 * k + 50), (f"{g_jewel}_ring", 5 * k + 5)] if k % 2 == 0 else
                     [(metal_bars[min(k, 3)], 20 * k + 20), (f"{silk}_robe", 2 * k + 2)] for k in range(5)])
    for k, nm in enumerate(R["trader"]):
        trader.append((nm, [5, 25, 45, 65, 85][k] + 3 * (r - 1), [0, 10, 0, 0, 0][k] * r, [0, 0, 250_000, 0, 0][k] * r * r, [0, 0, 0, 5, 0][k], k == 4, r))
        names["ui"][f"trader_{nm}"] = title(nm)
    for k, nm in enumerate(R["inventor"]):
        inventor.append((nm, [5, 25, 45, 65, 88][k] + 3 * (r - 1), 5 + r if k >= 2 else 4 + r, k == 1, k == 3, k == 4, r))
        names["ui"][f"inventor_{nm}"] = title(nm)
    for k, nm in enumerate(R["cosmic"]):
        cosmic.append((nm, [5, 25, 45, 65, 95][k] + 1 * (r - 1), [0, 1_500, 0, 0, 0][k] * r, [0, 0, 20, 80, 300][k] * r, [0, 3, 5, 8, 12][k] + 2 * r, r))
        names["ui"][f"cosmic_{nm}"] = title(nm)
    skies.append((R["sign_minutes"], [(s, SK[a], SK[b]) for s, a, b in R["signs"]]))
    for s, _, _ in R["signs"]:
        names["ui"][f"sign_{s}"] = "The " + title(s)
    for rm, base, eff, arg in R["rooms"]:
        rooms.append((rm, base, eff, arg, r))
        names["ui"][f"room_{rm}"] = title(rm)
    for m, *_ in R["machines"]:
        names["ui"][f"bp_{m}"] = title(m)
    for u, m in R["upgrades"]:
        names["ui"][f"bp_{u}"] = title(u)
        names["ui"][f"bp_{u}_does"] = f"the {m.replace('_', ' ')} makes twice as much"
        names["ui"][f"gate_build_{u}"] = "Needs its blueprint, from research"
    for m, inp, ni, out, no, fuel in R["machines"]:
        names["ui"][f"bp_{m}_does"] = (f"{(inp or '').replace('_', ' ')} into {out.replace('_', ' ')}" if inp else f"makes {out.replace('_', ' ')}") + (" alongside you" if fuel else ", burning no fuel")
        names["ui"][f"gate_build_{m}"] = "Needs its blueprint, from research"

# gems (Mining, Q40 A) and other things the engines make
for g, v in [("sapphire", 60), ("emerald", 120), ("ruby", 220), ("diamond", 400), ("fine_ashes", 4)]:
    item(g, v)

# ---------------------------------------------------------------- Rust
def rs_needs(needs):
    return "&[" + ", ".join(f'("{i}", {n})' for i, n in needs) + "]"

def num(n):
    n = int(n)
    s = f"{n:,}".replace(",", "_")
    return s

ROOM_EFFECT = {"caravan": "RoomEffect::Caravan", "faster": "RoomEffect::Faster({})", "research": "RoomEffect::Research",
               "income": "RoomEffect::Income", "patch": "RoomEffect::Patch"}

def rows_actions():
    out = []
    for id_, skill, L, ms, xp, needs, (mi, mn), r, kind in acts:
        if kind == "gather":
            out.append(f'    region({r}, gather("{id_}", {skill}, {L}, {num(ms)}, {xp}, "{mi}")),')
        else:
            out.append(f'    region({r}, make("{id_}", {skill}, {L}, {num(ms)}, {xp}, {rs_needs(needs)}, ("{mi}", {mn}))),')
    return out

def rows_enemies():
    out = []
    for en, L, hp, hit, weak, drops, gold, r, ch in enemies:
        d = ", ".join(f'("{i}", {n}, {o})' for i, n, o in drops)
        out.append(f'    Enemy {{ id: "{en}", level: {L}, hp: {num(hp)}, hit: {hit}, weak: {weak}, drops: &[{d}], gold: {num(gold)}, region: {r} }},')
    return out

def gen_blocks():
    block = CONTENT.read_text().split("pub const VALUES")[1].split("// <gen:values>")[0]
    vale_values = set(re.findall(r'\("([a-z_0-9]+)", [\d_]+\)', block))
    B = {}
    B["actions"] = rows_actions()
    B["enemies"] = rows_enemies()
    B["crops"] = [f'    region_crop({r}, crop("{c}", {L}, {m}, "{s}", {xp}, {xe}, {cost})),' for c, L, m, s, xp, xe, cost, r in crops]
    B["recipes"] = [f'    Recipe {{ id: "{i}", enemy: "{e}", one_in: {o}, region: {r} }},' for i, e, o, r in recipes]
    # only items not valued in the Vale block
    newv = [(k, v) for k, v in values.items() if k not in vale_values]
    B["values"] = ["    " + ", ".join(f'("{k}", {num(v)})' for k, v in newv[i:i + 6]) + "," for i in range(0, len(newv), 6)]
    B["towns"] = [f'    Town {{ id: "{t}", level: {L}, wants: &[{", ".join(repr(w).replace(chr(39), chr(34)) for w in wants)}], sells: "{s}", back_per: {bp}, trip_ms: {num(ms)}, danger: {d}, region: {r}, vehicle: "{v}" }},'
                  for t, L, wants, s, bp, ms, d, r, v in towns]
    B["blueprints"] = [f'    Blueprint {{ id: "{i}", level: {L}, needs: {rs_needs(n)}, gold: {num(g)}, ms: {m} * MINUTE, xp: {num(x)}, build: "{b}", region: {r} }},'
                       for i, L, n, g, m, x, b, r in blueprints]
    B["machines"] = [f'    Machine {{ id: "{m}", needs: {rs_needs([(i, n)] if i else [])}, makes: ("{o}", {no}), ms: {num(ms)}, fuel: {"true" if f else "false"}, region: {r} }},'
                     for m, i, n, o, no, ms, f, r in machines]
    B["upgrades"] = [f'    ("{u}", "{m}"),' for u, m in upgrades]
    B["trinkets"] = [f'    ("{i}", Trinket::{k.capitalize()}, {p}),' for i, k, p in trinkets]
    B["furniture"] = [f'    ("{f}", "{r}"),' for f, r in furniture]
    B["room_planks"] = ["    " + ", ".join(f'"{p}"' for p in room_planks) + ","]
    B["rooms"] = [f'    Room {{ id: "{rm}", level: {b}, max: 5, region: {r}, effect: {ROOM_EFFECT[e].format(CONST[arg]) if e == "faster" else ROOM_EFFECT[e]} }},'
                  for rm, b, e, arg, r in rooms]
    B["dungeons"] = [f'    Dungeon {{ id: "{d}", region: {r}, stages: &[{", ".join(chr(34) + s + chr(34) for s in st)}] }},' for d, r, st in dungeons]
    B["champions"] = [f'    [{", ".join(chr(34) + c + chr(34) for c in row)}],' for row in champions]
    B["tributes"] = ["    [" + ", ".join(rs_needs(t) for t in row) + "],"for row in tributes]
    B["noble_ranks"] = []
    for r in (1, 2, 3):
        rs = [x for x in noble if x[5] == r]
        B["noble_ranks"].append("    [" + " ".join(f'NobleRank {{ id: "{i}", gold: {num(g)}, rooms: {rm}, income: {num(inc)}, servants: {s} }},' for i, g, rm, inc, s, _ in rs) + "],")
    B["trader_ranks"] = []
    for r in (1, 2, 3):
        rs = [x for x in trader if x[6] == r]
        B["trader_ranks"].append("    [" + " ".join(f'TraderRank {{ id: "{i}", level: {L}, orders: {o}, fee: {num(f)}, each_town: {e}, charter: {"true" if c else "false"} }},' for i, L, o, f, e, c, _ in rs) + "],")
    B["inventor_ranks"] = []
    for r in (1, 2, 3):
        rs = [x for x in inventor if x[6] == r]
        B["inventor_ranks"].append("    [" + " ".join(f'InventorRank {{ id: "{i}", level: {L}, workshop: {w}, machine: {str(m).lower()}, star_core: {str(s).lower()}, every_machine: {str(e).lower()} }},' for i, L, w, m, s, e, _ in rs) + "],")
    B["cosmic_ranks"] = []
    for r in (1, 2, 3):
        rs = [x for x in cosmic if x[5] == r]
        B["cosmic_ranks"].append("    [" + " ".join(f'CosmicRank {{ id: "{i}", level: {L}, starlight: {num(sl)}, stardust: {sd}, lenses: {le} }},' for i, L, sl, sd, le, _ in rs) + "],")
    B["skies"] = [f'    Sky {{ sign_ms: {m} * MINUTE, signs: [{", ".join(f"({chr(34)}{s}{chr(34)}, [{a}, {b}])" for s, a, b in signs)}] }},' for m, signs in skies]
    return B

def styles_blocks():
    """The whole held and spent lists, best first, Vale rows included."""
    vale = {
        "melee": [("giants_maul", 5), ("mithril_sword", 4), ("steel_sword", 3), ("iron_sword", 2), ("bronze_sword", 1)],
        "ranged": [("knights_longbow", 7), ("magic_shortbow", 6), ("yew_shortbow", 5), ("maple_shortbow", 4), ("willow_shortbow", 3), ("oak_shortbow", 2), ("shortbow", 1)],
        "magic": [("dragonfire_staff", 6)],
        "arrows": [("mithril_arrows", 4), ("steel_arrows", 3), ("iron_arrows", 2), ("bronze_arrows", 1)],
        "runes": [("astral_rune", 6), ("death_rune", 5), ("chaos_rune", 4), ("fire_rune", 3), ("water_rune", 2), ("air_rune", 1)],
    }
    def srt(l): return sorted(l, key=lambda x: -x[1])
    def fmt(l): return ["        " + ", ".join(f'("{i}", {t})' for i, t in l[k:k + 6]) + "," for k in range(0, len(l), 6)]
    return {"melee_held": fmt(srt(vale["melee"] + held["melee"])), "ranged_held": fmt(srt(vale["ranged"] + held["ranged"])),
            "magic_held": fmt(srt(vale["magic"] + held["magic"])), "ranged_spent": fmt(srt(vale["arrows"] + spent["ranged"])),
            "magic_spent": fmt(srt(vale["runes"] + spent["magic"]))}

def food_armour_blocks():
    vale_food = [("dragon_pepper_feast", 260), ("shark", 200), ("monkfish", 160), ("swordfish", 140), ("goblin_hot_pot", 120),
                 ("lobster", 120), ("farmers_stew", 90), ("salmon", 90), ("trout", 70), ("cooked_sweetcorn", 60), ("baked_potato", 50), ("shrimp", 30)]
    vale_armour = [("mossplate", 4, 0), ("dragonhide_body", 4, 0), ("silkweave_vest", 3, 0), ("silk_robe", 3, 0),
                   ("studded_leather", 2, 0), ("hard_leather_body", 2, 0), ("leather_body", 1, 0)]
    f = sorted(vale_food + food, key=lambda x: -x[1])
    a = sorted(vale_armour + armour, key=lambda x: (-x[1], -x[2]))
    return {"food": ["    " + ", ".join(f'("{i}", {h})' for i, h in f[k:k + 6]) + "," for k in range(0, len(f), 6)],
            "armour": ["    " + ", ".join(f'("{i}", {t}, {hp})' for i, t, hp in a[k:k + 5]) + "," for k in range(0, len(a), 5)]}

def write(check):
    src = CONTENT.read_text()
    blocks = {**gen_blocks(), **styles_blocks(), **food_armour_blocks()}
    new = src
    for name, rows in blocks.items():
        pat = re.compile(r"(?P<a>[ \t]*// <gen:" + name + r">\n).*?(?P<b>[ \t]*// </gen:" + name + r">)", re.S)
        if not pat.search(new):
            sys.exit(f"content.rs has no <gen:{name}> block")
        new = pat.sub(lambda m: m.group("a") + "".join(r + "\n" for r in rows) + m.group("b"), new)
    strings = json.loads(STRINGS.read_text())
    before = json.dumps(strings, sort_keys=True)
    for part in ("actions", "items", "enemies", "ui"):
        for k, v in names[part].items():
            strings[part].setdefault(k, v)
    changed = new != src or json.dumps(strings, sort_keys=True) != before
    if check:
        if changed:
            sys.exit("tools/content/regions.py would change content.rs or strings.json: run it and commit")
        print("region content is up to date")
        return
    CONTENT.write_text(new)
    STRINGS.write_text(json.dumps(strings, indent=1, ensure_ascii=False) + "\n")
    print(f"wrote {len(acts)} actions, {len(enemies)} enemies, {len(crops)} crops, {len(recipes)} recipes, "
          f"{len(towns)} towns, {len(blueprints)} blueprints, {len(machines)} machines for three regions")

if __name__ == "__main__":
    write("--check" in sys.argv)
