export function initCursor() {
  const cursor = document.querySelector(".custom-cursor");
  if (
    !(cursor instanceof HTMLElement) ||
    window.matchMedia("(pointer: coarse)").matches
  ) {
    return;
  }

  const interactiveSelector = "a, button, input, textarea, select, label";

  window.addEventListener("mousemove", (event) => {
    cursor.style.left = `${event.clientX}px`;
    cursor.style.top = `${event.clientY}px`;
    document.body.dataset.cursorHidden = "false";
  });

  document.addEventListener("mouseleave", () => {
    document.body.dataset.cursorHidden = "true";
  });

  document.addEventListener("mouseover", (event) => {
    const target = event.target;
    if (!(target instanceof Element)) {
      return;
    }
    document.body.dataset.cursorHover = target.closest(interactiveSelector)
      ? "true"
      : "false";
  });
}
