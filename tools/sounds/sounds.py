#!/usr/bin/env python3
"""Write data/sounds.json: every sound the core can name (T18, IG-Q50 B).

A sound is a few notes the page plays with the browser's own synthesiser,
so the game ships no audio files. Each note is a frequency in hertz, a start
and a length in seconds, a waveform and a volume. The work sounds are short
and quiet, because they repeat; each song gets a tune of its own, drawn from
a five-note scale and seeded by its name, played in its instrument's voice.
`buzz` is the phone's vibration pattern, in milliseconds, where it has one.

    python3 tools/sounds/sounds.py           # write
    python3 tools/sounds/sounds.py --check   # fail if the file is stale
"""
import hashlib
import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[2]
OUT = ROOT / "data" / "sounds.json"
CONTENT = (ROOT / "crates" / "core" / "src" / "content.rs").read_text()


def n(f, at, length, wave="triangle", vol=0.05):
    return {"f": round(f, 1), "at": round(at, 3), "len": round(length, 3), "wave": wave, "vol": vol}


def knock(f, wave="square", vol=0.04):
    return [n(f, 0, 0.06, wave, vol), n(f * 0.75, 0.04, 0.08, wave, vol * 0.6)]


WORK = {
    "chop": knock(180), "mine": knock(420, "square", 0.035), "smith": knock(660, "square", 0.03) + [n(1320, 0.02, 0.15, "sine", 0.02)],
    "build": knock(220), "slash": [n(900, 0, 0.05, "sawtooth", 0.025), n(500, 0.03, 0.07, "sawtooth", 0.02)],
    "shoot": [n(300, 0, 0.12, "triangle", 0.04), n(150, 0.08, 0.1, "triangle", 0.03)],
    "cast": [n(660, 0, 0.18, "sine", 0.035), n(990, 0.08, 0.2, "sine", 0.03)],
    "fish": [n(520, 0, 0.08, "sine", 0.04), n(390, 0.07, 0.1, "sine", 0.03)],
    "fire": [n(140, 0, 0.12, "sawtooth", 0.02)], "stir": [n(300, 0, 0.1, "triangle", 0.03)],
    "plant": [n(250, 0, 0.1, "triangle", 0.035)], "whittle": [n(1200, 0, 0.04, "square", 0.015)],
    "play": [n(440, 0, 0.2, "triangle", 0.03)],
    "level": [n(523.3, 0, 0.12, "triangle", 0.07), n(659.3, 0.11, 0.12, "triangle", 0.07), n(784, 0.22, 0.25, "triangle", 0.07)],
    "coin": [n(988, 0, 0.07, "square", 0.03), n(1319, 0.06, 0.16, "square", 0.03)],
    "fanfare": [n(392, 0, 0.14, "sawtooth", 0.035), n(523.3, 0.13, 0.14, "sawtooth", 0.035), n(659.3, 0.26, 0.14, "sawtooth", 0.035),
                n(784, 0.39, 0.4, "sawtooth", 0.04)],
}

VOICE = {"wooden_flute": ("sine", 2), "lute": ("triangle", 1), "golden_harp": ("triangle", 2), "iron_horn": ("sawtooth", 0),
         "conch_horn": ("sawtooth", 0), "alpine_horn": ("sawtooth", -1), "war_drum": ("square", -2)}
SCALE = [0, 2, 4, 7, 9]  # a major five-note scale, in semitones


def tune(song, instrument):
    wave, octave = VOICE[instrument]
    seed = hashlib.sha256(song.encode()).digest()
    root = 220 * 2 ** octave
    step, notes, at = 0.16, [], 0.0
    for k in range(8):
        b = seed[k]
        degree = SCALE[b % 5] + 12 * ((b >> 3) % 2)
        f = root * 2 ** (degree / 12)
        length = step * (2 if b % 7 == 0 else 1)
        vol = 0.045 if wave in ("sawtooth", "square") else 0.06
        notes.append(n(f, at, length * 0.9, wave, vol))
        at += length
    return notes


def build():
    songs = re.findall(r'\("([a-z_]+)", "([a-z_]+)"\),', CONTENT.split("pub const SONG_INSTRUMENTS")[1].split("];")[0])
    sounds = dict(WORK)
    for song, inst in songs:
        sounds[f"song_{song}"] = tune(song, inst)
    return {"sounds": sounds, "buzz": {"level": [40, 30, 40], "fanfare": [60, 40, 60, 40, 120]}}


def main():
    text = json.dumps(build(), indent=1, sort_keys=True) + "\n"
    if "--check" in sys.argv:
        if not OUT.exists() or OUT.read_text() != text:
            raise SystemExit("data/sounds.json is stale: run python3 tools/sounds/sounds.py")
        print("sounds are up to date")
        return
    OUT.write_text(text)
    print(f"wrote data/sounds.json with {len(build()['sounds'])} sounds")


if __name__ == "__main__":
    main()
