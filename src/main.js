import "./styles.css";
import { initHeroReveal } from "./scripts/hero.js";
import { initScrollEffects } from "./scripts/scroll.js";
import { initFrameAnimation } from "./scripts/frames.js";
import { initCounters } from "./scripts/counter.js";
import { initModal } from "./scripts/modal.js";
import { initCursor } from "./scripts/cursor.js";

window.addEventListener("DOMContentLoaded", () => {
  initScrollEffects();
  initHeroReveal();
  initFrameAnimation();
  initCounters();
  initModal();
  initCursor();
});
