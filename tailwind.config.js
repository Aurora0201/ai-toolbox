/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  darkMode: 'class', // Enable class-based dark mode
  theme: {
    extend: {
      colors: {
        // Design System Mapping
        primary: {
          DEFAULT: '#2563EB', // --primary
          hover: '#1D4ED8',   // --primary-hover
          foreground: '#FFFFFF',
        },
        danger: {
          DEFAULT: '#EF4444', // --danger
        },
        success: {
          DEFAULT: '#10B981', // --success
        },
        background: {
          app: 'var(--bg-app)',         // Dynamic based on dark mode
          surface: 'var(--bg-surface)', // Dynamic based on dark mode
          element: 'var(--bg-element)', // Dynamic based on dark mode
        },
        border: 'var(--border)',
        text: {
          main: 'var(--text-main)',
          sub: 'var(--text-sub)',
        }
      },
      fontFamily: {
        sans: [
          '"Google Sans"', 
          '"Roboto"', 
          '"Noto Sans SC"', 
          '"Microsoft YaHei"', 
          '"Segoe UI"', 
          'sans-serif'
        ],
        // [新增] 代码字体栈
        mono: [
          '"Roboto Mono"', 
          '"Menlo"', 
          '"Consolas"', 
          'monospace'
        ],
      },
      borderRadius: {
        lg: '12px', // Cards/Containers
        md: '8px',  // Inputs/Buttons
      },
      boxShadow: {
        sm: '0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06)',
      }
    },
  },
  plugins: [],
}
