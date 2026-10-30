// The phone's jobs in the app (T17): the purchase, notifications, the rating
// prompt, Game Center and iCloud, through Tauri. On the web there is no
// Tauri, and every function here does nothing. Whether to buy, when to ask
// for a rating and which notifications to schedule are the core's decisions;
// this file passes them on and passes Apple's answers back.
const tauri = window.__TAURI__;
export const inApp = !!tauri;

// The one purchase, as named in App Store Connect (see docs/APP-STORE.md).
const PRODUCT = "io.github.sgilson7.idlewright.regions";

function call(method, args) {
  return tauri.core.invoke("plugin:apple|call", { method, args: args || {} });
}

// What the store says before the game starts: owned, and the local price.
export async function storeFacts() {
  if (!inApp) return {};
  try {
    const owned = await call("owned", { id: PRODUCT });
    const price = await call("price", { id: PRODUCT });
    return { owned: owned.owned, price: price.price };
  } catch (e) {
    return {};
  }
}

// Buy or restore; the answer becomes the core's act: store.owned,
// store.cancelled, store.pending, store.failed or store.none.
export async function buy() {
  try {
    return "store." + (await call("buy", { id: PRODUCT })).result;
  } catch (e) {
    return "store.failed";
  }
}

export async function restore() {
  try {
    return "store." + (await call("restore", { id: PRODUCT })).result;
  } catch (e) {
    return "store.failed";
  }
}

export function review() {
  if (inApp) call("review").catch(() => {});
}

export function alerts(list) {
  if (inApp) tauri.core.invoke("alerts_set", { alerts: list }).catch(() => {});
}

export function clearAlerts() {
  if (inApp) tauri.core.invoke("alerts_clear").catch(() => {});
}

export function gameCenter() {
  if (inApp) call("gameCenter").catch(() => {});
}

export function report(gc) {
  if (!inApp || !gc) return;
  call("score", { board: gc.board, value: gc.score }).catch(() => {});
  call("achieve", { ids: gc.achieved }).catch(() => {});
}

export async function cloudRead() {
  if (!inApp) return "";
  try {
    return (await call("cloudGet", { key: "save" })).value;
  } catch (e) {
    return "";
  }
}

export function cloudWrite(text) {
  if (inApp) call("cloudSet", { key: "save", value: text }).catch(() => {});
}
