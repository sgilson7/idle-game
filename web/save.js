// The one file that names browser storage (change to D17, IG-6). It stores
// the core's save text and reads it back; it never reads inside the text.
const KEY = "idle-game-save";
const UNREADABLE = "idle-game-unreadable-save";

export function load() {
  try {
    return localStorage.getItem(KEY) || "";
  } catch (e) {
    return "";
  }
}

export function store(text) {
  try {
    localStorage.setItem(KEY, text);
  } catch (e) {
    // Full or blocked storage: the export file is the way to keep the game.
  }
}

// A save the core could not read is kept under its own name, not overwritten.
export function stash(text) {
  try {
    if (text) localStorage.setItem(UNREADABLE, text);
  } catch (e) {
    // nothing to do
  }
}

// Ask the browser to keep this site's storage; Safari and Chrome decide.
export function persist() {
  try {
    if (navigator.storage && navigator.storage.persist) navigator.storage.persist();
  } catch (e) {
    // nothing to do
  }
}
