module.exports = {
  root: true,
  env: {
    browser: true,
    node: true,
    es2021: true,
  },
  extends: [
    'eslint:recommended', // Base ESLint rules
    'plugin:vue/vue3-recommended', // Recommended Vue.js 3 rules
    'plugin:@typescript-eslint/recommended', // TypeScript recommended rules
    'plugin:@typescript-eslint/recommended-requiring-type-checking', // TypeScript rules requiring type-checking
    'plugin:prettier/recommended', // Prettier integration
  ],
  parser: 'vue-eslint-parser',
  parserOptions: {
    parser: '@typescript-eslint/parser', // Use TypeScript parser
    ecmaVersion: 2021, // Latest ECMAScript features
    sourceType: 'module', // Use ES modules
    project: './tsconfig.json', // Enable type-checking
    extraFileExtensions: ['.vue'], // Support for .vue files
  },
  plugins: ['vue', '@typescript-eslint', 'prettier'],
  rules: {
    'prettier/prettier': 'error',
    // General Rules
    'no-unused-vars': 'off', // Disabled in favor of @typescript-eslint
    '@typescript-eslint/no-unused-vars': [
      'error',
      { args: 'none', ignoreRestSiblings: true },
    ],

    // TypeScript Rules
    '@typescript-eslint/no-explicit-any': 'error', // Disallow `any`
    '@typescript-eslint/explicit-function-return-type': [
      'error',
      {
        allowExpressions: false,
        allowTypedFunctionExpressions: true,
      },
    ],
    '@typescript-eslint/no-floating-promises': 'error', // Ensure promises are handled
    '@typescript-eslint/strict-boolean-expressions': 'error', // Enforce strict boolean logic
    '@typescript-eslint/prefer-readonly': 'error', // Use readonly for immutable variables
    '@typescript-eslint/no-non-null-assertion': 'error', // Disallow `!` for non-null assertions
    '@typescript-eslint/consistent-type-imports': 'error', // Enforce consistent type imports
    '@typescript-eslint/member-delimiter-style': [
      'error',
      {
        multiline: { delimiter: 'semi', requireLast: true },
        singleline: { delimiter: 'semi', requireLast: false },
      },
    ],

    // Vue Rules
    'vue/no-unused-vars': 'error', // Disallow unused variables in templates
    'vue/require-default-prop': 'error', // Require default values for props
    'vue/require-prop-types': 'error', // Require prop types
    'vue/html-indent': ['error', 2], // Enforce 2-space indentation in templates
    'vue/max-attributes-per-line': ['error', { singleline: 2, multiline: 1 }], // Limit attributes per line
    'vue/order-in-components': [
      'error',
      {
        order: [
          'el',
          'name',
          'key',
          'parent',
          'functional',
          ['delimiters', 'comments'],
          ['components', 'directives', 'filters'],
          'extends',
          'mixins',
          'inheritAttrs',
          'model',
          ['props', 'propsData'],
          'data',
          'computed',
          'watch',
          'methods',
          'LIFECYCLE_HOOKS',
          ['template', 'render'],
          'renderError',
        ],
      },
    ],

    // Prettier Rules
    'prettier/prettier': 'error', // Enforce Prettier formatting
  },
  overrides: [
    {
      files: ['*.ts', '*.tsx'], // TypeScript-specific overrides
      rules: {
        '@typescript-eslint/no-unused-vars': ['error'],
      },
    },
    {
      files: ['*.vue'], // Vue-specific overrides
      rules: {
        'vue/multi-word-component-names': 'off', // Disable for single-word component names
      },
    },
  ],
};
