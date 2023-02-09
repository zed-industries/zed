module.exports = {
  plugins: ["import", "react", "@typescript-eslint"],
  env: {
    browser: true,
    es2021: true,
  },
  extends: [
    "plugin:react/recommended",
    "plugin:@next/next/recommended",
    "airbnb",
    "prettier",
  ],
  parser: "@typescript-eslint/parser",
  parserOptions: {
    ecmaFeatures: {
      jsx: true,
    },
    ecmaVersion: "latest",
    sourceType: "module",
  },
  rules: {
    "import/extensions": [0],
    "import/no-extraneous-dependencies": "off",
    "react/jsx-filename-extension": [1, { extensions: [".tsx", ".jsx"] }],
    "react/react-in-jsx-scope": "off",
    "react/require-default-props": "off",
    "no-underscore-dangle ": 0,
  },
  settings: {
    "import/parsers": {
      "@typescript-eslint/parser": [".ts", ".tsx"],
    },
    "import/resolver": {
      typescript: {
        alwaysTryTypes: true,
      },
      project: "./",
    },
  },
};
