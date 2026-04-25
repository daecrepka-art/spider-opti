function easeOutExpo(value) {
  return value === 1 ? 1 : 1 - 2 ** (-10 * value);
}

export function initCounters() {
  const counterCards = Array.from(document.querySelectorAll("[data-counter]"));

  const animateCounter = (card) => {
    if (card.dataset.animated === "true") {
      return;
    }

    card.dataset.animated = "true";
    const target = Number(card.dataset.target ?? "0");
    const prefix = card.dataset.prefix ?? "";
    const suffix = card.dataset.suffix ?? "";
    const valueElement = card.querySelector(".stat-card__value");

    if (!valueElement) {
      return;
    }

    const duration = 1600;
    const start = performance.now();

    const step = (now) => {
      const elapsed = Math.min(1, (now - start) / duration);
      const current = Math.round(target * easeOutExpo(elapsed));
      valueElement.textContent = `${prefix}${current}${suffix}`;

      if (elapsed < 1) {
        window.requestAnimationFrame(step);
      } else {
        valueElement.textContent = `${prefix}${target}${suffix}`;
      }
    };

    window.requestAnimationFrame(step);
  };

  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          animateCounter(entry.target);
        }
      });
    },
    { threshold: 0.35 },
  );

  counterCards.forEach((card) => observer.observe(card));
}
