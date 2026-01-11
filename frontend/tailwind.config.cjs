module.exports = {
  content: ["./index.html", "./src/**/*.{ts,tsx,js,jsx}"],
  theme: {
    extend: {
      fontFamily: {
        'space-grotesk': ['Space Grotesk', 'sans-serif'],
        'jost': ['Jost', 'sans-serif'],
      },
      colors: {
        'spartan-green': '#18453B',
        'spartan-dark': '#191A23'
      }
    }
  },
  plugins: []
}
