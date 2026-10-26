#!/usr/bin/env bash
# The app and the web run one engine (IG-Q52 A): compare the WebAssembly
# module and the page in dist/ (what the app bundles) with what GitHub Pages
# serves (the gh-pages branch). Run after `cargo tauri ios build`.
set -euo pipefail
cd "$(dirname "$0")/.."
git fetch -q origin gh-pages
sha() { shasum -a 256 | cut -c1-16; }
status=0
for f in pkg/tool_shim_bg.wasm pkg/tool_shim.js app.js index.html; do
  here=$(sha < "dist/$f")
  live=$(git show "origin/gh-pages:$f" | sha)
  if [ "$here" = "$live" ]; then echo "same      $f $here"; else echo "DIFFERENT $f app $here, web $live"; status=1; fi
done
[ $status -eq 0 ] && echo "The app carries the web's engine, byte for byte." || echo "Publish the web from this commit, or rebuild the app from the published one." >&2
exit $status
