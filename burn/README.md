# BURN — Energy Drink

A premium, single-page parallax product website for **BURN Energy Drink** — four flavor variants, scroll-driven WebP frame animation, dark/light mode, custom cursor, zero dependencies.

> _This is not a website. This is an experience._

**Live demo (GitHub Pages):** [https://daecrepka-art.github.io/burn-energy/](https://daecrepka-art.github.io/burn-energy/)

---

## Architecture

One file. Vanilla HTML + CSS + JavaScript. Zero frameworks. Zero npm dependencies. Zero build step.

| Layer | Role |
|---|---|
| `<canvas>` | Renders the active product frame; redrawn only when `currentFrame` changes |
| `#vignette` | Dark radial gradient that keeps text readable without ever covering the product |
| `#scroll-spacer` | 600vh tall spacer that drives `window.scrollY` — its only job is to give the user something to scroll |
| Top nav / left block / right strip / bottom cluster | Fixed UI overlay layers |
| Custom cursor | 12 px ring, 6-frame lerp trail, expands to 40 px glow on hover |

The visible viewport stays at `100vw × 100vh` forever. The scroll position is captured in JS, mapped to a frame index, and the canvas redraws — nothing in the layout actually moves.

## Frame Sequences

Each variant has its own folder of 192 sequential WebP frames:

| Variant | Folder | Theme color |
|---|---|---|
| 01 — ORIGINAL | `frames-original/` | `#CC2200` (deep red) |
| 02 — DARK ENERGY | `frames-dark-energy/` | `#E8A000` (amber gold) |
| 03 — TROPICAL MIX | `frames-tropical-mix/` | `#7B2FBE` (deep violet) |
| 04 — FRUIT PUNCH | `frames-fruit-punch/` | `#1A6FCC` (electric blue) |

Frames are named `frame_001.webp` through `frame_192.webp`. Total payload across all 4 variants is ~35 MB; only the active variant is blocking, the other three stream in the background after the user is interactive.

## Scroll → Frame Mapping

```js
targetFrame    = Math.round(scrollProgress * (FRAME_COUNT - 1));
displayedFrame += (targetFrame - displayedFrame) * SMOOTHING;   // SMOOTHING = 0.18
```

- The render loop is **idle-suspended**: `requestAnimationFrame` is only scheduled when there's actually a delta to process. When the user stops scrolling, the rAF chain bails out cleanly — no background ticking.
- Scroll listener is `passive: true`.
- Resize is rAF-throttled.

## Image Loading Strategy

- Active variant (variant 0) loads with a **6-worker concurrency pool** behind a blocking loader. Roughly 6× faster than a serial `await` chain over HTTP/2.
- Inactive variants stream in the background after a 1.2 s grace period, throttled by `setTimeout(_, 8)` between frames to avoid starving the scroll handler.
- Concurrent requests for the same `(variant, frame)` are deduped via a per-variant in-flight `Map`.
- Previously decoded `Image()` objects live in a 4×192 cache and are never re-fetched.
- On mobile, only variant 0 is preloaded — bandwidth saver.

## Variant Switch Sequence

Click `PREV` / `NEXT`, press an arrow key, or click a dot, and the following choreography runs in parallel:

1. CSS `@property` animates `--color-accent`, `--color-accent-glow`, `--color-accent-soft` over **600 ms** with `cubic-bezier(0.22, 1, 0.36, 1)`. Every accent surface — CTA fill, separator lines, dots, glow, strip-divider gradient — shifts together.
2. Flavor name and ghost number slide up & out (300 ms), new copy slides in from below (400 ms).
3. Description fades out (200 ms) → in (300 ms).
4. Canvas cross-fades to frame 1 of the new variant (220 ms).
5. `window.scrollTo({ top: 0 })` resets the scroll spacer so the new can starts clean.
6. Switch is debounced to 700 ms.

## Dark / Light Mode

`document.documentElement.dataset.theme = "light" | "dark"`. Persisted to `localStorage` under `burn-theme`. Toggling re-tints `--color-void`, `--color-text-primary`, `--color-text-secondary`, `--color-overlay-edge` over 500 ms. The canvas redraws once with the new void color (used for the letterbox margins of the cover-fit). Accent colors are mode-agnostic — they belong to the variant, not the theme.

## Mobile Graceful Degradation

Below 640 px the site collapses to a static hero (variant 0, frame 1) with the left content block anchored at the bottom. The scroll spacer, right nav, scroll progress indicator, and all background variant preloading are disabled. Custom cursor hidden via `(hover: none) or (pointer: coarse)`. Layout is verified to never break.

## Performance Targets

| Metric | Target | Result |
|---|---|---|
| Console errors | 0 | 0 |
| Console warnings | 0 | 0 |
| Scroll fps | 60 | 60 (idle: 0 rAF) |
| Time to first interactive frame | < 3 s on broadband | varies by network |
| Time to all 192 active frames decoded | < 12 s | ~6 s observed locally |

## Browser Support

Modern Chromium, Firefox 128+, Safari 16.4+ (for `@property`). Graceful fallback: if `@property` is unsupported, accent colors will still update — they just won't crossfade smoothly.

## Local Development

```sh
cd burn-energy
python3 -m http.server 8765
# open http://localhost:8765/
```

No build step. Edit `index.html` and refresh.

## Layout

```
burn-energy/
├── index.html              # ~36 KB — all of HTML / CSS / JS
├── README.md
├── frames-original/        # 192 WebP, ~10 MB
├── frames-dark-energy/     # 192 WebP, ~9 MB
├── frames-tropical-mix/    # 192 WebP, ~9 MB
└── frames-fruit-punch/     # 192 WebP, ~7 MB
```

## License

MIT.
