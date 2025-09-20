import eslint from "@eslint/js";
import { defineConfig } from "eslint/config";
import tseslint from "typescript-eslint";
import globals from "globals";
import importPlugin from "eslint-plugin-import";

export default defineConfig(
  eslint.configs.recommended,
  tseslint.configs.strict,
  tseslint.configs.stylistic,
  importPlugin.flatConfigs.recommended,
  importPlugin.flatConfigs.typescript,
  {
    settings: {
      "import/resolver": {
        typescript: {},
      },
    },
    languageOptions: {
      globals: {
        ...globals.browser,
        ...globals.commonjs,
        ...globals.es2021,
      },

      ecmaVersion: "latest",
      sourceType: "script",
    },
    rules: {
      "@typescript-eslint/no-non-null-assertion": "off",
    },
  },
  {
    files: [
      "webpack.config.js",
      "postcss.config.js",
      "tailwind.config.js",
      "eslint.config.mjs",
    ],
    languageOptions: {
      globals: {
        ...globals.node,
      },
    },
  }
);
