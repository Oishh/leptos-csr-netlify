/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs",],
  theme: {
    extend: {
      fontFamily: {
        poppins: ["Poppins", "sans-serif"],
        merriweather: ["Merriweather", "sans-serif"],
        librebaskerville: ["Libre Baskerville", "sans-serif"],
    },
    },
  },
  plugins: [require("daisyui")], // add to tailwind.config.js
  daisyui: {
    themes: ["dark", "light", "business",
      {
        darkpurple: {
          "primary": "#312e81",
          "secondary": "#a78bfa",
          "accent": "#ede9fe",
          "neutral": "#78716c",
          "base-100": "#1f2937",
          "info": "#6b7280",
          "success": "#a5b4fc",
          "warning": "#fde047",
          "error": "#f87171",
        },
      }
    ],
},
}