module.exports = {
    theme: {
        fontFamily: {
            display: [
                "Spectral", "Constantia", "Lucida Bright", "Lucidabright", "Lucida Serif", "Lucida", "DejaVu Serif", "Bitstream Vera Serif", 
                "Liberation Serif", "Georgia", "serif", "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol",
                "Noto Color Emoji"
            ],
            body: [
                "JetBrains Mono", "Andale Mono WT", "Andale Mono", "Lucida Console", "Lucida Sans Typewriter", "DejaVu Sans Mono", 
                "Bitstream Vera Sans Mono", "Liberation Mono", "Nimbus Mono L", "Courier New", "Apple Color Emoji", "Segoe UI Emoji", 
                "Segoe UI Symbol", "Noto Color Emoji"
            ],
            mono: [
                "JetBrains Mono", "Andale Mono WT", "Andale Mono", "Lucida Console", "Lucida Sans Typewriter", "DejaVu Sans Mono", 
                "Bitstream Vera Sans Mono", "Liberation Mono", "Nimbus Mono L", "Courier New", "Apple Color Emoji", "Segoe UI Emoji", 
                "Segoe UI Symbol", "Noto Color Emoji"
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