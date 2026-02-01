import type { Config } from 'tailwindcss'

const config: Config = {
  content: [
    './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      colors: {
        'spartan-green': '#18453B',
        'spartan-black': '#191A23',
        'spartan-white': '#F3F3F3',
      },
    },
  },
  plugins: [],
}
export default config
