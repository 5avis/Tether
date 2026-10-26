/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        canvas: '#0c0a17',
        surface: {
          DEFAULT: '#141126',
          elevated: '#1c1833',
          hover: '#252042',
          active: '#2e2752',
        },
        border: {
          dim: '#282244',
          subtle: '#38305c',
        },
        coral: {
          DEFAULT: '#a855f7',
          hover: '#9333ea',
          active: '#7e22ce',
          light: 'rgba(168, 85, 247, 0.16)',
          muted: 'rgba(168, 85, 247, 0.08)',
        },
        purple: {
          DEFAULT: '#a855f7',
          hover: '#9333ea',
          active: '#7e22ce',
          light: 'rgba(168, 85, 247, 0.16)',
          muted: 'rgba(168, 85, 247, 0.08)',
        },
        cyan: {
          DEFAULT: '#06b6d4',
          hover: '#0891b2',
          glow: 'rgba(6, 182, 212, 0.25)',
        },
        status: {
          healthy: '#10b981',
          warning: '#f59e0b',
          critical: '#f43f5e',
        },
        tx: {
          header: '#ffffff',
          body: '#ede9fe',
          meta: '#948fa8',
        },
      },
      fontFamily: {
        sans: [
          '-apple-system',
          'BlinkMacSystemFont',
          '"Segoe UI"',
          'Roboto',
          'Inter',
          'Helvetica',
          'Arial',
          'sans-serif',
        ],
        mono: [
          '"JetBrains Mono"',
          '"Fira Code"',
          'Consolas',
          'monospace',
        ],
      },
      boxShadow: {
        'subtle': '0 1px 3px 0 rgba(0, 0, 0, 0.4)',
        'modal': '0 20px 25px -5px rgba(0, 0, 0, 0.6), 0 8px 10px -6px rgba(0, 0, 0, 0.6)',
        'glow': '0 0 16px rgba(168, 85, 247, 0.35)',
      },
    },
  },
  plugins: [],
};
