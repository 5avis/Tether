/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        canvas: '#121217',
        surface: {
          DEFAULT: '#18181f',
          elevated: '#21212b',
          hover: '#282836',
          active: '#313142',
        },
        border: {
          dim: '#2b2b38',
          subtle: '#373747',
        },
        coral: {
          DEFAULT: '#8b5cf6',
          hover: '#7c3aed',
          active: '#6d28d9',
          light: 'rgba(139, 92, 246, 0.15)',
          muted: 'rgba(139, 92, 246, 0.08)',
        },
        purple: {
          DEFAULT: '#8b5cf6',
          hover: '#7c3aed',
          active: '#6d28d9',
          light: 'rgba(139, 92, 246, 0.15)',
          muted: 'rgba(139, 92, 246, 0.08)',
        },
        status: {
          healthy: '#10b981',
          warning: '#f59e0b',
          critical: '#ef4444',
        },
        tx: {
          header: '#ffffff',
          body: '#f4f4f6',
          meta: '#a1a1aa',
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
      },
    },
  },
  plugins: [],
};
