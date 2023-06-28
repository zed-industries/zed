module.exports = {
    plugins: ["import"],
    parser: "@typescript-eslint/parser",
    parserOptions: {
        sourceType: "module"
    },
    "settings": {
        "import/parsers": {
            "@typescript-eslint/parser": [".ts"]
        },
        "import/resolver": {
            "typescript": {
                "alwaysTryTypes": true,
            }
        }
    },
    rules: {
        "import/no-restricted-paths": [
            warn,
            {
                zones: [
                    {
                        "target": "./src/types/*",
                        "from": "./src",
                        "except": [
                            "./src/types/index.ts"
                        ]
                    }
                ]
            }
        ]
    }
}
