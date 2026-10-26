"""The game's browser interactions, run by check.py in every engine.

check.py has already loaded the page, waited for it to report ready, and let
it idle. These drive the game the way a player would and check what the
player would see. Expected values are worked out by hand from the content
(a tree is 3 s and 10 XP), never copied from the game's own output. Waits are
on selectors and page state, never a fixed time; time away is skipped with
Playwright's clock, not waited out.
"""

import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[2]
STRINGS = json.loads((ROOT / "data" / "strings.json").read_text())
SPRITE = set(re.findall(r'<symbol id="([^"]+)"', (ROOT / "web" / "icons.svg").read_text()))
UI = STRINGS["ui"]


# The bottom bar's groups (view.rs GROUPS, P21), written out by hand.
GROUPS = {
    "gather": ["woodcutting", "mining", "fishing", "farming"],
    "make": ["firemaking", "fletching", "smithing", "cooking", "crafting", "runecrafting", "construction"],
    "fight": ["combat", "recipes"],
    "engines": ["performance", "stage", "trader", "estate", "workshop", "observatory"],
    "more": ["bank", "shop", "book", "regions", "plan", "prestige", "settings"],
}


def open_tab(page, tab):
    """A tap on the tab; on a phone, first a tap on its group, as a player would."""
    if page.locator(f'#tabs .tab[data-tab="{tab}"]').is_hidden():
        group = next(g for g, tabs in GROUPS.items() if tab in tabs)
        page.locator(f'#groups .tab[data-tab="{group}"]').click()
    page.locator(f'#tabs .tab[data-tab="{tab}"]').click()
    page.wait_for_selector(f"body[data-screen='{tab}']", state="attached")


def icons_on(page):
    return set(page.evaluate("() => [...document.querySelectorAll('use')].map(u => (u.getAttribute('href') || '').split('#')[1])"))


def run(page, check, engine):
    phone = engine.endswith("iphone")
    check(page.locator("#tabs .tab").count() == 26, "twenty-six tabs drawn: skills, combat, recipes, the stage and four engines, and seven screens")
    check(page.locator("#groups .tab").count() == 5, "five groups drawn")
    if phone:
        check(page.locator("#tabs .tab:visible").count() == 4, "iPhone: the strip shows the Gather group's four tabs")
        check(page.locator("#groups").is_visible(), "iPhone: the groups sit along the bottom")
    else:
        check(page.locator("#tabs .tab:visible").count() in (4, 26), "the rail or the strip shows its tabs")
    check(page.inner_text(".sheet h2") == UI["welcome_title"], "a new game opens with the welcome")
    page.locator(".sheet .btn.primary").click()
    check(page.locator("#shade").is_hidden(), "Start closes the welcome")
    check(page.evaluate("getComputedStyle(document.body).touchAction") == "manipulation", "no double-tap zoom on the page")
    fs = page.locator('#top button[data-act="page.fullscreen"], #top button[data-act="help.home"]')
    if phone:
        check(page.locator('#top button[data-act="help.home"]').count() == 1, "iPhone: the fullscreen button explains the home screen")
        page.locator('#top button[data-act="help.home"]').click()
        check(page.inner_text(".sheet h2") == UI["home_title"], "iPhone: the home-screen steps open")
        page.locator(".sheet .btn.primary").click()
    else:
        check(fs.count() == 1, "a fullscreen button where the browser has fullscreen")

    # One chop: a tree is 3 s and 10 XP (content.rs), worked by hand.
    page.locator('button[data-act="do"][data-value="tree"]').click()
    page.wait_for_selector(".card.active")
    check(page.locator('#tabs .tab[data-tab="woodcutting"] .run').count() == 1, "the running skill's tab shows a dot")
    check(page.locator('#groups .tab[data-tab="gather"] .run').count() == 1, "and so does its group")
    page.wait_for_function("() => document.querySelector('.bar span b').textContent.startsWith('10 /')", timeout=10000)
    check(True, "one chop gives 10 XP")
    check(page.locator(".forecast").count() == 1, "the 20-minute forecast shows while something runs")
    missing = icons_on(page) - SPRITE
    check(not missing, f"every icon on the skill screen is in icons.svg {missing}")

    # The save survives a reload.
    page.reload()
    page.wait_for_selector("body[data-ready='true']", state="attached")
    xp = page.inner_text(".bar span b")
    check(not xp.startswith("0 /"), f"the save survives a reload ({xp})")

    # Every tab draws, with no icon missing.
    for tab in [t for tabs in GROUPS.values() for t in tabs][1:]:
        open_tab(page, tab)
        missing = icons_on(page) - SPRITE
        check(not missing, f"the {tab} screen draws and its icons exist {missing}")

    # An engine before its total level: the Trader opens at 150 (content.rs ENGINE_AT).
    open_tab(page, "trader")
    check(UI["opens_at"].replace("{n}", "150") in page.inner_text("#main"), "the Trader tab says it opens at total level 150")
    check(page.locator('#tabs .tab[data-tab="trader"].locked').count() == 1, "a tab not open yet is drawn dimmed")

    # The bank's search box keeps the keyboard up while it filters.
    open_tab(page, "bank")
    field = page.locator('input[data-act="bank.search"]')
    field.click()
    field.type("lo")
    check(page.evaluate("() => document.activeElement && document.activeElement.dataset.act") == "bank.search", "typing in the bank search keeps the box focused")
    check(field.input_value() == "lo", "the search box keeps what was typed")

    # Flag a moment.
    page.locator('#top button[data-act="flag.open"]').click()
    page.locator('.sheet button[data-act="flag.fun"][data-value="3"]').click()
    page.locator(".sheet textarea").fill("checking the flag")
    page.locator('.sheet button[data-act="flag.save"]').click()
    check(page.locator("#shade").is_hidden(), "saving a flag closes the sheet")

    # Files: the export and the playtest file download where there is no share sheet.
    if engine in ("chromium", "firefox"):
        open_tab(page, "settings")
        with page.expect_download() as d:
            page.locator('button[data-act="page.export"]').click()
        saved = json.loads(pathlib.Path(d.value.path()).read_text())
        check(saved["game"] == "idle-game" and len(saved["state"]["xp"]) == 19, "Export downloads a save with nineteen skills")
        with page.expect_download() as d:
            page.locator('button[data-act="page.playtest"]').click()
        report = json.loads(pathlib.Path(d.value.path()).read_text())
        check(report["kind"] == "idle-game-playtest", "Send playtest file downloads the playtest file")
        check(report["flags"][-1]["note"] == "checking the flag", "the flag is in the playtest file")

    # Away for 30 minutes: a second tab with a controllable clock.
    other = page.context.new_page()
    other.clock.install()
    other.goto(page.url)
    other.wait_for_selector("body[data-ready='true']", state="attached")
    open_tab(other, "woodcutting")
    # The save from the first tab may already be chopping; a tap would stop it.
    if other.locator(".card.active").count() == 0:
        other.locator('button[data-act="do"][data-value="tree"]').click()
    other.wait_for_selector(".card.active")
    other.clock.fast_forward("30:00")
    other.wait_for_selector(".sheet.return", timeout=10000)
    big = other.inner_text(".sheet .big")
    # 30 minutes skipped, plus the few hundred milliseconds the tab took to load and tap.
    check(big.startswith("30m"), f"30 minutes away shows the return screen ({big})")
    other.locator('.sheet button[data-act="sheet.close"]').click()
    other.close()
