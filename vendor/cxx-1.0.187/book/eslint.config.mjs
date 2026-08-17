import pluginJs from '@eslint/js';

/** @type {import('eslint').Linter.Config[]} */
export default [
  { ignores: ['build/*'] },
  { files: ['**/*.js'], languageOptions: { sourceType: 'commonjs' } },
  pluginJs.configs.recommended,
];
