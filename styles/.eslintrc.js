module.exports = {
    'env': {
        "node": true
    },
    'extends': [
        'eslint:recommended',
        'plugin:@typescript-eslint/recommended'
    ],
    'parser': '@typescript-eslint/parser',
    'parserOptions': {
        'ecmaVersion': 'latest',
        'sourceType': 'module'
    },
    'plugins': [
        '@typescript-eslint', 'import'
    ],
    globals: {
        module: true
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
    'rules': {
        'indent': [
            'error',
            4
        ],
        'linebreak-style': [
            'error',
            'unix'
        ],
        'semi': [
            'error',
            'never'
        ],
        "import/no-restricted-paths": [
            'error',
            {
                'zones': [
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
