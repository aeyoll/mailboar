import eslintConfigPrettier from 'eslint-config-prettier';
import pluginVue from 'eslint-plugin-vue';

export default [
  {
    ignores: ['static', 'node_modules'],
  },
  ...pluginVue.configs['flat/recommended'],
  eslintConfigPrettier,
  {
    rules: {
      'unicorn/filename-case': 'off',
      'vue/no-deprecated-slot-attribute': 'off',
    },
  }
];
