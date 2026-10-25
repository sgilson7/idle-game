# Brief

Stage 1. The person's own words come first; the rest can be written by the agent from them and read by the person. `C1 Frame the problem and set the rules`, `B1 Establish context`.

## The description, word for word

The idea arrived as chat messages. The paste repeated each message twice; each is kept once here, in order, with the speaker's name left out because this repository may become public.

> I'm at work but had a game idea. And this might be a little lame compared to Yalls ideas, but: an incremental idle game except it has RuneScape like skills. Ex: wood chopping skill, the higher level you have the skill the better wood you can chop.
>
> And within wood chopping if you have like Teak wood- you can have the teak wood have a level to get to lvl 99
>
> Just wait around, you could have a 3d model like chopping wood or something to have a visual. But just a basic incremental game.
>
> I've been playing a bunch of Archmage idle and letting that cook while I work.
>
> I basically want an incremental/idle game without all the gawd damn ads lmao
>
> It will probably be more lame than y'all's games but I'm just looking for an idle game so it doesn't need a ton of bells and whistles
>
> Less bells and whistles but make it very deep and long?
>
> These games are very ui driven making sure things are satisfying to do which I'll need your help on once v1 is ready for your review

The person's instructions to the agent, word for word:

> You will be using builder setting to help constrain and direct your actions as you help act as as a video game creator and co-designer / developer, with the goal of deploying it to a github pages so its freely browser accessible, with an emphasis on optimization for running in the browser.

> I'm use to wack ui from idle games, use the video as proof of the ui i am willing to work with. after intepreting builder into the design journal, do an analysis on archmage and other idle long form incremental progress type games to see what is fun, what systems they share, how to make idling feel worth it, how to make a player look forward to idling for 20 minutes and its outcomes, and use the results of that analysis (captured on a lit review tab) to make mock ups of the idle game we are making, it should work on mobile with a fullscreen button that works on iphone and ipad, it should have a way to save progress longterm in a json and short-term in the browser like other builder made games

The video is a 29-second screen recording of Archmage Idle on a phone (`../BD0C0702-5D5B-4018-AA0E-E6664657886D.MOV`, kept out of this repository).

## The tool

An ad-free incremental idle game in the browser with RuneScape-like skills. Each skill levels to 99 and unlocks better resources as it rises (Woodcutting: better trees). Each resource also has its own mastery level to 99 (Teak has a level of its own). The game runs while the player waits, including while the tab is closed, and is meant to be deep and long rather than feature-heavy. It is played on phones, tablets and desktops, often left running at work and checked on.

## Done means

- A skill can be trained by choosing an action and waiting; it gains experience and levels to 99, and higher levels unlock better resources.
- Each resource has its own mastery level to 99 with a visible effect.
- Progress made while the page was closed is credited on return, up to a stated limit, and a summary says what was gained.
- Progress is kept in the browser between visits, and can be exported to and imported from a JSON file.
- The page works by touch on an iPhone and an iPad; a fullscreen control is offered where the browser supports it, and the home-screen route is explained where it does not.
- The game makes no request to another origin and shows no ads.
- `./scripts/check.sh` passes, including the browser check in Chromium, Firefox, WebKit, an iPhone profile and an iPad profile.

## Decisions that stay with the person

- The game's name.
- Which skills ship in the first version, and the names of resources and items (an agent-generated list is a worklist row).
- Whether to deploy, and the repository and address.
- Any 3D or drawn art beyond what the mockups propose.
- Pacing targets: how long to 99 in a skill, how long the offline cap is.

## Accessibility

Levels, progress and rarity are carried by text and numbers as well as colour. Every action is a button reachable by touch, mouse and keyboard; touch targets are at least 44 CSS pixels.

## Style

The look of Archmage Idle as shown in the video: dark panels, dense cards, coloured level bars, a tab bar of skill icons along the bottom. The person accepts dense idle-game UI. A style pack for this project is to be written after the mockups are approved.

## Words

Skill, level, experience (XP), mastery, action, resource, offline progress, as defined in `GLOSSARY.md` once the plan is approved.

## Autonomy

The plan is not approved in advance: after the opening entry, the literature review and the mockups, the agent stops for the person. Where this brief is silent, the agent decides, records the decision in `SECOND-ORDER.md`, and continues. The agent may not loosen a Builder requirement or a boundary test, and may not add a server, an account, analytics, ads or any off-origin request. The agent may not create a repository, push or publish until the person names the repository and says so.

## Journal

`default`

Where the design journal lives. `default` is a folder beside this project, `../<this folder>-journal/`, kept as its own repository and never pushed; a relative path puts it elsewhere. Write `none`, with the reason, to build without a journal. With any other value the agent starts the journal and writes its opening entry before the plan, and `scripts/journal_check.py` fails while it is missing.

## Allowed core dependencies

`serde`, `serde_json` (reference default). Add others only here, with the reason.
