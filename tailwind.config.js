/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./index.html",
  ],
  theme: {
    extend: {
      fontFamily: {
        sans: ['Inter', 'system-ui', 'sans-serif'],
        mono: ['JetBrains Mono', 'Fira Code', 'monospace'],
      },
      colors: {
        dm: {
          bg:          '#0B1220',
          panel:       '#0F1724',
          elevated:    '#141E2E',
          hover:       '#1A2740',
          active:      '#1F3050',
          text:        '#E8ECF2',
          muted:       '#98A1B3',
          dim:         '#5A6478',
          accent:      '#4F7CFF',
          'accent-2':  '#7FB0FF',
          interactive: '#1B3FFF',
          success:     '#34D399',
          warning:     '#FBBF24',
          danger:      '#F87171',
          info:        '#60A5FA',
          purple:      '#A78BFA',
        },
      },
      borderColor: {
        dm: {
          DEFAULT: 'rgba(255, 255, 255, 0.08)',
          strong:  'rgba(255, 255, 255, 0.15)',
          accent:  'rgba(79, 124, 255, 0.4)',
        },
      },
      animation: {
        'dm-fade-in':    'dm-fade-in 200ms ease-out',
        'dm-fade-in-up': 'dm-fade-in-up 350ms ease-out',
        'dm-scale-in':   'dm-scale-in 200ms ease-out',
        'dm-glow':       'dm-glow 3s ease-in-out infinite',
        'dm-pulse':      'dm-pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'dm-spin':       'dm-spin 0.8s linear infinite',
        'dm-shimmer':    'dm-shimmer 1.5s ease-in-out infinite',
        'dm-toast-in':   'dm-toast-in 300ms ease-out',
        'dm-toast-out':  'dm-toast-out 200ms ease-in',
      },
      keyframes: {
        'dm-fade-in':     { from: { opacity: '0' }, to: { opacity: '1' } },
        'dm-fade-in-up':  { from: { opacity: '0', transform: 'translateY(8px)' }, to: { opacity: '1', transform: 'translateY(0)' } },
        'dm-scale-in':    { from: { opacity: '0', transform: 'scale(0.92)' }, to: { opacity: '1', transform: 'scale(1)' } },
        'dm-glow':        { '0%, 100%': { boxShadow: '0 0 8px rgba(79,124,255,0.35)' }, '50%': { boxShadow: '0 0 24px rgba(79,124,255,0.35), 0 0 48px rgba(79,124,255,0.12)' } },
        'dm-pulse':       { '0%, 100%': { opacity: '1' }, '50%': { opacity: '0.4' } },
        'dm-spin':        { from: { transform: 'rotate(0deg)' }, to: { transform: 'rotate(360deg)' } },
        'dm-shimmer':     { '0%': { backgroundPosition: '-200% 0' }, '100%': { backgroundPosition: '200% 0' } },
        'dm-toast-in':    { from: { opacity: '0', transform: 'translateX(100%) scale(0.95)' }, to: { opacity: '1', transform: 'translateX(0) scale(1)' } },
        'dm-toast-out':   { from: { opacity: '1', transform: 'translateX(0) scale(1)' }, to: { opacity: '0', transform: 'translateX(100%) scale(0.95)' } },
      },
    },
  },
  plugins: [],
}
