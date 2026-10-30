// Plays the sounds the core names (T18, IG-Q50 B). Which sounds play, and
// every number in them, come from the core and data/sounds.json; this file
// only hands the notes to the browser's synthesiser and the phone's buzz.
let ctx = null;
let bank = { sounds: {}, buzz: {} };

export async function loadSounds() {
  try {
    bank = await (await fetch("data/sounds.json?v=bd6b3673965f")).json();
  } catch (e) {
    // no sounds without the file
  }
}

// Browsers start audio only after the player touches the page.
export function unlock() {
  if (ctx) return;
  const Ctx = window.AudioContext || window.webkitAudioContext;
  if (Ctx) ctx = new Ctx();
}

export function play(names) {
  names.forEach((name) => {
    const buzz = bank.buzz[name];
    if (buzz && navigator.vibrate) navigator.vibrate(buzz);
    const notes = bank.sounds[name];
    if (!ctx || !notes) return;
    const start = ctx.currentTime;
    notes.forEach((x) => {
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      osc.type = x.wave;
      osc.frequency.value = x.f;
      gain.gain.setValueAtTime(x.vol, start + x.at);
      gain.gain.linearRampToValueAtTime(0, start + x.at + x.len);
      osc.connect(gain).connect(ctx.destination);
      osc.start(start + x.at);
      osc.stop(start + x.at + x.len);
    });
  });
}
