# Handoff

Stage 9, written by the agent. `C5 Consolidate in your own words`. A fresh session starts from this file, not from memory.

## What this is

An ad-free idle game with twelve RuneScape-like skills that feed each other, combat in three styles, a three-step plan, prestige, and a playtest file. Static files, built from a Rust core; played in a phone, tablet or desktop browser. The MVP is built on the default of every open question (BRIEF.md, Autonomy). Not published yet.

## Rules that hold it up

The rules that break silently if a later change ignores them, and the test that guards each.

## Shape of the code

Where each kind of thing lives, in a short table.

## Commands

`./scripts/check.sh` (prints BUILDER CHECK PASSED); `cargo test`; `./scripts/build.sh` then `python3 scripts/serve.py` to play locally; `python3 tools/icons/icons.py` after adding content.

## What will bite within the hour

What a new session gets wrong first.

## The journal

`../idle-game-journal/index.html`, published privately at https://claude.ai/artifact/Sd1T2jh5mG3qaHbZQEA67N (republish from the same file to keep the link). Newest entry: the first mockups (2026-10-25).

## Open for the person

Every `SECOND-ORDER.md` row with status *person*, by number, with what the person has to decide or supply. `scripts/notebook_check.py` fails while one is missing.

- R1.4: answer IG-Q1 to IG-Q9 in the journal's opening entry.
- R1.5: approve or replace the agent-written skill, tree and item names in the mockups.
- R1.6: answer IG-Q10 to IG-Q13 in the journal's Mockups entry.
- R2.1, R2.2, R2.3, R2.4, R2.5, R2.6: answer IG-Q14 to IG-Q20 on the journal's Round 2 tab (one tap per card).
- R3.8: tuning numbers, to judge from playtest files.
- R3.9: approve the content names and icons.
- R3.10: real-device checks on a phone.
- R3.11: name a repository and grant publishing so the co-designer can play.

## Mistakes so far

By system, from `SECOND-ORDER.md`, with the rule each one produced.

## What passed

The last lines of `./scripts/check.sh` and `./scripts/count-tests.sh`, with the date and commit.

## The next action

Publish to GitHub Pages once the person names the repository (R3.11), then read the first playtest file.
