module.exports = {
  darkMode: 'class',
  plugins: [require('@tailwindcss/typography')],
  theme: {
    extend: {
      colors: {
        git: {
          darker: '#d11e11',
          lighter: '#ff3e22',
          primary: '#f14e32',
        },
      },
    },
    screens: {
      lg: '1024px',
      md: '768px',
      sm: '640px',
      xl: '1280px',
    },
  },
}
