/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        "primary": "#135bec",
        "primary-hover": "#0e4bce",
        "primary-dark": "#0f4bc2",
        "background-light": "#f6f6f8",
        "background-dark": "#101622",
        "surface-light": "#ffffff",
        "surface-dark": "#1a2234",
        "border-light": "#e7ebf3",
        "border-dark": "#2a3447",
        "text-main": "#0d121b",
        "text-secondary": "#4c669a",
        "success": "#16a34a",
        "warning": "#ca8a04",
        "error": "#dc2626",
      },
      fontFamily: {
        "display": ["Inter", "Noto Sans SC", "sans-serif"],
        "mono": ["Roboto Mono", "ui-monospace", "SFMono-Regular", "Menlo", "Monaco", "Consolas", "monospace"],
      },
    },
  },
  plugins: [],
}
