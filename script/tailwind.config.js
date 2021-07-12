module.exports = {
    theme: {
        fontFamily: {
            display: [
                "Visby CF", "ui-sans-serif", "system-ui", "-apple-system", "BlinkMacSystemFont", "Segoe UI", "Roboto",
                "Helvetica Neue", "Arial", "Noto Sans", "sans-serif", "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol",
                "Noto Color Emoji"
            ],
            body: [
                "Open Sans", "ui-sans-serif", "system-ui", "-apple-system", "BlinkMacSystemFont", "Segoe UI", "Roboto",
                "Helvetica Neue", "Arial", "Noto Sans", "sans-serif", "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol",
                "Noto Color Emoji"
            ],
        },
        extend: {
            typography: (theme) => ({
                DEFAULT: {
                    css: {
                        h1: {
                            fontFamily: theme("fontFamily.display").join(", ")
                        },
                        h2: {
                            fontFamily: theme("fontFamily.display").join(", ")
                        },
                        h3: {
                            fontFamily: theme("fontFamily.display").join(", ")
                        },
                        h4: {
                            fontFamily: theme("fontFamily.display").join(", ")
                        }
                    }
                }
            })
        }
    },
    variants: {
    },
    plugins: [
        require('@tailwindcss/typography'),
    ],
    purge: [
        "../server/templates/**/*.hbs"
    ]
}