//! The engines' screens (T10 to T14) and the regions (T15): the recipe book,
//! the Trader's towns, the estate, the workshop, the observatory, the regions
//! screen, and the screen a tab shows before its engine opens. Like the rest
//! of the view, every number here is already formatted, and the page draws
//! these as ordinary blocks.

use crate::content::{self as c, Engine};
use crate::text::{dur, num, width};
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

    pub(crate) fn region_name(&self, r: usize) -> String {
        self.t.s(&format!("region_{}", c::REGIONS[r].0))
    }

    /// The open regions, the Vale first, or only the one the filter shows (T25).
    fn open_regions(&self) -> Vec<usize> {
        (0..c::REGIONS.len())
            .filter(|&r| self.s.region_open(r) && self.region_shown(r))
            .collect()
    }

    /// A tab whose engine has not opened: what opens it, and how far you are.
    pub(crate) fn locked(&self, id: &str) -> Vec<Block> {
        let e = Self::engine_of_tab(id);
        let at = if matches!(id, "performance" | "stage") {
            c::FAME_AT
        } else {
            self.s.engine_at(e)
        };
        let tl = self.s.total_level();
        vec![
            self.head(id, self.tab_name(id), self.t.s("rank_tag"), ""),
            Block {
                text: self.t.s(&format!("about_{id}")),
                ..block("note")
            },
            self.total_bar(tl, at),
            Block {
                text: self.t.t("opens_at", &[("n", at.to_string())]),
                ..block("note")
            },
        ]
    }

    fn total_bar(&self, tl: u32, at: u32) -> Block {
        Block {
            bar: Some(Bar {
                tone: "pres".into(),
                width: width((tl as u64 * 10_000 / at.max(1) as u64).min(10_000)),
                left: self
                    .t
                    .t("total_of", &[("n", tl.to_string()), ("need", at.to_string())]),
                right: if tl >= at {
                    self.t.s("ready")
                } else {
                    self.t.t("to_go", &[("n", (at - tl).to_string())])
                },
            }),
            ..block("bar")
        }
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

    fn ladder_title(&self, r: usize) -> String {
        self.t.t("ladder_in", &[("region", self.region_name(r))])
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
        let at = v
            .iter()
            .position(|b| {
                b.kind == "cards" || (b.kind == "buttons" && b.buttons.iter().any(|x| x.act == "list.all"))
            })
            .unwrap_or(v.len());
        v.splice(at..at, extra);
        v
    }

    // ---------- the regions (T15) ----------

    pub(crate) fn regions(&self) -> Vec<Block> {
        let tl = self.s.total_level();
        let mut out = vec![self.head(
            "regions",
            self.t.s("tab_regions"),
            self.t.t(
                "regions_open",
                &[
                    ("n", self.s.regions_open().to_string()),
                    ("max", c::REGIONS.len().to_string()),
                ],
            ),
            "pres",
        )];
        out.push(Block {
            text: self.t.s("regions_intro"),
            ..block("note")
        });
        out.extend(self.store());
        for (r, &(id, at)) in c::REGIONS.iter().enumerate() {
            let gathers = |a: &c::Action| matches!(a.skill, c::WOODCUTTING | c::MINING | c::FISHING);
            let gather = c::ACTIONS.iter().filter(|a| a.region == r && gathers(a)).count();
            let make = c::ACTIONS.iter().filter(|a| a.region == r && !gathers(a)).count();
            let enemies = c::ENEMIES
                .iter()
                .filter(|e| e.region == r && !c::is_champion(e.id))
                .count();
            let champs = c::ENEMIES
                .iter()
                .filter(|e| e.region == r && c::is_champion(e.id))
                .count();
            let crops = c::CROPS.iter().filter(|cr| cr.region == r).count();
            let towns = c::TOWNS.iter().filter(|t| t.region == r).count();
            let open = self.s.region_open(r);
            let mut rows = vec![Row {
                icon: format!("region_{id}"),
                title: if open {
                    self.t.s("region_is_open")
                } else if self.s.region_reached(r) {
                    self.t.s("region_sealed")
                } else {
                    self.t.t("opens_at", &[("n", at.to_string())])
                },
                sub: self.t.t(
                    "region_holds",
                    &[
                        ("gather", gather.to_string()),
                        ("make", make.to_string()),
                        ("enemies", enemies.to_string()),
                        ("champions", champs.to_string()),
                        ("crops", crops.to_string()),
                        ("towns", towns.to_string()),
                    ],
                ),
                tone: if open { "up".into() } else { "used".into() },
                ..Row::default()
            }];
            if let Some(d) = c::DUNGEONS.iter().find(|d| d.region == r) {
                rows.push(Row {
                    icon: d.stages[d.stages.len() - 1].into(),
                    title: self.t.s(&format!("dungeon_{}", d.id)),
                    sub: self.t.s("dungeon_about"),
                    buttons: if open {
                        vec![button(self.t.skill("combat").name.clone(), "tab", "combat")]
                    } else {
                        Vec::new()
                    },
                    ..Row::default()
                });
            }
            out.push(Block {
                title: self.region_name(r),
                rows,
                ..block("rows")
            });
            if !open {
                out.push(self.total_bar(tl, at));
            }
        }
        out.push(Block {
            text: self
                .t
                .t("regions_prestige", &[("n", c::PRESTIGE_BASE.to_string())]),
            ..block("note")
        });
        out
    }

    /// The sheet shown once in the app when the Coast is reached without the
    /// purchase (T18, IG-Q50 B).
    pub(crate) fn offer_sheet(&self) -> crate::view::Sheet {
        let buy = if self.device.price.is_empty() {
            self.t.s("store_buy_plain")
        } else {
            self.t.t("store_buy", &[("price", self.device.price.clone())])
        };
        crate::view::Sheet {
            kind: "notice".into(),
            title: self.t.s("offer_title"),
            lines: vec![self.t.s("offer_text"), self.t.s("store_offer")],
            buttons: vec![
                button(self.t.s("not_yet"), "sheet.close", ""),
                button(buy, "page.buy", "").tone("primary"),
            ],
            ..Default::default()
        }
    }

    /// The app's one purchase (IG-Q46 B): what it opens, a button to buy it
    /// and one to restore it. Nothing on the web, which is whole (IG-Q49 A).
    fn store(&self) -> Vec<Block> {
        if !self.device.app {
            return Vec::new();
        }
        if !self.s.sealed {
            return vec![Block {
                text: self.t.s("store_owned"),
                ..block("note")
            }];
        }
        let buy = if self.device.price.is_empty() {
            self.t.s("store_buy_plain")
        } else {
            self.t.t("store_buy", &[("price", self.device.price.clone())])
        };
        vec![Block {
            title: self.t.s("store_title"),
            text: self.t.s("store_offer"),
            buttons: vec![
                button(buy, "page.buy", "").tone("primary"),
                button(self.t.s("store_restore"), "page.restore", ""),
            ],
            ..block("buttons")
        }]
    }

    // ---------- the stage: Fame, shows and followers (T18) ----------

    pub(crate) fn stage(&self) -> Vec<Block> {
        let mut out = vec![self.head(
            "stage",
            self.t.s("tab_stage"),
            self.t.t("fame_tag", &[("pct", self.s.fame_pct().to_string())]),
            "pres",
        )];
        out.push(Block {
            text: self.t.t(
                "stage_intro",
                &[
                    ("pct", c::FAME_PCT.to_string()),
                    ("n", c::FOLLOWER_RANKS.len().to_string()),
                ],
            ),
            ..block("note")
        });
        // The show running, or the venues to book.
        if let Some(sh) = &self.s.show {
            let v = c::venue_index(&sh.venue).map(|i| &c::VENUES[i]);
            let total = v.map_or(1, |v| v.ms);
            let left = sh.back.saturating_sub(self.s.sim_ms);
            out.push(Block {
                title: self
                    .t
                    .t("show_on", &[("venue", self.t.s(&format!("venue_{}", sh.venue)))]),
                bar: Some(Bar {
                    tone: "pres".into(),
                    width: width((total - left.min(total)) * 10_000 / total.max(1)),
                    left: self.t.t("left_time", &[("t", dur(left))]),
                    right: v.map_or(String::new(), |v| {
                        self.t
                            .t("show_pays", &[("n", num(v.gold)), ("renown", num(v.renown))])
                    }),
                }),
                ..block("bar")
            });
        }
        let rows = c::VENUES
            .iter()
            .filter(|v| self.s.region_open(v.region))
            .map(|v| {
                let ready = self.s.venue_ready(v);
                let rank_ok = self.s.fame[v.region] >= v.rank;
                Row {
                    icon: "stage".into(),
                    title: self.t.s(&format!("venue_{}", v.id)),
                    sub: if rank_ok {
                        self.t.t(
                            "venue_sub",
                            &[
                                ("t", dur(v.ms)),
                                ("needs", self.list(v.needs)),
                                ("n", num(v.gold)),
                                ("renown", num(v.renown)),
                            ],
                        )
                    } else {
                        self.t.t(
                            "venue_needs_rank",
                            &[(
                                "rank",
                                self.t.s(&format!("fame_{}", c::FAME_RANKS[v.region][v.rank].id)),
                            )],
                        )
                    },
                    tone: if rank_ok { String::new() } else { "used".into() },
                    buttons: if rank_ok {
                        vec![button(self.t.s("book"), "book", v.id)
                            .tone("primary")
                            .enabled(ready.is_ok())]
                    } else {
                        Vec::new()
                    },
                    ..Row::default()
                }
            })
            .collect();
        out.push(Block {
            title: self.t.s("venues_title"),
            rows,
            ..block("rows")
        });
        // Followers.
        let mut rows: Vec<Row> = Vec::new();
        for i in 0..self.s.followers.len() {
            let f = &self.s.followers[i];
            let job = if f.job.is_empty() {
                self.t.s("follower_rests")
            } else {
                self.t.t("follower_job", &[("what", self.t.action(&f.job))])
            };
            let mut buttons: Vec<Button> = [c::WOODCUTTING, c::MINING, c::FISHING]
                .iter()
                .filter_map(|&sk| {
                    let best = c::ACTIONS
                        .iter()
                        .filter(|a| a.skill == sk && self.s.follower_can(a.id))
                        .max_by_key(|a| (a.level, a.region))?;
                    Some(
                        button(self.skill_name(sk), "follower.set", &format!("{i}:{}", best.id))
                            .icon(c::SKILLS[sk].id)
                            .pressed(f.job == best.id),
                    )
                })
                .collect();
            buttons.push(button(self.t.s("follower_rest"), "follower.set", &format!("{i}:")).tone("plain"));
            rows.push(Row {
                icon: "follower".into(),
                title: self.t.t(
                    "follower_title",
                    &[
                        ("k", (i + 1).to_string()),
                        ("lvl", self.s.follower_level(i).to_string()),
                    ],
                ),
                sub: self.t.t(
                    "follower_sub",
                    &[("job", job), ("pct", self.s.follower_pct(i).to_string())],
                ),
                buttons,
                ..Row::default()
            });
        }
        let title = self.t.t(
            "followers_title",
            &[
                ("n", self.s.followers.len().to_string()),
                ("max", c::FOLLOWER_RANKS.len().to_string()),
            ],
        );
        // A rows block with no rows is one the page cannot draw: with no
        // followers yet, say how to get one instead.
        out.push(if rows.is_empty() {
            Block {
                title,
                text: self.t.s("followers_none"),
                ..block("card")
            }
        } else {
            Block {
                title,
                rows,
                ..block("rows")
            }
        });
        // The ladders.
        for r in self.open_regions() {
            let ranks = c::FAME_RANKS[r]
                .iter()
                .map(|rk| {
                    (
                        self.t.s(&format!("fame_{}", rk.id)),
                        self.t.t(
                            "fame_needs",
                            &[
                                ("lvl", rk.level.to_string()),
                                ("renown", num(rk.renown)),
                                ("gold", num(rk.gold)),
                            ],
                        ),
                    )
                })
                .collect();
            let rise = vec![button(self.t.s("rise"), "rise.fame", &r.to_string())
                .tone("pres")
                .enabled(self.s.fame_ready_in(r))];
            out.push(self.ladder(self.ladder_title(r), ranks, self.s.fame[r], rise));
        }
        out
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
        for r in self.open_regions() {
            let rows = c::RECIPES
                .iter()
                .filter(|rc| rc.region == r)
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
                title: self.region_name(r),
                rows,
                ..block("rows")
            });
        }
        let closed = c::RECIPES
            .iter()
            .filter(|rc| !self.s.region_open(rc.region))
            .count();
        if closed > 0 {
            out.push(Block {
                text: self.t.t("recipes_closed", &[("n", closed.to_string())]),
                ..block("note")
            });
        }
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
        let mut chips = Vec::new();
        for veh in ["cart", "ship", "mule_train", "sandship"] {
            if veh != "cart"
                && !c::TOWNS
                    .iter()
                    .any(|t| t.vehicle == veh && self.s.region_open(t.region))
            {
                continue;
            }
            let (on_road, most) = self.s.caravan_room_for(veh);
            chips.push(chip(
                "t",
                self.t.t(
                    "vehicles_out",
                    &[
                        ("what", self.t.item(veh)),
                        ("n", on_road.to_string()),
                        ("max", most.to_string()),
                    ],
                ),
            ));
        }
        chips.push(chip(
            "g",
            self.t.t(
                "orders_done",
                &[("n", num(self.s.counts.get("orders").copied().unwrap_or(0)))],
            ),
        ));
        chips.push(chip(
            "x",
            self.t.t(
                "trader_bonus",
                &[(
                    "pct",
                    (c::TRADER_RANK_PCT * self.s.rank_sum(Engine::Trader) as u64).to_string(),
                )],
            ),
        ));
        out.push(Block {
            chips,
            ..block("chips")
        });
        if self.s.caravan_room().1 == 0 {
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
                    icon: c::TOWNS[cv.town].vehicle.into(),
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
        for r in self.open_regions().into_iter().rev() {
            for (t, town) in c::TOWNS.iter().enumerate().filter(|(_, t)| t.region == r) {
                out.push(self.town(t, town));
            }
        }
        for r in self.open_regions() {
            let ranks = c::TRADER_RANKS[r]
                .iter()
                .map(|rk| {
                    let key = if r == 0 {
                        format!("trader_needs_{}", rk.id)
                    } else {
                        "trader_needs_tier".to_string()
                    };
                    (
                        self.t.s(&format!("trader_{}", rk.id)),
                        self.t.t(
                            &key,
                            &[
                                ("lvl", rk.level.to_string()),
                                ("orders", rk.orders.to_string()),
                                ("fee", num(rk.fee)),
                                ("each", rk.each_town.to_string()),
                                ("region", self.region_name(r)),
                            ],
                        ),
                    )
                })
                .collect();
            let rise = vec![button(self.t.s("rise"), "rise.trader", &r.to_string())
                .tone("pres")
                .enabled(self.s.trader_ready_in(r))];
            out.push(self.ladder(
                self.ladder_title(r),
                ranks,
                self.s.rank_in(Engine::Trader, r),
                rise,
            ));
        }
        out
    }

    fn town(&self, t: usize, town: &c::Town) -> Block {
        let name = self.t.s(&format!("town_{}", town.id));
        let title = if town.region > 0 {
            format!("{name} · {}", self.region_name(town.region))
        } else {
            name
        };
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
                title,
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
        let (out_now, most) = self.s.caravan_room_for(town.vehicle);
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
                icon: town.vehicle.into(),
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
        if most == 0 {
            rows.push(Row {
                icon: town.vehicle.into(),
                title: self.t.t(
                    "needs_vehicle",
                    &[("what", self.t.item(town.vehicle).to_lowercase())],
                ),
                buttons: vec![button(
                    self.t.skill("construction").name.clone(),
                    "tab",
                    "construction",
                )],
                ..Row::default()
            });
        }
        Block {
            title,
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
        let top = self
            .open_regions()
            .into_iter()
            .rev()
            .find(|&r| r == 0 || self.s.rank_in(Engine::Nobility, r) > 0)
            .unwrap_or(0);
        let mut out = vec![self.head(
            "estate",
            self.t.s("tab_estate"),
            self.t.s(&format!(
                "noble_{}",
                c::NOBLE_RANKS[top][self.s.rank_in(Engine::Nobility, top)].id
            )),
            "pres",
        )];
        let (ins, outs) = crate::view::io_of("estate");
        out.push(self.links("estate", &ins, &outs));
        out.push(Block {
            chips: vec![
                chip("g", self.t.t("income_per_hour", &[("n", num(self.s.income()))])),
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
            .filter(|(_, room)| self.s.region_open(room.region))
            .map(|(k, room)| {
                let l = self.s.room(k);
                let cost = self.s.room_cost(k);
                let extra = self.s.room_effect(k).saturating_sub(l);
                let mut sub = self.t.s(&format!("room_{}_does", room.id));
                if extra > 0 {
                    sub = format!(
                        "{sub} · {}",
                        self.t.t("furniture_adds", &[("n", extra.to_string())])
                    );
                }
                if let Some((plank, p, n, st, g, lvl)) = cost {
                    sub = format!(
                        "{sub} · {}",
                        self.t.t(
                            "room_cost",
                            &[
                                ("list", self.list(&[(plank, p), ("nails", n), ("limestone", st)])),
                                ("gold", num(g)),
                                ("lvl", lvl.to_string()),
                            ],
                        )
                    );
                }
                Row {
                    icon: room.id.into(),
                    title: self.t.t(
                        "room_lvl",
                        &[
                            ("room", self.t.s(&format!("room_{}", room.id))),
                            ("n", l.to_string()),
                            ("max", self.s.room_max(k).to_string()),
                        ],
                    ),
                    sub,
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
        // a ladder for each open region
        for r in self.open_regions() {
            let ranks = c::NOBLE_RANKS[r]
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
                                    ("enemy", self.t.enemy(c::CHAMPIONS[r][d])),
                                    ("tribute", self.list(c::TRIBUTES[r][d])),
                                ],
                            )
                        );
                    }
                    if k == 0 {
                        needs = if r == 0 {
                            self.t.s("noble_start")
                        } else {
                            self.t.t("tier_start", &[("region", self.region_name(r))])
                        };
                    }
                    (self.t.s(&format!("noble_{}", rk.id)), needs)
                })
                .collect();
            let now = self.s.rank_in(Engine::Nobility, r);
            let rise = match self.s.noble_next_in(r) {
                Some((gold, rooms, champion, tribute)) => {
                    let base = gold == 0 && rooms == 0;
                    if now + 1 < c::DEED_FROM {
                        vec![button(self.t.s("rise"), "rise.noble", &r.to_string())
                            .tone("pres")
                            .enabled(base)]
                    } else {
                        vec![
                            button(self.t.s("rise_champion"), "rise.noble", &format!("{r}:champion"))
                                .tone("pres")
                                .enabled(base && champion),
                            button(self.t.s("rise_tribute"), "rise.noble", &format!("{r}:tribute"))
                                .tone("pres")
                                .enabled(base && tribute),
                        ]
                    }
                }
                None => Vec::new(),
            };
            out.push(self.ladder(self.ladder_title(r), ranks, now, rise));
        }
        out
    }

    // ---------- the workshop (T14) ----------

    pub(crate) fn workshop(&self) -> Vec<Block> {
        if !self.s.engine_open(Engine::Inventor) {
            return self.needs_room("workshop");
        }
        let mut extra = Vec::new();
        if let Some(rs) = &self.s.research {
            let ms = self.s.research_ms(&rs.id).unwrap_or(1).max(1);
            extra.push(Block {
                title: self
                    .t
                    .t("researching", &[("what", self.t.s(&format!("bp_{}", rs.id)))]),
                bar: Some(Bar {
                    tone: "mast".into(),
                    width: width(rs.progress.min(ms) * 10_000 / ms),
                    left: dur(rs.progress.min(ms)),
                    right: self.t.t("left_t", &[("t", dur(ms.saturating_sub(rs.progress)))]),
                }),
                ..block("bar")
            });
        }
        let rows = c::BLUEPRINTS
            .iter()
            .filter(|bp| self.s.region_open(bp.region))
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
                                ("t", dur(self.s.research_ms(bp.id).unwrap_or(bp.ms))),
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
                } else if m.fuel && self.s.fuel_for(m).is_none() {
                    self.t.s("machine_no_fuel")
                } else if !m.fuel {
                    self.t.t(
                        "machine_on_free",
                        &[
                            ("t", dur(self.s.machine_cycle(m))),
                            ("n", self.s.machine_makes(m).to_string()),
                            ("item", self.t.item(m.makes.0)),
                        ],
                    )
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
        for r in self.open_regions() {
            let ranks = c::INVENTOR_RANKS[r]
                .iter()
                .map(|rk| {
                    let key = if r == 0 {
                        format!("inventor_needs_{}", rk.id)
                    } else {
                        "inventor_needs_tier".to_string()
                    };
                    let also = if rk.every_machine {
                        self.t.s("inv_every_machine")
                    } else if rk.star_core {
                        self.t.s("inv_star_core")
                    } else if rk.machine {
                        self.t.s("inv_a_machine")
                    } else {
                        self.t.s("inv_nothing_else")
                    };
                    (
                        self.t.s(&format!("inventor_{}", rk.id)),
                        self.t.t(
                            &key,
                            &[
                                ("lvl", rk.level.to_string()),
                                ("room", rk.workshop.to_string()),
                                ("region", self.region_name(r)),
                                ("also", also),
                            ],
                        ),
                    )
                })
                .collect();
            let rise = vec![button(self.t.s("rise"), "rise.inventor", &r.to_string())
                .tone("pres")
                .enabled(self.s.inventor_ready_in(r))];
            extra.push(self.ladder(
                self.ladder_title(r),
                ranks,
                self.s.rank_in(Engine::Inventor, r),
                rise,
            ));
        }
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
        let mut extra = Vec::new();
        let skills = |sky: usize, k: usize| {
            c::SKIES[sky].signs[k]
                .1
                .iter()
                .map(|&s| self.skill_name(s))
                .collect::<Vec<_>>()
                .join(", ")
        };
        for sky in (0..c::SKIES.len()).filter(|&k| self.s.sky_open(k)) {
            let now = self.s.sky_sign_at(sky, self.s.sim_ms);
            let id = c::SKIES[sky].signs[now].0;
            let left = self.s.sky_left(sky);
            let mut rows = vec![Row {
                icon: format!("sign_{id}"),
                title: self.t.t("sign_up", &[("sign", self.t.s(&format!("sign_{id}")))]),
                sub: self
                    .t
                    .t("sign_sub", &[("skills", skills(sky, now)), ("left", dur(left))]),
                tone: "up".into(),
                ..Row::default()
            }];
            let start = self.s.sim_ms + left;
            for k in 0..5u64 {
                let at = start + k * c::SKIES[sky].sign_ms;
                let next = self.s.sky_sign_at(sky, at);
                let nid = c::SKIES[sky].signs[next].0;
                rows.push(Row {
                    icon: format!("sign_{nid}"),
                    title: self.t.s(&format!("sign_{nid}")),
                    sub: self.t.t(
                        "sign_later",
                        &[("skills", skills(sky, next)), ("in", dur(at - self.s.sim_ms))],
                    ),
                    ..Row::default()
                });
            }
            extra.push(Block {
                title: self.t.t("sky_of", &[("region", self.region_name(sky))]),
                rows,
                ..block("rows")
            });
        }
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
        if self.s.sky_open(1) {
            chips.push(chip("p", self.t.s("constellations")));
        }
        extra.push(Block {
            chips,
            ..block("chips")
        });
        for r in self.open_regions() {
            let ranks = c::COSMIC_RANKS[r]
                .iter()
                .map(|rk| {
                    let key = if r == 0 {
                        format!("cosmic_needs_{}", rk.id)
                    } else {
                        "cosmic_needs_tier".to_string()
                    };
                    (
                        self.t.s(&format!("cosmic_{}", rk.id)),
                        self.t.t(
                            &key,
                            &[
                                ("lvl", rk.level.to_string()),
                                ("starlight", num(rk.starlight)),
                                ("stardust", num(rk.stardust)),
                                ("lenses", rk.lenses.to_string()),
                                ("region", self.region_name(r)),
                            ],
                        ),
                    )
                })
                .collect();
            let rise = vec![button(self.t.s("rise"), "rise.cosmic", &r.to_string())
                .tone("pres")
                .enabled(self.s.cosmic_ready_in(r))];
            extra.push(self.ladder(
                self.ladder_title(r),
                ranks,
                self.s.rank_in(Engine::Cosmic, r),
                rise,
            ));
        }
        self.with_cards(self.skill(c::ATTUNEMENT), extra, "observatory")
    }
}
