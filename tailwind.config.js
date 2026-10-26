/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
      colors: {
        canvas: '#0b0d19',
        surface: {
          DEFAULT: 'rgba(22, 27, 46, 0.7)',
          elevated: 'rgba(30, 36, 61, 0.7)',
          hover: 'rgba(38, 45, 77, 0.7)',
          active: 'rgba(46, 55, 94, 0.8)',
          glass: 'rgba(255, 255, 255, 0.03)',
        },
        border: {
          dim: 'rgba(255, 255, 255, 0.08)',
          subtle: 'rgba(255, 255, 255, 0.14)',
          glow: 'rgba(139, 92, 246, 0.3)',
        },
        coral: {
          DEFAULT: '#8b5cf6',
          hover: '#a855f7',
          active: '#7c3aed',
          light: 'rgba(139, 92, 246, 0.16)',
          muted: 'rgba(139, 92, 246, 0.08)',
        },
        purple: {
          DEFAULT: '#8b5cf6',
          hover: '#a855f7',
          active: '#7c3aed',
          light: 'rgba(139, 92, 246, 0.16)',
          muted: 'rgba(139, 92, 246, 0.08)',
        },
        cyan: {
          DEFAULT: '#00f2fe',
          hover: '#38bdf8',
          glow: 'rgba(0, 242, 254, 0.25)',
        },
        neon: {
          cyan: '#00f2fe',
          purple: '#8b5cf6',
          pink: '#ec4899',
          orange: '#ff5e62',
        },
        status: {
          healthy: '#10b981',
          warning: '#f59e0b',
          critical: '#f43f5e',
        },
        tx: {
          header: '#ffffff',
          body: '#f1f5f9',
          meta: '#94a3b8',
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
        'subtle': '0 2px 8px rgba(0, 0, 0, 0.3)',
        'modal': '0 25px 50px -12px rgba(0, 0, 0, 0.7), 0 0 30px rgba(139, 92, 246, 0.25)',
        'glow': '0 0 20px rgba(139, 92, 246, 0.25)',
        'neon': '0 0 20px rgba(139, 92, 246, 0.25)',
        'neon-cyan': '0 0 20px rgba(0, 242, 254, 0.25)',
        'neon-pink': '0 0 20px rgba(236, 72, 153, 0.25)',
        'glass': '0 8px 32px 0 rgba(0, 0, 0, 0.37)',
      },
    },
  },
  plugins: [],
};
