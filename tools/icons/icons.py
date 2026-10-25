#!/usr/bin/env python3
"""Write web/icons.svg: one <symbol> per skill, tab, action, item and enemy.

Flat shapes, drawn from a few rules (a log, an ore, a fish, a bar, a bow ...)
and a colour per material. The page shows a symbol with <use href="icons.svg#id">.
Run it after adding content; tests/browser/interactions.py checks that every
icon the core names exists here.

    python3 tools/icons/icons.py
"""
import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[2]
STR = json.loads((ROOT / "data" / "strings.json").read_text())

MAT = {"bronze": "#c8743a", "iron": "#8a8f9a", "steel": "#b8c2d6", "mithril": "#4a5fb8", "adamant": "#3f8a5a",
       "adamantite": "#3f8a5a", "rune": "#4ab8c8", "runite": "#4ab8c8", "gold": "#e8b42a", "copper": "#c8743a",
       "tin": "#a9aeb8", "coal": "#2a2a30", "essence": "#c9c3e8"}
WOOD = {"tree": "#4f8a3c", "logs": "#4f8a3c", "oak": "#6a9a3a", "willow": "#8ab04a", "teak": "#b07a3a", "maple": "#d2632e",
        "mahogany": "#8e3b2a", "yew": "#2f6a4a", "magic": "#5b7be0", "redwood": "#b5302a"}
FISH = {"shrimp": "#e88a7a", "trout": "#a8b87a", "salmon": "#e8907a", "lobster": "#c8402a", "swordfish": "#6a8ac8",
        "monkfish": "#8a7a6a", "shark": "#8a9aa8"}
RUNE = {"air": "#d8e2ee", "water": "#4f8ef7", "fire": "#f08a2a", "chaos": "#c84ac8", "death": "#e8e8e8"}
ENEMY = {"chicken": "#f2efe6", "cow": "#d8cbb8", "goblin": "#6aa84f", "spider": "#4a4a5a", "hill_giant": "#a07a5a",
         "black_knight": "#2a2a34", "moss_giant": "#5a8a3a", "green_dragon": "#2f9a4a"}

def first(word, table, default="#9aa4b8"):
    for k, v in table.items():
        if k in word:
            return v
    return default

def tree(c): return f'<rect x="17" y="22" width="6" height="14" fill="#6b4a2a"/><circle cx="20" cy="15" r="11" fill="{c}"/><circle cx="13" cy="19" r="7" fill="{c}"/><circle cx="27" cy="19" r="7" fill="{c}"/>'
def log(c): return f'<rect x="5" y="15" width="30" height="11" rx="5.5" fill="{c}" stroke="#2a1a0a"/><ellipse cx="33" cy="20.5" rx="3.5" ry="5.5" fill="#e8c890" stroke="#2a1a0a"/>'
def rock(c, dark=False): return f'<path d="M5 31l5-15 11-7 13 7 3 15z" fill="{"#2a2a30" if dark else "#5a5f6e"}" stroke="#1a1a20"/><circle cx="16" cy="21" r="3.2" fill="{c}"/><circle cx="26" cy="25" r="2.8" fill="{c}"/><circle cx="22" cy="16" r="2.2" fill="{c}"/>'
def fish(c, cooked=False):
    body = "#b8763a" if cooked else c
    return f'<path d="M5 20c6-9 19-9 25 0-6 9-19 9-25 0z" fill="{body}" stroke="#222"/><path d="M30 20l7-7v14z" fill="{body}" stroke="#222"/><circle cx="11" cy="18.5" r="1.6" fill="#111"/>'
def water(): return '<path d="M3 26q6-5 11 0t11 0 12 0v9H3z" fill="#2a5a8a"/><path d="M8 20q4-3 8 0" stroke="#9fd0ff" fill="none"/>'
def bar(c): return f'<path d="M5 27l6-11h24l-6 11z" fill="{c}" stroke="#222"/><path d="M11 16h24l-1 2H12z" fill="#fff" opacity=".25"/>'
def bow(c): return f'<path d="M13 4c15 6 15 26 0 32" fill="none" stroke="{c}" stroke-width="4" stroke-linecap="round"/><path d="M13 4v32" stroke="#eee" stroke-width="1.2"/>'
def arrow(c, shaft=False, head_only=False):
    out = '' if head_only else '<path d="M7 33L31 9" stroke="#c8a070" stroke-width="2.6"/>'
    if not shaft: out += f'<path d="M33 7l-9 1.5 7.5 7.5z" fill="{c}" stroke="#222" stroke-width=".6"/>'
    if not shaft and not head_only: out += '<path d="M7 33l2-8M7 33l8-2" stroke="#eee" stroke-width="2.2"/>'
    return out
def sword(c): return f'<path d="M9 31L30 10l3-5-5 3L7 29" fill="{c}" stroke="#222"/><path d="M6 24l10 10" stroke="#8a6a4a" stroke-width="3.5" stroke-linecap="round"/>'
def rune(c): return f'<circle cx="20" cy="20" r="14" fill="#5a5f70" stroke="#222"/><circle cx="20" cy="20" r="7" fill="{c}"/>'
def enemy(c): return f'<ellipse cx="20" cy="24" rx="14" ry="11" fill="{c}" stroke="#111"/><circle cx="15" cy="21" r="2.2" fill="#111"/><circle cx="25" cy="21" r="2.2" fill="#111"/><path d="M15 29q5 3 10 0" stroke="#111" fill="none"/>'
CROP = {"potato": "#c8a060", "onion": "#e8d0b0", "cabbage": "#7ac85a", "tomato": "#e04a3a", "sweetcorn": "#f0c830",
        "strawberry": "#e83a5a", "watermelon": "#3a9a4a"}
def produce(c, name):
    if name == "watermelon": return f'<ellipse cx="20" cy="22" rx="15" ry="11" fill="{c}" stroke="#123"/><path d="M8 22q12-6 24 0" stroke="#9ad89a" fill="none"/>'
    if name == "sweetcorn": return f'<ellipse cx="20" cy="20" rx="7" ry="14" fill="{c}" stroke="#753"/><path d="M13 30q-6-8 2-20M27 30q6-8-2-20" stroke="#5a9a3a" stroke-width="3" fill="none"/>'
    if name == "strawberry": return f'<path d="M20 35C8 26 9 14 20 14s12 12 0 21z" fill="{c}" stroke="#611"/><path d="M14 12l6 4 6-4-6-6z" fill="#5a9a3a"/>'
    return f'<circle cx="20" cy="22" r="12" fill="{c}" stroke="#333"/><path d="M20 10l-3-5m3 5l4-4" stroke="#5a9a3a" stroke-width="2.5"/>'
def seeds(c): return f'<path d="M9 12h22l-2 22H11z" fill="#c8a878" stroke="#5a4a2a"/><path d="M9 12q11-6 22 0" fill="#b0905a" stroke="#5a4a2a"/><circle cx="20" cy="24" r="5" fill="{c}" stroke="#333"/>'
def body(c): return f'<path d="M12 6l-8 8 4 5 4-3v19h16V16l4 3 4-5-8-8-4 3h-8z" fill="{c}" stroke="#222"/>'
def fire(): return '<path d="M20 4c4 7 10 10 10 18a10 10 0 0 1-20 0c0-6 4-8 5-13 2 4 3 5 5 5-1-4-1-7 0-10z" fill="#f08a2a"/><path d="M20 18c2 3 5 5 5 9a5 5 0 0 1-10 0c0-2 2-4 3-6z" fill="#ffd36a"/>'

GENERIC = {
    "feathers": '<path d="M9 35C13 18 22 8 33 5c-2 13-11 24-24 30z" fill="#f2efe6" stroke="#999"/>',
    "bones": '<path d="M10 29l19-17" stroke="#e8e2d0" stroke-width="5" stroke-linecap="round"/><circle cx="9" cy="27" r="3" fill="#e8e2d0"/><circle cx="31" cy="12" r="3" fill="#e8e2d0"/>',
    "big_bones": '<path d="M8 31l23-21" stroke="#e8e2d0" stroke-width="7" stroke-linecap="round"/>',
    "dragon_bones": '<path d="M8 31l23-21" stroke="#c8e8c8" stroke-width="7" stroke-linecap="round"/>',
    "cowhide": '<path d="M7 9h26l-3 22H10z" fill="#e8e0d0" stroke="#555"/><circle cx="16" cy="17" r="4" fill="#333"/><circle cx="25" cy="24" r="3" fill="#333"/>',
    "spider_silk": '<circle cx="20" cy="20" r="11" fill="#eeeef6" stroke="#888"/><path d="M12 14q8 6 16 0M12 26q8-6 16 0" stroke="#999" fill="none"/>',
    "dragon_scale": '<path d="M20 5l12 9-4 18H12L8 14z" fill="#2f9a4a" stroke="#123"/>',
    "bird_nest": '<ellipse cx="20" cy="26" rx="15" ry="7" fill="#8a6a3a" stroke="#4a3a1a"/><circle cx="15" cy="22" r="4" fill="#d8e2ee"/><circle cx="23" cy="21" r="4" fill="#cfe0c8"/>',
    "ashes": '<ellipse cx="20" cy="27" rx="13" ry="6" fill="#777"/><ellipse cx="20" cy="24" rx="8" ry="4" fill="#999"/>',
    "arrow_shafts": arrow("#c8a070", shaft=True),
    "leather": '<path d="M7 9l26 2-3 20-22 1z" fill="#a0703a" stroke="#4a2a1a"/>',
    "leather_gloves": '<path d="M10 34V14l3-6 3 6V8l3-2 3 2v6l3-5 3 3v25z" fill="#a0703a" stroke="#4a2a1a"/>',
    "silk_cloth": '<path d="M6 10h28v20H6z" fill="#eeeef6" stroke="#888"/><path d="M6 16h28M6 22h28" stroke="#ccd"/>',
    "gold_ring": '<circle cx="20" cy="22" r="10" fill="none" stroke="#e8b42a" stroke-width="5"/><circle cx="20" cy="11" r="4" fill="#7ad8f0"/>',
    "gold_amulet": '<path d="M8 6q12 22 24 0" fill="none" stroke="#e8b42a" stroke-width="2.5"/><circle cx="20" cy="27" r="7" fill="#e8b42a" stroke="#9c6a00"/><circle cx="20" cy="27" r="3" fill="#e04a6a"/>',
    "leather_body": body("#a0703a"), "hard_leather_body": body("#6a4a2a"), "silk_robe": body("#c8c8e8"), "dragonhide_body": body("#2f9a4a"),
    "baked_potato": '<ellipse cx="20" cy="23" rx="14" ry="10" fill="#8a5a2a" stroke="#3a2a1a"/><path d="M12 20q8 6 16 0" stroke="#f0e0a0" stroke-width="4" fill="none"/>',
    "cooked_sweetcorn": '<ellipse cx="20" cy="20" rx="7" ry="14" fill="#e8a020" stroke="#753"/><path d="M15 12h10M14 18h12M14 24h12M15 30h10" stroke="#a06010"/>',
}
UI = {
    "woodcutting": '<path d="M7 34l17-17" stroke="#c8a070" stroke-width="4" stroke-linecap="round"/><path d="M22 6c7 0 12 5 12 10l-10 2-5-5z" fill="#b8c2d6" stroke="#222"/>',
    "mining": '<path d="M8 34l18-18" stroke="#c8a070" stroke-width="4" stroke-linecap="round"/><path d="M6 13c9-9 20-9 28 0-8-4-17-4-28 0z" fill="#b8c2d6" stroke="#222"/>',
    "fishing": '<path d="M6 34L30 6" stroke="#c8a070" stroke-width="3"/><path d="M30 6v18a3 3 0 1 1-6 0" stroke="#9fb6e0" stroke-width="2" fill="none"/>',
    "firemaking": fire(),
    "fletching": bow("#c8a070") + '<path d="M5 20h28" stroke="#b8c2d6" stroke-width="2"/>',
    "smithing": '<path d="M4 14h20l6 6H11v5H4z" fill="#9aa4b8" stroke="#222"/><rect x="12" y="25" width="10" height="9" fill="#6a7488"/>',
    "cooking": '<ellipse cx="18" cy="24" rx="14" ry="7" fill="#6a7488" stroke="#222"/><path d="M32 23h6" stroke="#9aa4b8" stroke-width="3"/><path d="M13 15c0-3 3-3 3-7M20 15c0-3 3-3 3-7" stroke="#cfd6e6" stroke-width="2" fill="none"/>',
    "crafting": '<path d="M8 32L26 14" stroke="#c8a070" stroke-width="3"/><path d="M24 6l10 10-4 4-10-10z" fill="#b8c2d6" stroke="#222"/><path d="M6 24q6-4 12 2" stroke="#a0703a" stroke-width="4" fill="none"/>',
    "farming": '<path d="M20 35V16" stroke="#5a9a3a" stroke-width="3"/><path d="M20 22c-10 0-13-8-13-12 8 0 13 4 13 12zM20 18c8 0 12-6 12-10-7 0-12 3-12 10z" fill="#6ac84a" stroke="#2a5a1a"/><path d="M6 35h28" stroke="#6b4a2a" stroke-width="4"/>',
    "patch": '<path d="M3 26l7-10h20l7 10z" fill="#6b4a2a" stroke="#3a2a1a"/><path d="M8 22h24M6 25h28" stroke="#4a3018"/>',
    "runecrafting": '<path d="M20 3l14 8v18l-14 8-14-8V11z" fill="#5b4ab8" stroke="#222"/><path d="M20 11v18M13 15l14 10M27 15l-14 10" stroke="#d8e2ee" stroke-width="2"/>',
    "combat": '<path d="M6 34L28 12l2-7-7 2L4 30" fill="#d8dee8" stroke="#222"/><path d="M34 34L12 12l-2-7 7 2 19 23" fill="#c8a070" stroke="#222"/>',
    "melee": sword("#d8dee8"),
    "ranged": bow("#c8a070") + '<path d="M4 20h30" stroke="#b8c2d6" stroke-width="2"/>',
    "magic": '<path d="M8 35L25 12" stroke="#8a6a4a" stroke-width="3.5" stroke-linecap="round"/><circle cx="27" cy="10" r="7" fill="#7aa8ff"/><circle cx="27" cy="10" r="3" fill="#e8f0ff"/>',
    "hitpoints": '<path d="M20 35C8 26 4 21 4 14a8 8 0 0 1 16-2 8 8 0 0 1 16 2c0 7-4 12-16 21z" fill="#d9534f" stroke="#611"/>',
    "bank": '<rect x="5" y="14" width="30" height="20" rx="3" fill="#7a5a3a" stroke="#3a2a1a"/><rect x="5" y="9" width="30" height="8" rx="3" fill="#9a7448" stroke="#3a2a1a"/><rect x="17" y="19" width="6" height="6" fill="#f0a920"/>',
    "shop": '<path d="M6 15l4-9h20l4 9z" fill="#c84a4a" stroke="#222"/><rect x="8" y="15" width="24" height="19" fill="#9aa4b8" stroke="#222"/><rect x="17" y="22" width="6" height="12" fill="#5a6478"/>',
    "plan": '<rect x="7" y="5" width="26" height="31" rx="3" fill="#cfd6e6" stroke="#222"/><path d="M12 13h16M12 20h16M12 27h10" stroke="#3d5bb8" stroke-width="2.5"/>',
    "prestige": '<path d="M20 3l5 10 11 2-8 8 2 11-10-5-10 5 2-11-8-8 11-2z" fill="#e86fb0" stroke="#612"/>',
    "settings": '<circle cx="20" cy="20" r="6" fill="none" stroke="#9aa4b8" stroke-width="3"/><path d="M20 4v6M20 30v6M4 20h6M30 20h6M8.7 8.7l4.2 4.2M27.1 27.1l4.2 4.2M8.7 31.3l4.2-4.2M27.1 12.9l4.2-4.2" stroke="#9aa4b8" stroke-width="3"/>',
    "flag": '<path d="M9 36V5" stroke="#cfd6e6" stroke-width="3"/><path d="M10 6h20l-5 7 5 7H10z" fill="#f0a920" stroke="#222"/>',
    "fullscreen": '<path d="M6 15V6h9M34 15V6h-9M6 25v9h9M34 25v9h-9" stroke="#cfd6e6" stroke-width="3" fill="none"/>',
    "send": '<path d="M5 20L35 6 27 34l-7-11z" fill="#46c47a" stroke="#123"/><path d="M20 23l15-17" stroke="#123"/>',
    "save": '<rect x="6" y="6" width="28" height="28" rx="3" fill="#3d5bb8" stroke="#123"/><rect x="11" y="6" width="16" height="10" fill="#cfd6e6"/><rect x="11" y="22" width="18" height="12" fill="#e8ecf5"/>',
    "reset": '<path d="M30 14A12 12 0 1 0 32 22" stroke="#cfd6e6" stroke-width="3.5" fill="none"/><path d="M33 5v10H23" stroke="#cfd6e6" stroke-width="3.5" fill="none"/>',
    "gold": '<circle cx="20" cy="20" r="13" fill="#f0c040" stroke="#9c6a00" stroke-width="2"/><circle cx="20" cy="20" r="8" fill="none" stroke="#c89010" stroke-width="2"/>',
}

def item_icon(i):
    if i in GENERIC: return GENERIC[i]
    if i.endswith("_seed"): return seeds(CROP[i[:-5]])
    if i in CROP: return produce(CROP[i], i)
    if i.endswith("_logs") or i == "logs": return log(first(i, WOOD))
    if i.endswith("_ore") or i in ("coal", "rune_essence"): return rock(first(i, MAT), dark=(i == "coal"))
    if i.startswith("raw_"): return fish(first(i, FISH))
    if i in FISH: return fish(FISH[i], cooked=True)
    if i.endswith("_bar"): return bar(first(i, MAT))
    if "bow" in i: return bow(first(i, WOOD, "#c8a070"))
    if i.endswith("arrowheads"): return arrow(first(i, MAT), head_only=True)
    if i.endswith("arrows"): return arrow(first(i, MAT))
    if i.endswith("sword"): return sword(first(i, MAT))
    if i.endswith("rune"): return rune(first(i, RUNE))
    raise SystemExit(f"no icon rule for item {i}")

def action_icon(a, made):
    if a in WOOD or a in ("magic_tree", "redwood", "mahogany", "teak"): return tree(first(a.replace("_tree", ""), WOOD))
    if a.endswith("_spot"): return water() + f'<g transform="translate(4 -6) scale(.8)">{fish(first(a, FISH))}</g>'
    if a in ("essence", "copper", "tin", "iron", "coal", "gold_rock", "mithril", "adamantite", "runite"):
        return rock(first(a.replace("_rock", ""), MAT), dark=(a == "coal"))
    if a.startswith("burn_"): return f'<g transform="translate(0 10) scale(1 .7)">{log(first(a, WOOD))}</g>' + fire().replace('fill="#f08a2a"', 'fill="#f08a2a" opacity=".9"', 1)
    return item_icon(made)

sym = {}
sym.update(UI)
src = (ROOT / "crates" / "core" / "src" / "content.rs").read_text()
for a, made in re.findall(r'(?:gather|make)\("([a-z_]+)",.*?"([a-z_]+)"(?:\)|, \d+\)\),)', src):
    pass
acts = re.findall(r'^\s*(gather|make)\("([a-z_]+)"(.*)$', src, re.M)
for kind, a, rest in acts:
    made = re.findall(r'"([a-z_]+)"', rest)[-1]
    sym[a] = action_icon(a, made)
for i in STR["items"]:
    sym.setdefault(i, item_icon(i))
for crop in CROP:
    sym[crop] = produce(CROP[crop], crop)
for e in STR["enemies"]:
    sym[e] = enemy(ENEMY[e])
for s in STR["skills"]:
    if s not in sym: raise SystemExit(f"no icon for skill {s}")
body = "\n".join(f'<symbol id="{k}" viewBox="0 0 40 40">{v}</symbol>' for k, v in sorted(sym.items()))
(ROOT / "web" / "icons.svg").write_text(f'<svg xmlns="http://www.w3.org/2000/svg">\n{body}\n</svg>\n')
print(f"wrote web/icons.svg with {len(sym)} symbols")
