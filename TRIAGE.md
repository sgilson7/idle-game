# Triage

Stage 7. One row per request after the person has used the build: a bug, a change, or a new idea, in the person's own words. Rows are numbered T1, T2 and so on, and each commit that answers one names it ("Fix the timer reset (T4)"). `C4 Verify against hand-computed cases`.

Compare at least one result with a case worked out by hand. **Severity** is *blocks use*, *wrong*, or *wanted*. **Cost** is the agent's estimate. **Disposition** is *done*, *open*, *declined* (with the reason), or *try* (built on a branch and published as a TRY commit that says how to undo it).

| # | Request, in the person's words | Expected | What happened | Severity | Cost | Disposition |
| --- | --- | --- | --- | --- | --- | --- |
| T1 | "combat skills. So ranged, melee, magic" | Melee, Ranged and Magic as skills to 99, each fed by another skill | Designed and mocked up (journal, Round 2 cards 3–4); not built | wanted | large | open |
| T2 | "a prestige button to reset the game with a permanent xp multiplier based on prestige times!" | Prestige resets skills for a permanent XP multiplier that grows with each prestige | Designed and mocked up (Round 2 cards 5–6); not built | wanted | medium | open |
| T3 | "Things should just scale off each other I guess is the other point. So in terms of RuneScape: you have wood cutting- and then from wood cutting you can either do fletching or fire making from that wood." | Each skill uses what another makes | Designed and mocked up: twelve skills linked by recipes (Round 2 cards 1–2, 7); not built | wanted | medium | open |
| T4 | "IG-Q14: A" (XP only, as in RuneScape) | Burning logs gives Firemaking XP and nothing else | The MVP lights a bonfire (+10% gathering XP); the default was B | wanted | small | open |
| T5 | "IG-Q15: C" (Fourteen: add Crafting (hides, silk) and Farming) | Crafting turns hides, silk, gold and scales into armour and jewellery; Farming grows crops in patches while you do something else | The MVP has twelve skills; the default was A | wanted | large | open |
| T6 | "IG-Q18: B" (The multiplier only: a true fresh start) | A prestige keeps the XP multiplier and nothing else from the game | The MVP keeps mastery and kill counts; the default was A | wanted | small | open |
| T7 | "IG-Q19: B" (+25% each; always 600, runs get short) | Every prestige asks total level 600 | The MVP asks 25 more levels each time; the default was A | wanted | small | open |
