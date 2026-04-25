# UI test plan — Caffè Milano 1887 landing page

## What changed
- The previous Spider-Opti React/Tauri frontend was replaced with a single-page vanilla HTML/CSS/JS marketing site for a heritage Italian espresso machine.
- The page now includes a cinematic hero, a scroll-driven sticky canvas using 150 preloaded frames, scroll-triggered feature reveals, animated statistics, and a waitlist modal that opens both on CTA click and near the end of the page.

## Environment / access
- Local preview: `http://127.0.0.1:4173`
- Auth required: none
- Assets required: `public/Luxury_espresso.mp4` and `public/frames/frame_0001.png` → `frame_0150.png` are present locally.

## Code evidence
- Hero/video/title entry: `index.html:40-74`, `src/scripts/hero.js:1-23`
- Sticky canvas + caption reveal: `index.html:98-122`, `src/scripts/frames.js:12-101`
- Nav blur/parallax/reveal behavior: `src/scripts/scroll.js:1-35`, `src/styles.css:130-203`
- Counter animation: `index.html` stat cards with `data-counter`, `src/scripts/counter.js:5-53`
- Modal click + 95% scroll trigger + success state: `index.html` CTA/modal markup, `src/scripts/modal.js:1-74`
- Responsive breakpoints: `src/styles.css` media queries at `@media (max-width: 980px)` and `@media (max-width: 720px)`

## Primary flow
Open the page, verify the hero presentation, scroll through the pinned canvas/features/stats, then trigger and submit the waitlist modal.

## Test cases

### 1) Hero load and top navigation state
1. Open `/` at desktop width.
   - Pass if the hero shows the eyebrow `Artigianale · Dal 1887`, the heading words `L'Arte`, `del Caffè`, `Perfetto`, and the CTA text `Scopri la Macchina ↓`.
   - Pass if the hero background video is visible and playing behind the copy.
2. Wait for the entrance sequence to finish without scrolling.
   - Pass if the title words reveal in sequence rather than appearing fully visible immediately.
   - Pass if the subtitle becomes visible after the title sequence, not before it.
3. Scroll slightly past the top.
   - Pass if the fixed nav changes from transparent to a parchment-tinted blurred bar with a thin gold bottom rule.

### 2) Scroll-driven product narrative
1. Continue scrolling into the sticky canvas section.
   - Pass if the canvas remains pinned while the page scrolls around it.
   - Pass if the image inside the canvas visibly changes across scroll positions (early vs mid vs late section), proving the frame sequence is advancing instead of staying static.
2. Reach at least 80% of the canvas section.
   - Pass if the caption `Every surface shaped by hand. Every curve deliberate.` becomes visible only near the end of that section.
3. Continue through the five feature sections.
   - Pass if each section reveals text/image with motion rather than all appearing in the final state from the start.
4. Reach the stats section.
   - Pass if the four values animate to exactly `9 Bar`, `±1°C`, `30 Secondi`, and `58mm`.

### 3) Waitlist modal trigger and success state
1. In the CTA section, click `Richiedi l'Invito`.
   - Pass if a centered parchment modal opens with heading `Unisciti alla Lista` and an email field placeholder `nome@atelier.it`.
2. Close the modal with `×` or the overlay, then scroll to the end of the page.
   - Pass if the modal auto-opens again once the page is near the bottom (about 95% scroll), proving the scroll trigger works independently of the button.
3. Submit the form with a valid email.
   - Pass if the form disappears and the success text `Richiesta ricevuta. Il prossimo invito partirà direttamente da Milano.` becomes visible.

### Responsive checkpoints
- Repeat the visual smoke check at tablet width and mobile width.
- Pass if content remains readable, the nav wraps without overlap, the canvas and media fit the viewport, and sections collapse to a single-column layout rather than clipping horizontally.

### Performance checkpoint
- Run Lighthouse against the local preview in desktop mode.
- Pass if Performance is `>= 90` and there is no CLS regression reported by Lighthouse.
