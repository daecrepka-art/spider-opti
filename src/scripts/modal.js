export function initModal() {
  const modal = document.getElementById("waitlist-modal");
  const openButton = document.querySelector("[data-modal-open]");
  const closeButtons = Array.from(
    document.querySelectorAll("[data-modal-close]"),
  );
  const form = document.getElementById("waitlist-form");
  const successMessage = document.getElementById("waitlist-success");
  const emailInput = document.getElementById("waitlist-email");

  if (!(modal instanceof HTMLElement)) {
    return;
  }

  let hasAutoOpened = false;

  const openModal = () => {
    modal.classList.add("is-open");
    modal.setAttribute("aria-hidden", "false");
    if (emailInput instanceof HTMLInputElement) {
      window.setTimeout(() => emailInput.focus(), 60);
    }
  };

  const closeModal = () => {
    modal.classList.remove("is-open");
    modal.setAttribute("aria-hidden", "true");
  };

  openButton?.addEventListener("click", openModal);
  closeButtons.forEach((button) =>
    button.addEventListener("click", closeModal),
  );

  document.addEventListener("keydown", (event) => {
    if (event.key === "Escape") {
      closeModal();
    }
  });

  const onScroll = () => {
    const maxScroll = Math.max(
      document.documentElement.scrollHeight - window.innerHeight,
      1,
    );
    const progress = window.scrollY / maxScroll;
    if (progress >= 0.95 && !hasAutoOpened) {
      hasAutoOpened = true;
      openModal();
    }
  };

  window.addEventListener("scroll", onScroll, { passive: true });
  onScroll();

  form?.addEventListener("submit", (event) => {
    event.preventDefault();
    if (!(emailInput instanceof HTMLInputElement)) {
      return;
    }
    const email = emailInput.value.trim();
    if (!email) {
      emailInput.focus();
      return;
    }
    try {
      window.localStorage.setItem("caffe-milano-waitlist-email", email);
    } catch {
      // Storage can be unavailable in private contexts.
    }
    form.setAttribute("hidden", "true");
    successMessage?.removeAttribute("hidden");
  });
}
