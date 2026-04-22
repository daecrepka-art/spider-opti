import type { Config } from "tailwindcss";

export default {
  content: ["./index.html", "./src/**/*.{ts,tsx}"],
  theme: {
    extend: {
      colors: {
        "bg-deep": "#1a102e",
        accent: "#9d4edd",
        "accent-bright": "#c77dff",
      },
      fontFamily: {
        mono: ['"JetBrains Mono"', "Menlo", "Consolas", "monospace"],
      },
    },
  },
  plugins: [],
} satisfies Config;
