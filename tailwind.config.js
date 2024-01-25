/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      aria: {
        current: 'current="page"',
      },
    },
  },
  plugins: [],
}
