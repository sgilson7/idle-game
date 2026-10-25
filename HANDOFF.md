# Handoff

Stage 9, written by the agent. `C5 Consolidate in your own words`. A fresh session starts from this file, not from memory.

## What this is

An ad-free idle game with eighteen RuneScape-like skills that feed each other, combat in three styles with recipes from enemies, four engines beside it (the Trader, the estate, the Inventor's workshop and the observatory's turning sky), farming with compost, a four-step plan, prestige, and a playtest file. Static files, built from a Rust core; played in a phone, tablet or desktop browser, and published at https://sgilson7.github.io/idle-game/.

## Rules that hold it up

- Time is credited by one rule (`State::advance`): it moves to whichever timer completes first (your action, servants, machines, research, caravans, the estate's hour), does everything due in a fixed order, and repeats; a split run equals a whole one (`one_long_step_equals_many_short_ones`, `engines_running_together_give_the_same_result_however_time_is_split`).
- Engines open by total level (150, 250, 350, 450); the Inventor and the sky also need their estate rooms. Prestige resets them; recipes stay.
- No new skill takes from or feeds more than three (`no_new_skill_takes_from_or_feeds_more_than_three`, R7.3).
- An action that cannot start ends before its time is spent (`a_plan_runs_its_steps_in_order`).
- Away time counts for 24 h at most (36 h and 48 h at Cosmic ranks); the return screen waits for 10 min away (`api.rs` tests).
- Only `web/save.js` names storage (`no_storage.rs`); an unreadable save starts over and is kept.
- The page holds no number and no rule (`boundary.rs`).
- `tests/breaks.json`: 60 of 60 breaks caught.

## Shape of the code

| Where | What |
| --- | --- |
| `crates/core/src/content.rs` | every tuning number and the content table |
| `crates/core/src/game.rs` | state, time, actions, combat, plan, prestige |
| `crates/core/src/engines.rs` | recipes, the Trader, the estate, the workshop, the sky |
| `crates/core/src/view.rs` | the screens as blocks; the five groups and the tabs |
| `crates/core/src/view_engines.rs` | the recipe book and the four engine screens |
| `crates/core/src/api.rs` | the `Game` the shim holds; save, import, export, playtest file |
| `data/strings.json`, `GLOSSARY.md` | every word |
| `web/app.js`, `web/save.js` | drawing; the one storage door |
| `tools/icons/icons.py` | writes `web/icons.svg` |
| `tools/playtest/readout.py` | reads a playtest file into a Markdown readout |

## Commands

`./scripts/check.sh` (prints BUILDER CHECK PASSED); `cargo test`; `./scripts/build.sh` then `python3 scripts/serve.py` to play locally; `python3 tools/icons/icons.py` after adding content.

## What will bite within the hour

- Add content in `content.rs` and its name in `data/strings.json`, then run `python3 tools/icons/icons.py`; the browser check fails on a missing icon.
- A key chosen by a condition (`weak_to` / `weak_to_yes`) escapes a simple key scan; `every_screen_and_sheet_finds_all_its_words` catches it.
- Page JavaScript may hold no number but 0 and 1 and no template literal.
- Playtest intake: save the friend's file, then `python3 tools/playtest/readout.py <file>`; a flag becomes a TRIAGE row only when the person says so.

## The journal

Live game: https://sgilson7.github.io/idle-game/ (repository sgilson7/idle-game).

`../idle-game-journal/index.html`, published privately at https://claude.ai/artifact/Sd1T2jh5mG3qaHbZQEA67N (republish from the same file to keep the link). Newest entries: Engines built and the Full game plan (2026-10-25). Answers land in the page's `answers` collection; read them with the artifact database before building.

## Open for the person

Every `SECOND-ORDER.md` row with status *person*, by number, with what the person has to decide or supply. `scripts/notebook_check.py` fails while one is missing.

- R1.4: answer IG-Q1 to IG-Q9 in the journal's opening entry.
- R1.5: approve or replace the agent-written skill, tree and item names in the mockups.
- R1.6: answer IG-Q10 to IG-Q13 in the journal's Mockups entry.
- R2.1, R2.2, R2.3, R2.4, R2.5, R2.6: answer IG-Q14 to IG-Q20 on the journal's Round 2 tab (one tap per card).
- R3.8: tuning numbers, to judge from playtest files.
- R3.9: approve the content names and icons.
- R3.10: real-device checks on a phone.
- R3.11: granted 2026-10-25 (sgilson7/idle-game).
- R3.12: confirm the MIT licence.
- R5.2, R5.3: the co-designer checks the Farming and Crafting designs.
- R5.5: answered IG-Q21 B (compost); built as T8.
- R7.6: every engine number is a first guess; judge from playtest files.
- R7.11: approve or replace the engine names, strings and icons.
- R8.1, R8.2, R8.3: review the full-game plan and answer IG-Q36 to IG-Q45 (journal, Full game tabs). Nothing of Plan 3 is built until then.

## Mistakes so far

By system, from `SECOND-ORDER.md`, with the rule each one produced.

## What passed

2026-10-25 (engines, T8 to T14): published from `eb4c567` with `scripts/publish.sh`; the browser check against https://sgilson7.github.io/idle-game passed in all five engines (one earlier publish attempt stopped on a page-load wait, R8.4). `./scripts/check.sh` BUILDER CHECK PASSED (browser check in chromium, firefox, webkit, webkit-iphone, webkit-ipad); `./scripts/count-tests.sh` 124 passed, 0 failed; breaks 60 of 60.

2026-10-25: `./scripts/check.sh` BUILDER CHECK PASSED (browser check in chromium, firefox, webkit, webkit-iphone, webkit-ipad); `./scripts/count-tests.sh` 80 passed, 0 failed; breaks 12 of 12; browser check against https://sgilson7.github.io/idle-game passed in all five.

## The next action

Read the answers to IG-Q36 to IG-Q45, then build Plan 3 in the order IG-Q45 picks; read the first playtest file when it arrives.

## 2026-10-25 (engines)

> all of the design decisions are in the design log as choices in each tab, please take those decisions and build it out. then, I want to plan to convert from MVP to full game by quadrupaling the length and content of all skills and engines

The agent decided: the eight tapped answers (IG-Q26 to IG-Q35, all defaults) and the defaults of the four left untapped (IG-Q22 to IG-Q25) are the person's decisions (R7.1); Plan 2 is approved and built as T8 to T14. The full-game plan (T15) is written for review in the journal before any code, as Plan 3.

## YYYY-MM-DD

> The person's words.

The agent decided:
