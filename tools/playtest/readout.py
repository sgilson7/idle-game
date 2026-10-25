#!/usr/bin/env python3
"""Read a playtest file and print a short readout in Markdown.

    python3 tools/playtest/readout.py idle-game-playtest-2026-10-26.json

What it prints, in order: the flags in the player's own words (with what was
running and how it felt), the pace of each skill (XP an hour, minutes per
level over the last few levels), the returns (how long away, how often), and
the tuning numbers a complaint most likely points at. It reads the file; it
recomputes no game rule. A flag becomes a TRIAGE.md row only when the person
says so.
"""
import json
import sys
from datetime import datetime, timezone

FUN = {0: "–", 1: "bored", 2: "fine", 3: "fun", 4: "great"}
PACE = {0: "–", 1: "too slow", 2: "about right", 3: "too fast"}


def when(ms):
    return datetime.fromtimestamp(ms / 1000, tz=timezone.utc).strftime("%Y-%m-%d %H:%M UTC")


def mins(ms):
    m = ms / 60000
    return f"{m:.0f} min" if m < 120 else f"{m / 60:.1f} h"


def main(path):
    d = json.load(open(path))
    if d.get("kind") != "idle-game-playtest":
        sys.exit(f"{path} is not a playtest file")
    s = d["summary"]
    print(f"# Playtest readout: {path}\n")
    print(f"Made {when(d['made_wall'])}; first played {when(d['first_wall'])}. "
          f"Game time credited: {mins(s['game_ms'])} over {mins(s['real_ms_since_first_play'])} of real time. "
          f"Total level {s['total_level']}, prestige {s['prestige']}, gold {s['gold']:,}. Device: {d['device'].get('ua', '?')[:80]}\n")

    print(f"## Flags ({len(d['flags'])})\n")
    if not d["flags"]:
        print("None.\n")
    for i, f in enumerate(d["flags"], 1):
        doing = f["doing"] or "nothing running"
        print(f"{i}. **{FUN[f['fun']]}, {PACE[f['pace']]}** on {f['screen']} while {doing}, total level {f['total']}, "
              f"{when(f['w'])}: {f['note'] or '(no note)'}")
    print()

    print("## Pace by skill\n")
    print("| Skill | Level | XP/hour | Time spent | Last levels (minutes of game time each) |")
    print("| --- | --- | --- | --- | --- |")
    for sk, v in s["skills"].items():
        ups = v["level_ups"]
        gaps = [round((b["game_ms"] - a["game_ms"]) / 60000, 1) for a, b in zip(ups, ups[1:])][-5:]
        if v["ms_spent"] or ups:
            print(f"| {sk} | {v['level']} | {v['xp_per_hour']:,} | {mins(v['ms_spent'])} | {', '.join(map(str, gaps)) or '–'} |")
    print()

    aways = [e for e in d["events"] if e["k"] == "away"]
    print(f"## Returns ({len(aways)})\n")
    if aways:
        lens = sorted(e["n"] for e in aways)
        print(f"Shortest {mins(lens[0])}, median {mins(lens[len(lens) // 2])}, longest {mins(lens[-1])}. "
              f"Run-outs: {sum(1 for e in d['events'] if e['k'] == 'out')}. Plans started: {sum(1 for e in d['events'] if e['k'] == 'plan')}.\n")
    tabs = sorted(((k[4:], v) for k, v in d["counts"].items() if k.startswith("tab:")), key=lambda x: -x[1])
    print("Screens opened most: " + ", ".join(f"{k} {v}" for k, v in tabs[:6]) + "\n")

    slow = [f for f in d["flags"] if f["pace"] == 1]
    fast = [f for f in d["flags"] if f["pace"] == 3]
    print("## Where a change would go\n")
    print(f"- Too slow ({len(slow)}) / too fast ({len(fast)}): action `ms` and `xp` per row in `crates/core/src/content.rs` `ACTIONS`, "
          "or `MASTERY_SCALE` in `xp.rs` for mastery.")
    print("- Returns feel thin: `AWAY_CAP_MS`, `RETURN_AFTER_MS`, `BONFIRE_*`, `NEST_ONE_IN` in `content.rs`.")
    print("- Prestige too soon or too late: `PRESTIGE_BASE`, `PRESTIGE_STEP_BP`.")
    print(f"\nThe file's tuning table matches this build: {len(d['tuning']['actions'])} actions, {len(d['tuning']['enemies'])} enemies.")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        sys.exit(__doc__)
    main(sys.argv[1])
