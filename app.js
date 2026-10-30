// The page draws what the core returns and decides nothing.
//
// The core describes every screen as blocks of a few kinds (a bar, a row of
// chips, a grid of cards ...), and this file turns each kind into DOM without
// knowing what a level or a tree is. Buttons send their action name and value
// back through the core's one door, `act`. The page's own jobs are the
// device's: the clock, the timer for the next action, saving the core's text,
// fullscreen, and handing files to the player.
import init, { Game } from "./pkg/tool_shim.js?v=bd6b3673965f";
import { load, store, stash, kept, unstash, persist } from "./save.js?v=bd6b3673965f";
import { loadSounds, unlock, play } from "./sound.js?v=bd6b3673965f";
import * as phone from "./device.js?v=bd6b3673965f";

const SVG = "http://www.w3.org/2000/svg";
const el = (id) => document.getElementById(id);
const now = () => Date.now();

let game = null;
let ui = null;
let timer = null;
let saved = "";

function h(tag, cls, text) {
  const node = document.createElement(tag);
  if (cls) node.className = cls;
  if (text !== undefined && text !== "") node.textContent = text;
  return node;
}

function icon(id, cls) {
  const svg = document.createElementNS(SVG, "svg");
  svg.setAttribute("class", cls || "ic");
  svg.setAttribute("viewBox", "0 0 40 40");
  svg.setAttribute("aria-hidden", "true");
  const use = document.createElementNS(SVG, "use");
  use.setAttribute("href", "icons.svg#" + id);
  svg.append(use);
  return svg;
}

function timing(node, dur, delay) {
  if (dur) node.style.setProperty("--dur", dur);
  if (delay) node.style.setProperty("--delay", delay);
}

// ---------- buttons go through the core, or to the device ----------

const device = {
  "page.fullscreen": fullscreen,
  "page.export": exportSave,
  "page.import": () => el("importer").click(),
  "page.playtest": sendPlaytest,
  "page.buy": async () => draw(game.act(await phone.buy(), "1", now())),
  "page.restore": async () => draw(game.act(await phone.restore(), "1", now())),
};

function press(b) {
  if (device[b.act]) return device[b.act]();
  draw(game.act(b.act, b.value || "", now()));
}

function btn(b, cls) {
  const node = h("button", "btn " + (b.tone || "") + " " + (cls || ""));
  if (b.icon) node.append(icon(b.icon));
  const label = h("span", "label", b.label);
  node.append(label);
  if (b.sub) node.append(h("small", "", b.sub));
  node.disabled = !b.enabled;
  if (b.pressed) node.setAttribute("aria-pressed", "true");
  node.dataset.act = b.act;
  node.dataset.value = b.value || "";
  node.addEventListener("click", () => press(b));
  return node;
}

function bar(b) {
  const box = h("div", "bar " + b.tone);
  const fill = h("i");
  fill.style.width = b.width;
  const text = h("span");
  text.append(h("b", "", b.left), h("b", "", b.right));
  box.append(fill, text);
  return box;
}

function chips(list) {
  const box = h("div", "chips");
  // A chip with a tip (T91): tapping it shows the tip under the row.
  const more = h("p", "chip-more");
  more.hidden = true;
  list.forEach((c) => {
    const chip = h(c.tip ? "button" : "span", "chip " + c.tone);
    chip.append(h("i", "dot"), h("span", "", c.text));
    if (c.tip) {
      chip.type = "button";
      chip.title = c.tip;
      chip.addEventListener("click", () => {
        const same = !more.hidden && more.textContent === c.tip;
        more.hidden = same;
        more.textContent = c.tip;
      });
    }
    box.append(chip);
  });
  if (list.some((c) => c.tip)) {
    const wrap = h("div", "");
    wrap.append(box, more);
    return wrap;
  }
  return box;
}

function progress(dur, delay, cls) {
  const box = h("div", "act-bar " + (cls || ""));
  const fill = h("i");
  timing(fill, dur, delay);
  box.append(fill);
  return box;
}

function card(c) {
  const box = h("article", "panel card" + (c.active ? " active" : "") + (c.locked ? " locked" : ""));
  const top = h("div", "card-top");
  const art = h("div", "art");
  art.append(icon(c.icon, "ic big"));
  const name = h("div", "nm");
  name.append(h("strong", "", c.title), h("small", "", c.sub));
  top.append(art, name);
  if (c.tag) top.append(h("span", "tag " + c.tone, c.tag));
  box.append(top, chips(c.chips));
  if (c.bar) box.append(bar(c.bar));
  if (c.checks && c.checks.length) {
    const checks = h("div", "checks");
    c.checks.forEach((k) => checks.append(h("div", k.on ? "on" : "", k.text)));
    box.append(checks);
  }
  if (c.active) box.append(progress(c.dur, c.delay));
  const row = h("div", "card-buttons");
  (c.buttons || []).forEach((b, i) => row.append(btn(b, i ? "small" : "go")));
  box.append(row);
  return box;
}

function rows(list) {
  const box = h("div", "rows");
  list.forEach((r) => {
    const row = h("div", "rowi " + (r.tone || ""));
    if (r.icon) row.append(icon(r.icon, "ic row-ic"));
    const text = h("div", "grow");
    text.append(h("b", "", r.title));
    if (r.sub) text.append(h("small", "", r.sub));
    if (r.bar) text.append(bar(r.bar));
    // What a rank asks for, each marked done or not (T40); the core says which.
    if (r.marks && r.marks.length) {
      const ul = h("ul", "marks");
      r.marks.forEach((m) => {
        const li = h("li", m.done ? "done" : "not");
        li.append(h("span", "say", m.say + ": "), document.createTextNode(m.text));
        ul.append(li);
      });
      text.append(ul);
    }
    row.append(text);
    if (r.chip) row.append(h("span", "chip", r.chip));
    if (r.buttons && r.buttons.length) {
      const bs = h("div", "row-buttons");
      (r.buttons || []).forEach((b) => bs.append(btn(b, "small")));
      row.append(bs);
    }
    box.append(row);
  });
  return box;
}

function scene(b) {
  const box = h("div", "scene" + (b.running ? " running" : "") + " pose-" + (b.pose || "chop") + " place-" + (b.place || "vale"));
  const svg = el("scene-tpl").content.firstElementChild.cloneNode(true);
  svg.querySelector(".target").setAttribute("href", "icons.svg#" + b.icon);
  timing(box, b.dur, b.delay);
  box.append(svg, h("span", "cap", b.text));
  return box;
}

// Modules of the rocket ship's tree the person has folded (T86): how the
// page looks, not a rule, so it lives here.
const folded = new Set();

const blocks = {
  head: (b) => {
    const box = h("div", "head");
    const ico = h("div", "ico");
    ico.append(icon(b.icon));
    box.append(ico, h("h1", "", b.title));
    if (b.tag) box.append(h("span", "tag " + b.tone, b.tag));
    return box;
  },
  bar: (b) => bar(b.bar),
  links: (b) => {
    const box = h("div", "flow");
    box.append(h("span", "", b.title));
    (b.buttons || []).forEach((x) => box.append(btn(x, "link")));
    if (b.buttons2 && b.buttons2.length) {
      box.append(h("span", "", "→ " + b.label2));
      b.buttons2.forEach((x) => box.append(btn(x, "link")));
    }
    return box;
  },
  scene,
  // The plan as blocks to drag, Scratch-style (T24). The core decides what
  // a move means; the page only says which block went where.
  program: (b) => {
    const box = h("div", "program");
    const palette = b.buttons || [];
    const rows = b.rows || [];
    if (palette.length) {
      const pal = h("div", "palette");
      if (b.title) pal.append(h("h3", "", b.title));
      palette.forEach((x) => {
        const blk = h("div", "blk new " + x.tone, x.label);
        blk.append(h("span", "grip", "⠿"));
        draggable(blk, blk, x.act, x.value);
        pal.append(blk);
      });
      box.append(pal);
    }
    const stack = h("div", "stack");
    rows.forEach((r, k) => {
      const blk = h("div", "blk " + r.tone + (r.chip ? " now" : ""));
      blk.dataset.k = k;
      const grip = h("span", "grip", "⠿");
      grip.title = b.label2;
      const text = h("div", "grow");
      text.append(h("b", "", r.title));
      if (r.sub) text.append(h("small", "", r.sub));
      // The running step's progress (T66); the core gives the width.
      if (r.bar) text.append(bar(r.bar));
      blk.append(grip);
      if (r.icon) blk.append(icon(r.icon, "ic row-ic"));
      blk.append(text);
      if (r.chip) blk.append(h("span", "chip", r.chip));
      const bs = h("div", "row-buttons");
      (r.buttons || []).forEach((x) => bs.append(btn(x, "small")));
      blk.append(bs);
      draggable(grip, blk, b.act, String(k));
      stack.append(blk);
    });
    const end = h("div", "blk end", b.text);
    end.dataset.k = rows.length;
    stack.append(end);
    box.append(stack);
    return box;
  },
  // The lists' region filter and order (T25): two rows of small toggles.
  filters: (b) => {
    const box = h("div", "filters");
    [[b.title, b.buttons || []], [b.label2, b.buttons2 || []]].forEach(([label, list]) => {
      if (!list.length) return;
      const row = h("div", "filter-row");
      row.append(h("span", "filter-label", label));
      list.forEach((x) => row.append(btn(x, "chipbtn")));
      box.append(row);
    });
    return box;
  },
  // The Cult's regime (T37): links drawn as given, each node a button at the
  // place the core gives it; a tap goes back as the block's act.
  graph: (b) => {
    const box = h("div", "panel cultgraph");
    if (b.title) box.append(h("h3", "", b.title));
    const plot = h("div", "graph-plot");
    const svg = document.createElementNS(SVG, "svg");
    svg.setAttribute("viewBox", "0 0 100 100");
    svg.setAttribute("aria-hidden", "true");
    (b.graph.links || []).forEach((d) => {
      const line = document.createElementNS(SVG, "path");
      line.setAttribute("d", d);
      svg.append(line);
    });
    plot.append(svg);
    (b.graph.nodes || []).forEach((n, k) => {
      const node = h("button", "gnode " + n.tone);
      node.style.setProperty("--x", n.x);
      node.style.setProperty("--y", n.y);
      node.append(icon(n.icon), h("span", "", n.label));
      node.addEventListener("click", () => draw(game.act(b.graph.act, String(k), now())));
      plot.append(node);
    });
    box.append(plot);
    if (b.text) box.append(h("p", "note", b.text));
    (b.buttons || []).forEach((x) => box.append(btn(x)));
    return box;
  },
  // The Cult's reader (T37): a figure at a lectern, and the teachings named
  // one after another in the order the core gives.
  reader: (b) => {
    const box = h("div", "scene reader pose-cast place-vale" + (b.running ? " running" : ""));
    const svg = el("scene-tpl").content.firstElementChild.cloneNode(true);
    svg.querySelector(".target").setAttribute("href", "icons.svg#cult");
    const lines = h("div", "reads");
    const strip = h("div", "strip");
    const list = b.list || [];
    strip.style.setProperty("--n", list.length || 1);
    timing(strip, b.dur, "");
    list.forEach((t) => strip.append(h("span", "", t)));
    lines.append(strip);
    box.append(svg, lines, h("span", "cap", b.text));
    return box;
  },
  // A graph the core shapes (T23): its path, a marker and four labels.
  curve: (b) => {
    const box = h("div", "panel curve");
    if (b.title) box.append(h("h3", "", b.title));
    const plot = h("div", "plot");
    const svg = document.createElementNS(SVG, "svg");
    svg.setAttribute("viewBox", b.curve.view);
    svg.setAttribute("preserveAspectRatio", "none");
    svg.setAttribute("aria-hidden", "true");
    const line = document.createElementNS(SVG, "path");
    line.setAttribute("d", b.curve.line);
    line.setAttribute("vector-effect", "non-scaling-stroke");
    svg.append(line);
    const mark = h("i", "mark");
    mark.style.setProperty("--x", b.curve.x);
    mark.style.setProperty("--y", b.curve.y);
    plot.append(svg, mark, h("span", "y-top", b.curve.top), h("span", "y-low", b.curve.low));
    const axis = h("div", "axis");
    axis.append(h("span", "", b.curve.left), h("span", "", b.curve.right));
    box.append(plot, axis);
    if (b.text) box.append(h("p", "note", b.text));
    return box;
  },
  progress: (b) => progress(b.dur, b.delay, "now"),
  chips: (b) => chips(b.chips),
  forecast: (b) => {
    const box = h("section", "panel forecast");
    box.append(h("h2", "sec-h", b.title), chips(b.chips));
    return box;
  },
  cards: (b) => {
    const box = h("div", "cards");
    (b.cards || []).forEach((c) => box.append(card(c)));
    if (!b.title) return box;
    const group = h("section", "group");
    group.append(h("h2", "sec-h", b.title), box);
    return group;
  },
  note: (b) => h("p", "note", b.text),
  styles: (b) => {
    const box = h("div", "styles");
    (b.buttons || []).forEach((x) => box.append(btn(x, "style")));
    return box;
  },
  fight: (b) => {
    const box = h("div", "fight");
    const you = h("div", "fighter");
    you.append(h("b", "", b.title), bar(b.bar));
    const foe = h("div", "fighter");
    foe.append(h("b", "", b.label2), bar(b.bar2));
    box.append(you, h("span", "vs", "vs"), foe);
    return box;
  },
  bank: (b) => {
    const group = h("section", "group");
    if (b.title) group.append(h("h2", "sec-h", b.title));
    const box = h("div", "panel bank");
    group.append(box);
    (b.buttons || []).forEach((x) => {
      const slot = h("button", "slot " + (x.tone || ""));
      slot.append(h("b", "", x.sub), icon(x.icon, "ic slot-ic"), h("small", "", x.label));
      slot.setAttribute("aria-label", x.label + ", " + x.sub);
      slot.dataset.act = x.act;
      slot.dataset.value = x.value;
      slot.addEventListener("click", () => press(x));
      box.append(slot);
    });
    return group;
  },
  // The bank's search box (IG-Q41 A): each keystroke goes to the core, and
  // the redraw keeps the box focused so the keyboard stays up.
  search: (b) => {
    const box = h("label", "search");
    box.append(h("span", "sec-h", b.title));
    const field = h("input");
    field.type = "search";
    field.value = b.value || "";
    field.placeholder = b.text;
    field.dataset.act = b.act;
    field.addEventListener("input", () => {
      draw(game.act(b.act, field.value, now()));
      const again = document.querySelector('input[data-act="' + b.act + '"]');
      if (again) {
        again.focus();
        again.setSelectionRange(again.value.length, again.value.length);
      }
    });
    box.append(field);
    return box;
  },
  // A number typed into the plan's step sheet (T61): sent as typed, with the
  // block's tag in front, when Enter is pressed or the box is left. The core
  // reads the number and refuses what it cannot take.
  number: (b) => {
    const box = h("label", "typed");
    box.append(h("span", "sec-h", b.title));
    const field = h("input");
    field.type = "text";
    field.inputMode = "numeric";
    field.value = b.value || "";
    field.dataset.act = b.act;
    field.addEventListener("change", () => draw(game.act(b.act, b.tag + field.value, now())));
    box.append(field);
    return box;
  },
  rows: (b) => {
    const box = h("section", "group");
    if (b.title) box.append(h("h2", "sec-h", b.title));
    const list = rows(b.rows);
    list.classList.add("panel");
    box.append(list);
    return box;
  },
  // A long tab's sub-tabs (T92): the core says which is chosen.
  subtabs: (b) => {
    const box = h("nav", "subtabs");
    (b.buttons || []).forEach((x) => box.append(btn(x, "sub")));
    return box;
  },
  // A module of the rocket ship's tree (T86): it folds, and each part's
  // row says, from the core, how far it has come.
  tree: (b) => {
    const box = h("details", "group tree " + (b.tone || ""));
    box.open = !folded.has(b.value);
    box.addEventListener("toggle", () => (box.open ? folded.delete(b.value) : folded.add(b.value)));
    const sum = h("summary", "sec-h");
    sum.append(h("span", "dot"), document.createTextNode(b.title), h("span", "tag", b.tag));
    const list = rows(b.rows);
    list.classList.add("panel");
    box.append(sum, list);
    return box;
  },
  buttons: (b) => {
    const box = h("div", "button-row");
    if (b.text) box.append(h("p", "note wide", b.text));
    (b.buttons || []).forEach((x) => box.append(btn(x)));
    return box;
  },
  keep: (b) => {
    const box = h("div", "panel keep");
    [{ head: b.title, items: b.list, cls: "k" }, { head: b.label2, items: b.list2, cls: "l" }].forEach((part) => {
      const col = h("div", part.cls);
      const list = h("ul");
      (part.items || []).forEach((t) => list.append(h("li", "", t)));
      col.append(h("h3", "", part.head), list);
      box.append(col);
    });
    return box;
  },
  card: (b) => {
    const box = h("section", "panel info");
    box.append(h("h2", "", b.title), h("p", "", b.text));
    return box;
  },
  choice: (b) => {
    const box = h("div", "choice");
    box.append(h("h3", "", b.title));
    const row = h("div", "button-row");
    (b.buttons || []).forEach((x) => row.append(btn(x, "pick")));
    box.append(row);
    return box;
  },
  input: (b) => {
    const box = h("label", "input");
    box.append(h("h3", "", b.title));
    const area = h("textarea");
    area.rows = "3";
    area.value = b.value || "";
    area.placeholder = b.text;
    // The note goes to the core as it is typed; the screen is not redrawn, so
    // the keyboard stays up.
    area.addEventListener("input", () => game.act(b.act, area.value, now()));
    box.append(area);
    return box;
  },
};

// The screen redraws only the blocks that changed (R22.2): a long screen
// (the bank shows every stack) is not rebuilt each time an action
// finishes, only the block whose words or numbers moved.
let shown = [];
function drawScreen(main, list, same) {
  const next = list.map((b) => {
    const key = JSON.stringify(b);
    const old = same && shown.find((x) => x.key === key && !x.used);
    if (old) {
      old.used = true;
      return old;
    }
    const make = blocks[b.kind];
    return { key, node: make ? make(b) : null, used: true };
  });
  next.forEach((x) => {
    x.used = false;
  });
  const nodes = next.filter((x) => x.node).map((x) => x.node);
  const now = [...main.children];
  if (nodes.length !== now.length || nodes.some((n, k) => n !== now[k])) main.replaceChildren(...nodes);
  shown = next;
}

function drawBlocks(list, parent) {
  list.forEach((b) => {
    const make = blocks[b.kind];
    if (make) parent.append(make(b));
  });
}

// ---------- the frame: top bar, tabs, screen, sheet, toasts ----------

function drawTop(top) {
  const box = el("top");
  const gold = h("div", "pill");
  gold.append(icon("gold", "ic coin"), h("span", "", top.gold));
  box.replaceChildren(gold, h("div", "pill", top.total), tipPanel());
  if (top.mult) box.append(h("div", "pill pres", top.mult));
  // Strengths running out over time (T58): hover, focus or tap for what each does.
  if (top.buffs && top.buffs.length) {
    const row = h("div", "buffs");
    top.buffs.forEach((b) => {
      const c = h("button", "chip buff " + (b.tone || ""));
      c.append(h("i", "dot"), h("span", "", b.text));
      c.setAttribute("aria-label", b.text + ". " + b.tip);
      const show = () => buffTip(c, b.tip);
      c.addEventListener("mouseenter", show);
      c.addEventListener("focus", show);
      c.addEventListener("click", show);
      c.addEventListener("mouseleave", () => buffTip(null));
      c.addEventListener("blur", () => buffTip(null));
      row.append(c);
    });
    box.append(row);
  }
  box.append(h("span", "sp"));
  // The plan, one tap away (IG-Q91 A), with the running step's bar.
  const plan = btn(top.plan, "plan-btn");
  if (top.plan_width) {
    const fill = h("i", "plan-fill");
    fill.style.width = top.plan_width;
    plan.append(fill);
  }
  box.append(plan);
  box.append(btn(top.dict, "icon-btn dict-btn"));
  box.append(btn(top.flag, "icon-btn flag-btn"));
  if (top.screen) box.append(btn(top.screen, "icon-btn"));
}

// One panel under the top bar for the strength hovered or tapped; it is
// kept across redraws, and CSS places it.
function tipPanel() {
  let tip = el("buff-tip");
  if (!tip) {
    tip = h("div", "buff-tip");
    tip.id = "buff-tip";
    tip.setAttribute("role", "tooltip");
    tip.hidden = true;
  }
  return tip;
}

function buffTip(chip, text) {
  const tip = tipPanel();
  tip.hidden = !chip;
  if (chip) tip.textContent = text;
}

// One button per tab or group. On a phone the strip shows only the tabs of
// the group on screen (the core marks them) and the bottom bar the five
// groups; on a wide screen the rail shows every tab under its group's name.
function tabButton(t, cls) {
  const node = h("button", cls + (t.strip ? " in" : "") + (t.locked ? " locked" : ""));
  node.dataset.tab = t.id;
  if (t.current) node.setAttribute("aria-current", "page");
  node.append(h("span", "lvl", t.level), icon(t.id));
  const label = h("span", "tab-label");
  label.append(h("span", "short", t.short), h("span", "full", t.label));
  node.append(label);
  // How far an expedition has come, and a red dot when something waits on
  // you (T64); the core gives the width.
  if (t.progress) {
    const track = h("i", "tbar");
    const fill = h("i");
    fill.style.width = t.progress;
    track.append(fill);
    node.append(track);
  }
  if (t.alert) node.append(h("i", "alert"));
  if (t.running) node.append(h("i", "run"));
  node.addEventListener("click", () => draw(game.act(t.act, t.value, now())));
  return node;
}

// The strip or rail of tabs. It keeps where it was scrolled across a
// redraw, and brings the current tab into view only when the screen changes
// (R21.1): an action finishing must not pull the rail back under the pointer.
let lastTabs = "";
function drawTabs(tabs, groups, moved) {
  // Unchanged tabs are left as they are, so a tab does not move or vanish
  // under a pointer while an action ticks (R21.1).
  const sig = JSON.stringify([tabs, groups]);
  if (sig === lastTabs) return;
  lastTabs = sig;
  const box = el("tabs");
  const top = box.scrollTop;
  const left = box.scrollLeft;
  const rail = [];
  tabs.forEach((t) => {
    if (t.heading) rail.push(h("span", "rail-h", t.heading));
    rail.push(tabButton(t, "tab"));
  });
  box.replaceChildren(...rail);
  el("groups").replaceChildren(...groups.map((g) => tabButton(g, "tab group")));
  box.scrollTop = top;
  box.scrollLeft = left;
  const here = box.querySelector('[aria-current="page"]');
  if (moved && here && here.scrollIntoView) here.scrollIntoView({ block: "nearest", inline: "nearest" });
}

let lastSheet = "";
function drawSheet(s) {
  const shade = el("shade");
  if (!s) {
    shade.hidden = true;
    lastSheet = "";
    return;
  }
  // An unchanged sheet is left as it is, so its rows do not rise in again.
  const same = JSON.stringify(s);
  if (same === lastSheet && !shade.hidden) return;
  lastSheet = same;
  const box = el("sheet");
  // A number being typed in the sheet (T61) keeps its digits and the
  // keyboard when the action timer redraws it (R54.3), as the search box
  // does (R19.1).
  const inputs = () => Array.from(box.querySelectorAll("input"));
  const typing = inputs().indexOf(document.activeElement);
  const typed = typing >= 0 ? document.activeElement.value : "";
  box.className = "sheet " + s.kind;
  box.replaceChildren();
  if (s.eyebrow) box.append(h("p", "eyebrow", s.eyebrow));
  if (s.title) box.append(h("h2", "", s.title));
  if (s.big) box.append(h("div", "big", s.big));
  (s.lines || []).forEach((t) => box.append(h("p", "", t)));
  if (s.gains && s.gains.length) {
    const gains = rows(s.gains);
    gains.classList.add("gains");
    Array.from(gains.children).forEach((row, i) => row.style.setProperty("--i", i));
    box.append(gains);
  }
  if (s.ready && s.ready.length) {
    box.append(h("h3", "sec-h", s.ready_title));
    const ready = rows(s.ready);
    ready.classList.add("panel", "ready");
    box.append(ready);
  }
  (s.notes || []).forEach((t) => box.append(h("p", "capnote", t)));
  drawBlocks(s.blocks || [], box);
  const row = h("div", "button-row sheet-buttons");
  (s.buttons || []).forEach((b) => row.append(btn(b)));
  box.append(row);
  shade.hidden = false;
  const again = typing >= 0 && inputs()[typing];
  if (again) {
    again.value = typed;
    again.focus();
    again.setSelectionRange(typed.length, typed.length);
  }
}

function drawToasts(list) {
  const box = el("toasts");
  list.forEach((t, i) => {
    const toast = h("div", "toast", t);
    toast.style.setProperty("--i", i);
    toast.addEventListener("animationend", () => toast.remove());
    box.append(toast);
  });
}

function hint(nodes, hints) {
  nodes.forEach((node) => {
    const found = hints.find((x) => x.term === node.textContent);
    if (found) {
      node.title = found.definition;
      node.dataset.term = found.term;
    }
  });
}

// Dragging a plan block (T24): a ghost follows the pointer; dropping on a
// block (or the end) sends the act with "value:place". A tap with no move
// drops on the end, so the palette also works by tapping.
function draggable(handle, blk, act, value) {
  handle.addEventListener("pointerdown", (e) => {
    if (e.target.closest("button")) return;
    e.preventDefault();
    const box = blk.getBoundingClientRect();
    const ghost = blk.cloneNode(true);
    ghost.classList.add("ghost");
    ghost.style.left = box.left + "px";
    ghost.style.top = box.top + "px";
    ghost.style.width = box.width + "px";
    document.body.append(ghost);
    blk.classList.add("lifted");
    const x0 = e.clientX;
    const y0 = e.clientY;
    let moved = false;
    let over = null;
    let x = x0;
    let y = y0;
    const main = el("main");
    const aim = () => {
      const hit = document.elementFromPoint(x, y);
      const target = hit && hit.closest(".stack .blk[data-k]");
      if (over && over !== target) over.classList.remove("over");
      over = target;
      if (over) over.classList.add("over");
    };
    // Near the top or bottom edge, a block's height from it, the screen
    // scrolls, faster the closer the pointer is to the edge.
    let frame = requestAnimationFrame(function edge() {
      const view = main.getBoundingClientRect();
      const low = y - (view.bottom - box.height);
      const high = view.top + box.height - y;
      if (low > 0) main.scrollTop += low;
      if (high > 0) main.scrollTop -= high;
      if (low > 0 || high > 0) aim();
      frame = requestAnimationFrame(edge);
    });
    const move = (m) => {
      moved = moved || m.clientX !== x0 || m.clientY !== y0;
      x = m.clientX;
      y = m.clientY;
      ghost.style.transform = "translate(" + (x - x0) + "px, " + (y - y0) + "px)";
      aim();
    };
    const up = () => {
      cancelAnimationFrame(frame);
      document.removeEventListener("pointermove", move);
      document.removeEventListener("pointerup", up);
      document.removeEventListener("pointercancel", up);
      ghost.remove();
      blk.classList.remove("lifted");
      const end = document.querySelector(".stack .blk.end");
      const place = moved ? over : blk.dataset.k === undefined ? end : null;
      if (place) draw(game.act(act, value + ":" + place.dataset.k, now()));
    };
    document.addEventListener("pointermove", move);
    document.addEventListener("pointerup", up);
    document.addEventListener("pointercancel", up);
  });
}

let lastScreen = "";
function draw(json) {
  const result = JSON.parse(json);
  ui = result;
  if (ui.stash) stash(saved);
  // Save the restored game before letting the kept copy go.
  if (ui.unstash) {
    store(game.save());
    unstash();
  }
  document.title = ui.title;
  drawTop(ui.top);
  drawTabs(ui.tabs, ui.groups, lastScreen !== ui.screen);
  const main = el("main");
  const keep = lastScreen === ui.screen ? main.scrollTop : 0;
  // A redraw from the action timer must not take the keyboard away from a
  // search box being typed in (R19.1): note it, and give focus back.
  const typing = document.activeElement && document.activeElement.dataset ? document.activeElement.dataset.act : "";
  drawScreen(main, ui.blocks, lastScreen === ui.screen);
  main.scrollTop = keep;
  const again = typing && main.querySelector('input[data-act="' + typing + '"]');
  if (again) {
    again.focus();
    again.setSelectionRange(again.value.length, again.value.length);
  }
  lastScreen = ui.screen;
  document.body.dataset.screen = ui.screen;
  drawSheet(ui.sheet);
  drawToasts(ui.toasts);
  play(ui.sounds || []);
  if (ui.review) phone.review();
  hint(document.querySelectorAll(".head h1, .tab .full, .sheet h2"), ui.hints);
  schedule();
  return ui;
}

// One timer, set for the moment the running action completes (IG-8): no
// frame loop. The bars move in CSS.
function schedule() {
  clearTimeout(timer);
  if (ui.next_ms && !document.hidden) timer = setTimeout(() => draw(game.tick(now())), ui.next_ms);
}

function keep() {
  if (game) store(game.save());
}

// ---------- the device's jobs ----------

function deviceInfo(facts) {
  const standalone =
    navigator.standalone === true ||
    matchMedia("(display-mode: standalone)").matches ||
    matchMedia("(display-mode: fullscreen)").matches;
  return JSON.stringify({
    fullscreen: !!(document.fullscreenEnabled || document.webkitFullscreenEnabled),
    ios: /iPhone|iPad|iPod/.test(navigator.userAgent) || (navigator.platform === "MacIntel" && navigator.maxTouchPoints > 1),
    standalone,
    ua: navigator.userAgent,
    w: screen.width,
    h: screen.height,
    app: phone.inApp,
    owned: !!facts.owned,
    price: facts.price || "",
  });
}

function fullscreen() {
  const root = document.documentElement;
  if (document.fullscreenElement || document.webkitFullscreenElement) {
    (document.exitFullscreen || document.webkitExitFullscreen).call(document);
  } else if (root.requestFullscreen) {
    root.requestFullscreen().catch(() => draw(game.act("help.home", "", now())));
  } else if (root.webkitRequestFullscreen) {
    root.webkitRequestFullscreen();
  }
}

function stamp() {
  return new Date().toISOString().split("T")[0];
}

function download(name, text) {
  const link = document.createElement("a");
  link.href = URL.createObjectURL(new Blob([text], { type: "application/json" }));
  link.download = name;
  document.body.append(link);
  link.click();
  link.remove();
  setTimeout(() => URL.revokeObjectURL(link.href), ui.autosave_ms);
}

function exportSave() {
  download("idle-game-save-" + stamp() + ".json", game.export(now()));
  keep();
  draw(game.tick(now()));
}

// On a phone the share sheet sends the file by Messages or mail; elsewhere
// it downloads.
async function sendPlaytest() {
  const name = "idle-game-playtest-" + stamp() + ".json";
  const text = game.playtest(now());
  keep();
  const file = new File([text], name, { type: "application/json" });
  try {
    if (navigator.canShare && navigator.canShare({ files: [file] })) {
      await navigator.share({ files: [file], title: name });
    } else {
      download(name, text);
    }
  } catch (e) {
    if (e.name !== "AbortError") download(name, text);
  }
  draw(game.act("playtest.sent", "", now()));
}

el("importer").addEventListener("change", async (e) => {
  const file = e.target.files[0];
  e.target.value = "";
  if (!file) return;
  draw(game.import(await file.text(), now()));
  keep();
});

el("shade").addEventListener("click", (e) => {
  if (e.target.id === "shade" && ui.sheet && ui.sheet.kind !== "flag") draw(game.act("sheet.close", "", now()));
});

document.addEventListener("visibilitychange", () => {
  if (document.hidden) {
    clearTimeout(timer);
    const away = JSON.parse(game.act("seen.hide", "", now()));
    keep();
    phone.alerts(away.alerts);
    phone.report(away.game_center);
    phone.cloudWrite(game.save());
  } else {
    phone.clearAlerts();
    draw(game.act("seen.show", "", now()));
  }
});
addEventListener("pagehide", keep);

async function main() {
  await init();
  await loadSounds();
  document.addEventListener("pointerdown", unlock);
  const strings = await (await fetch("data/strings.json?v=bd6b3673965f")).text();
  const glossary = await (await fetch("data/glossary.md?v=bd6b3673965f")).text();
  saved = await load();
  const facts = await phone.storeFacts();
  try {
    game = new Game(strings, glossary, saved, deviceInfo(facts), now());
  } catch (e) {
    el("message").textContent = String(e);
    return;
  }
  draw(game.tick(now()));
  keep();
  persist();
  const old = kept();
  if (old) draw(game.act("stash.found", old, now()));
  phone.gameCenter();
  const cloud = await phone.cloudRead();
  if (cloud) draw(game.act("cloud.found", cloud, now()));
  setInterval(keep, ui.autosave_ms);
  document.body.dataset.ready = "true";
}

main();
