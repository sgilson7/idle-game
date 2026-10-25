# Second-order notebook

Stage 6, kept by the agent as the work happens, not afterwards. `B3 Build in self-questioning`.

The code and tests are the work; this file is the record about the work. It is where a decision the agent made alone, an assumption that turned out wrong, or something only the person can do becomes visible to the person. With the plan approved in advance, it is the main place those decisions surface before the person uses the tool. [`docs/procedure.md`](https://github.com/sgilson7/builder-setting/blob/main/docs/procedure.md#the-second-order-notebook) says why.

One row per assumption, surprise or open item. **Kind** is *divergence* (what was assumed, what turned out to be true, what changed), *finding*, or *worklist* (the person's to do). Number rows by round (R1.1, R1.2, then R2.1 after the first triage) so a later round does not renumber an earlier one. **Rule** is the rule the row produced for later sessions, and where it now lives (`CLAUDE.md`, a test, `tests/breaks.json`), or blank. **Status** is *done*, *open*, or *person*; every *person* row is named in `HANDOFF.md`. `scripts/notebook_check.py` checks the form of each row and that the notebook is not empty once `PLAN.md` holds a decision, not what the rows say.

| # | Kind | Assumed or found | What is true | What changed | Rule | Status |
| --- | --- | --- | --- | --- | --- | --- |
| R1.1 | finding | The pasted description was one copy of the chat. | The paste held every message twice, and the person said to drop the repeats. | `BRIEF.md` keeps each message once, in order, without the speaker's name. | | done |
| R1.2 | divergence | The template keeps nothing between visits (D17). | The person asked for a browser save and a JSON save file. | Planned: the core writes and reads the save, only `web/save.js` names storage, and `no_storage.rs` becomes a fence. Recorded as a change to D17 once built (IG-5, IG-6). | | open |
| R1.3 | divergence | iPhone Safari could take a fullscreen button. | The mobile kit records that a phone's Safari has no Fullscreen API for a page; iPad Safari has it. | Proposed: the button opens home-screen instructions on iPhone (IG-Q4). | | open |
| R1.4 | worklist | | The game's name, the version 1 skills, the offline cap, the art route, combat, pacing and deploy are the person's (IG-Q1 to IG-Q9). | Asked in the journal's opening entry with a default for each. | | person |
| R1.5 | worklist | | Every skill, tree and item name in the mockups was written by the agent from RuneScape's conventions. | The person approves or replaces the list before it goes in `data/` (IG-11). | | person |

## Strings written by the agent

Each reader-visible string the agent wrote, with its key, for the person to read.

| Key | String |
| --- | --- |
