#!/usr/bin/env python3
"""Draw the app icon (T18, IG-Q51 A) as SVG and render it to PNG.

The icon is a tree in a gold ring, with an axe and a pickaxe crossed behind
it, on the game's night sky: the first thing a new player does, at a size
that still reads in a store's search results. It is square and opaque, as
the App Store asks; iOS rounds the corners itself.

    .venv/bin/python tools/icons/app_icon.py

Writes data/icons/app-icon.svg, and from it app-icon-1024.png (the store),
icon-512.png and icon-180.png (the web page and the home screen). Rendering
uses the browser check's Chromium.
"""
import pathlib

ROOT = pathlib.Path(__file__).resolve().parents[2]
OUT = ROOT / "data" / "icons"

SVG = """<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" width="1024" height="1024">
  <defs>
    <linearGradient id="sky" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0" stop-color="#2c3d6a"/><stop offset="1" stop-color="#10141c"/>
    </linearGradient>
    <radialGradient id="disc" cx=".5" cy=".42" r=".6">
      <stop offset="0" stop-color="#2f4a7a"/><stop offset="1" stop-color="#17213a"/>
    </radialGradient>
  </defs>
  <rect width="1024" height="1024" fill="url(#sky)"/>
  <circle cx="190" cy="230" r="6" fill="#f5e6a8"/><circle cx="300" cy="140" r="4" fill="#f5e6a8"/>
  <circle cx="700" cy="330" r="5" fill="#f5e6a8"/><circle cx="880" cy="420" r="4" fill="#f5e6a8"/>
  <!-- the axe and the pickaxe, crossed behind the ring -->
  <path d="M250 830L770 250" stroke="#a07040" stroke-width="54" stroke-linecap="round"/>
  <path d="M700 230c120-20 190 60 170 170l-150-40-60-60z" fill="#cfd6e6" stroke="#22262e" stroke-width="16"/>
  <path d="M774 830L254 250" stroke="#a07040" stroke-width="54" stroke-linecap="round"/>
  <path d="M150 330c90-130 260-150 330-60-100-30-210-10-330 60z" fill="#cfd6e6" stroke="#22262e" stroke-width="16"/>
  <!-- the ring and the tree -->
  <circle cx="512" cy="560" r="290" fill="url(#disc)" stroke="#e8b42a" stroke-width="34"/>
  <path d="M290 720Q400 650 512 700T734 700A290 290 0 0 1 290 720z" fill="#2a4a2e"/>
  <rect x="482" y="560" width="60" height="190" rx="10" fill="#6b4a2a"/>
  <circle cx="512" cy="480" r="130" fill="#4f8a3c"/>
  <circle cx="420" cy="540" r="90" fill="#4f8a3c"/>
  <circle cx="604" cy="540" r="90" fill="#4f8a3c"/>
  <circle cx="470" cy="440" r="40" fill="#6ac84a" opacity=".6"/>
</svg>
"""


def main():
    from playwright.sync_api import sync_playwright

    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "app-icon.svg").write_text(SVG)
    with sync_playwright() as p:
        browser = p.chromium.launch()
        for name, size in (("app-icon-1024.png", 1024), ("icon-512.png", 512), ("icon-180.png", 180)):
            page = browser.new_page(viewport={"width": size, "height": size})
            sized = SVG.replace('width="1024" height="1024"', f'width="{size}" height="{size}" style="display:block"', 1)
            page.set_content(f'<html><body style="margin:0">{sized}</body></html>')
            page.wait_for_timeout(200)
            page.screenshot(path=str(OUT / name), omit_background=False)
            page.close()
        browser.close()
    print("wrote data/icons/app-icon.svg, app-icon-1024.png, icon-512.png, icon-180.png")


if __name__ == "__main__":
    main()
