# Plan

Stage 2. The MVP, built on the default of every open question (BRIEF.md, Autonomy, 2026-10-25). Each decision names its reason and the test that shows it holds. The journal's Round 2 tab and Mockups tab are the pictures this plan follows (`mockups/v2.html`).

| # | Decision | Reason | Held by |
| --- | --- | --- | --- |
| P1 | Twelve skills: Woodcutting, Mining, Fishing gather; Firemaking, Fletching, Smithing, Cooking, Runecrafting make from what others gather; Melee, Ranged, Magic fight, with Hitpoints. | T1, T3; IG-Q15 default A | `content.rs` tests: every recipe's inputs are made by some action |
| P2 | RuneScape's experience table, levels 1 to 99, as integers in the core. | IG-2 | `xp.rs` tests against the published values |
| P3 | One action at a time; a plan of up to three steps runs in order; picking an action by hand clears the plan. | IG-Q10 | `game.rs` tests |
| P4 | Time enters as the page's clock in milliseconds. One rule credits it: watching, a background tab and a closed tab are the same. Away time counts up to 24 hours. | IG-3, IG-4, IG-Q3 | test: one 8-hour step equals 28,800 one-second steps |
| P5 | Away 10 minutes or longer: the return screen (gains, used, kills, unlocks, what ran out and who makes more, ready for you). Shorter: toasts. | IG-Q11, LR-9 | `game.rs` tests |
| P6 | Mastery per action to 99, four checkpoints each (10, 25, 50, 99). | IG-Q12 | tests on each checkpoint's effect |
| P7 | Firemaking lights a bonfire: each log adds 30 s, up to an hour ahead; gathering XP +10% while it burns. | IG-Q14 default B | test: boost on and off, independent of step size |
| P8 | Combat: one style at a time; each enemy weak to one style; a sword, a bow and arrows, or runes; food eaten below 45% HP; out of food means retreat with nothing lost. | IG-Q16 A, IG-Q17 A | `combat` tests |
| P9 | Prestige at total level 600, +25 per prestige; +25% skill XP per prestige; keeps mastery and the multiplier. | IG-Q18 A, IG-Q19 A, IG-Q20 A | tests |
| P10 | Gold from selling; tools (axe, pickaxe, rod) in six tiers make one skill faster. | IG-Q13 | tests |
| P11 | Save: the core writes a versioned save; `web/save.js` stores it in `localStorage` every few seconds and on leaving; export and import a `.json` file; an unreadable save starts a new game and is kept, not overwritten. Change to D17. | IG-5, IG-6 | `no_storage.rs` becomes a fence; save tests |
| P12 | Playtest file: the core keeps a log (levels, unlocks, returns, run-outs, purchases, prestiges, screens opened) and the player's flagged moments (fun, pace, a note). "Send playtest file" shares it from a phone or downloads it. Nothing is sent by the page. | the person's request, 2026-10-25 | playtest tests; browser check sees the download |
| P13 | The core describes every screen (`ui`): blocks the page draws by kind. The page holds no number, and re-draws only when the core says something changed or an action completes; bars and the swing animate in CSS. | Builder D3; IG-8 | `boundary.rs`; browser check |
| P14 | Mobile kit: manifest, home-screen icon, touch baseline, a fullscreen button where the browser has fullscreen and home-screen steps on iPhone; the browser check adds iPhone and iPad passes. Changes D4 and D7. | IG-10, IG-Q4 | browser check |
| P15 | 2D art: one SVG sprite of icons, drawn from shapes; no WebGL. | IG-Q5 default A | review |
| P16 | Working name "Idle Game" until IG-Q1 is answered. | IG-Q1 | — |

Not in the MVP: Crafting and Farming (IG-Q15 C), a mastery pool, more than three plan steps, sound.

## Plan 2: recipes and four engines (2026-10-25; answered in the journal and built the same day)

Stage 2 for T8 to T14. The design is on the journal's Engines tabs, with diagrams, mockups and twelve questions (IG-Q24 to IG-Q35). **Not approved in advance: nothing is built until the person has read it.**

The person answered on 2026-10-25: "all of the design decisions are in the design log as choices in each tab, please take those decisions and build it out." Every tapped answer was the default; the four left untapped (IG-Q22 to IG-Q25) take their defaults (R7.1).

| # | Decision | Reason | Held by (once built) |
| --- | --- | --- | --- |
| P17 | Ashes are compost: planting uses one if you have any, and that crop grows 25% faster. | T8, IG-Q21 B | a farming test and a break row |
| P18 | A plan holds four steps. | T9 | `PLAN_STEPS`; a plan test |
| P19 | Nine recipes drop from enemies (1 in 64 to 1 in 512) and unlock unique weapons, armour and food that need two or three skills' goods. | T10 | content tests: every recipe's inputs are made somewhere |
| P20 | Four engines (Trader, Nobility, Inventor, Cosmic), each with a ladder, one timer alongside the action, and something to collect; one new skill each (Trading, Construction, Invention, Attunement). | T11 to T14 | a content test: no skill takes from or feeds more than three |
| P21 | Navigation becomes five groups with a strip of skills or engines. | 18 skills and 5 engines do not fit 16 tabs | browser check |
| P22 | Build order and prestige behaviour follow IG-Q24 and IG-Q25: recipes, Trader, Nobility, Inventor, Cosmic; the engines start over at prestige and recipes stay (IG-Q26 A). | the person's | tests `recipes_are_kept_through_prestige_and_engines_are_not` |

## Plan 3: the full game (2026-10-25; answered in the journal and built the same day)

Stage 2 for T15: four times the length and content of every skill and engine. The design is on the journal's Full game tabs, with ten questions (IG-Q36 to IG-Q45). **Not approved in advance: nothing is built until the person has read it.** The rows below assume each question's default.

| # | Decision | Reason | Held by (once built) |
| --- | --- | --- | --- |
| P23 | Content rows carry a region; content tests run per region. | four regions (IG-Q38) | content tests |
| P24 | Save version 4: a game in progress carries over into the first region. | IG-5 | a migration test |
| P25 | The Coast first, as the template: its skill rows, enemies, champions, recipes, crops, towns and engine tier. | IG-Q45 | content tests; a playtest |
| P26 | The run to the first prestige about four times today's: prestige at total level 900. | IG-Q36, IG-Q37 (changes IG-Q19's 600) | a pace test from the model; playtest files |
| P27 | The bank grouped by skill with a search box; skill screens for four times the rows. | IG-Q41 | browser check |
| P28 | One new mechanic for each skill, and a dungeon in each new region. | IG-Q40, IG-Q42 | a test and a break row each |
| P29 | The Peaks, then the Wastes, each from a written list the person has read. | IG-Q39 | as P25 |
| P30 | A balance pass over all four regions from playtest files. | R7.6 | readout tool; triage rows |

The person answered on 2026-10-25: "the answers are in the log, build the full game". The tapped answers were all defaults, and the three left untapped (IG-Q36 to IG-Q38) take theirs (R9.1). All three regions were built at once (R9.2).
