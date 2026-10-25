/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        canvas: '#1f1f1f',
        surface: {
          DEFAULT: '#262626',
          elevated: '#2d2d2d',
          hover: '#2e2e2e',
          active: '#333333',
        },
        border: {
          dim: '#333333',
          subtle: '#383838',
        },
        coral: {
          DEFAULT: '#e25c48',
          hover: '#d04b37',
          active: '#b93f2c',
          light: 'rgba(226, 92, 72, 0.15)',
          muted: 'rgba(226, 92, 72, 0.08)',
        },
        status: {
          healthy: '#46a758',
          warning: '#f59e0b',
          critical: '#e25c48',
        },
        tx: {
          header: '#ffffff',
          body: '#f0f0f0',
          meta: '#a3a3a3',
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
        'subtle': '0 1px 2px 0 rgba(0, 0, 0, 0.3)',
        'modal': '0 20px 25px -5px rgba(0, 0, 0, 0.5), 0 8px 10px -6px rgba(0, 0, 0, 0.5)',
      },
    },
  },
  plugins: [],
};
