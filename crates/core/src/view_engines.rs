//! The engines' screens (T10 to T14): the recipe book, the Trader's towns,
//! the estate, the workshop and the observatory, and the screen a tab shows
//! before its engine opens. Like the rest of the view, every number here is
//! already formatted, and the page draws these as ordinary blocks.

use crate::content::{self as c, Engine};
use crate::text::{dur, num};
use crate::view::{block, button, chip, Bar, Block, Button, Row, View};

impl View<'_> {
    fn engine_of_tab(id: &str) -> Engine {
        match id {
            "estate" => Engine::Nobility,
            "workshop" => Engine::Inventor,
            "observatory" => Engine::Cosmic,
            _ => Engine::Trader,
        }
    }

    /// A tab whose engine has not opened: what opens it, and how far you are.
    pub(crate) fn locked(&self, id: &str) -> Vec<Block> {
        let e = Self::engine_of_tab(id);
        let at = self.s.engine_at(e);
        let tl = self.s.total_level();
        vec![
            self.head(id, self.tab_name(id), self.t.s("rank_tag"), ""),
            Block {
                text: self.t.s(&format!("about_{id}")),
                ..block("note")
            },
            Block {
                bar: Some(Bar {
                    tone: "pres".into(),
                    width: crate::text::width((tl as u64 * 10_000 / at.max(1) as u64).min(10_000)),
                    left: self
                        .t
                        .t("total_of", &[("n", tl.to_string()), ("need", at.to_string())]),
                    right: self.t.t("to_go", &[("n", at.saturating_sub(tl).to_string())]),
                }),
                ..block("bar")
            },
            Block {
                text: self.t.t("opens_at", &[("n", at.to_string())]),
                ..block("note")
            },
        ]
    }

    /// A ladder of ranks: done, now, and what the next one needs.
    fn ladder(&self, title: String, rows: Vec<(String, String)>, now: usize, rise: Vec<Button>) -> Block {
        let rows = rows
            .into_iter()
            .enumerate()
            .map(|(k, (name, needs))| Row {
                icon: if k <= now {
                    "rank".into()
                } else {
                    "rank_locked".into()
                },
                title: name,
                sub: needs,
                chip: if k == now { self.t.s("now") } else { String::new() },
                tone: if k == now {
                    "up".into()
                } else if k < now {
                    String::new()
                } else {
                    "used".into()
                },
                buttons: if k == now + 1 { rise.clone() } else { Vec::new() },
            })
            .collect();
        Block {
            title,
            rows,
            ..block("rows")
        }
    }

    fn list(&self, needs: &[(&str, u64)]) -> String {
        needs
            .iter()
            .map(|(i, n)| {
                self.t.t(
                    "need_have",
                    &[
                        ("n", num(*n)),
                        ("item", self.t.item(i)),
                        ("have", num(self.s.have(i))),
                    ],
                )
            })
            .collect::<Vec<_>>()
            .join(" + ")
    }

    /// Puts engine blocks into a skill screen, before its action cards.
    fn with_cards(&self, mut v: Vec<Block>, extra: Vec<Block>, tab: &str) -> Vec<Block> {
        if let Some(head) = v.first_mut() {
            head.icon = tab.into();
            head.title = self.tab_name(tab);
        }
        let at = v.iter().position(|b| b.kind == "cards").unwrap_or(v.len());
        v.splice(at..at, extra);
        v
    }

    // ---------- the recipe book (T10) ----------

    pub(crate) fn recipes(&self) -> Vec<Block> {
        let found = c::RECIPES
            .iter()
            .filter(|rc| self.s.recipes.iter().any(|k| k == rc.id))
            .count();
        let mut out = vec![self.head(
            "recipes",
            self.t.s("tab_recipes"),
            self.t.t(
                "found_of",
                &[("n", found.to_string()), ("max", c::RECIPES.len().to_string())],
            ),
            "mast",
        )];
        out.push(Block {
            text: self.t.s("recipes_intro"),
            ..block("note")
        });
        let rows = c::RECIPES
            .iter()
            .map(|rc| {
                let known = self.s.recipes.iter().any(|k| k == rc.id);
                let a = &c::ACTIONS[c::action_index(rc.id).unwrap_or(0)];
                Row {
                    icon: rc.id.into(),
                    title: self.t.action(rc.id),
                    sub: if known {
                        format!(
                            "{} · {}",
                            self.t.t(
                                "needs_level",
                                &[("skill", self.skill_name(a.skill)), ("n", a.level.to_string())]
                            ),
                            self.t.s(&format!("does_{}", rc.id))
                        )
                    } else {
                        self.t.t(
                            "recipe_from",
                            &[("enemy", self.t.enemy(rc.enemy)), ("n", rc.one_in.to_string())],
                        )
                    },
                    chip: if known { self.t.s("found") } else { String::new() },
                    tone: if known { "up".into() } else { "used".into() },
                    buttons: if known {
                        vec![button(self.skill_name(a.skill), "tab", c::SKILLS[a.skill].id)]
                    } else {
                        vec![button(self.t.skill("combat").name.clone(), "tab", "combat")]
                    },
                }
            })
            .collect();
        out.push(Block {
            rows,
            ..block("rows")
        });
        out.push(Block {
            text: self.t.s("recipes_copy"),
            ..block("note")
        });
        out
    }

    // ---------- the Trader (T12) ----------

    pub(crate) fn trader(&self) -> Vec<Block> {
        let mut out = vec![self.head(
            "trader",
            self.t.s("tab_trader"),
            self.t.t("lvl", &[("n", self.s.level(c::TRADING).to_string())]),
            "xp",
        )];
        out.push(Block {
            bar: Some(self.xp_bar(c::TRADING)),
            ..block("bar")
        });
        let (ins, outs) = crate::view::io_of("trader");
        out.push(self.links("trader", &ins, &outs));
        let (on_road, most) = self.s.caravan_room();
        out.push(Block {
            chips: vec![
                chip(
                    "t",
                    self.t.t(
                        "carts_out",
                        &[("n", on_road.to_string()), ("max", most.to_string())],
                    ),
                ),
                chip(
                    "g",
                    self.t.t(
                        "orders_done",
                        &[("n", num(self.s.counts.get("orders").copied().unwrap_or(0)))],
                    ),
                ),
                chip(
                    "x",
                    self.t.t(
                        "trader_bonus",
                        &[(
                            "pct",
                            (c::TRADER_RANK_PCT * self.s.rank(Engine::Trader) as u64).to_string(),
                        )],
                    ),
                ),
            ],
            ..block("chips")
        });
        if most == 0 {
            out.push(Block {
                text: self.t.s("need_cart"),
                buttons: vec![button(
                    self.t.skill("construction").name.clone(),
                    "tab",
                    "construction",
                )],
                ..block("buttons")
            });
        }
        if !self.s.caravans.is_empty() {
            let rows = self
                .s
                .caravans
                .iter()
                .map(|cv| Row {
                    icon: "cart".into(),
                    title: self.t.t(
                        "caravan_to",
                        &[
                            ("n", num(cv.n)),
                            ("item", self.t.item(&cv.item)),
                            ("town", self.t.s(&format!("town_{}", c::TOWNS[cv.town].id))),
                        ],
                    ),
                    sub: self.t.t(
                        "caravan_back",
                        &[
                            ("left", dur(cv.back.saturating_sub(self.s.sim_ms))),
                            ("gold", num(self.s.caravan_gold(cv, cv.back))),
                        ],
                    ),
                    chip: if cv.guards {
                        self.t.s("guarded")
                    } else {
                        String::new()
                    },
                    ..Row::default()
                })
                .collect();
            out.push(Block {
                title: self.t.s("on_the_road"),
                rows,
                ..block("rows")
            });
        }
        for (t, town) in c::TOWNS.iter().enumerate() {
            out.push(self.town(t, town));
        }
        let ranks = c::TRADER_RANKS
            .iter()
            .map(|rk| {
                (
                    self.t.s(&format!("trader_{}", rk.id)),
                    self.t.t(
                        &format!("trader_needs_{}", rk.id),
                        &[
                            ("lvl", rk.level.to_string()),
                            ("orders", rk.orders.to_string()),
                            ("fee", num(rk.fee)),
                            ("each", rk.each_town.to_string()),
                        ],
                    ),
                )
            })
            .collect();
        let rise = vec![button(self.t.s("rise"), "rise.trader", "")
            .tone("pres")
            .enabled(self.s.trader_ready())];
        out.push(self.ladder(self.t.s("ladder"), ranks, self.s.rank(Engine::Trader), rise));
        out
    }

    fn town(&self, t: usize, town: &c::Town) -> Block {
        let name = self.t.s(&format!("town_{}", town.id));
        if !self.s.town_open(t) {
            let why = if t == c::EMBERFALL && self.s.level(c::TRADING) >= town.level {
                self.t.s("needs_charter")
            } else {
                self.t.t(
                    "needs_level",
                    &[
                        ("skill", self.skill_name(c::TRADING)),
                        ("n", town.level.to_string()),
                    ],
                )
            };
            return Block {
                title: name,
                rows: vec![Row {
                    icon: "town".into(),
                    title: why,
                    sub: self.town_sub(town),
                    tone: "used".into(),
                    ..Row::default()
                }],
                ..block("rows")
            };
        }
        let at = self.s.sim_ms;
        let hours: Vec<String> = (1..=6u64)
            .map(|h| {
                self.t.t(
                    "price_in",
                    &[
                        ("h", h.to_string()),
                        ("pct", self.s.price_pct(t, at + h * c::HOUR).to_string()),
                    ],
                )
            })
            .collect();
        let mut rows = vec![Row {
            icon: "town".into(),
            title: self
                .t
                .t("price_now", &[("pct", self.s.price_pct(t, at).to_string())]),
            sub: format!("{} · {}", self.town_sub(town), hours.join(" · ")),
            ..Row::default()
        }];
        let next_orders = dur(c::ORDER_MS - at % c::ORDER_MS);
        for o in self.s.orders_now(t) {
            let filled = self.s.order_filled(&o);
            rows.push(Row {
                icon: o.item.into(),
                title: self
                    .t
                    .t("order_for", &[("n", num(o.n)), ("item", self.t.item(o.item))]),
                sub: self.t.t(
                    "order_pays",
                    &[
                        ("gold", num(o.gold)),
                        ("xp", num(o.xp)),
                        ("have", num(self.s.have(o.item))),
                        ("left", next_orders.clone()),
                    ],
                ),
                chip: if filled { self.t.s("filled") } else { String::new() },
                tone: if filled { "used".into() } else { String::new() },
                buttons: if filled {
                    Vec::new()
                } else {
                    vec![button(self.t.s("fill"), "order.fill", &format!("{t}:{}", o.slot))
                        .tone("good")
                        .enabled(self.s.have(o.item) >= o.n)]
                },
            });
        }
        let (out_now, most) = self.s.caravan_room();
        for item in town.wants.iter().filter(|i| self.s.have(i) > 0) {
            let n = self.s.have(item).min(c::CART_LOAD);
            let cv = crate::game::Caravan {
                town: t,
                item: (*item).into(),
                n,
                back: at + town.trip_ms,
                guards: false,
            };
            let free = out_now < most;
            rows.push(Row {
                icon: "cart".into(),
                title: self.t.t("send_n", &[("n", num(n)), ("item", self.t.item(item))]),
                sub: self.t.t(
                    "send_sub",
                    &[
                        ("trip", dur(town.trip_ms)),
                        ("gold", num(self.s.caravan_gold(&cv, cv.back))),
                        ("pct", self.s.price_pct(t, cv.back).to_string()),
                        ("risk", self.s.bandit_pct(t, false).to_string()),
                        ("safe", self.s.bandit_pct(t, true).to_string()),
                        ("back", num(n / town.back_per).to_string()),
                        ("sells", self.t.item(town.sells).to_lowercase()),
                    ],
                ),
                buttons: vec![
                    button(self.t.s("send"), "caravan.send", &format!("{t}:{item}"))
                        .tone("primary")
                        .enabled(free),
                    button(
                        self.t.t("send_guarded", &[("cost", num(self.s.guard_cost(t)))]),
                        "caravan.guarded",
                        &format!("{t}:{item}"),
                    )
                    .enabled(free && self.s.gold >= self.s.guard_cost(t)),
                ],
                ..Row::default()
            });
        }
        Block {
            title: name,
            rows,
            ..block("rows")
        }
    }

    fn town_sub(&self, town: &c::Town) -> String {
        let wants: Vec<String> = town.wants.iter().map(|i| self.t.item(i).to_lowercase()).collect();
        self.t.t(
            "town_sub",
            &[
                ("wants", wants.join(", ")),
                ("sells", self.t.item(town.sells).to_lowercase()),
            ],
        )
    }

    // ---------- the estate (T11) ----------

    pub(crate) fn estate(&self) -> Vec<Block> {
        let rank = self.s.rank(Engine::Nobility);
        let mut out = vec![self.head(
            "estate",
            self.t.s("tab_estate"),
            self.t.s(&format!("noble_{}", c::NOBLE_RANKS[rank].id)),
            "pres",
        )];
        let (ins, outs) = crate::view::io_of("estate");
        out.push(self.links("estate", &ins, &outs));
        out.push(Block {
            chips: vec![
                chip(
                    "g",
                    self.t
                        .t("income_per_hour", &[("n", num(c::NOBLE_RANKS[rank].income))]),
                ),
                chip(
                    "t",
                    self.t.t(
                        "servant_slots",
                        &[
                            (
                                "n",
                                self.s
                                    .servants
                                    .iter()
                                    .filter(|s| !s.job.is_empty())
                                    .count()
                                    .to_string(),
                            ),
                            ("max", self.s.servant_slots().to_string()),
                        ],
                    ),
                ),
                chip(
                    "t",
                    self.t
                        .t("room_levels", &[("n", self.s.room_levels().to_string())]),
                ),
                chip(
                    "t",
                    self.t
                        .t("con_level", &[("n", self.s.level(c::CONSTRUCTION).to_string())]),
                ),
            ],
            ..block("chips")
        });
        // rooms
        let rows = c::ROOMS
            .iter()
            .enumerate()
            .map(|(k, room)| {
                let l = self.s.rooms[k];
                let cost = self.s.room_cost(k);
                Row {
                    icon: room.id.into(),
                    title: self.t.t(
                        "room_lvl",
                        &[
                            ("room", self.t.s(&format!("room_{}", room.id))),
                            ("n", l.to_string()),
                            ("max", room.max.to_string()),
                        ],
                    ),
                    sub: match cost {
                        Some((plank, p, n, st, g, lvl)) => format!(
                            "{} · {}",
                            self.t.s(&format!("room_{}_does", room.id)),
                            self.t.t(
                                "room_cost",
                                &[
                                    ("list", self.list(&[(plank, p), ("nails", n), ("limestone", st)])),
                                    ("gold", num(g)),
                                    ("lvl", lvl.to_string()),
                                ],
                            )
                        ),
                        None => self.t.s(&format!("room_{}_does", room.id)),
                    },
                    buttons: if cost.is_some() {
                        vec![button(
                            if l == 0 {
                                self.t.s("build")
                            } else {
                                self.t.s("upgrade")
                            },
                            "room.build",
                            &k.to_string(),
                        )
                        .tone("primary")
                        .enabled(self.s.can_build(k))]
                    } else {
                        Vec::new()
                    },
                    ..Row::default()
                }
            })
            .collect();
        out.push(Block {
            title: self.t.s("rooms_title"),
            rows,
            ..block("rows")
        });
        // servants
        let slots = self.s.servant_slots();
        if slots > 0 {
            let doing = self
                .s
                .active
                .clone()
                .filter(|id| c::action_index(id).is_some())
                .unwrap_or_default();
            let rows = (0..slots as usize)
                .map(|i| {
                    let sv = self.s.servants.get(i).cloned().unwrap_or_default();
                    let need = if i < self.s.servants.len() {
                        self.s.servant_need(i)
                    } else {
                        None
                    };
                    let mut buttons = vec![button(
                        if doing.is_empty() {
                            self.t.s("servant_give_none")
                        } else {
                            self.t
                                .t("servant_give", &[("what", self.t.action(&doing).to_lowercase())])
                        },
                        "servant.set",
                        &i.to_string(),
                    )
                    .enabled(!doing.is_empty())];
                    if !sv.job.is_empty() {
                        buttons.push(
                            button(self.t.s("dismiss"), "servant.dismiss", &i.to_string()).tone("plain"),
                        );
                    }
                    Row {
                        icon: if sv.job.is_empty() {
                            "servant".into()
                        } else {
                            sv.job.clone()
                        },
                        title: if sv.job.is_empty() {
                            self.t.s("servant_free")
                        } else {
                            self.t.t("servant_doing", &[("what", self.t.action(&sv.job))])
                        },
                        sub: if sv.job.is_empty() {
                            self.t.s("servant_free_sub")
                        } else if sv.unpaid {
                            self.t.s("servant_unpaid")
                        } else {
                            match need {
                                Some(ms) => self.t.t("servant_each", &[("t", dur(ms))]),
                                None => self.t.s("servant_waiting"),
                            }
                        },
                        tone: if sv.unpaid { "used".into() } else { String::new() },
                        buttons,
                        ..Row::default()
                    }
                })
                .collect();
            out.push(Block {
                title: self.t.s("servants_title"),
                rows,
                ..block("rows")
            });
            out.push(Block {
                text: self.t.t("servants_note", &[("wage", num(c::SERVANT_WAGE))]),
                ..block("note")
            });
        }
        // the ladder
        let ranks = c::NOBLE_RANKS
            .iter()
            .enumerate()
            .map(|(k, rk)| {
                let mut needs = self.t.t(
                    "noble_needs",
                    &[
                        ("gold", num(rk.gold)),
                        ("rooms", rk.rooms.to_string()),
                        ("income", num(rk.income)),
                    ],
                );
                if k >= c::DEED_FROM {
                    let d = k - c::DEED_FROM;
                    needs = format!(
                        "{needs} · {}",
                        self.t.t(
                            "deed",
                            &[
                                ("enemy", self.t.enemy(c::CHAMPIONS[d])),
                                ("tribute", self.list(c::TRIBUTES[d])),
                            ],
                        )
                    );
                }
                if k == 0 {
                    needs = self.t.s("noble_start");
                }
                (self.t.s(&format!("noble_{}", rk.id)), needs)
            })
            .collect();
        let rise = match self.s.noble_next() {
            Some((gold, rooms, champion, tribute)) => {
                let base = gold == 0 && rooms == 0;
                if rank + 1 < c::DEED_FROM {
                    vec![button(self.t.s("rise"), "rise.noble", "")
                        .tone("pres")
                        .enabled(base)]
                } else {
                    vec![
                        button(self.t.s("rise_champion"), "rise.noble", "champion")
                            .tone("pres")
                            .enabled(base && champion),
                        button(self.t.s("rise_tribute"), "rise.noble", "tribute")
                            .tone("pres")
                            .enabled(base && tribute),
                    ]
                }
            }
            None => Vec::new(),
        };
        out.push(self.ladder(self.t.s("ladder"), ranks, rank, rise));
        out
    }

    // ---------- the workshop (T14) ----------

    pub(crate) fn workshop(&self) -> Vec<Block> {
        if !self.s.engine_open(Engine::Inventor) {
            return self.needs_room("workshop");
        }
        let mut extra = Vec::new();
        let rank = self.s.rank(Engine::Inventor);
        if let Some(rs) = &self.s.research {
            let bp = &c::BLUEPRINTS[c::blueprint_index(&rs.id).unwrap_or(0)];
            extra.push(Block {
                title: self
                    .t
                    .t("researching", &[("what", self.t.s(&format!("bp_{}", bp.id)))]),
                bar: Some(Bar {
                    tone: "mast".into(),
                    width: crate::text::width(rs.progress.min(bp.ms) * 10_000 / bp.ms.max(1)),
                    left: dur(rs.progress.min(bp.ms)),
                    right: self
                        .t
                        .t("left_t", &[("t", dur(bp.ms.saturating_sub(rs.progress)))]),
                }),
                ..block("bar")
            });
        }
        let rows = c::BLUEPRINTS
            .iter()
            .map(|bp| {
                let done = self.s.blueprints.iter().any(|k| k == bp.id);
                let now = self.s.research.as_ref().is_some_and(|r| r.id == bp.id);
                Row {
                    icon: bp.id.into(),
                    title: self.t.s(&format!("bp_{}", bp.id)),
                    sub: format!(
                        "{} · {}",
                        self.t.s(&format!("bp_{}_does", bp.id)),
                        self.t.t(
                            "bp_cost",
                            &[
                                ("lvl", bp.level.to_string()),
                                ("list", self.list(bp.needs)),
                                ("gold", num(bp.gold)),
                                ("t", dur(bp.ms)),
                            ],
                        )
                    ),
                    chip: if self.s.have(bp.id) > 0 {
                        self.t.s("built")
                    } else if done {
                        self.t.s("researched")
                    } else if now {
                        self.t.s("now")
                    } else {
                        String::new()
                    },
                    tone: if done { "up".into() } else { String::new() },
                    buttons: if done || now || self.s.have(bp.id) > 0 {
                        Vec::new()
                    } else {
                        vec![button(self.t.s("research_go"), "research", bp.id)
                            .tone("primary")
                            .enabled(self.s.can_research(bp.id))]
                    },
                }
            })
            .collect();
        extra.push(Block {
            title: self.t.s("blueprints_title"),
            rows,
            ..block("rows")
        });
        let rows = c::MACHINES
            .iter()
            .filter(|m| self.s.have(m.id) > 0)
            .map(|m| {
                let off = self.s.machines_off.iter().any(|k| k == m.id);
                let status = if off {
                    self.t.s("machine_off")
                } else if m.needs.iter().any(|(i, n)| self.s.have(i) < *n) {
                    self.t.t("machine_needs", &[("item", self.t.item(m.needs[0].0))])
                } else if self.s.fuel_for(m).is_none() {
                    self.t.s("machine_no_fuel")
                } else {
                    self.t.t(
                        "machine_on",
                        &[
                            ("t", dur(self.s.machine_cycle(m))),
                            ("item", self.t.item(m.makes.0)),
                            (
                                "fuel",
                                self.t.item(self.s.fuel_for(m).unwrap_or("logs")).to_lowercase(),
                            ),
                        ],
                    )
                };
                Row {
                    icon: m.id.into(),
                    title: self.t.s(&format!("bp_{}", m.id)),
                    sub: status,
                    buttons: vec![button(
                        if off {
                            self.t.s("switch_on")
                        } else {
                            self.t.s("switch_off")
                        },
                        "machine.toggle",
                        m.id,
                    )],
                    ..Row::default()
                }
            })
            .collect::<Vec<_>>();
        if !rows.is_empty() {
            extra.push(Block {
                title: self.t.s("machines_title"),
                rows,
                ..block("rows")
            });
        }
        let gadgets: Vec<_> = c::GADGETS
            .iter()
            .filter(|(g, _)| self.s.have(g) > 0)
            .map(|(g, sk)| {
                chip(
                    "g",
                    self.t.t(
                        "gadget_on",
                        &[
                            ("what", self.t.item(g)),
                            ("skill", self.skill_name(*sk)),
                            ("pct", c::GADGET_PCT.to_string()),
                        ],
                    ),
                )
            })
            .collect();
        if !gadgets.is_empty() {
            extra.push(Block {
                chips: gadgets,
                ..block("chips")
            });
        }
        let ranks = c::INVENTOR_RANKS
            .iter()
            .map(|rk| {
                (
                    self.t.s(&format!("inventor_{}", rk.id)),
                    self.t.t(
                        &format!("inventor_needs_{}", rk.id),
                        &[("lvl", rk.level.to_string()), ("room", rk.workshop.to_string())],
                    ),
                )
            })
            .collect();
        let rise = vec![button(self.t.s("rise"), "rise.inventor", "")
            .tone("pres")
            .enabled(self.s.inventor_ready())];
        extra.push(self.ladder(self.t.s("ladder"), ranks, rank, rise));
        self.with_cards(self.skill(c::INVENTION), extra, "workshop")
    }

    fn needs_room(&self, id: &str) -> Vec<Block> {
        vec![
            self.head(id, self.tab_name(id), self.t.s("rank_tag"), ""),
            Block {
                text: self.t.s(&format!("about_{id}")),
                ..block("note")
            },
            Block {
                text: self.t.s(&format!("needs_room_{id}")),
                buttons: vec![button(self.t.s("tab_estate"), "tab", "estate").tone("primary")],
                ..block("buttons")
            },
        ]
    }

    // ---------- the observatory (T13) ----------

    pub(crate) fn observatory(&self) -> Vec<Block> {
        if !self.s.engine_open(Engine::Cosmic) {
            return self.needs_room("observatory");
        }
        let rank = self.s.rank(Engine::Cosmic);
        let sign = self.s.sign();
        let mut extra = Vec::new();
        let skills = |k: usize| {
            c::SIGNS[k]
                .1
                .iter()
                .map(|&s| self.skill_name(s))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let mut rows = vec![Row {
            icon: format!("sign_{}", c::SIGNS[sign].0),
            title: self.t.t(
                "sign_up",
                &[("sign", self.t.s(&format!("sign_{}", c::SIGNS[sign].0)))],
            ),
            sub: self.t.t(
                "sign_sub",
                &[("skills", skills(sign)), ("left", dur(self.s.sign_left()))],
            ),
            tone: "up".into(),
            ..Row::default()
        }];
        let start = self.s.sim_ms + self.s.sign_left();
        for k in 0..(c::SIGNS.len() - 1) as u64 {
            let at = start + k * c::SIGN_MS;
            let next = self.s.sign_at(at);
            rows.push(Row {
                icon: format!("sign_{}", c::SIGNS[next].0),
                title: self.t.s(&format!("sign_{}", c::SIGNS[next].0)),
                sub: self.t.t(
                    "sign_later",
                    &[("skills", skills(next)), ("in", dur(at - self.s.sim_ms))],
                ),
                ..Row::default()
            });
        }
        extra.push(Block {
            title: self.t.s("sky_title"),
            rows,
            ..block("rows")
        });
        let mut chips = vec![
            chip(
                "p",
                self.t
                    .t("starlight_have", &[("n", num(self.s.have("starlight")))]),
            ),
            chip(
                "p",
                self.t.t("stardust_have", &[("n", num(self.s.have("stardust")))]),
            ),
            chip("t", self.t.t("lens_have", &[("n", num(self.s.have("lens")))])),
            chip("t", self.t.t("away_cap_now", &[("t", dur(self.s.away_cap()))])),
        ];
        if self.s.harvest_moon() {
            chips.push(chip(
                "g",
                self.t.t(
                    "harvest_moon_on",
                    &[("pct", (100 - c::HARVEST_MOON_PCT).to_string())],
                ),
            ));
        }
        extra.push(Block {
            chips,
            ..block("chips")
        });
        let ranks = c::COSMIC_RANKS
            .iter()
            .map(|rk| {
                (
                    self.t.s(&format!("cosmic_{}", rk.id)),
                    self.t.t(
                        &format!("cosmic_needs_{}", rk.id),
                        &[
                            ("lvl", rk.level.to_string()),
                            ("starlight", num(rk.starlight)),
                            ("stardust", num(rk.stardust)),
                            ("lenses", rk.lenses.to_string()),
                        ],
                    ),
                )
            })
            .collect();
        let rise = vec![button(self.t.s("rise"), "rise.cosmic", "")
            .tone("pres")
            .enabled(self.s.cosmic_ready())];
        extra.push(self.ladder(self.t.s("ladder"), ranks, rank, rise));
        self.with_cards(self.skill(c::ATTUNEMENT), extra, "observatory")
    }
}
