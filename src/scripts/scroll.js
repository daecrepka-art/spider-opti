export function initScrollEffects() {
  const root = document.documentElement;
  const revealItems = Array.from(document.querySelectorAll("[data-reveal]"));
  let ticking = false;

  const updateScrollState = () => {
    const scrollY = window.scrollY || window.pageYOffset;
    root.style.setProperty("--scroll-y", String(scrollY));
    document.body.dataset.scrolled = scrollY > 80 ? "true" : "false";
    ticking = false;
  };

  const onScroll = () => {
    if (!ticking) {
      ticking = true;
      window.requestAnimationFrame(updateScrollState);
    }
  };

  window.addEventListener("scroll", onScroll, { passive: true });
  updateScrollState();

  const observer = new IntersectionObserver(
    (entries) => {
      entries.forEach((entry) => {
        if (entry.isIntersecting) {
          entry.target.classList.add("is-visible");
        }
      });
    },
    { threshold: 0.2 },
  );

  revealItems.forEach((item) => observer.observe(item));
}
