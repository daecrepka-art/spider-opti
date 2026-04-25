export function initHeroReveal() {
  const words = Array.from(document.querySelectorAll(".hero__title-word"));
  const subtitle = document.querySelector(".hero__subtitle");

  window.requestAnimationFrame(() => {
    document.body.dataset.loaded = "true";
  });

  words.forEach((word, index) => {
    window.setTimeout(() => {
      word.classList.add("is-visible");
    }, index * 200);
  });

  if (subtitle) {
    window.setTimeout(
      () => {
        subtitle.classList.add("is-visible");
      },
      words.length * 200 + 1700,
    );
  }
}
