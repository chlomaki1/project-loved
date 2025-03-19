// @ts-check
import eslint from "@eslint/js";
import stylistic from "@stylistic/eslint-plugin";
import tsEslint from "typescript-eslint";

export default [
    eslint.configs.recommended,
    ...tsEslint.configs.recommended,
    stylistic.configs["recommended-flat"],
    {
        plugins: {
            "@stylistic": stylistic
        },
        rules: {
            "@typescript-eslint/explicit-function-return-type": "off",
            "@stylistic/indent": ["error", 4],
            "@stylistic/jsx-indent-props": ["error", 4],
            "@stylistic/comma-dangle": ["warn", "never"],
            "@stylistic/quotes": ["warn", "double"],
            "@stylistic/semi": ["warn", "always"],
            "sort-imports": "warn",
            "prefer-const": "warn",
            "no-var": "error"
        }
    }
];
