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

fn chip(tone: &str, text: String) -> Chip {
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
}

fn block(kind: &str) -> Block {
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
}

pub const TABS: [&str; 16] = [
    "woodcutting",
    "mining",
    "fishing",
    "farming",
    "firemaking",
    "fletching",
    "smithing",
    "cooking",
    "crafting",
    "runecrafting",
    "combat",
    "bank",
    "shop",
    "plan",
    "prestige",
    "settings",
];

/// The tabs that are skills (the rest are screens).
pub const SKILL_TABS: usize = 11;

/// Everything the view needs to read.
pub struct View<'a> {
    pub t: &'a Text,
    pub s: &'a State,
    pub device: &'a Device,
}

impl View<'_> {
    fn name(&self, id: &str) -> String {
        match c::enemy_index(id) {
            Some(_) => self.t.enemy(id),
            None => self.t.action(id),
        }
    }

    fn skill_name(&self, i: usize) -> String {
        self.t.skill(c::SKILLS[i].id).name.clone()
    }

    fn xp_bar(&self, skill: usize) -> Bar {
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

    fn mastery_bar(&self, id: &str) -> Bar {
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

    pub fn tabs(&self) -> Vec<Tab> {
        let running = self.s.active.as_deref().map(|id| {
            if c::enemy_index(id).is_some() {
                "combat"
            } else {
                c::SKILLS[self.s.skill_of(id)].id
            }
        });
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
                        if id == "prestige" && self.s.prestige > 0 {
                            self.s.prestige.to_string()
                        } else {
                            String::new()
                        },
                    ),
                };
                Tab {
                    id: id.into(),
                    label,
                    short,
                    level,
                    running: running == Some(id),
                    current: self.s.screen == id,
                }
            })
            .collect()
    }

    pub fn screen(&self) -> Vec<Block> {
        match self.s.screen.as_str() {
            "combat" => self.combat(),
            "bank" => self.bank(),
            "shop" => self.shop(),
            "plan" => self.plan(),
            "prestige" => self.prestige(),
            "settings" => self.settings(),
            "farming" => self.farming(),
            id => self.skill(c::skill_index(id).unwrap_or(0)),
        }
    }

    // ---------- skills ----------

    /// Skills that make what `ins` needs, and skills that use what `outs` makes.
    fn links(&self, me: &str, ins: &[&str], outs: &[&str]) -> Block {
        let mut uses: Vec<&str> = Vec::new();
        let mut feeds: Vec<&str> = Vec::new();
        for tab in TABS.iter().take(SKILL_TABS) {
            if *tab == me {
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
        let b = |id: &&str| button(self.t.skill(id).name.clone(), "tab", id).icon(id);
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

    fn head(&self, icon: &str, title: String, tag: String, tone: &str) -> Block {
        Block {
            icon: icon.into(),
            title,
            tag,
            tone: tone.into(),
            ..block("head")
        }
    }

    fn scene(&self, icon: &str, caption: String, running: bool, ms: u64) -> Block {
        Block {
            icon: icon.into(),
            text: caption,
            running,
            dur: format!("{ms}ms"),
            delay: format!("-{}ms", self.s.progress_ms),
            ..block("scene")
        }
    }

    fn skill(&self, i: usize) -> Vec<Block> {
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
        let mut cards = Vec::new();
        let mut locked_shown = 0;
        let mut hidden = 0;
        for a in c::ACTIONS.iter().filter(|a| a.skill == i) {
            if self.s.level(i) < a.level {
                // LR-2: the next two locked ones show; the rest wait out of sight.
                if locked_shown >= 2 {
                    hidden += 1;
                    continue;
                }
                locked_shown += 1;
            }
            cards.push(self.action_card(a, active == c::action_index(a.id)));
        }
        out.push(Block {
            cards,
            ..block("cards")
        });
        if hidden > 0 {
            out.push(Block {
                text: self.t.t("more_later", &[("n", hidden.to_string())]),
                ..block("note")
            });
        }
        out
    }

    fn forecast(&self, a: &c::Action) -> Block {
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

    fn action_card(&self, a: &c::Action, on: bool) -> Card {
        let skill = &c::SKILLS[a.skill];
        let w = self.t.skill(skill.id);
        let open = self.s.level(a.skill) >= a.level;
        let ml = self.s.mastery(a.id);
        let missing = self.s.missing(a.id);
        let sub = if !open {
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
        if a.makes.1 > 1 {
            chips.push(chip("g", self.t.t("makes_n", &[("n", a.makes.1.to_string())])));
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
        if open && self.s.plan.len() < c::PLAN_STEPS {
            buttons.push(button(self.t.s("plan_add"), "plan.add", a.id));
        }
        Card {
            icon: a.id.into(),
            title: self.t.action(a.id),
            sub,
            tag: if open {
                self.t.t("m_tag", &[("n", ml.to_string())])
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
        let fighting = self.s.active.as_deref().and_then(c::enemy_index);
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
                    .filter(|e| e.weak == s.skill)
                    .map(|e| self.t.enemy(e.id).to_lowercase())
                    .collect();
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
                .below(self.t.t("strong_vs", &[("list", strong.join(", "))]))
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
            ("", _) => chip("t", self.t.s("armour_none")),
            (item, tier) => chip(
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
        for list in [st.held, st.spent] {
            if list.is_empty() {
                continue;
            }
            let (best, _) = self.s.tier(list);
            chips.push(match best.as_str() {
                "" => chip(
                    "bad",
                    self.t
                        .t("supply_none", &[("item", self.t.item(list[list.len() - 1].0))]),
                ),
                b => chip(
                    "t",
                    self.t.t(
                        "supply_have",
                        &[("item", self.t.item(b)), ("n", num(self.s.have(b)))],
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
        let cards = c::ENEMIES
            .iter()
            .map(|en| self.enemy_card(en, fighting == c::enemy_index(en.id)))
            .collect();
        out.push(Block {
            cards,
            ..block("cards")
        });
        out
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
        if self.s.plan.len() < c::PLAN_STEPS {
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
        let buttons = items
            .iter()
            .map(|(k, n)| {
                button(self.t.item(k), "sell", k)
                    .icon(k)
                    .below(num(**n))
                    .tone(if c::value(k) > 0 { "" } else { "keep" })
            })
            .collect();
        out.push(Block {
            buttons,
            ..block("bank")
        });
        out
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
        out
    }

    fn seed_rows(&self) -> Vec<Row> {
        let l = self.s.level(c::FARMING);
        c::CROPS
            .iter()
            .filter(|cr| cr.level <= l + 10)
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
                    ("max", c::PATCH_LEVELS.len().to_string()),
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
        for cr in c::CROPS {
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
                    chip("t", self.t.t("grows_in", &[("t", dur(cr.grow_ms))])),
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
        out.push(Block {
            cards,
            ..block("cards")
        });
        out
    }

    fn plan(&self) -> Vec<Block> {
        let mut out = vec![self.head(
            "plan",
            self.t.s("tab_plan"),
            self.t.t(
                "steps_of",
                &[
                    ("n", self.s.plan.len().to_string()),
                    ("max", c::PLAN_STEPS.to_string()),
                ],
            ),
            "",
        )];
        out.push(Block {
            text: self.t.t("plan_intro", &[("n", c::PLAN_STEPS.to_string())]),
            ..block("note")
        });
        let rows: Vec<Row> = self
            .s
            .plan
            .iter()
            .enumerate()
            .map(|(k, st)| {
                let skill = self.s.skill_of(&st.id);
                let verb = if c::enemy_index(&st.id).is_some() {
                    self.t.skill("combat").verb.clone()
                } else {
                    self.t.skill(c::SKILLS[skill].id).verb.clone()
                };
                let mut buttons: Vec<Button> = c::PLAN_COUNTS
                    .iter()
                    .map(|&n| {
                        let label = if n == 0 {
                            self.t.s("until_out")
                        } else {
                            self.t.t("times_n", &[("n", num(n))])
                        };
                        button(label, "plan.count", &format!("{k}:{n}")).pressed(st.n == n)
                    })
                    .collect();
                buttons.push(button(self.t.s("remove"), "plan.remove", &k.to_string()).tone("plain"));
                let running = k == 0 && self.s.active.as_deref() == Some(st.id.as_str());
                Row {
                    icon: st.id.clone(),
                    title: self.t.t(
                        "plan_step",
                        &[
                            ("k", (k + 1).to_string()),
                            ("verb", verb),
                            ("what", self.name(&st.id).to_lowercase()),
                        ],
                    ),
                    sub: if st.n > 0 {
                        self.t.t("done_of", &[("done", num(st.done)), ("n", num(st.n))])
                    } else {
                        self.t.s("until_out_sub")
                    },
                    chip: if running { self.t.s("now") } else { String::new() },
                    buttons,
                    ..Row::default()
                }
            })
            .collect();
        if rows.is_empty() {
            out.push(Block {
                text: self.t.s("plan_empty"),
                ..block("note")
            });
        } else {
            out.push(Block {
                rows,
                ..block("rows")
            });
            out.push(Block {
                buttons: vec![
                    button(self.t.s("plan_start"), "plan.start", "").tone("primary"),
                    button(self.t.s("plan_clear"), "plan.clear", ""),
                ],
                ..block("buttons")
            });
        }
        out.push(Block {
            title: self.t.s("away_title"),
            text: self.t.t(
                "away_text",
                &[("cap", dur(c::AWAY_CAP_MS)), ("after", dur(c::RETURN_AFTER_MS))],
            ),
            ..block("card")
        });
        out
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
                self.t.s("keep_log"),
            ],
            list2: vec![
                self.t.s("lose_levels"),
                self.t.s("lose_mastery"),
                self.t.s("lose_bank"),
                self.t.s("lose_tools"),
                self.t.s("lose_plan"),
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
                .filter(|a| a.skill == i && a.level > l0 && a.level <= l1)
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
            notes.push(
                self.t
                    .t("capped", &[("cap", dur(c::AWAY_CAP_MS)), ("lost", dur(r.capped))]),
            );
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
                let maker = TABS
                    .iter()
                    .take(SKILL_TABS)
                    .find(|t| io_of(t).1.contains(&item.as_str()))
                    .copied();
                Row {
                    title: self.t.t("stopped_out", &[("item", self.t.item(item))]),
                    buttons: maker
                        .map(|m| {
                            vec![button(
                                self.t.t("makes_more", &[("skill", self.t.skill(m).name.clone())]),
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

fn first_action(skill: usize) -> usize {
    c::ACTIONS.iter().position(|a| a.skill == skill).unwrap_or(0)
}

/// What a tab's skill takes in and gives out, for the uses and feeds links.
pub fn io_of(tab: &str) -> (Vec<&'static str>, Vec<&'static str>) {
    if tab == "farming" {
        return (
            c::CROPS.iter().map(|cr| cr.seed).collect(),
            c::CROPS.iter().map(|cr| cr.produce).collect(),
        );
    }
    if tab == "combat" {
        let mut ins: Vec<&str> = c::STYLES
            .iter()
            .flat_map(|s| s.held.iter().chain(s.spent.iter()).map(|(i, _)| *i))
            .collect();
        ins.extend(c::FOOD.iter().map(|(f, _)| *f));
        ins.extend(c::ARMOUR.iter().map(|(a, _)| *a));
        let outs = c::ENEMIES
            .iter()
            .flat_map(|e| e.drops.iter().map(|d| d.0))
            .collect();
        return (ins, outs);
    }
    let Some(i) = c::skill_index(tab) else {
        return (Vec::new(), Vec::new());
    };
    let acts = c::ACTIONS.iter().filter(|a| a.skill == i);
    let ins = acts
        .clone()
        .flat_map(|a| a.needs.iter().map(|(n, _)| *n))
        .collect();
    let outs = acts.map(|a| a.makes.0).collect();
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
    fn combat_uses_smithing_fletching_runecrafting_and_cooking() {
        let (ins, outs) = io_of("combat");
        for t in ["smithing", "fletching", "runecrafting", "cooking"] {
            assert!(io_of(t).1.iter().any(|o| ins.contains(o)), "{t}");
        }
        assert!(outs.contains(&"feathers"));
    }
}
