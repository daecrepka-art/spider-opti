const FRAME_COUNT = 150;
const LERP_FACTOR = 0.12;

function lerp(current, target, factor) {
  return current + (target - current) * factor;
}

function getFramePath(index) {
  return `./frames/frame_${String(index).padStart(4, "0")}.png`;
}

export function initFrameAnimation() {
  const section = document.querySelector(".frame-section");
  const canvas = document.getElementById("espresso-canvas");
  const caption = document.getElementById("frame-caption");

  if (
    !(section instanceof HTMLElement) ||
    !(canvas instanceof HTMLCanvasElement)
  ) {
    return;
  }

  const context = canvas.getContext("2d");
  if (!context) {
    return;
  }

  const frames = Array.from({ length: FRAME_COUNT }, (_, frameIndex) => {
    const image = new Image();
    image.decoding = "async";
    image.src = getFramePath(frameIndex + 1);
    return image;
  });

  const drawFrame = (image) => {
    const dpr = window.devicePixelRatio || 1;
    const rect = canvas.getBoundingClientRect();
    const width = Math.max(1, Math.round(rect.width * dpr));
    const height = Math.max(1, Math.round(rect.height * dpr));

    if (canvas.width !== width || canvas.height !== height) {
      canvas.width = width;
      canvas.height = height;
    }

    context.clearRect(0, 0, canvas.width, canvas.height);
    const scale = Math.min(
      canvas.width / image.width,
      canvas.height / image.height,
    );
    const drawWidth = image.width * scale;
    const drawHeight = image.height * scale;
    const dx = (canvas.width - drawWidth) / 2;
    const dy = (canvas.height - drawHeight) / 2;
    context.drawImage(image, dx, dy, drawWidth, drawHeight);
  };

  frames[0].addEventListener("load", () => drawFrame(frames[0]), {
    once: true,
  });

  let currentFrame = 0;

  const animate = () => {
    const viewportHeight = window.innerHeight;
    const sectionTop = section.offsetTop;
    const maxScroll = Math.max(section.offsetHeight - viewportHeight, 1);
    const progress = Math.min(
      1,
      Math.max(0, (window.scrollY - sectionTop) / maxScroll),
    );
    const targetFrame = progress * (FRAME_COUNT - 1);
    currentFrame = lerp(currentFrame, targetFrame, LERP_FACTOR);
    const frameIndex = Math.min(
      FRAME_COUNT - 1,
      Math.max(0, Math.floor(currentFrame)),
    );
    const activeFrame = frames[frameIndex];

    if (activeFrame.complete) {
      drawFrame(activeFrame);
    }

    if (caption) {
      caption.classList.toggle("is-visible", progress >= 0.8);
    }

    window.requestAnimationFrame(animate);
  };

  window.addEventListener("resize", () => {
    const activeFrame =
      frames[Math.min(FRAME_COUNT - 1, Math.max(0, Math.floor(currentFrame)))];
    if (activeFrame?.complete) {
      drawFrame(activeFrame);
    }
  });

  window.requestAnimationFrame(animate);
}
