module.exports = {
  extends: [
    '@nuxtjs/eslint-config-typescript',
    'plugin:prettier/recommended',
    'plugin:perfectionist/recommended-natural',
  ],
  root: true,
  rules: { 'import/order': 'off' },
}
