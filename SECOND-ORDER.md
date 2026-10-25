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
| R1.6 | worklist | | The mockups decide one action at a time with a plan, a 10-minute return threshold, four mastery checkpoints and tools as the gold sink. | Asked as IG-Q10 to IG-Q13 in the journal's Mockups entry, each with a default. | | person |
| R2.1 | worklist | | T3 needs a job for Firemaking beyond XP for "scale off each other"; the agent proposed a bonfire boost to gathering XP. | Asked as IG-Q14 (Round 2 card 1). | | person |
| R2.2 | divergence | IG-Q2 proposed five skills for version 1. | T1 and T3 together need twelve: three gathering, five making, three combat styles and Hitpoints. Runecrafting was added so Magic has a supply. | Asked as IG-Q15 (card 2); IG-Q2's default is replaced. | | person |
| R2.3 | divergence | IG-Q6 defaulted to no combat. | The co-designer asked for Melee, Ranged and Magic (T1). | Combat designed: one style at a time, each fed by a skill, food from Cooking, retreat without loss by default. IG-Q16 and IG-Q17 open. | | person |
| R2.4 | worklist | | Prestige (T2) must say what survives a reset; the agent's default keeps mastery so masteries stay lifetime goals. | Asked as IG-Q18 (card 5). | | person |
| R2.5 | finding | A rising prestige requirement keeps runs from collapsing. | Modelled on RuneScape's curve: +25% per prestige with +75 levels per prestige makes run 9 about 42 times run 1; a flat requirement shrinks runs from 41 h to 14 h; +25 levels per prestige keeps runs between 40 h and 71 h. | Default A in IG-Q19 (card 6); the model is illustrative (twelve skills, 30,000 XP an hour). | | person |
| R2.6 | worklist | | The meta-progression timeline's times (week 1, first prestige at total 600) are the agent's guess. | Asked as IG-Q20 (card 7); final times wait on IG-Q9. | | person |
| R2.7 | finding | The journal is read on a phone at work. | The co-designer reviews in short bursts. | Round 2 is seven cards, each with one picture and one tap-to-answer decision; answers save to the published page's store, or stay on the device with a copy button. | | done |

## Strings written by the agent

Each reader-visible string the agent wrote, with its key, for the person to read.

| Key | String |
| --- | --- |
