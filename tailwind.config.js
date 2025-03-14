/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [],
  safelist: [
    { pattern: /.*/ }, // (Optional) Forces Tailwind to include all classes, useful for debugging.
  ],
};
