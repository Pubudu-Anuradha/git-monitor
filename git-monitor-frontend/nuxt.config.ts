// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  build: {
    transpile: ['vuetify'],
  },
  colorMode: {
    classSuffix: '',
  },

  css: ['vuetify/lib/styles/main.sass'],

  devtools: { enabled: true },
  modules: ['@nuxtjs/color-mode', '@nuxtjs/tailwindcss'],

  typescript: {
    strict: true,
  },
})
