// The one file that names browser storage (change to D17, IG-6). It stores
// the core's save text and reads it back; it never reads inside the text.
// In the app (T17) the save is a file the phone keeps, not browser storage,
// which a phone may clear.
const KEY = "idle-game-save";
const UNREADABLE = "idle-game-unreadable-save";
const tauri = window.__TAURI__;

export async function load() {
  if (tauri) return tauri.core.invoke("save_read").catch(() => "");
  try {
    return localStorage.getItem(KEY) || "";
  } catch (e) {
    return "";
  }
}

export function store(text) {
  if (tauri) {
    tauri.core.invoke("save_write", { text }).catch(() => {});
    return;
  }
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
