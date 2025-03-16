/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [
    require('daisyui'),
  ],
  safelist: [
    { pattern: /.*/ }, // (Optional) Forces Tailwind to include all classes, useful for debugging.
  ],
};
