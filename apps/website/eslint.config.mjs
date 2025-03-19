// @ts-check
import eslint from "@eslint/js";
import tsEslint from "typescript-eslint";
import stylistic from "@stylistic/eslint-plugin";

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
            "@stylistic/semi": ["warn", "always"]
        }
    }
];
