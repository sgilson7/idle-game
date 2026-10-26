//! The view model (P13): everything the page draws, as blocks of a few kinds.
//! The page knows how to draw a `bar` or a `cards` block; it does not know
//! what a level or a tree is. Every number in here is already formatted.

use serde::Serialize;

use crate::content::{self as c, Kind};
use crate::game::{Report, State, Stop};
use crate::text::{dur, num, secs, times, width, Text};
use crate::xp;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Button {
    pub label: String,
    pub act: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub value: String,
    pub enabled: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub tone: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub icon: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub sub: String,
    #[serde(skip_serializing_if = "is_false")]
    pub pressed: bool,
}

fn is_false(b: &bool) -> bool {
    !*b
}

pub fn button(label: String, act: &str, value: &str) -> Button {
    Button {
        label,
        act: act.into(),
        value: value.into(),
        enabled: true,
        ..Button::default()
    }
}

impl Button {
    pub fn tone(mut self, t: &str) -> Button {
        self.tone = t.into();
        self
    }
    pub fn enabled(mut self, on: bool) -> Button {
        self.enabled = on;
        self
    }
    pub fn icon(mut self, i: &str) -> Button {
        self.icon = i.into();
        self
    }
    pub fn below(mut self, s: String) -> Button {
        self.sub = s;
        self
    }
    pub fn pressed(mut self, p: bool) -> Button {
        self.pressed = p;
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Chip {
    pub tone: String,
    pub text: String,
}

pub(crate) fn chip(tone: &str, text: String) -> Chip {
    Chip {
        tone: tone.into(),
        text,
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Bar {
    pub tone: String,
    pub width: String,
    pub left: String,
    pub right: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Check {
    pub text: String,
    pub on: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Card {
    pub icon: String,
    pub title: String,
    pub sub: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub tag: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub tone: String,
    #[serde(skip_serializing_if = "is_false")]
    pub locked: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub active: bool,
    pub dur: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub delay: String,
    pub chips: Vec<Chip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<Bar>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub checks: Vec<Check>,
    pub buttons: Vec<Button>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Row {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub icon: String,
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub sub: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub chip: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub tone: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub buttons: Vec<Button>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Block {
    pub kind: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub icon: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub text: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub tag: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub tone: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub dur: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub delay: String,
    #[serde(skip_serializing_if = "is_false")]
    pub running: bool,
    /// A scene's figure: what the work looks like ("chop", "fish", "play" ...).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub pose: String,
    /// A scene's backdrop: the region it happens in.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub place: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar: Option<Bar>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bar2: Option<Bar>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub label2: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub chips: Vec<Chip>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cards: Vec<Card>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rows: Vec<Row>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub buttons: Vec<Button>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub buttons2: Vec<Button>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub list: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub list2: Vec<String>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub act: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curve: Option<Curve>,
}

/// A line graph the page draws as given (T23): an SVG path in the `view`
/// box, a marker placed by percentages, and four labels.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Curve {
    pub view: String,
    pub line: String,
    /// The marker's place, as CSS percentages of the box from left and top.
    pub x: String,
    pub y: String,
    pub top: String,
    pub low: String,
    pub left: String,
    pub right: String,
}

pub(crate) fn block(kind: &str) -> Block {
    Block {
        kind: kind.into(),
        ..Block::default()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Sheet {
    pub kind: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub eyebrow: String,
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub big: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lines: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub gains: Vec<Row>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub ready_title: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ready: Vec<Row>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<Block>,
    pub buttons: Vec<Button>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Tab {
    pub id: String,
    pub label: String,
    pub short: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub level: String,
    #[serde(skip_serializing_if = "is_false")]
    pub running: bool,
    #[serde(skip_serializing_if = "is_false")]
    pub current: bool,
    /// Not open yet: drawn dimmed, and its screen says what opens it.
    #[serde(skip_serializing_if = "is_false")]
    pub locked: bool,
    /// In the group on screen: a phone shows only these, in the strip (P21).
    #[serde(skip_serializing_if = "is_false")]
    pub strip: bool,
    /// The group's name, on the first tab of each group, for the wide rail.
    #[serde(skip_serializing_if = "String::is_empty")]
    pub heading: String,
    /// What a tap sends: `tab`, or `group` for the five group buttons.
    pub act: String,
    pub value: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Top {
    pub gold: String,
    pub total: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub mult: String,
    pub flag: Button,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen: Option<Button>,
}

/// The page's device, as the page reports it.
#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Deserialize, Serialize)]
#[serde(default)]
pub struct Device {
    pub fullscreen: bool,
    pub ios: bool,
    pub standalone: bool,
    pub ua: String,
    pub w: u64,
    pub h: u64,
    /// Running as the store app, not the web page (T17).
    pub app: bool,
    /// The store says the purchase is made (IG-Q46 B).
    pub owned: bool,
    /// The purchase's price as the store formats it for this player.
    pub price: String,
}

/// The five groups of the bottom bar, and the tabs in each (P21): one tap
/// to a group, one tap to a skill.
#[rustfmt::skip]
pub const GROUPS: [(&str, &[&str]); 5] = [
    ("gather", &["woodcutting", "mining", "fishing", "farming"]),
    ("make", &["firemaking", "fletching", "smithing", "cooking", "crafting", "runecrafting", "construction"]),
    ("fight", &["combat", "weapons", "recipes"]),
    ("engines", &["performance", "stage", "trader", "estate", "workshop", "observatory"]),
    ("more", &["bank", "shop", "book", "regions", "plan", "prestige", "settings"]),
];

#[rustfmt::skip]
pub const TABS: [&str; 27] = [
    "woodcutting", "mining", "fishing", "farming",
    "firemaking", "fletching", "smithing", "cooking", "crafting", "runecrafting", "construction",
    "combat", "weapons", "recipes",
    "performance", "stage", "trader", "estate", "workshop", "observatory",
    "bank", "shop", "book", "regions", "plan", "prestige", "settings",
];

/// The tabs whose skills or engines take things in and give things out,
/// for the uses and feeds links.
#[rustfmt::skip]
pub const LINK_TABS: [&str; 18] = [
    "woodcutting", "mining", "fishing", "farming",
    "firemaking", "fletching", "smithing", "cooking", "crafting", "runecrafting", "construction",
    "combat", "performance", "stage", "trader", "estate", "workshop", "observatory",
];

/// The tab where a skill's actions are: its own, or its engine's.
pub fn tab_of_skill(skill: usize) -> &'static str {
    match skill {
        c::TRADING => "trader",
        c::INVENTION => "workshop",
        c::ATTUNEMENT => "observatory",
        c::MELEE | c::RANGED | c::MAGIC | c::HITPOINTS => "combat",
        i => c::SKILLS[i].id,
    }
}

pub fn group_of(tab: &str) -> &'static str {
    GROUPS
        .iter()
        .find(|(_, tabs)| tabs.contains(&tab))
        .map(|(g, _)| *g)
        .unwrap_or("more")
}

/// Everything the view needs to read.
pub struct View<'a> {
    pub t: &'a Text,
    pub s: &'a State,
    pub device: &'a Device,
    /// The bank search box (IG-Q41 A).
    pub search: &'a str,
    /// Skill screens show every row.
    pub all: bool,
    /// Sections opened by the player (T25).
    pub unfolded: &'a [String],
    /// The encyclopedia's search box and open group (T19).
    pub book_search: &'a str,
    pub book_group: &'a str,
}

/// How many of a skill's newest rows show before "Show all" (T15).
pub const NEWEST: usize = 8;

impl View<'_> {
    fn name(&self, id: &str) -> String {
        match c::enemy_index(id) {
            Some(_) => self.t.enemy(id),
            None => self.t.action(id),
        }
    }

    pub(crate) fn skill_name(&self, i: usize) -> String {
        self.t.skill(c::SKILLS[i].id).name.clone()
    }

    /// A tab's name: a skill's, or a screen's.
    pub(crate) fn tab_name(&self, id: &str) -> String {
        if c::skill_index(id).is_some() || id == "combat" {
            self.t.skill(id).name.clone()
        } else {
            self.t.s(&format!("tab_{id}"))
        }
    }

    pub(crate) fn xp_bar(&self, skill: usize) -> Bar {
        let x = self.s.xp[skill];
        let l = xp::level(x);
        if l >= xp::MAX_LEVEL {
            return Bar {
                tone: "max".into(),
                width: width(10_000),
                left: self.t.t("xp_total", &[("xp", num(x))]),
                right: self.t.s("max"),
            };
        }
        let (a, b) = (xp::at(l), xp::at(l + 1));
        Bar {
            tone: "xp".into(),
            width: width(xp::progress_bp(x, a, b)),
            left: self.t.t("xp_of", &[("have", num(x - a)), ("need", num(b - a))]),
            right: self.t.t("xp_total", &[("xp", num(x))]),
        }
    }

    pub(crate) fn mastery_bar(&self, id: &str) -> Bar {
        let m = self.s.mxp.get(id).copied().unwrap_or(0);
        let l = xp::mastery_level(m);
        if l >= xp::MAX_LEVEL {
            return Bar {
                tone: "mast".into(),
                width: width(10_000),
                left: self.t.t("mastery_n", &[("n", l.to_string())]),
                right: self.t.s("max"),
            };
        }
        let (a, b) = (xp::mastery_at(l), xp::mastery_at(l + 1));
        Bar {
            tone: "mast".into(),
            width: width(xp::progress_bp(m, a, b)),
            left: self.t.t("mastery_n", &[("n", l.to_string())]),
            right: self.t.t("of", &[("have", num(m - a)), ("need", num(b - a))]),
        }
    }

    pub fn top(&self) -> Top {
        let screen = if self.device.fullscreen {
            Some(button(self.t.s("fullscreen"), "page.fullscreen", "").icon("fullscreen"))
        } else if self.device.ios && !self.device.standalone {
            Some(button(self.t.s("fullscreen"), "help.home", "").icon("fullscreen"))
        } else {
            None
        };
        Top {
            gold: num(self.s.gold),
            total: self
                .t
                .t("total_level", &[("n", self.s.total_level().to_string())]),
            mult: if self.s.prestige > 0 {
                self.t.t(
                    "mult_pill",
                    &[("n", self.s.prestige.to_string()), ("x", times(self.s.mult_bp()))],
                )
            } else {
                String::new()
            },
            flag: button(self.t.s("flag_short"), "flag.open", "").icon("flag"),
            screen,
        }
    }

    fn running_tab(&self) -> Option<&'static str> {
        self.s.active.as_deref().map(|id| {
            if c::enemy_index(id).is_some() {
                "combat"
            } else {
                tab_of_skill(self.s.skill_of(id))
            }
        })
    }

    /// The bottom bar's five groups (P21). A group opens on the running tab
    /// when it holds it, and on its first tab otherwise.
    pub fn groups(&self) -> Vec<Tab> {
        let running = self.running_tab();
        let here = group_of(&self.s.screen);
        GROUPS
            .iter()
            .map(|(g, tabs)| {
                let to = running.filter(|r| tabs.contains(r)).unwrap_or(tabs[0]);
                Tab {
                    id: (*g).into(),
                    label: self.t.s(&format!("group_{g}")),
                    short: self.t.s(&format!("group_{g}")),
                    running: running.is_some_and(|r| tabs.contains(&r)),
                    current: here == *g,
                    act: "tab".into(),
                    value: to.into(),
                    ..Tab::default()
                }
            })
            .collect()
    }

    /// Whether a tab is open yet, and the engine that opens it.
    pub fn tab_locked(&self, id: &str) -> bool {
        match id {
            "construction" | "trader" => !self.s.engine_reached(c::Engine::Trader),
            "estate" => !self.s.engine_reached(c::Engine::Nobility),
            "workshop" => !self.s.engine_reached(c::Engine::Inventor),
            "observatory" => !self.s.engine_reached(c::Engine::Cosmic),
            "performance" | "stage" => !self.s.fame_open(),
            _ => false,
        }
    }

    pub fn tabs(&self) -> Vec<Tab> {
        let running = self.running_tab();
        let here = group_of(&self.s.screen);
        TABS.iter()
            .map(|&id| {
                let (label, short, level) = match c::skill_index(id) {
                    Some(i) => {
                        let w = self.t.skill(id);
                        (w.name.clone(), w.short.clone(), self.s.level(i).to_string())
                    }
                    None if id == "combat" => {
                        let w = self.t.skill(id);
                        (
                            w.name.clone(),
                            w.short.clone(),
                            self.s.level(c::STYLES[self.s.style].skill).to_string(),
                        )
                    }
                    None => (
                        self.t.s(&format!("tab_{id}")),
                        self.t.s(&format!("tab_{id}_short")),
                        match id {
                            "prestige" if self.s.prestige > 0 => self.s.prestige.to_string(),
                            "regions" => self.s.regions_open().to_string(),
                            "trader" => self.s.level(c::TRADING).to_string(),
                            "workshop" => self.s.level(c::INVENTION).to_string(),
                            "observatory" => self.s.level(c::ATTUNEMENT).to_string(),
                            _ => String::new(),
                        },
                    ),
                };
                let group = group_of(id);
                let first = GROUPS.iter().any(|(_, tabs)| tabs[0] == id);
                Tab {
                    id: id.into(),
                    label,
                    short,
                    level: if self.tab_locked(id) { String::new() } else { level },
                    running: running == Some(id),
                    current: self.s.screen == id,
                    locked: self.tab_locked(id),
                    strip: group == here,
                    heading: if first {
                        self.t.s(&format!("group_{group}"))
                    } else {
                        String::new()
                    },
                    act: "tab".into(),
                    value: id.into(),
                }
            })
            .collect()
    }

    pub fn screen(&self) -> Vec<Block> {
        if self.tab_locked(&self.s.screen) {
            return self.locked(&self.s.screen);
        }
        let mut blocks = self.screen_blocks();
        if let Some(bar) = self.filter_bar() {
            let at = 1.min(blocks.len());
            blocks.insert(at, bar);
        }
        blocks
    }

    // ---------- filtering by region and sorting (T25) ----------

    /// Screens whose lists sort; the region filter shows wherever regions do.
    fn sorts(&self) -> bool {
        let s = self.s.screen.as_str();
        c::skill_index(s).is_some() || matches!(s, "combat" | "bank" | "book" | "weapons")
    }

    fn filters_regions(&self) -> bool {
        let s = self.s.screen.as_str();
        self.sorts() && s != "weapons"
            || matches!(
                s,
                "recipes" | "trader" | "estate" | "observatory" | "stage" | "shop"
            )
    }

    /// The region the lists show, or `None` for every open region. A
    /// filter on a region that has closed (after prestige) shows them all.
    pub(crate) fn region_filter(&self) -> Option<usize> {
        let v = self.s.view_region;
        (v > 0 && self.s.region_open(v - 1)).then(|| v - 1)
    }

    pub(crate) fn region_shown(&self, r: usize) -> bool {
        self.region_filter().is_none_or(|f| f == r)
    }

    /// The bar above a list: a chip for each open region, and the orders.
    fn filter_bar(&self) -> Option<Block> {
        let regions = self.filters_regions() && self.s.regions_open() > 1;
        let sorts = self.sorts();
        if !regions && !sorts {
            return None;
        }
        let here = self.region_filter().map_or(0, |r| r + 1);
        let buttons = if regions {
            std::iter::once(button(self.t.s("view_all"), "view.region", "0").pressed(here == 0))
                .chain((0..c::REGIONS.len()).filter(|&r| self.s.region_open(r)).map(|r| {
                    button(self.region_title(r), "view.region", &(r + 1).to_string()).pressed(here == r + 1)
                }))
                .collect()
        } else {
            Vec::new()
        };
        let buttons2 = if sorts {
            [("", "sort_level"), ("high", "sort_high"), ("name", "sort_name")]
                .iter()
                .map(|&(v, k)| button(self.t.s(k), "view.sort", v).pressed(self.s.view_sort == v))
                .collect()
        } else {
            Vec::new()
        };
        Some(Block {
            title: self.t.s("view_show"),
            label2: self.t.s("view_order"),
            buttons,
            buttons2,
            ..block("filters")
        })
    }

    fn region_title(&self, r: usize) -> String {
        self.t.s(&format!("region_{}", c::REGIONS[r].0))
    }

    /// Orders things by level (lowest first), highest first, or name.
    pub(crate) fn order<T>(
        &self,
        list: &mut [T],
        level: impl Fn(&T) -> (u32, usize),
        name: impl Fn(&T) -> String,
    ) {
        match self.s.view_sort.as_str() {
            "name" => list.sort_by_key(|x| name(x).to_lowercase()),
            "high" => list.sort_by_key(|x| std::cmp::Reverse(level(x))),
            _ => list.sort_by_key(|x| level(x)),
        }
    }

    /// Where an item comes from: the region of what makes it, grows it,
    /// drops it or sells it, the Vale for anything else.
    pub(crate) fn item_region(&self, item: &str) -> usize {
        c::ACTIONS
            .iter()
            .filter(|a| a.makes.0 == item)
            .map(|a| a.region)
            .chain(
                c::CROPS
                    .iter()
                    .filter(|cr| cr.produce == item || cr.seed == item)
                    .map(|cr| cr.region),
            )
            .chain(
                c::ENEMIES
                    .iter()
                    .filter(|e| e.drops.iter().any(|d| d.0 == item))
                    .map(|e| e.region),
            )
            .chain(c::TOWNS.iter().filter(|t| t.sells == item).map(|t| t.region))
            .min()
            .unwrap_or(0)
    }

    /// The level that first makes an item, for ordering lists of items.
    pub(crate) fn item_level(&self, item: &str) -> u32 {
        c::ACTIONS
            .iter()
            .filter(|a| a.makes.0 == item)
            .map(|a| a.level)
            .chain(c::CROPS.iter().filter(|cr| cr.produce == item).map(|cr| cr.level))
            .min()
            .unwrap_or(0)
    }

    fn screen_blocks(&self) -> Vec<Block> {
        match self.s.screen.as_str() {
            "recipes" => self.recipes(),
            "regions" => self.regions(),
            "trader" => self.trader(),
            "estate" => self.estate(),
            "workshop" => self.workshop(),
            "observatory" => self.observatory(),
            "stage" => self.stage(),
            "combat" => self.combat(),
            "weapons" => self.weapons(),
            "bank" => self.bank(),
            "shop" => self.shop(),
            "book" => self.book(),
            "plan" => self.plan(),
            "prestige" => self.prestige(),
            "settings" => self.settings(),
            "farming" => self.farming(),
            id => self.skill(c::skill_index(id).unwrap_or(0)),
        }
    }

    // ---------- skills ----------

    /// Skills that make what `ins` needs, and skills that use what `outs` makes.
    pub(crate) fn links(&self, me: &str, ins: &[&str], outs: &[&str]) -> Block {
        let mut uses: Vec<&str> = Vec::new();
        let mut feeds: Vec<&str> = Vec::new();
        for tab in LINK_TABS.iter() {
            if *tab == me || self.tab_locked(tab) {
                continue;
            }
            let (t_in, t_out) = io_of(tab);
            if t_out.iter().any(|o| ins.contains(o)) {
                uses.push(tab);
            }
            if t_in.iter().any(|i| outs.contains(i)) {
                feeds.push(tab);
            }
        }
        let b = |id: &&str| button(self.tab_name(id), "tab", id).icon(id);
        Block {
            kind: "links".into(),
            title: if uses.is_empty() {
                self.t.s("gathers")
            } else {
                self.t.s("uses")
            },
            label2: self.t.s("feeds"),
            buttons: uses.iter().map(b).collect(),
            buttons2: feeds.iter().map(b).collect(),
            ..block("links")
        }
    }

    pub(crate) fn head(&self, icon: &str, title: String, tag: String, tone: &str) -> Block {
        Block {
            icon: icon.into(),
            title,
            tag,
            tone: tone.into(),
            ..block("head")
        }
    }

    pub(crate) fn scene(&self, icon: &str, caption: String, running: bool, ms: u64) -> Block {
        Block {
            icon: icon.into(),
            text: caption,
            running,
            pose: self.pose_of(icon).into(),
            place: c::REGIONS[self.region_of(icon)].0.into(),
            dur: format!("{ms}ms"),
            delay: format!("-{}ms", self.s.progress_ms),
            ..block("scene")
        }
    }

    /// How the figure works at an action or a fight (T18, IG-Q51 A); also
    /// the name of the work's sound.
    pub(crate) fn pose(&self, id: &str) -> &'static str {
        self.pose_of(id)
    }

    fn pose_of(&self, id: &str) -> &'static str {
        if c::enemy_index(id).is_some() || c::dungeon_index(id).is_some() {
            return match c::STYLES[self.s.style].skill {
                c::RANGED => "shoot",
                c::MAGIC => "cast",
                _ => "slash",
            };
        }
        match c::action_index(id).map(|i| c::ACTIONS[i].skill) {
            Some(c::WOODCUTTING) => "chop",
            Some(c::MINING) => "mine",
            Some(c::FISHING) => "fish",
            Some(c::FIREMAKING) => "fire",
            Some(c::SMITHING) => "smith",
            Some(c::COOKING) => "stir",
            Some(c::RUNECRAFTING) | Some(c::ATTUNEMENT) => "cast",
            Some(c::FARMING) => "plant",
            Some(c::CONSTRUCTION) | Some(c::INVENTION) => "build",
            Some(c::PERFORMANCE) => "play",
            _ => "whittle",
        }
    }

    /// The region an action or enemy belongs to; the Vale for anything else.
    fn region_of(&self, id: &str) -> usize {
        c::action_index(id)
            .map(|i| c::ACTIONS[i].region)
            .or_else(|| c::enemy_index(id).map(|e| c::ENEMIES[e].region))
            .or_else(|| c::dungeon_index(id).map(|d| c::DUNGEONS[d].region))
            .unwrap_or(0)
    }

    pub(crate) fn skill(&self, i: usize) -> Vec<Block> {
        let sk = &c::SKILLS[i];
        let w = self.t.skill(sk.id);
        let active = self
            .s
            .active
            .as_deref()
            .and_then(c::action_index)
            .filter(|&a| c::ACTIONS[a].skill == i);
        let mut out = vec![self.head(
            sk.id,
            w.name.clone(),
            self.t.t("lvl", &[("n", self.s.level(i).to_string())]),
            "xp",
        )];
        out.push(Block {
            bar: Some(self.xp_bar(i)),
            ..block("bar")
        });
        if !c::skill_actions(i).is_empty() {
            out.extend(self.pool_blocks(i));
        }
        let (ins, outs) = io_of(sk.id);
        out.push(self.links(sk.id, &ins, &outs));
        let shown = active
            .map(|a| &c::ACTIONS[a])
            .unwrap_or(&c::ACTIONS[first_action(i)]);
        let ms = self.s.duration(shown.id);
        out.push(self.scene(
            shown.id,
            match active {
                Some(_) => self.t.t(
                    "doing",
                    &[("verb", w.verb.clone()), ("what", self.t.action(shown.id))],
                ),
                None => self.t.s("pick_below"),
            },
            active.is_some(),
            ms,
        ));
        if active.is_some() {
            out.push(Block {
                dur: format!("{ms}ms"),
                delay: format!("-{}ms", self.s.progress_ms),
                ..block("progress")
            });
        }
        if let Some(a) = active {
            out.push(self.forecast(&c::ACTIONS[a]));
        }
        out.extend(self.action_cards(i, active));
        out
    }

    /// A skill's action cards. With four regions a skill has up to 36 rows,
    /// so a phone shows the newest eight you can do and the next two to
    /// unlock, with a button for the rest (T15).
    pub(crate) fn action_cards(&self, i: usize, active: Option<usize>) -> Vec<Block> {
        let mut rows: Vec<&c::Action> = c::ACTIONS
            .iter()
            .filter(|a| {
                a.skill == i
                    && self.shown(a.id)
                    && self.s.region_open(a.region)
                    && self.region_shown(a.region)
            })
            .collect();
        rows.sort_by_key(|a| (a.level, a.region));
        let closed = c::ACTIONS
            .iter()
            .filter(|a| a.skill == i && !self.s.region_open(a.region))
            .count();
        let lvl = self.s.level(i);
        let open: Vec<&c::Action> = rows.iter().copied().filter(|a| a.level <= lvl).collect();
        let locked: Vec<&c::Action> = rows.iter().copied().filter(|a| a.level > lvl).collect();
        let older = open.len().saturating_sub(NEWEST);
        let mut shown: Vec<&c::Action> = open
            .iter()
            .enumerate()
            .filter(|(k, a)| self.all || *k >= older || active == c::action_index(a.id))
            .map(|(_, a)| *a)
            .collect();
        self.order(&mut shown, |a| (a.level, a.region), |a| self.t.action(a.id));
        let mut cards: Vec<Card> = shown
            .into_iter()
            .map(|a| self.action_card(a, active == c::action_index(a.id)))
            .collect();
        // LR-2: the next two locked ones show; the rest wait out of sight.
        for a in locked.iter().take(2) {
            cards.push(self.action_card(a, false));
        }
        let mut out = Vec::new();
        if older > 0 {
            out.push(Block {
                buttons: vec![button(
                    if self.all {
                        self.t.s("show_newest")
                    } else {
                        self.t.t("show_all", &[("n", open.len().to_string())])
                    },
                    "list.all",
                    "",
                )],
                ..block("buttons")
            });
        }
        out.push(Block {
            cards,
            ..block("cards")
        });
        let hidden = locked.len().saturating_sub(2);
        if hidden > 0 {
            out.push(Block {
                text: self.t.t("more_later", &[("n", hidden.to_string())]),
                ..block("note")
            });
        }
        if closed > 0 {
            out.push(Block {
                text: self.t.t("more_in_regions", &[("n", closed.to_string())]),
                ..block("note")
            });
        }
        out
    }

    /// Recipe actions show once the recipe is found, and build actions once
    /// their blueprint is done and the thing is not built yet (T10, T14).
    pub(crate) fn shown(&self, id: &str) -> bool {
        if c::recipe_index(id).is_some() || c::BLUEPRINTS.iter().any(|b| b.build == id) {
            return self.s.gate_open(id);
        }
        true
    }

    pub(crate) fn forecast(&self, a: &c::Action) -> Block {
        let ms = self.s.duration(a.id);
        let n = c::FORECAST_MS / ms.max(1);
        let per = self.s.xp_for(a);
        let gained = n * per;
        let x = self.s.xp[a.skill];
        let l0 = xp::level(x);
        let l1 = xp::level(x + gained);
        let made = n * a.makes.1 * (100 + self.s.double_pct(a.id)) / 100;
        let mut chips = vec![
            chip(
                "g",
                self.t.t(
                    "plus_item",
                    &[
                        ("n", num(if a.needs.is_empty() { made } else { n * a.makes.1 })),
                        ("item", self.t.item(a.makes.0)),
                    ],
                ),
            ),
            chip("x", self.t.t("plus_xp", &[("n", num(gained))])),
        ];
        if l1 > l0 {
            chips.push(chip(
                "x",
                self.t
                    .t("lvl_to", &[("a", l0.to_string()), ("b", l1.to_string())]),
            ));
        }
        if !a.needs.is_empty() {
            let used: Vec<String> = a
                .needs
                .iter()
                .map(|(i, k)| self.t.t("n_item", &[("n", num(n * k)), ("item", self.t.item(i))]))
                .collect();
            chips.push(chip("t", self.t.t("uses_list", &[("list", used.join(", "))])));
        }
        if l0 < xp::MAX_LEVEL && per > 0 {
            let need = xp::at(l0 + 1) - x;
            let left = need.div_ceil(per) * ms - self.s.progress_ms.min(ms);
            chips.push(chip("t", self.t.t("next_level_in", &[("left", dur(left))])));
        }
        Block {
            title: self.t.s("if_you_leave"),
            chips,
            ..block("forecast")
        }
    }

    pub(crate) fn action_card(&self, a: &c::Action, on: bool) -> Card {
        let skill = &c::SKILLS[a.skill];
        let w = self.t.skill(skill.id);
        let gated = !self.s.gate_open(a.id);
        let open = self.s.level(a.skill) >= a.level && !gated;
        let ml = self.s.mastery(a.id);
        let missing = self.s.missing(a.id);
        let sub = if gated {
            self.t.s(&format!("gate_{}", a.id))
        } else if !open {
            self.t.t(
                "needs_level",
                &[("skill", w.name.clone()), ("n", a.level.to_string())],
            )
        } else if a.needs.is_empty() {
            self.t.t("gives", &[("item", self.t.item(a.makes.0))])
        } else {
            a.needs
                .iter()
                .map(|(i, n)| {
                    self.t.t(
                        "need_have",
                        &[
                            ("n", n.to_string()),
                            ("item", self.t.item(i)),
                            ("have", num(self.s.have(i))),
                        ],
                    )
                })
                .collect::<Vec<_>>()
                .join(" + ")
        };
        let ms = self.s.duration(a.id);
        let mut chips = vec![
            chip("t", secs(ms)),
            chip("x", self.t.t("plus_xp", &[("n", num(self.s.xp_for(a)))])),
        ];
        // How many of what it makes you have (T18).
        if open {
            chips.push(chip(
                "g",
                self.t.t(
                    "have_chip",
                    &[
                        ("item", self.t.item(a.makes.0)),
                        ("n", num(self.s.have(a.makes.0))),
                    ],
                ),
            ));
        }
        if a.makes.1 > 1 {
            chips.push(chip("g", self.t.t("makes_n", &[("n", a.makes.1.to_string())])));
        }
        if a.region > 0 {
            chips.push(chip("p", self.t.s(&format!("region_{}", c::REGIONS[a.region].0))));
        }
        if open && self.s.rare_tree() == Some(a.id) {
            let left = c::RARE_TREE_MS - self.s.sim_ms % c::RARE_TREE_MS;
            chips.push(chip("fire", self.t.t("rare_tree", &[("left", dur(left))])));
        }
        if open && a.needs.is_empty() {
            chips.push(chip(
                "m",
                self.t
                    .t("double_pct", &[("n", self.s.double_pct(a.id).to_string())]),
            ));
        }
        if open && self.s.keep_pct(a.id) > 0 {
            chips.push(chip(
                "m",
                self.t.t("keep_pct", &[("n", self.s.keep_pct(a.id).to_string())]),
            ));
        }
        let checks = if open {
            c::CHECKPOINTS
                .iter()
                .map(|&cp| Check {
                    text: self.t.t(&format!("cp_{}_{cp}", kind_key(skill.kind)), &[]),
                    on: ml >= cp,
                })
                .collect()
        } else {
            Vec::new()
        };
        let main = if on {
            button(self.t.s("stop"), "do", a.id).tone("stop")
        } else if !open {
            button(self.t.s("locked"), "do", a.id).enabled(false)
        } else if let Some(item) = &missing {
            button(self.t.t("need_item", &[("item", self.t.item(item))]), "do", a.id).enabled(false)
        } else {
            button(
                format!("{} {}", w.verb, self.t.action(a.id).to_lowercase()),
                "do",
                a.id,
            )
            .tone("primary")
        };
        let mut buttons = vec![main];
        if open && self.s.plan.len() < self.s.plan_max() {
            buttons.push(button(self.t.s("plan_add"), "plan.add", a.id));
        }
        Card {
            icon: a.id.into(),
            title: self.t.action(a.id),
            sub,
            tag: if open {
                self.t.t("m_tag", &[("n", ml.to_string())])
            } else if gated {
                self.t.s("rank_tag")
            } else {
                self.t.t("lock_tag", &[("n", a.level.to_string())])
            },
            tone: if open { "mast".into() } else { "lock".into() },
            locked: !open,
            active: on,
            dur: format!("{ms}ms"),
            delay: if on {
                format!("-{}ms", self.s.progress_ms)
            } else {
                String::new()
            },
            chips,
            bar: if open { Some(self.mastery_bar(a.id)) } else { None },
            checks,
            buttons,
        }
    }

    // ---------- combat ----------

    fn combat(&self) -> Vec<Block> {
        let st = &c::STYLES[self.s.style];
        let fighting = self.s.active.as_deref().and_then(|id| self.s.foe(id));
        let in_dungeon = self.s.active.as_deref().and_then(c::dungeon_index);
        let e = &c::ENEMIES[fighting.unwrap_or(0)];
        let mut out = vec![self.head(
            "combat",
            self.t.skill("combat").name.clone(),
            self.t
                .t("hitpoints_tag", &[("n", self.s.level(c::HITPOINTS).to_string())]),
            "hp",
        )];
        let styles = c::STYLES
            .iter()
            .enumerate()
            .map(|(k, s)| {
                let strong: Vec<String> = c::ENEMIES
                    .iter()
                    .filter(|e| e.weak == s.skill && self.s.unlocked(e.id))
                    .map(|e| self.t.enemy(e.id).to_lowercase())
                    .collect();
                // a long list becomes a count once the regions open (T15)
                let strong_text = if strong.len() > 4 {
                    self.t.t("strong_vs_n", &[("n", strong.len().to_string())])
                } else {
                    self.t.t("strong_vs", &[("list", strong.join(", "))])
                };
                button(
                    self.t.t(
                        "style_lvl",
                        &[
                            ("style", self.skill_name(s.skill)),
                            ("n", self.s.level(s.skill).to_string()),
                        ],
                    ),
                    "style",
                    &k.to_string(),
                )
                .icon(c::SKILLS[s.skill].id)
                .below(strong_text)
                .pressed(k == self.s.style)
            })
            .collect();
        out.push(Block {
            buttons: styles,
            ..block("styles")
        });
        out.push(Block {
            bar: Some(self.xp_bar(st.skill)),
            ..block("bar")
        });
        out.push(self.fold("levels", "fold_levels"));
        if self.unfolded.iter().any(|k| k == "levels") {
            out.push(self.combat_levels());
        }
        let (ins, outs) = io_of("combat");
        out.push(self.links("combat", &ins, &outs));
        out.push(self.scene(
            e.id,
            match fighting {
                Some(_) => self.t.t(
                    "fighting",
                    &[
                        ("enemy", self.t.enemy(e.id)),
                        ("style", self.skill_name(st.skill)),
                    ],
                ),
                None => self.t.s("pick_enemy"),
            },
            fighting.is_some(),
            st.ms,
        ));
        if fighting.is_some() {
            out.push(Block {
                dur: format!("{}ms", st.ms),
                delay: format!("-{}ms", self.s.progress_ms),
                ..block("progress")
            });
        }
        let max_hp = self.s.max_hp();
        let foe = if fighting.is_some() { self.s.foe_hp } else { e.hp };
        out.push(Block {
            title: self.t.s("you"),
            label2: self.t.enemy(e.id),
            bar: Some(Bar {
                tone: "hp".into(),
                width: width(self.s.hp * 10_000 / max_hp.max(1)),
                left: self.t.t("of", &[("have", num(self.s.hp)), ("need", num(max_hp))]),
                right: String::new(),
            }),
            bar2: Some(Bar {
                tone: "hp".into(),
                width: width(foe * 10_000 / e.hp.max(1)),
                left: self.t.t("of", &[("have", num(foe)), ("need", num(e.hp))]),
                right: String::new(),
            }),
            ..block("fight")
        });
        let food = match self.s.food() {
            Some((f, heal)) => self.t.t(
                "food_have",
                &[
                    ("n", num(self.s.have(f))),
                    ("item", self.t.item(f)),
                    ("heal", heal.to_string()),
                ],
            ),
            None => self.t.s("food_none"),
        };
        let mut chips = vec![chip("h", food)];
        chips.push(match self.s.armour() {
            ("", _, _) => chip("t", self.t.s("armour_none")),
            (item, tier, _) => chip(
                "t",
                self.t.t(
                    "armour_have",
                    &[
                        ("item", self.t.item(item)),
                        ("pct", (tier * c::ARMOUR_PCT_PER_TIER).to_string()),
                    ],
                ),
            ),
        });
        // A held weapon is not used up and only the best counts; ammunition
        // is spent one a swing (T21: say which, so stacking is clear).
        for (list, key) in [(st.held, "supply_held"), (st.spent, "supply_spent")] {
            if list.is_empty() {
                continue;
            }
            let (best, tier) = if key == "supply_held" {
                self.s.weapon_of(self.s.style)
            } else {
                self.s.tier(list)
            };
            let extra = if key == "supply_held" {
                self.s.weapon_hit(&best)
            } else {
                0
            };
            chips.push(match best.as_str() {
                "" => chip(
                    "bad",
                    self.t
                        .t("supply_none", &[("item", self.t.item(list[list.len() - 1].0))]),
                ),
                b => chip(
                    "t",
                    self.t.t(
                        key,
                        &[
                            ("item", self.t.item(b)),
                            ("n", num(self.s.have(b))),
                            ("hit", num(tier * c::MAX_HIT_PER_TIER + extra)),
                        ],
                    ),
                ),
            });
        }
        out.push(Block {
            chips,
            ..block("chips")
        });
        if let Some(ei) = fighting {
            let e = &c::ENEMIES[ei];
            let swings = c::FORECAST_MS / st.ms;
            let strong = e.weak == st.skill;
            let hit = if strong { c::HIT_PCT_STRONG } else { c::HIT_PCT };
            let avg = hit * (1 + self.s.max_hit(e)) / 200;
            let kills = swings * avg / e.hp;
            let gained = swings * avg * c::STYLE_XP_TENTHS * self.s.mult_bp() / 100_000;
            let mut chips = vec![
                chip("g", self.t.t("about_kills", &[("n", num(kills))])),
                chip(
                    "x",
                    self.t.t(
                        "plus_style_xp",
                        &[("n", num(gained)), ("style", self.skill_name(st.skill))],
                    ),
                ),
            ];
            if !st.spent.is_empty() {
                chips.push(chip(
                    "t",
                    self.t.t(
                        "uses_n",
                        &[
                            ("n", num(swings)),
                            ("item", self.t.item(&self.s.tier(st.spent).0)),
                        ],
                    ),
                ));
            }
            chips.push(chip("h", self.t.s("retreat_note")));
            out.push(Block {
                title: self.t.s("if_you_leave"),
                chips,
                ..block("forecast")
            });
        }
        if !self.s.blows.is_empty() {
            out.push(self.fold("log", "fold_log"));
            if self.unfolded.iter().any(|k| k == "log") {
                out.push(self.combat_log());
            }
        }
        let mut dungeons: Vec<Card> = c::DUNGEONS
            .iter()
            .enumerate()
            .filter(|(_, d)| self.s.unlocked(d.id) && self.region_shown(d.region))
            .map(|(k, d)| self.dungeon_card(d, in_dungeon == Some(k)))
            .collect();
        if !dungeons.is_empty() {
            dungeons.reverse();
            out.push(Block {
                title: self.t.s("dungeons_title"),
                cards: dungeons,
                ..block("cards")
            });
        }
        // the newest region's enemies first
        let mut list: Vec<&c::Enemy> = c::ENEMIES
            .iter()
            .filter(|en| self.s.unlocked(en.id) && self.region_shown(en.region))
            .collect();
        list.sort_by_key(|en| (std::cmp::Reverse(en.region), en.level));
        if !self.s.view_sort.is_empty() {
            self.order(&mut list, |en| (en.level, en.region), |en| self.t.enemy(en.id));
        }
        let cards = list
            .into_iter()
            .map(|en| self.enemy_card(en, in_dungeon.is_none() && fighting == c::enemy_index(en.id)))
            .collect();
        out.push(Block {
            cards,
            ..block("cards")
        });
        out
    }

    /// The skill's mastery pool (T27): a bar with the next bonus, and,
    /// opened, each step and the highest-mastery bonus.
    fn pool_blocks(&self, i: usize) -> Vec<Block> {
        let (have, most) = self.s.pool(i);
        let pct = self.s.pool_pct(i);
        let steps = self.s.pool_steps(i);
        let names = [
            self.t.t("pool_b1", &[("n", c::POOL_XP_PCT.to_string())]),
            self.t.t("pool_b2", &[("n", c::POOL_FASTER_PCT.to_string())]),
            self.t.t("pool_b3", &[("n", c::POOL_LUCK_PCT.to_string())]),
            self.t.t("pool_b4", &[("n", c::POOL_XP_MORE_PCT.to_string())]),
            self.t.s("pool_b5"),
        ];
        let next = c::POOL_STEPS.get(steps).map_or(self.t.s("pool_all"), |&at| {
            self.t.t(
                "pool_next",
                &[("at", at.to_string()), ("what", names[steps].clone())],
            )
        });
        let mut out = vec![
            Block {
                bar: Some(Bar {
                    tone: "mast".into(),
                    width: width(have * 10_000 / most.max(1)),
                    left: self.t.t(
                        "pool_bar",
                        &[("pct", pct.to_string()), ("have", num(have)), ("most", num(most))],
                    ),
                    right: next,
                }),
                ..block("bar")
            },
            self.fold("pool", "fold_pool"),
        ];
        if self.unfolded.iter().any(|k| k == "pool") {
            let mut rows: Vec<Row> = c::POOL_STEPS
                .iter()
                .zip(names)
                .enumerate()
                .map(|(k, (at, what))| Row {
                    icon: if k < steps {
                        "rank".into()
                    } else {
                        "rank_locked".into()
                    },
                    title: self.t.t("pool_step", &[("at", at.to_string()), ("what", what)]),
                    tone: if k < steps { "up".into() } else { "used".into() },
                    ..Row::default()
                })
                .collect();
            let top = self.s.top_mastery(i);
            rows.push(Row {
                icon: "prestige".into(),
                title: self.t.t("pool_top", &[("n", top.to_string())]),
                sub: self.t.s("pool_top_sub"),
                ..Row::default()
            });
            out.push(Block {
                title: self.t.s("pool_title"),
                text: self.t.s("pool_about"),
                rows,
                ..block("rows")
            });
        }
        out
    }

    /// A button that opens or folds a section (T25).
    fn fold(&self, key: &str, label: &str) -> Block {
        let open = self.unfolded.iter().any(|k| k == key);
        Block {
            buttons: vec![button(
                self.t.t(
                    if open { "fold_hide" } else { "fold_show" },
                    &[("what", self.t.s(label))],
                ),
                "fold",
                key,
            )
            .pressed(open)],
            ..block("buttons")
        }
    }

    /// What each combat level does (T19): the max hit a style's level gives
    /// and what the next level adds; the health Hitpoints gives.
    fn combat_levels(&self) -> Block {
        let mut rows: Vec<Row> = c::STYLES
            .iter()
            .enumerate()
            .map(|(k, s)| {
                let l = self.s.level(s.skill);
                let next = (l + 1).min(xp::MAX_LEVEL);
                Row {
                    icon: c::SKILLS[s.skill].id.into(),
                    title: self.t.t(
                        "lvl_of",
                        &[("skill", self.skill_name(s.skill)), ("n", l.to_string())],
                    ),
                    sub: self.t.t(
                        "style_level_does",
                        &[
                            ("hit", num(self.s.level_hit(s.skill))),
                            ("per", c::MAX_HIT_PER_LEVEL.to_string()),
                            ("gear", num(self.s.gear_hit(k))),
                            ("strong", c::MAX_HIT_STRONG.to_string()),
                        ],
                    ),
                    chip: self.t.t(
                        "next_level_hit",
                        &[
                            ("n", next.to_string()),
                            ("hit", num(c::MAX_HIT_BASE + next as u64 * c::MAX_HIT_PER_LEVEL)),
                        ],
                    ),
                    tone: if k == self.s.style {
                        "up".into()
                    } else {
                        String::new()
                    },
                    ..Row::default()
                }
            })
            .collect();
        let l = self.s.level(c::HITPOINTS);
        let next = (l + 1).min(xp::MAX_LEVEL);
        let from_level = l as u64 * c::HP_PER_LEVEL;
        rows.push(Row {
            icon: "hitpoints".into(),
            title: self.t.t(
                "lvl_of",
                &[("skill", self.skill_name(c::HITPOINTS)), ("n", l.to_string())],
            ),
            sub: self.t.t(
                "hp_level_does",
                &[
                    ("hp", num(from_level)),
                    ("per", c::HP_PER_LEVEL.to_string()),
                    ("gear", num(self.s.max_hp() - from_level)),
                ],
            ),
            chip: self.t.t(
                "next_level_hp",
                &[
                    ("n", next.to_string()),
                    ("hp", num(next as u64 * c::HP_PER_LEVEL)),
                ],
            ),
            ..Row::default()
        });
        Block {
            title: self.t.s("levels_do_title"),
            text: self.t.t(
                "levels_do_text",
                &[
                    ("hit", c::HIT_PCT.to_string()),
                    ("strong", c::HIT_PCT_STRONG.to_string()),
                ],
            ),
            rows,
            ..block("rows")
        }
    }

    /// The combat log (T19): the newest lines first.
    fn combat_log(&self) -> Block {
        let rows = self
            .s
            .blows
            .iter()
            .rev()
            .map(|b| {
                let name = match b.kind.as_str() {
                    "eat" | "drop" => self.t.item(&b.what),
                    _ => self.t.enemy(&b.what),
                };
                Row {
                    icon: match b.kind.as_str() {
                        "eat" | "drop" => b.what.clone(),
                        "hurt" => "hitpoints".into(),
                        _ => b.what.clone(),
                    },
                    title: self
                        .t
                        .t(&format!("blow_{}", b.kind), &[("what", name), ("n", num(b.n))]),
                    tone: match b.kind.as_str() {
                        "hit" | "kill" => "up".into(),
                        "hurt" => "bad".into(),
                        "drop" => "rare".into(),
                        _ => String::new(),
                    },
                    ..Row::default()
                }
            })
            .collect();
        Block {
            title: self.t.s("combat_log_title"),
            rows,
            ..block("rows")
        }
    }

    /// Weapon mastery (T23): the style's weapons, each with its level, its
    /// XP speed against the gate and what its level adds; the weapon in
    /// hand with a graph of its gate; a button to fight with each.
    fn weapons(&self) -> Vec<Block> {
        let k = self.s.style;
        let st = &c::STYLES[k];
        let (in_hand, _) = self.s.weapon_of(k);
        let mut out = vec![self.head(
            "weapons",
            self.t.s("tab_weapons"),
            self.t.t("weapons_tag", &[("style", self.skill_name(st.skill))]),
            "xp",
        )];
        out.push(Block {
            text: self.t.t(
                "weapons_intro",
                &[
                    ("hit", c::WEAPON_HIT_EVERY.to_string()),
                    ("land", c::WEAPON_LAND_EVERY.to_string()),
                    ("per", c::WEAPON_GATE_PER_TIER.to_string()),
                    ("min", c::WEAPON_RATE_MIN.to_string()),
                ],
            ),
            ..block("note")
        });
        out.push(Block {
            buttons: c::STYLES
                .iter()
                .enumerate()
                .map(|(j, s)| {
                    button(self.skill_name(s.skill), "style", &j.to_string())
                        .icon(c::SKILLS[s.skill].id)
                        .pressed(j == k)
                })
                .collect(),
            ..block("buttons")
        });
        if !in_hand.is_empty() {
            out.push(self.gate_block(k, &in_hand));
        }
        let mut list: Vec<(&str, u64)> = st.held.to_vec();
        list.sort_by_key(|(i, t)| (*t, *i));
        let (shown, hidden): (Vec<_>, Vec<_>) = list
            .into_iter()
            .partition(|(i, _)| self.s.have(i) > 0 || self.s.weapon_xp.contains_key(*i));
        let rows: Vec<Row> = shown
            .iter()
            .map(|&(i, tier)| {
                let here = i == in_hand;
                Row {
                    icon: i.into(),
                    title: self.t.t(
                        "weapon_title",
                        &[
                            ("item", self.t.item(i)),
                            ("n", self.s.weapon_level(i).to_string()),
                        ],
                    ),
                    sub: self.t.t(
                        "weapon_sub",
                        &[
                            ("tier", tier.to_string()),
                            ("rate", self.s.weapon_rate(k, i).to_string()),
                            ("hit", self.s.weapon_hit(i).to_string()),
                            ("land", self.s.weapon_land(i).to_string()),
                            ("have", num(self.s.have(i))),
                        ],
                    ),
                    chip: if here { self.t.s("in_hand") } else { String::new() },
                    tone: if here { "up".into() } else { String::new() },
                    buttons: vec![button(self.t.s("wield"), "wield", i)
                        .pressed(here)
                        .enabled(self.s.have(i) > 0)],
                }
            })
            .collect();
        if rows.is_empty() {
            out.push(Block {
                text: self.t.s("weapons_none"),
                ..block("note")
            });
        } else {
            out.push(Block {
                title: self.t.t("weapons_of", &[("style", self.skill_name(st.skill))]),
                rows,
                ..block("rows")
            });
        }
        if !hidden.is_empty() {
            out.push(Block {
                text: self.t.t("weapons_more", &[("n", hidden.len().to_string())]),
                ..block("note")
            });
        }
        out
    }

    /// The graph of a weapon's XP speed against its weaker weapons' levels,
    /// with a mark where you stand (IG-Q60 A).
    fn gate_block(&self, style: usize, item: &str) -> Block {
        let (need, have) = self.s.weapon_gate(style, item);
        let rate = self.s.weapon_rate(style, item);
        // x: 0 to a quarter past the gate; y: 0% at the bottom, 100% at the top.
        let xmax = (need + need / 4).max(have).max(1);
        let x = |v: u64| v * 1_000 / xmax;
        let y = |pct: u64| 500 - pct * 5;
        let floor_at = need * c::WEAPON_RATE_MIN / 100;
        let line = if need == 0 {
            format!("M0 {} L1000 {}", y(100), y(100))
        } else {
            format!(
                "M0 {low} L{a} {low} L{b} {top} L1000 {top}",
                low = y(c::WEAPON_RATE_MIN),
                a = x(floor_at),
                b = x(need),
                top = y(100),
            )
        };
        Block {
            title: self.t.t("gate_title", &[("item", self.t.item(item))]),
            text: if need == 0 {
                self.t.s("gate_free")
            } else {
                self.t.t(
                    "gate_text",
                    &[
                        ("have", have.to_string()),
                        ("need", need.to_string()),
                        ("rate", rate.to_string()),
                    ],
                )
            },
            curve: Some(Curve {
                view: "0 0 1000 500".into(),
                line,
                x: format!("{}%", have.min(xmax) * 100 / xmax),
                y: format!("{}%", 100 - rate),
                top: self.t.s("gate_top"),
                low: self.t.t("gate_low", &[("n", c::WEAPON_RATE_MIN.to_string())]),
                left: "0".into(),
                right: self.t.t("gate_right", &[("n", xmax.to_string())]),
            }),
            ..block("curve")
        }
    }

    fn dungeon_card(&self, d: &c::Dungeon, on: bool) -> Card {
        let n = d.stages.len();
        let stage = if on { self.s.dungeon_stage.min(n - 1) } else { 0 };
        let foe = d.stages[stage];
        let chips = vec![
            chip(
                "t",
                self.t.t(
                    "dungeon_stage",
                    &[
                        ("k", (stage + 1).to_string()),
                        ("n", n.to_string()),
                        ("enemy", self.t.enemy(foe)),
                    ],
                ),
            ),
            chip("g", self.t.s("dungeon_chest")),
            chip("p", self.t.s(&format!("region_{}", c::REGIONS[d.region].0))),
        ];
        let main = if on {
            button(self.t.s("retreat"), "do", d.id).tone("stop")
        } else if let Some(item) = self.s.missing(d.id) {
            button(self.t.t("need_item", &[("item", self.t.item(&item))]), "do", d.id).enabled(false)
        } else {
            button(self.t.s("dungeon_go"), "do", d.id).tone("primary")
        };
        Card {
            icon: d.stages[n - 1].into(),
            title: self.t.s(&format!("dungeon_{}", d.id)),
            sub: self.t.t(
                "dungeon_sub",
                &[("n", n.to_string()), ("boss", self.t.enemy(d.stages[n - 1]))],
            ),
            tag: self.t.s("dungeon_tag"),
            tone: "mast".into(),
            active: on,
            dur: format!("{}ms", c::STYLES[self.s.style].ms),
            delay: if on {
                format!("-{}ms", self.s.progress_ms)
            } else {
                String::new()
            },
            chips,
            buttons: vec![main],
            ..Card::default()
        }
    }

    fn enemy_card(&self, e: &c::Enemy, on: bool) -> Card {
        let st = &c::STYLES[self.s.style];
        let strong = e.weak == st.skill;
        let mut chips = vec![chip(
            if strong { "g" } else { "t" },
            self.t.t(
                if strong { "weak_to_yes" } else { "weak_to" },
                &[("style", self.skill_name(e.weak))],
            ),
        )];
        for (item, n, one_in) in e.drops {
            let what = if *n > 1 {
                self.t
                    .t("n_item", &[("n", n.to_string()), ("item", self.t.item(item))])
            } else {
                self.t.item(item)
            };
            chips.push(if *one_in > 1 {
                chip(
                    "g",
                    self.t
                        .t("drop_rare", &[("what", what), ("n", one_in.to_string())]),
                )
            } else {
                chip("t", what)
            });
        }
        if e.gold > 0 {
            chips.push(chip("g", self.t.t("gold_n", &[("n", num(e.gold))])));
        }
        for rc in c::RECIPES.iter().filter(|rc| rc.enemy == e.id) {
            let known = self.s.recipes.iter().any(|k| k == rc.id);
            chips.push(chip(
                if known { "t" } else { "m" },
                self.t.t(
                    if known { "recipe_known_chip" } else { "recipe_chip" },
                    &[("what", self.t.action(rc.id)), ("n", rc.one_in.to_string())],
                ),
            ));
        }
        for (reg, row) in c::CHAMPIONS.iter().enumerate() {
            if let Some(k) = row.iter().position(|&ch| ch == e.id) {
                let rank = c::NOBLE_RANKS[reg][k + c::DEED_FROM].id;
                chips.push(chip(
                    "p",
                    self.t
                        .t("champion_chip", &[("rank", self.t.s(&format!("noble_{rank}")))]),
                ));
            }
        }
        if e.region > 0 {
            chips.push(chip("p", self.t.s(&format!("region_{}", c::REGIONS[e.region].0))));
        }
        let missing = self.s.missing(e.id);
        let main = if on {
            button(self.t.s("retreat"), "do", e.id).tone("stop")
        } else if let Some(item) = &missing {
            button(self.t.t("need_item", &[("item", self.t.item(item))]), "do", e.id).enabled(false)
        } else {
            button(
                self.t.t(
                    "fight_with",
                    &[("style", self.skill_name(st.skill).to_lowercase())],
                ),
                "do",
                e.id,
            )
            .tone("primary")
        };
        let mut buttons = vec![main];
        if self.s.plan.len() < self.s.plan_max() {
            buttons.push(button(self.t.s("plan_add"), "plan.add", e.id));
        }
        Card {
            icon: e.id.into(),
            title: self.t.enemy(e.id),
            sub: self.t.t(
                "enemy_sub",
                &[
                    ("hp", e.hp.to_string()),
                    ("hit", e.hit.to_string()),
                    ("kills", num(self.s.kills.get(e.id).copied().unwrap_or(0))),
                ],
            ),
            tag: self.t.t("lvl_tag", &[("n", e.level.to_string())]),
            tone: "lock".into(),
            active: on,
            dur: format!("{}ms", st.ms),
            delay: if on {
                format!("-{}ms", self.s.progress_ms)
            } else {
                String::new()
            },
            chips,
            buttons,
            ..Card::default()
        }
    }

    // ---------- bank, shop, plan, prestige, settings ----------

    fn bank(&self) -> Vec<Block> {
        let items: Vec<(&String, &u64)> = self.s.bank.iter().filter(|(_, n)| **n > 0).collect();
        let worth: u64 = items.iter().map(|(k, n)| c::value(k) * **n).sum();
        let mut out = vec![self.head(
            "bank",
            self.t.s("tab_bank"),
            self.t.t("kinds", &[("n", items.len().to_string())]),
            "xp",
        )];
        out.push(Block {
            chips: vec![
                chip("g", self.t.t("worth", &[("n", num(worth))])),
                chip("t", self.t.s("tap_to_sell")),
            ],
            ..block("chips")
        });
        if items.is_empty() {
            out.push(Block {
                text: self.t.s("bank_empty"),
                ..block("note")
            });
        }
        out.extend(self.stacks(&items));
        out
    }

    /// The encyclopedia (T19): every item, its picture, what makes it and
    /// what uses it. A search, or one group at a time.
    fn book(&self) -> Vec<Block> {
        let mut all: Vec<&'static str> = c::VALUES
            .iter()
            .map(|(i, _)| *i)
            .filter(|i| self.region_shown(self.item_region(i)))
            .collect();
        all.sort_by_key(|i| self.t.item(i).to_lowercase());
        self.order(
            &mut all,
            |i| (self.item_level(i), self.item_region(i)),
            |i| self.t.item(i),
        );
        let mut out = vec![self.head(
            "book",
            self.t.s("tab_book"),
            self.t.t("kinds", &[("n", all.len().to_string())]),
            "xp",
        )];
        out.push(Block {
            title: self.t.s("book_search"),
            act: "book.search".into(),
            value: self.book_search.to_string(),
            text: self.t.s("book_search_hint"),
            ..block("search")
        });
        let q = self.book_search.trim().to_lowercase();
        if !q.is_empty() {
            let found: Vec<&str> = all
                .iter()
                .copied()
                .filter(|i| self.t.item(i).to_lowercase().contains(&q))
                .collect();
            if found.is_empty() {
                out.push(Block {
                    text: self.t.s("bank_none_match"),
                    ..block("note")
                });
            } else {
                let more = found.len().saturating_sub(c::BOOK_ROWS);
                out.push(Block {
                    rows: found.iter().take(c::BOOK_ROWS).map(|i| self.entry(i)).collect(),
                    ..block("rows")
                });
                if more > 0 {
                    out.push(Block {
                        text: self.t.t("book_more", &[("n", more.to_string())]),
                        ..block("note")
                    });
                }
            }
            return out;
        }
        let mut groups: Vec<(&'static str, Vec<&'static str>)> = Vec::new();
        for i in all {
            let tab = maker_tab(i);
            match groups.iter_mut().find(|(t, _)| *t == tab) {
                Some((_, list)) => list.push(i),
                None => groups.push((tab, vec![i])),
            }
        }
        groups.sort_by_key(|(t, _)| TABS.iter().position(|x| x == t).unwrap_or(TABS.len()));
        out.push(Block {
            text: self.t.s("book_intro"),
            buttons: groups
                .iter()
                .map(|(tab, list)| {
                    let name = if *tab == "other" {
                        self.t.s("bank_other")
                    } else {
                        self.tab_name(tab)
                    };
                    button(name, "book.group", tab)
                        .icon(if *tab == "other" { "book" } else { tab })
                        .below(list.len().to_string())
                        .pressed(self.book_group == *tab)
                })
                .collect(),
            ..block("buttons")
        });
        if let Some((tab, list)) = groups.iter().find(|(t, _)| *t == self.book_group) {
            out.push(Block {
                title: if *tab == "other" {
                    self.t.s("bank_other")
                } else {
                    self.tab_name(tab)
                },
                rows: list.iter().map(|i| self.entry(i)).collect(),
                ..block("rows")
            });
        }
        out
    }

    /// What an item does, exactly, with the core's own numbers (T22): food
    /// in a fight, compost on a crop, a weapon's or armour's strength, a
    /// trinket, a gadget, a seed, an instrument, furniture, fuel.
    fn effects(&self, item: &str) -> Vec<String> {
        let t = &self.t;
        let mut out = Vec::new();
        if let Some(&(_, heal)) = c::FOOD.iter().find(|(f, _)| *f == item) {
            out.push(t.t(
                "fx_food",
                &[
                    ("heal", heal.to_string()),
                    ("below", c::EAT_BELOW_PCT.to_string()),
                ],
            ));
        }
        match item {
            "ashes" => out.push(t.t("fx_compost", &[("pct", c::COMPOST_GROW_PCT.to_string())])),
            "fine_ashes" => out.push(t.t("fx_compost", &[("pct", c::FINE_COMPOST_GROW_PCT.to_string())])),
            "spices" => out.push(t.t("fx_spices", &[("pct", c::SPICE_HEAL_PCT.to_string())])),
            "goblin_hot_pot" => out.push(t.t(
                "fx_hot_pot",
                &[
                    ("pct", c::HOT_POT_GOLD_PCT.to_string()),
                    ("t", dur(c::HOT_POT_MS)),
                ],
            )),
            _ => {}
        }
        for s in c::STYLES.iter() {
            if let Some(&(_, tier)) = s.held.iter().find(|(i, _)| *i == item) {
                out.push(t.t(
                    "fx_held",
                    &[
                        ("hit", (tier * c::MAX_HIT_PER_TIER).to_string()),
                        ("style", self.skill_name(s.skill)),
                    ],
                ));
            }
            if let Some(&(_, tier)) = s.spent.iter().find(|(i, _)| *i == item) {
                out.push(t.t(
                    "fx_spent",
                    &[
                        ("hit", (tier * c::MAX_HIT_PER_TIER).to_string()),
                        ("style", self.skill_name(s.skill)),
                    ],
                ));
            }
        }
        if let Some(&(_, tier, hp)) = c::ARMOUR.iter().find(|(a, _, _)| *a == item) {
            out.push(t.t(
                "fx_armour",
                &[
                    ("pct", (tier * c::ARMOUR_PCT_PER_TIER).to_string()),
                    ("hp", hp.to_string()),
                ],
            ));
        }
        if let Some(&(_, kind, n)) = c::TRINKETS.iter().find(|(i, _, _)| *i == item) {
            let key = match kind {
                c::Trinket::Power => "fx_power",
                c::Trinket::Vigour => "fx_vigour",
                c::Trinket::Wealth => "fx_wealth",
                c::Trinket::Warding => "fx_warding",
            };
            out.push(t.t(key, &[("n", n.to_string())]));
        }
        if let Some(&(_, skill)) = c::GADGETS.iter().find(|(g, _)| *g == item) {
            out.push(t.t(
                "fx_gadget",
                &[
                    ("skill", self.skill_name(skill)),
                    ("pct", c::GADGET_PCT.to_string()),
                ],
            ));
        }
        if let Some(cr) = c::CROPS.iter().find(|cr| cr.seed == item) {
            out.push(t.t(
                "fx_seed",
                &[
                    ("t", dur(cr.grow_ms)),
                    ("min", cr.yield_min.to_string()),
                    ("max", cr.yield_max.to_string()),
                    ("crop", t.item(cr.produce)),
                ],
            ));
        }
        let songs: Vec<String> = c::SONG_INSTRUMENTS
            .iter()
            .filter(|(_, i)| *i == item)
            .map(|(s, _)| t.action(s))
            .collect();
        if !songs.is_empty() {
            out.push(t.t("fx_instrument", &[("list", songs.join(", "))]));
        }
        if let Some(&(_, room)) = c::FURNITURE.iter().find(|(f, _)| *f == item) {
            out.push(t.t("fx_furniture", &[("room", t.s(&format!("room_{room}")))]));
        }
        if c::FUEL.contains(&item) {
            out.push(t.s("fx_fuel"));
        }
        out
    }

    /// One item's encyclopedia row: what makes it, what uses it, what it sells for.
    fn entry(&self, item: &str) -> Row {
        let mut from: Vec<String> = Vec::new();
        let push = |list: &mut Vec<String>, s: String| {
            if !list.contains(&s) {
                list.push(s);
            }
        };
        for a in c::ACTIONS.iter().filter(|a| a.makes.0 == item) {
            push(
                &mut from,
                self.t.t(
                    "lvl_of",
                    &[("skill", self.skill_name(a.skill)), ("n", a.level.to_string())],
                ),
            );
        }
        for cr in c::CROPS.iter().filter(|cr| cr.produce == item) {
            push(
                &mut from,
                self.t.t(
                    "lvl_of",
                    &[
                        ("skill", self.skill_name(c::FARMING)),
                        ("n", cr.level.to_string()),
                    ],
                ),
            );
        }
        for e in c::ENEMIES.iter().filter(|e| e.drops.iter().any(|d| d.0 == item)) {
            push(&mut from, self.t.enemy(e.id));
        }
        for t in c::TOWNS.iter().filter(|t| t.sells == item) {
            push(&mut from, self.t.s(&format!("town_{}", t.id)));
        }
        for m in c::MACHINES.iter().filter(|m| m.makes.0 == item) {
            push(&mut from, self.t.action(m.id));
        }
        if c::GEMS.iter().any(|g| g.0 == item) {
            push(&mut from, self.skill_name(c::MINING));
        }
        if c::CROPS.iter().any(|cr| cr.seed == item) {
            push(&mut from, self.t.s("tab_shop"));
        }
        if self
            .s
            .shelves()
            .iter()
            .any(|(_, _, items, _)| items.contains(&item))
        {
            push(&mut from, self.t.s("tab_shop"));
        }
        let mut uses: Vec<String> = Vec::new();
        for a in c::ACTIONS
            .iter()
            .filter(|a| a.needs.iter().any(|(i, _)| *i == item))
        {
            push(&mut uses, self.skill_name(a.skill));
        }
        if c::SONG_INSTRUMENTS.iter().any(|(_, i)| *i == item) {
            push(&mut uses, self.skill_name(c::PERFORMANCE));
        }
        if c::CROPS.iter().any(|cr| cr.seed == item) {
            push(&mut uses, self.skill_name(c::FARMING));
        }
        for s in c::STYLES
            .iter()
            .filter(|s| s.held.iter().chain(s.spent.iter()).any(|(i, _)| *i == item))
        {
            push(&mut uses, self.skill_name(s.skill));
        }
        if c::FOOD.iter().any(|(f, _)| *f == item) {
            push(&mut uses, self.t.s("book_use_food"));
        }
        if is_held(item) {
            push(&mut uses, self.t.s("book_use_held"));
        }
        if c::ARMOUR.iter().any(|(a, _, _)| *a == item) {
            push(&mut uses, self.t.s("book_use_armour"));
        }
        if c::VENUES.iter().any(|v| v.needs.iter().any(|(i, _)| *i == item)) || item == "renown" {
            push(&mut uses, self.t.s("tab_stage"));
        }
        if c::TOWNS.iter().any(|t| t.wants.contains(&item)) {
            push(&mut uses, self.t.s("tab_trader"));
        }
        if c::MACHINES
            .iter()
            .any(|m| m.needs.iter().any(|(i, _)| *i == item))
        {
            push(&mut uses, self.t.s("tab_workshop"));
        }
        if c::TRIBUTES
            .iter()
            .flatten()
            .any(|t| t.iter().any(|(i, _)| *i == item))
            || c::ROOM_PLANKS.contains(&item)
        {
            push(&mut uses, self.t.s("tab_estate"));
        }
        let none = self.t.s("book_none");
        let mut sub = self.effects(item);
        sub.extend([
            self.t.t(
                "book_from",
                &[(
                    "list",
                    if from.is_empty() {
                        none.clone()
                    } else {
                        from.join(", ")
                    },
                )],
            ),
            self.t.t(
                "book_uses",
                &[("list", if uses.is_empty() { none } else { uses.join(", ") })],
            ),
        ]);
        if c::value(item) > 0 {
            sub.push(self.t.t("book_sells", &[("n", num(c::value(item)))]));
        }
        Row {
            icon: item.into(),
            title: self.t.item(item),
            sub: sub.join(" · "),
            chip: if self.s.have(item) > 0 {
                num(self.s.have(item))
            } else {
                String::new()
            },
            ..Row::default()
        }
    }

    /// The bank's search box and its stacks grouped by the tab that makes
    /// them (IG-Q41 A); tapping one asks how many to sell. The bank and the
    /// shop both show it (T22).
    fn stacks(&self, items: &[(&String, &u64)]) -> Vec<Block> {
        let mut out = Vec::new();
        out.push(Block {
            title: self.t.s("bank_search"),
            act: "bank.search".into(),
            value: self.search.to_string(),
            text: self.t.s("bank_search_hint"),
            ..block("search")
        });
        let q = self.search.trim().to_lowercase();
        let mut groups: Vec<(&'static str, Vec<Button>)> = Vec::new();
        // the region filter and the order (T25)
        let mut items: Vec<(&String, &u64)> = items
            .iter()
            .copied()
            .filter(|(k, _)| self.region_shown(self.item_region(k)))
            .collect();
        self.order(
            &mut items,
            |(k, _)| (self.item_level(k), self.item_region(k)),
            |(k, _)| self.t.item(k),
        );
        for (k, n) in &items {
            let name = self.t.item(k);
            if !q.is_empty() && !name.to_lowercase().contains(&q) {
                continue;
            }
            let tab = maker_tab(k);
            let b = button(name, "sell.ask", k)
                .icon(k)
                .below(num(**n))
                .tone(if c::value(k) > 0 { "" } else { "keep" });
            match groups.iter_mut().find(|(t, _)| *t == tab) {
                Some((_, list)) => list.push(b),
                None => groups.push((tab, vec![b])),
            }
        }
        groups.sort_by_key(|(t, _)| TABS.iter().position(|x| x == t).unwrap_or(TABS.len()));
        if groups.is_empty() && !items.is_empty() {
            out.push(Block {
                text: self.t.s("bank_none_match"),
                ..block("note")
            });
        }
        for (tab, buttons) in groups {
            out.push(Block {
                title: if tab == "other" {
                    self.t.s("bank_other")
                } else {
                    self.tab_name(tab)
                },
                buttons,
                ..block("bank")
            });
        }
        out
    }

    /// How many to sell (T22): set amounts, all but one, or all.
    pub(crate) fn sell_sheet(&self, item: &str) -> Sheet {
        let have = self.s.have(item);
        let each = c::value(item);
        let mut buttons: Vec<Button> = c::SELL_AMOUNTS
            .iter()
            .filter(|&&n| n < have)
            .map(|&n| {
                button(
                    self.t.t("sell_n", &[("n", num(n)), ("gold", num(n * each))]),
                    "sell",
                    &format!("{item}:{n}"),
                )
            })
            .collect();
        if have > 1 {
            buttons.push(button(
                self.t.t("sell_keep1", &[("gold", num((have - 1) * each))]),
                "sell",
                &format!("{item}:keep1"),
            ));
        }
        buttons.push(
            button(
                self.t
                    .t("sell_all", &[("n", num(have)), ("gold", num(have * each))]),
                "sell",
                &format!("{item}:all"),
            )
            .tone("primary"),
        );
        buttons.push(button(self.t.s("cancel"), "sheet.close", ""));
        Sheet {
            kind: "notice".into(),
            title: self.t.t("sell_title", &[("item", self.t.item(item))]),
            lines: vec![if each > 0 {
                self.t.t("sell_have", &[("n", num(have)), ("each", num(each))])
            } else {
                self.t.t("sell_keep", &[("n", num(have))])
            }],
            buttons: if each > 0 {
                buttons
            } else {
                vec![button(self.t.s("ok"), "sheet.close", "")]
            },
            ..Sheet::default()
        }
    }

    fn shop(&self) -> Vec<Block> {
        let mut out = vec![self.head("shop", self.t.s("tab_shop"), String::new(), "")];
        out.push(Block {
            text: self.t.s("shop_intro"),
            ..block("note")
        });
        let rows = c::TOOL_SKILLS
            .iter()
            .enumerate()
            .map(|(line, &skill)| {
                let names = self.t.tools.get(c::SKILLS[skill].id).cloned().unwrap_or_default();
                let tier = self.s.tools[line];
                let name = |k: usize| names.get(k).cloned().unwrap_or_default();
                let now = self.t.t(
                    "tool_now",
                    &[
                        ("tool", name(tier)),
                        ("pct", c::TOOL_TIERS[tier].1.to_string()),
                        ("skill", self.skill_name(skill)),
                    ],
                );
                let buttons = match c::TOOL_TIERS.get(tier + 1) {
                    Some(_) if !self.s.tool_tier_open(tier + 1) => vec![button(
                        self.t.t(
                            "tool_needs_rank",
                            &[(
                                "rank",
                                self.t
                                    .s(&format!("trader_{}", c::TRADER_RANKS[0][c::TOOL_TIER_RANK].id)),
                            )],
                        ),
                        "tab",
                        "trader",
                    )],
                    Some(&(cost, pct)) => vec![button(
                        self.t.t(
                            "buy_for",
                            &[
                                ("tool", name(tier + 1)),
                                ("cost", num(cost)),
                                ("pct", pct.to_string()),
                            ],
                        ),
                        "buy",
                        &line.to_string(),
                    )
                    .tone("primary")
                    .enabled(self.s.gold >= cost)],
                    None => Vec::new(),
                };
                Row {
                    icon: c::SKILLS[skill].id.into(),
                    title: name(tier),
                    sub: now,
                    buttons,
                    ..Row::default()
                }
            })
            .collect();
        out.push(Block {
            rows,
            ..block("rows")
        });
        out.push(Block {
            title: self.t.s("seeds_title"),
            rows: self.seed_rows(),
            ..block("rows")
        });
        out.extend(self.shelf_blocks());
        let items: Vec<(&String, &u64)> = self.s.bank.iter().filter(|(_, n)| **n > 0).collect();
        if !items.is_empty() {
            out.push(Block {
                text: self.t.s("tap_to_sell"),
                title: self.t.s("shop_sell_title"),
                ..block("card")
            });
            out.extend(self.stacks(&items));
        }
        out
    }

    /// The Trader's shelves (T18, IG-Q55 A): open ones to buy from, the
    /// next shut one with what opens it.
    fn shelf_blocks(&self) -> Vec<Block> {
        let mut out = Vec::new();
        let mut shown_shut = false;
        for (r, rank, items, open) in self.s.shelves() {
            if !self.s.region_open(r) && r > 0 {
                continue;
            }
            let rank_name = self.t.s(&format!("trader_{}", c::TRADER_RANKS[r][rank].id));
            let title = if r == 0 {
                self.t.t("shelf_title", &[("rank", rank_name.clone())])
            } else {
                self.t.t(
                    "shelf_region",
                    &[("region", self.t.s(&format!("region_{}", c::REGIONS[r].0)))],
                )
            };
            if !open {
                if !shown_shut {
                    shown_shut = true;
                    out.push(Block {
                        title,
                        text: self.t.t(
                            "shelf_opens",
                            &[("rank", rank_name), ("n", items.len().to_string())],
                        ),
                        ..block("card")
                    });
                }
                continue;
            }
            let rows = items
                .iter()
                .map(|&i| {
                    let cost = c::shop_price(i) * c::SHOP_PACK;
                    Row {
                        icon: i.into(),
                        title: self.t.item(i),
                        sub: self.t.t("shelf_sub", &[("have", num(self.s.have(i)))]),
                        buttons: vec![button(
                            self.t.t(
                                "shelf_buy",
                                &[("n", c::SHOP_PACK.to_string()), ("cost", num(cost))],
                            ),
                            "buy.shelf",
                            i,
                        )
                        .enabled(self.s.gold >= cost)],
                        ..Row::default()
                    }
                })
                .collect();
            out.push(Block {
                title,
                rows,
                ..block("rows")
            });
        }
        out
    }

    fn seed_rows(&self) -> Vec<Row> {
        let l = self.s.level(c::FARMING);
        c::CROPS
            .iter()
            .filter(|cr| cr.level <= l + 10 && self.s.region_open(cr.region) && self.region_shown(cr.region))
            .map(|cr| {
                let cost = cr.seed_cost * c::SEED_PACK;
                Row {
                    icon: cr.seed.into(),
                    title: self.t.item(cr.seed),
                    sub: self.t.t("seeds_have", &[("n", num(self.s.have(cr.seed)))]),
                    buttons: vec![button(
                        self.t.t(
                            "buy_seeds",
                            &[("n", c::SEED_PACK.to_string()), ("cost", num(cost))],
                        ),
                        "buy.seed",
                        cr.id,
                    )
                    .enabled(self.s.gold >= cost)],
                    ..Row::default()
                }
            })
            .collect()
    }

    // ---------- farming (T5) ----------

    fn farming(&self) -> Vec<Block> {
        let w = self.t.skill("farming");
        let lvl = self.s.level(c::FARMING);
        let mut out = vec![self.head(
            "farming",
            w.name.clone(),
            self.t.t("lvl", &[("n", lvl.to_string())]),
            "xp",
        )];
        out.push(Block {
            bar: Some(self.xp_bar(c::FARMING)),
            ..block("bar")
        });
        let (ins, outs) = io_of("farming");
        out.push(self.links("farming", &ins, &outs));
        let open = self.s.patches_open();
        let mut rows: Vec<Row> = (0..open)
            .map(|i| {
                let p = self.s.patch(i);
                match self.s.grow_left(i) {
                    None => Row {
                        icon: "patch".into(),
                        title: self.t.s("patch_empty"),
                        sub: self.t.s("patch_plant_hint"),
                        ..Row::default()
                    },
                    Some(0) => Row {
                        icon: p.crop.clone(),
                        title: self.t.action(&p.crop),
                        sub: self.t.s("patch_ready"),
                        tone: "rare".into(),
                        buttons: vec![
                            button(self.t.s("harvest"), "farm.harvest", &i.to_string()).tone("good")
                        ],
                        ..Row::default()
                    },
                    Some(left) => Row {
                        icon: p.crop.clone(),
                        title: self.t.action(&p.crop),
                        sub: self.t.t("patch_growing", &[("left", dur(left))]),
                        ..Row::default()
                    },
                }
            })
            .collect();
        if let Some(next) = c::PATCH_LEVELS.iter().find(|&&need| need > lvl) {
            rows.push(Row {
                icon: "patch".into(),
                title: self.t.t("patch_locked", &[("n", next.to_string())]),
                ..Row::default()
            });
        }
        out.push(Block {
            title: self.t.t(
                "patches_title",
                &[
                    ("open", open.to_string()),
                    (
                        "max",
                        (c::PATCH_LEVELS.len() as u64 + c::ROOMS[c::GARDEN].max).to_string(),
                    ),
                ],
            ),
            rows,
            ..block("rows")
        });
        if (0..open).any(|i| self.s.ready(i)) {
            out.push(Block {
                buttons: vec![button(self.t.s("harvest_all"), "farm.harvest_all", "").tone("good")],
                ..block("buttons")
            });
        }
        let empty = (0..open).any(|i| self.s.patch(i).crop.is_empty());
        let mut cards = Vec::new();
        let mut locked = 0;
        for cr in c::CROPS.iter().filter(|cr| self.s.region_open(cr.region)) {
            let unlocked = lvl >= cr.level;
            if !unlocked {
                locked += 1;
                if locked > 2 {
                    continue;
                }
            }
            let seeds = self.s.have(cr.seed);
            let plant = if !unlocked {
                button(self.t.s("locked"), "farm.plant", cr.id).enabled(false)
            } else if seeds == 0 {
                button(
                    self.t.t("need_item", &[("item", self.t.item(cr.seed))]),
                    "farm.plant",
                    cr.id,
                )
                .enabled(false)
            } else if !empty {
                button(self.t.s("patches_full"), "farm.plant", cr.id).enabled(false)
            } else {
                button(self.t.s("plant"), "farm.plant", cr.id).tone("primary")
            };
            let cost = cr.seed_cost * c::SEED_PACK;
            cards.push(Card {
                icon: cr.id.into(),
                title: self.t.action(cr.id),
                sub: self.t.t("seeds_have", &[("n", num(seeds))]),
                tag: if unlocked {
                    String::new()
                } else {
                    self.t.t("lock_tag", &[("n", cr.level.to_string())])
                },
                tone: "lock".into(),
                locked: !unlocked,
                dur: "0ms".into(),
                chips: vec![
                    chip("t", self.t.t("grows_in", &[("t", dur(self.s.grow_ms(cr)))])),
                    chip(
                        "x",
                        self.t.t(
                            "plant_xp",
                            &[("n", (cr.xp_plant * self.s.mult_bp() / 10_000).to_string())],
                        ),
                    ),
                    chip(
                        "x",
                        self.t.t(
                            "each_xp",
                            &[("n", (cr.xp_each * self.s.mult_bp() / 10_000).to_string())],
                        ),
                    ),
                    chip(
                        "g",
                        self.t.t(
                            "yield",
                            &[
                                ("a", cr.yield_min.to_string()),
                                ("b", cr.yield_max.to_string()),
                                ("item", self.t.item(cr.produce)),
                            ],
                        ),
                    ),
                ],
                buttons: vec![
                    plant,
                    button(
                        self.t.t(
                            "buy_seeds",
                            &[("n", c::SEED_PACK.to_string()), ("cost", num(cost))],
                        ),
                        "buy.seed",
                        cr.id,
                    )
                    .enabled(unlocked && self.s.gold >= cost),
                ],
                ..Card::default()
            });
        }
        out.push(Block {
            text: self.t.s("farming_intro"),
            ..block("note")
        });
        let mut boost = vec![match self.s.compost() {
            Some("fine_ashes") => chip(
                "fire",
                self.t.t(
                    "compost_fine",
                    &[
                        ("n", num(self.s.have("fine_ashes"))),
                        ("pct", (100 - c::FINE_COMPOST_GROW_PCT).to_string()),
                    ],
                ),
            ),
            Some(_) => chip(
                "fire",
                self.t.t(
                    "compost_have",
                    &[
                        ("n", num(self.s.have("ashes"))),
                        ("pct", (100 - c::COMPOST_GROW_PCT).to_string()),
                    ],
                ),
            ),
            None => chip("t", self.t.s("compost_none")),
        }];
        if self.s.harvest_moon() {
            boost.push(chip(
                "p",
                self.t.t(
                    "harvest_moon_on",
                    &[("pct", (100 - c::HARVEST_MOON_PCT).to_string())],
                ),
            ));
        }
        out.push(Block {
            chips: boost,
            ..block("chips")
        });
        out.push(Block {
            cards,
            ..block("cards")
        });
        out
    }

    /// The verb a plan step's action takes.
    fn step_verb(&self, id: &str) -> String {
        if c::enemy_index(id).is_some() || c::dungeon_index(id).is_some() {
            self.t.skill("combat").verb.clone()
        } else {
            self.t.skill(c::SKILLS[self.s.skill_of(id)].id).verb.clone()
        }
    }

    /// What a condition tests, in words: "500 Oak logs", "Mining level 30",
    /// "10,000 gold".
    fn cond_text(&self, cd: &crate::game::Cond) -> String {
        match cd.kind.as_str() {
            "item" => self
                .t
                .t("n_item", &[("n", num(cd.n)), ("item", self.t.item(&cd.id))]),
            "level" => self.t.t(
                "skill_level_n",
                &[
                    ("skill", self.t.skill(&cd.id).name.clone()),
                    ("n", cd.n.to_string()),
                ],
            ),
            _ => self.t.t("n_gold", &[("n", num(cd.n))]),
        }
    }

    /// A step's line on the plan screen.
    fn step_title(&self, k: usize) -> String {
        let st = &self.s.plan[k];
        let n = (k + 1).to_string();
        match st.kind.as_str() {
            "repeat" => self
                .t
                .t("plan_repeat", &[("k", n), ("to", (st.to + 1).to_string())]),
            "if" => {
                let what = st.cond.as_ref().map(|cd| self.cond_text(cd)).unwrap_or_default();
                let key = if st.below {
                    "plan_if_below"
                } else {
                    "plan_if_at_least"
                };
                self.t
                    .t(key, &[("k", n), ("what", what), ("to", (st.to + 1).to_string())])
            }
            _ => self.t.t(
                "plan_step",
                &[
                    ("k", n),
                    ("verb", self.step_verb(&st.id)),
                    ("what", self.name(&st.id).to_lowercase()),
                ],
            ),
        }
    }

    fn step_sub(&self, k: usize) -> String {
        let st = &self.s.plan[k];
        match st.kind.as_str() {
            "repeat" if st.to >= k => self.t.s("plan_repeat_forward"),
            "repeat" => {
                if st.times == 0 {
                    self.t.s("plan_forever")
                } else {
                    self.t
                        .t("done_of", &[("done", num(st.used)), ("n", num(st.times))])
                }
            }
            "if" => self.t.s("plan_if_else"),
            _ => {
                let count = if st.n > 0 {
                    self.t.t("done_of", &[("done", num(st.done)), ("n", num(st.n))])
                } else {
                    self.t.s("until_out_sub")
                };
                match &st.until {
                    Some(u) => self
                        .t
                        .t("plan_until", &[("count", count), ("what", self.cond_text(u))]),
                    None => count,
                }
            }
        }
    }

    /// What runs now, at the top of the plan screen (T18): the scene, the
    /// bar, and what comes next, so the screen can be left open.
    fn now_panel(&self) -> Vec<Block> {
        let Some(id) = self.s.active.clone() else {
            return vec![Block {
                title: self.t.s("now_title"),
                text: self.t.s("now_idle"),
                ..block("card")
            }];
        };
        let ms = self.s.duration(&id);
        let doing = self.t.t(
            "doing",
            &[("verb", self.step_verb(&id)), ("what", self.name(&id))],
        );
        let on = self.s.plan_on && self.s.plan_at < self.s.plan.len();
        let sub = if on {
            let next = self.s.plan_at + 1;
            let then = if next < self.s.plan.len() {
                self.step_title(next)
            } else {
                self.t.s("plan_then_end")
            };
            self.t.t(
                "now_step",
                &[
                    ("k", (self.s.plan_at + 1).to_string()),
                    ("n", self.s.plan.len().to_string()),
                    ("sub", self.step_sub(self.s.plan_at)),
                    ("then", then),
                ],
            )
        } else {
            self.t.s("now_by_hand")
        };
        vec![
            Block {
                title: self.t.s("now_title"),
                rows: vec![Row {
                    icon: id.clone(),
                    title: doing.clone(),
                    sub,
                    chip: self.t.s("now"),
                    ..Row::default()
                }],
                ..block("rows")
            },
            self.scene(&id, doing, true, ms),
            Block {
                dur: format!("{ms}ms"),
                delay: format!("-{}ms", self.s.progress_ms),
                ..block("progress")
            },
        ]
    }

    fn plan(&self) -> Vec<Block> {
        let max = self.s.plan_max();
        let adv = self.s.plan_advanced;
        let mut out = vec![self.head(
            "plan",
            self.t.s("tab_plan"),
            self.t.t(
                "steps_of",
                &[("n", self.s.plan.len().to_string()), ("max", max.to_string())],
            ),
            "",
        )];
        out.extend(self.now_panel());
        out.push(Block {
            text: if adv {
                self.t.t("plan_intro_advanced", &[("n", max.to_string())])
            } else {
                self.t.t("plan_intro", &[("n", max.to_string())])
            },
            ..block("note")
        });
        let rows: Vec<Row> = (0..self.s.plan.len())
            .map(|k| {
                let st = &self.s.plan[k];
                let mut buttons: Vec<Button> = if adv {
                    vec![button(self.t.s("plan_edit"), "plan.open", &k.to_string())]
                } else {
                    c::PLAN_COUNTS
                        .iter()
                        .map(|&n| {
                            let label = if n == 0 {
                                self.t.s("until_out")
                            } else {
                                self.t.t("times_n", &[("n", num(n))])
                            };
                            button(label, "plan.count", &format!("{k}:{n}")).pressed(st.n == n)
                        })
                        .collect()
                };
                buttons.push(button(self.t.s("remove"), "plan.remove", &k.to_string()).tone("plain"));
                let running = self.s.plan_on && k == self.s.plan_at;
                Row {
                    icon: if st.kind.is_empty() {
                        st.id.clone()
                    } else {
                        format!("plan_{}", st.kind)
                    },
                    title: self.step_title(k),
                    sub: self.step_sub(k),
                    chip: if running { self.t.s("now") } else { String::new() },
                    // The block's colour: a step, a repeat or an if (T24).
                    tone: if st.kind.is_empty() {
                        "do".into()
                    } else {
                        st.kind.clone()
                    },
                    buttons,
                }
            })
            .collect();
        // The plan as blocks to drag (T24): the stack, the end to drop on,
        // and in Advanced a palette of new repeat and if blocks.
        let palette = if adv && self.s.plan.len() < max {
            vec![
                button(self.t.s("plan_add_repeat"), "plan.insert", "repeat").tone("repeat"),
                button(self.t.s("plan_add_if"), "plan.insert", "if").tone("if"),
            ]
        } else {
            Vec::new()
        };
        let empty = rows.is_empty();
        out.push(Block {
            title: if palette.is_empty() {
                String::new()
            } else {
                self.t.s("plan_palette")
            },
            text: self.t.s(if empty { "plan_empty" } else { "plan_drop_end" }),
            label2: self.t.s("plan_drag_hint"),
            act: "plan.move".into(),
            rows,
            buttons: palette,
            ..block("program")
        });
        if !empty {
            out.push(Block {
                buttons: vec![
                    button(self.t.s("plan_start"), "plan.start", "").tone("primary"),
                    button(self.t.s("plan_clear"), "plan.clear", ""),
                ],
                ..block("buttons")
            });
        }
        out.push(Block {
            text: self.t.s("plan_advanced_about"),
            buttons: vec![button(
                self.t.s(if adv {
                    "plan_advanced_off"
                } else {
                    "plan_advanced_on"
                }),
                "plan.advanced",
                if adv { "0" } else { "1" },
            )
            .pressed(adv)],
            ..block("buttons")
        });
        out.push(Block {
            title: self.t.s("away_title"),
            text: self.t.t(
                "away_text",
                &[
                    ("cap", dur(self.s.away_cap())),
                    ("after", dur(c::RETURN_AFTER_MS)),
                ],
            ),
            ..block("card")
        });
        out
    }

    /// The sheet that edits plan step `k` in advanced mode (IG-Q53 A).
    pub(crate) fn plan_sheet(&self, k: usize) -> Sheet {
        let st = &self.s.plan[k];
        let edit = |what: &str, v: &str| format!("{k}:{what}:{v}");
        let pick = |text: String, buttons: Vec<Button>| Block {
            text,
            buttons,
            ..block("buttons")
        };
        let mut blocks = Vec::new();
        match st.kind.as_str() {
            "repeat" => {
                blocks.push(pick(
                    self.t.s("plan_back_to"),
                    (0..k)
                        .map(|t| {
                            button((t + 1).to_string(), "plan.edit", &edit("to", &t.to_string()))
                                .pressed(st.to == t)
                        })
                        .collect(),
                ));
                blocks.push(pick(
                    self.t.s("plan_how_often"),
                    c::PLAN_TIMES
                        .iter()
                        .map(|&n| {
                            let label = if n == 0 {
                                self.t.s("plan_forever")
                            } else {
                                self.t.t("times_n", &[("n", n.to_string())])
                            };
                            button(label, "plan.edit", &edit("times", &n.to_string())).pressed(st.times == n)
                        })
                        .collect(),
                ));
            }
            "if" => {
                let on = st.cond.clone().unwrap_or_default();
                let mut tests = vec![
                    button(self.t.s("gold"), "plan.edit", &edit("on", "gold")).pressed(on.kind == "gold")
                ];
                for item in self.s.plan_items() {
                    tests.push(
                        button(self.t.item(&item), "plan.edit", &edit("on", &item))
                            .pressed(on.kind == "item" && on.id == item),
                    );
                }
                blocks.push(pick(self.t.s("plan_if_test"), tests));
                blocks.push(pick(
                    self.t.s("plan_if_when"),
                    vec![
                        button(self.t.s("plan_fewer"), "plan.edit", &edit("below", "1")).pressed(st.below),
                        button(self.t.s("plan_at_least"), "plan.edit", &edit("below", "0"))
                            .pressed(!st.below),
                    ],
                ));
                blocks.push(pick(
                    self.t.s("plan_amount"),
                    c::PLAN_AMOUNTS
                        .iter()
                        .map(|&n| {
                            button(num(n), "plan.edit", &edit("amount", &n.to_string())).pressed(on.n == n)
                        })
                        .collect(),
                ));
                blocks.push(pick(
                    self.t.s("plan_go_to"),
                    (0..self.s.plan.len())
                        .filter(|&t| t != k)
                        .map(|t| {
                            button((t + 1).to_string(), "plan.edit", &edit("to", &t.to_string()))
                                .pressed(st.to == t)
                        })
                        .collect(),
                ));
            }
            _ => {
                blocks.push(pick(
                    self.t.s("plan_how_many"),
                    c::PLAN_COUNTS
                        .iter()
                        .map(|&n| {
                            let label = if n == 0 {
                                self.t.s("until_out")
                            } else {
                                self.t.t("times_n", &[("n", num(n))])
                            };
                            button(label, "plan.edit", &edit("count", &n.to_string())).pressed(st.n == n)
                        })
                        .collect(),
                ));
                let u = st.until.clone().unwrap_or_default();
                blocks.push(pick(
                    self.t.s("plan_until_none_about"),
                    vec![
                        button(self.t.s("plan_until_none"), "plan.edit", &edit("until", "none"))
                            .pressed(st.until.is_none()),
                    ],
                ));
                if let Some(item) = self.s.makes_of(&st.id) {
                    blocks.push(pick(
                        self.t.t("plan_until_item", &[("item", self.t.item(&item))]),
                        c::PLAN_AMOUNTS
                            .iter()
                            .map(|&n| {
                                button(num(n), "plan.edit", &edit("until", &format!("item:{n}")))
                                    .pressed(u.kind == "item" && u.n == n)
                            })
                            .collect(),
                    ));
                }
                let skill = self.s.skill_of(&st.id);
                let l = self.s.level(skill) as u64;
                let mut levels: Vec<u64> = [1, 5, 10]
                    .iter()
                    .map(|d| (l + d).min(xp::MAX_LEVEL as u64))
                    .filter(|&x| x > l)
                    .collect();
                levels.dedup();
                if u.kind == "level" && !levels.contains(&u.n) {
                    levels.insert(0, u.n);
                }
                if !levels.is_empty() {
                    blocks.push(pick(
                        self.t.t("plan_until_level", &[("skill", self.skill_name(skill))]),
                        levels
                            .iter()
                            .map(|&n| {
                                button(n.to_string(), "plan.edit", &edit("until", &format!("level:{n}")))
                                    .pressed(u.kind == "level" && u.n == n)
                            })
                            .collect(),
                    ));
                }
                blocks.push(pick(
                    self.t.s("plan_until_gold"),
                    c::PLAN_AMOUNTS
                        .iter()
                        .map(|&n| {
                            button(num(n), "plan.edit", &edit("until", &format!("gold:{n}")))
                                .pressed(u.kind == "gold" && u.n == n)
                        })
                        .collect(),
                ));
            }
        }
        Sheet {
            kind: "notice".into(),
            title: self.step_title(k),
            lines: vec![self.step_sub(k)],
            blocks,
            buttons: {
                let mut b = Vec::new();
                if k > 0 {
                    b.push(button(
                        self.t.s("plan_up"),
                        "plan.move",
                        &format!("{k}:{}", k - 1),
                    ));
                }
                if k + 1 < self.s.plan.len() {
                    b.push(button(
                        self.t.s("plan_down"),
                        "plan.move",
                        &format!("{k}:{}", k + 1),
                    ));
                }
                b.push(button(self.t.s("remove"), "plan.remove", &k.to_string()).tone("stop"));
                b.push(button(self.t.s("ok"), "sheet.close", "").tone("primary"));
                b
            },
            ..Sheet::default()
        }
    }

    fn prestige(&self) -> Vec<Block> {
        let tl = self.s.total_level();
        let need = self.s.prestige_needs();
        let next_bp = self.s.mult_bp() + c::PRESTIGE_STEP_BP;
        let mut out = vec![self.head(
            "prestige",
            self.t.s("tab_prestige"),
            if self.s.prestige > 0 {
                self.t.t("prestige_done", &[("n", self.s.prestige.to_string())])
            } else {
                self.t.s("prestige_none")
            },
            "pres",
        )];
        out.push(Block {
            text: self.t.t(
                "prestige_intro",
                &[
                    ("pct", (c::PRESTIGE_STEP_BP / 100).to_string()),
                    ("need", need.to_string()),
                ],
            ),
            ..block("note")
        });
        out.push(Block {
            bar: Some(Bar {
                tone: "pres".into(),
                width: width((tl as u64 * 10_000 / need as u64).min(10_000)),
                left: self
                    .t
                    .t("total_of", &[("n", tl.to_string()), ("need", need.to_string())]),
                right: if tl >= need {
                    self.t.s("ready")
                } else {
                    self.t.t("to_go", &[("n", (need - tl).to_string())])
                },
            }),
            ..block("bar")
        });
        out.push(Block {
            chips: vec![
                chip("p", self.t.t("mult_now", &[("x", times(self.s.mult_bp()))])),
                chip("p", self.t.t("mult_after", &[("x", times(next_bp))])),
            ],
            ..block("chips")
        });
        out.push(Block {
            title: self.t.s("you_keep"),
            label2: self.t.s("starts_over"),
            list: vec![
                self.t.t("keep_mult", &[("x", times(next_bp))]),
                self.t.s("keep_recipes"),
                self.t.s("keep_log"),
            ],
            list2: vec![
                self.t.s("lose_levels"),
                self.t.s("lose_mastery"),
                self.t.s("lose_bank"),
                self.t.s("lose_tools"),
                self.t.s("lose_plan"),
                self.t.s("lose_engines"),
            ],
            ..block("keep")
        });
        out.push(Block {
            buttons: vec![button(
                if tl >= need {
                    self.t.s("prestige_now")
                } else {
                    self.t.t("prestige_locked", &[("n", (need - tl).to_string())])
                },
                "prestige.ask",
                "",
            )
            .tone("pres")
            .enabled(tl >= need)],
            ..block("buttons")
        });
        out
    }

    fn settings(&self) -> Vec<Block> {
        let mut out = vec![self.head("settings", self.t.s("tab_settings"), String::new(), "")];
        let ago = |w: u64| {
            if w == 0 {
                self.t.s("never")
            } else {
                self.t.t("ago", &[("t", dur(self.s.last_wall.saturating_sub(w)))])
            }
        };
        let flagged = self.s.flags.len();
        out.push(Block {
            title: self.t.s("playtest_title"),
            rows: vec![
                Row {
                    icon: "flag".into(),
                    title: self.t.s("flag_title"),
                    sub: self.t.t("flag_sub", &[("n", flagged.to_string())]),
                    buttons: vec![button(self.t.s("flag_short"), "flag.open", "")],
                    ..Row::default()
                },
                Row {
                    icon: "send".into(),
                    title: self.t.s("playtest_send"),
                    sub: self.t.s("playtest_sub"),
                    buttons: vec![button(self.t.s("send"), "page.playtest", "").tone("good")],
                    ..Row::default()
                },
            ],
            ..block("rows")
        });
        let screen_row = if self.device.fullscreen {
            Some(button(self.t.s("fullscreen"), "page.fullscreen", ""))
        } else if self.device.ios && !self.device.standalone {
            Some(button(self.t.s("how"), "help.home", ""))
        } else {
            None
        };
        let mut rows = vec![
            Row {
                icon: "save".into(),
                title: self.t.s("saved_browser"),
                sub: self.t.t("saved_browser_sub", &[("every", dur(c::AUTOSAVE_MS))]),
                ..Row::default()
            },
            Row {
                icon: "save".into(),
                title: self.t.s("export_title"),
                sub: self.t.t("export_sub", &[("ago", ago(self.s.exported_wall))]),
                buttons: vec![button(self.t.s("export"), "page.export", "").tone("good")],
                ..Row::default()
            },
            Row {
                icon: "save".into(),
                title: self.t.s("import_title"),
                sub: self.t.s("import_sub"),
                buttons: vec![button(self.t.s("import"), "page.import", "")],
                ..Row::default()
            },
        ];
        if let Some(b) = screen_row {
            rows.push(Row {
                icon: "fullscreen".into(),
                title: self.t.s("fullscreen"),
                sub: if self.device.fullscreen {
                    self.t.s("fs_yes")
                } else {
                    self.t.s("fs_ios")
                },
                buttons: vec![b],
                ..Row::default()
            });
        }
        rows.push(Row {
            icon: "performance".into(),
            title: self.t.s("sound_title"),
            sub: self.t.s("sound_sub"),
            buttons: vec![button(
                self.t.s(if self.s.sound { "sound_on" } else { "sound_off" }),
                "sound.toggle",
                "",
            )
            .pressed(self.s.sound)],
            ..Row::default()
        });
        rows.push(Row {
            icon: "reset".into(),
            title: self.t.s("reset_title"),
            sub: self.t.s("reset_sub"),
            buttons: vec![button(self.t.s("reset"), "reset.ask", "")],
            ..Row::default()
        });
        out.push(Block {
            title: self.t.s("saving_title"),
            rows,
            ..block("rows")
        });
        out.push(Block {
            text: self
                .t
                .t("version_note", &[("v", crate::game::SAVE_VERSION.to_string())]),
            ..block("note")
        });
        out
    }

    // ---------- sheets ----------

    /// The return screen (P5) after a long absence.
    pub fn return_sheet(&self, r: &Report) -> Sheet {
        let mut gains = Vec::new();
        let mut ready = Vec::new();
        for (i, sk) in c::SKILLS.iter().enumerate() {
            if r.xp[i] == 0 {
                continue;
            }
            let (l0, l1) = (r.lv0[i], self.s.level(i));
            gains.push(Row {
                icon: sk.id.into(),
                title: if l1 > l0 {
                    self.t.t(
                        "skill_from_to",
                        &[
                            ("skill", self.skill_name(i)),
                            ("a", l0.to_string()),
                            ("b", l1.to_string()),
                        ],
                    )
                } else {
                    self.skill_name(i)
                },
                chip: self.t.t("plus_xp", &[("n", num(r.xp[i]))]),
                tone: if l1 > l0 { "up".into() } else { String::new() },
                ..Row::default()
            });
            let new: Vec<&c::Action> = c::ACTIONS
                .iter()
                .filter(|a| a.skill == i && a.level > l0 && a.level <= l1 && self.s.unlocked(a.id))
                .collect();
            if let Some(best) = new.last() {
                ready.push(Row {
                    title: self.t.t(
                        "unlocked",
                        &[
                            ("what", self.t.action(best.id)),
                            ("skill", self.skill_name(i)),
                            ("n", best.level.to_string()),
                        ],
                    ),
                    buttons: vec![button(
                        format!(
                            "{} {}",
                            self.t.skill(sk.id).verb,
                            self.t.action(best.id).to_lowercase()
                        ),
                        "do",
                        best.id,
                    )
                    .tone("primary")],
                    ..Row::default()
                });
            }
        }
        for (id, m0) in &r.m0 {
            let m1 = self.s.mastery(id);
            if m1 > *m0 {
                gains.push(Row {
                    icon: id.clone(),
                    title: self.t.t("mastery_of", &[("what", self.t.action(id))]),
                    chip: self
                        .t
                        .t("from_to", &[("a", m0.to_string()), ("b", m1.to_string())]),
                    tone: "up".into(),
                    ..Row::default()
                });
            }
        }
        for (cp_id, cp) in &r.checkpoints {
            if let Some(a) = c::action_index(cp_id) {
                let k = kind_key(c::SKILLS[c::ACTIONS[a].skill].kind);
                ready.push(Row {
                    title: self.t.t(
                        "checkpoint_on",
                        &[
                            ("what", self.t.action(cp_id)),
                            ("n", cp.to_string()),
                            ("effect", self.t.s(&format!("cp_{k}_{cp}"))),
                        ],
                    ),
                    ..Row::default()
                });
            }
        }
        for (id, n) in &r.kills {
            gains.push(Row {
                icon: id.clone(),
                title: self.t.t("defeated", &[("enemy", self.t.enemy(id))]),
                chip: self.t.t("times_n", &[("n", num(*n))]),
                ..Row::default()
            });
        }
        for (item, n) in &r.items {
            gains.push(Row {
                icon: item.clone(),
                title: self.t.item(item),
                chip: self.t.t("plus_n", &[("n", num(*n))]),
                ..Row::default()
            });
        }
        for (item, n) in &r.rare {
            gains.push(Row {
                icon: item.clone(),
                title: self.t.t("rare_item", &[("item", self.t.item(item))]),
                chip: self.t.t("plus_n", &[("n", num(*n))]),
                tone: "rare".into(),
                ..Row::default()
            });
        }
        for id in &r.recipes {
            gains.push(Row {
                icon: id.clone(),
                title: self.t.t("recipe_found", &[("what", self.t.action(id))]),
                tone: "rare".into(),
                ..Row::default()
            });
        }
        for (v, gold, renown) in &r.shows {
            gains.push(Row {
                icon: "stage".into(),
                title: self
                    .t
                    .t("show_ended", &[("venue", self.t.s(&format!("venue_{v}")))]),
                sub: self
                    .t
                    .t("n_item", &[("n", num(*renown)), ("item", self.t.item("renown"))]),
                chip: self.t.t("plus_n", &[("n", num(*gold))]),
                ..Row::default()
            });
        }
        for (t, gold, bandits) in &r.caravans {
            gains.push(Row {
                icon: "cart".into(),
                title: self.t.t(
                    if *bandits {
                        "caravan_home_bandits"
                    } else {
                        "caravan_home"
                    },
                    &[("town", self.t.s(&format!("town_{}", c::TOWNS[*t].id)))],
                ),
                chip: self.t.t("plus_n", &[("n", num(*gold))]),
                ..Row::default()
            });
        }
        for id in &r.chests {
            gains.push(Row {
                icon: "gold".into(),
                title: self
                    .t
                    .t("chest_from", &[("what", self.t.s(&format!("dungeon_{id}")))]),
                tone: "rare".into(),
                ..Row::default()
            });
        }
        if r.masterworks > 0 {
            gains.push(Row {
                icon: "smithing".into(),
                title: self.t.s("masterworks"),
                chip: self.t.t("plus_n", &[("n", num(r.masterworks))]),
                tone: "rare".into(),
                ..Row::default()
            });
        }
        for id in &r.research {
            ready.push(Row {
                title: self
                    .t
                    .t("blueprint_done", &[("what", self.t.s(&format!("bp_{id}")))]),
                buttons: vec![button(self.t.s("tab_workshop"), "tab", "workshop")],
                ..Row::default()
            });
        }
        if r.income > 0 {
            gains.push(Row {
                icon: "estate".into(),
                title: self.t.s("estate_income"),
                chip: self.t.t("plus_n", &[("n", num(r.income))]),
                ..Row::default()
            });
        }
        if r.wages > 0 {
            gains.push(Row {
                icon: "estate".into(),
                title: self.t.s("wages_paid"),
                chip: self.t.t("minus_n", &[("n", num(r.wages))]),
                tone: "used".into(),
                ..Row::default()
            });
        }
        if r.unpaid {
            ready.push(Row {
                title: self.t.s("servants_unpaid"),
                buttons: vec![button(self.t.s("tab_estate"), "tab", "estate")],
                ..Row::default()
            });
        }
        if r.gold > 0 {
            gains.push(Row {
                icon: "gold".into(),
                title: self.t.s("gold"),
                chip: self.t.t("plus_n", &[("n", num(r.gold))]),
                tone: "rare".into(),
                ..Row::default()
            });
        }
        for (item, n) in &r.used {
            gains.push(Row {
                icon: item.clone(),
                title: self.t.t("used_item", &[("item", self.t.item(item))]),
                chip: self.t.t("minus_n", &[("n", num(*n))]),
                tone: "used".into(),
                ..Row::default()
            });
        }
        if let Some(stop) = &r.stop {
            if self.s.active.is_none() {
                ready.push(self.stop_row(stop));
            }
        }
        if !r.steps.is_empty() {
            let names: Vec<String> = r.steps.iter().map(|s| self.name(s).to_lowercase()).collect();
            ready.push(Row {
                title: self.t.t("steps_done", &[("list", names.join(", "))]),
                ..Row::default()
            });
        }
        let ripe: Vec<String> = (0..self.s.patches_open())
            .filter(|&i| self.s.ready(i))
            .map(|i| self.t.action(&self.s.patch(i).crop).to_lowercase())
            .collect();
        if !ripe.is_empty() {
            ready.push(Row {
                title: self.t.t("crops_ready", &[("list", ripe.join(", "))]),
                buttons: vec![button(self.t.s("harvest_all"), "farm.harvest_all", "").tone("good")],
                ..Row::default()
            });
        }
        if self.s.can_prestige() {
            ready.push(Row {
                title: self
                    .t
                    .t("prestige_ready", &[("n", self.s.total_level().to_string())]),
                buttons: vec![button(self.t.s("tab_prestige"), "tab", "prestige").tone("pres")],
                ..Row::default()
            });
        }
        if self.s.active.is_none() && r.stop.is_none() {
            ready.push(Row {
                title: self.t.s("nothing_running"),
                ..Row::default()
            });
        }
        if self
            .s
            .last_wall
            .saturating_sub(self.s.exported_wall.max(self.s.first_wall))
            > c::EXPORT_REMIND_MS
        {
            ready.push(Row {
                title: self.t.s("export_remind"),
                buttons: vec![button(self.t.s("export"), "page.export", "").tone("good")],
                ..Row::default()
            });
        }
        let mut notes = Vec::new();
        if r.capped > 0 {
            notes.push(self.t.t(
                "capped",
                &[("cap", dur(self.s.away_cap())), ("lost", dur(r.capped))],
            ));
        }
        Sheet {
            kind: "return".into(),
            eyebrow: self.t.s("while_away"),
            title: String::new(),
            big: dur(r.elapsed),
            lines: vec![self.t.t("n_actions", &[("n", num(r.actions))])],
            gains,
            ready_title: if ready.is_empty() {
                String::new()
            } else {
                self.t.s("ready_for_you")
            },
            ready,
            notes,
            blocks: Vec::new(),
            buttons: vec![button(self.t.s("collect"), "sheet.close", "").tone("primary")],
        }
    }

    pub fn stop_row(&self, stop: &Stop) -> Row {
        match stop {
            Stop::Food => Row {
                title: self.t.s("stopped_food"),
                buttons: vec![button(self.t.skill("cooking").name.clone(), "tab", "cooking")],
                ..Row::default()
            },
            Stop::Missing(item) => {
                let maker = LINK_TABS
                    .iter()
                    .find(|t| io_of(t).1.contains(&item.as_str()))
                    .copied();
                Row {
                    title: self.t.t("stopped_out", &[("item", self.t.item(item))]),
                    buttons: maker
                        .map(|m| {
                            vec![button(
                                self.t.t("makes_more", &[("skill", self.tab_name(m))]),
                                "tab",
                                m,
                            )]
                        })
                        .unwrap_or_default(),
                    ..Row::default()
                }
            }
        }
    }
}

fn kind_key(k: Kind) -> &'static str {
    match k {
        Kind::Gather => "gather",
        _ => "make",
    }
}

/// The tab whose skill or engine makes an item, for grouping the bank.
/// A weapon a style holds, or armour: combat uses the best one you have and
/// never spends it, so a second one adds nothing in a fight (T21).
pub fn is_held(item: &str) -> bool {
    c::STYLES.iter().any(|s| s.held.iter().any(|(i, _)| *i == item))
        || c::ARMOUR.iter().any(|(a, _, _)| *a == item)
}

pub fn maker_tab(item: &str) -> &'static str {
    if let Some(a) = c::ACTIONS.iter().find(|a| a.makes.0 == item) {
        return tab_of_skill(a.skill);
    }
    if c::CROPS.iter().any(|cr| cr.produce == item) {
        return "farming";
    }
    if c::GEMS.iter().any(|g| g.0 == item) {
        return "mining";
    }
    if c::ENEMIES.iter().any(|e| e.drops.iter().any(|d| d.0 == item)) {
        return "combat";
    }
    if c::TOWNS.iter().any(|t| t.sells == item) {
        return "trader";
    }
    if c::MACHINES.iter().any(|m| m.makes.0 == item) || item.ends_with("_parts") {
        return "workshop";
    }
    match item {
        "starlight" | "stardust" => "observatory",
        "fine_ashes" | "ashes" => "firemaking",
        "bird_nest" => "woodcutting",
        _ => "other",
    }
}

fn first_action(skill: usize) -> usize {
    c::ACTIONS.iter().position(|a| a.skill == skill).unwrap_or(0)
}

/// What a tab's skill takes in and gives out, for the uses and feeds links.
pub fn io_of(tab: &str) -> (Vec<&'static str>, Vec<&'static str>) {
    if tab == "farming" {
        // seeds, and ashes as compost (T8)
        return (
            c::CROPS.iter().map(|cr| cr.seed).chain(["ashes"]).collect(),
            c::CROPS.iter().map(|cr| cr.produce).collect(),
        );
    }
    if tab == "combat" {
        let mut ins: Vec<&str> = c::STYLES
            .iter()
            .flat_map(|s| s.held.iter().chain(s.spent.iter()).map(|(i, _)| *i))
            .collect();
        ins.extend(c::FOOD.iter().map(|(f, _)| *f));
        ins.extend(c::ARMOUR.iter().map(|(a, _, _)| *a));
        let outs = c::ENEMIES
            .iter()
            .flat_map(|e| e.drops.iter().map(|d| d.0))
            .collect();
        return (ins, outs);
    }
    if tab == "performance" {
        let ins = c::SONG_INSTRUMENTS.iter().map(|(_, i)| *i).collect();
        return (ins, vec!["renown"]);
    }
    if tab == "stage" {
        let mut ins: Vec<&str> = c::VENUES
            .iter()
            .flat_map(|v| v.needs.iter().map(|(i, _)| *i))
            .collect();
        ins.push("renown");
        return (ins, Vec::new());
    }
    if tab == "trader" {
        let ins = c::TOWNS.iter().flat_map(|t| t.wants.iter().copied()).collect();
        let outs = c::TOWNS.iter().map(|t| t.sells).collect();
        return (ins, outs);
    }
    if tab == "estate" {
        let mut ins: Vec<&str> = c::ROOM_PLANKS.to_vec();
        ins.extend(["nails", "limestone"]);
        ins.extend(
            c::TRIBUTES
                .iter()
                .flatten()
                .flat_map(|t| t.iter().map(|(i, _)| *i)),
        );
        return (ins, Vec::new());
    }
    let i = match tab {
        "workshop" => c::INVENTION,
        "observatory" => c::ATTUNEMENT,
        _ => match c::skill_index(tab) {
            Some(i) => i,
            None => return (Vec::new(), Vec::new()),
        },
    };
    let acts = c::ACTIONS.iter().filter(|a| a.skill == i);
    let ins: Vec<&str> = acts
        .clone()
        .flat_map(|a| a.needs.iter().map(|(n, _)| *n))
        .collect();
    let mut ins: Vec<&str> = ins;
    let mut outs: Vec<&str> = acts.map(|a| a.makes.0).filter(|m| !m.is_empty()).collect();
    if i == c::INVENTION {
        for m in c::MACHINES {
            ins.extend(m.needs.iter().map(|(n, _)| *n));
            outs.push(m.makes.0);
        }
        ins.extend(c::FUEL);
    }
    (ins, outs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn woodcutting_feeds_firemaking_and_fletching() {
        let (_, outs) = io_of("woodcutting");
        let (fm_in, _) = io_of("firemaking");
        let (fl_in, _) = io_of("fletching");
        assert!(outs.iter().any(|o| fm_in.contains(o)));
        assert!(outs.iter().any(|o| fl_in.contains(o)));
    }

    #[test]
    fn firemaking_feeds_farming_with_compost() {
        let (_, fm_out) = io_of("firemaking");
        let (farm_in, _) = io_of("farming");
        assert!(fm_out.iter().any(|o| farm_in.contains(o)));
    }

    #[test]
    fn combat_uses_smithing_fletching_runecrafting_and_cooking() {
        let (ins, outs) = io_of("combat");
        for t in ["smithing", "fletching", "runecrafting", "cooking"] {
            assert!(io_of(t).1.iter().any(|o| ins.contains(o)), "{t}");
        }
        assert!(outs.contains(&"feathers"));
    }
}
