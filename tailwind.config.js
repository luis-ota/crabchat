/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all", // You can remove this unless you're doing something specific.
  content: [
    "./src/**/*.{rs,html,css}", // Good for Dioxus .rs files
    "./dist/**/*.html",
  ],
  theme: {
    extend: {
      colors: {
        rustOrange: "#f74c00",
      },
      fontFamily: {
        mono: ['"Fira Code"', "monospace"],
      },
      boxShadow: {
        wired: "0 0 8px #f74c00",
      },
      scale: {
        101: "1.01",
      },
    },
  },
  plugins: [],
};
