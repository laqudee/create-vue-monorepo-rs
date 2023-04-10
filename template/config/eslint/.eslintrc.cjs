/* eslint-env node */
require('@rushstack/eslint-patch/modern-module-resolution')

module.exports = {
  root: true,
  ignorePatterns: ["iconfont/*", "iconfont.js"], // 忽略对assets/iconfont/* 下所有文件进行检查
  'extends': [
    'plugin:vue/vue3-essential',
    'eslint:recommended',
    '@vue/eslint-config-prettier/skip-formatting'
  ],
  parserOptions: {
    ecmaVersion: 'latest'
  },
  rules: {
    "no-restricted-imports": [2, "lodash"], // 项目中只能引入一个工具函数库"lodash"
    eqeqeq: ["error", "always"], // 必须全等===
  },
}
