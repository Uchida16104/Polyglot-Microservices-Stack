/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        bg: '#0a0a0f',
        surface: '#111118',
        border: '#1e1e2e',
        accent: {
          DEFAULT: '#7c6af7',
          hover: '#9580ff',
        },
        success: '#38d9a9',
        warning: '#f7c948',
        danger: '#f06595',
        info: '#60a5fa',
      },
      fontFamily: {
        sans: ['Syne', 'system-ui', 'sans-serif'],
        mono: ['DM Mono', 'monospace'],
      },
    },
  },
  plugins: [],
};
