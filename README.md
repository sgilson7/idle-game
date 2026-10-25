# Idle Game

An ad-free idle game in the browser with RuneScape-like skills. Twelve skills level to 99 and feed each other: logs go to Firemaking or Fletching, ore to Smithing, fish to Cooking, and swords, arrows, runes and food go to combat in three styles. Every tree, rock, fishing spot and recipe has its own mastery to 99. The game keeps going while the page is closed, for up to a day, and shows what happened when you come back. Prestige starts every skill again for more XP.

Play it at https://sgilson7.github.io/idle-game/ on a phone, a tablet or a desktop. On an iPhone, Add to Home Screen gives a full-screen game.

It saves in your browser, and Settings exports and imports a save file. Nothing leaves your device unless you send a file yourself: there are no ads, accounts, servers or analytics.

## Playtesting

Tap **Flag** at the top whenever something feels slow, fast, confusing or fun. **Settings → Send playtest file** makes one file with your save, a log of what happened and when, and your flags, and opens the share sheet on a phone.

## How it is built

Built with [Builder](https://github.com/sgilson7/builder-setting): every rule of the game is in a Rust library (`crates/core`) that does no input or output, a WebAssembly shim passes values to the page, and the page draws what the library describes. The tuning numbers are in `crates/core/src/content.rs`; the words are in `data/strings.json`.

```bash
./scripts/setup.sh    # once: Playwright and the browser engines
./scripts/check.sh    # format, lint, tests, build, and the browser check
```

## License

MIT. See `LICENSES.md`.
