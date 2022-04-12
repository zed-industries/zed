const colors = require('tailwindcss/colors')

module.exports = {
    corePlugins: {
        float: false,
        tableLayout: false,
        sepia: false,
        saturate: false,
    },

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
        colors: {
            transparent: 'transparent',
            current: 'currentColor',
            black: colors.black,
            white: colors.white,
            gray: colors.trueGray,
        },
        extend: {},
    },

    variants: {
    },
    darkMode: false,
    purge: [
        "../crates/collab/templates/**/*.hbs",
        "../crates/collab/templates/*.hbs"
    ]
}