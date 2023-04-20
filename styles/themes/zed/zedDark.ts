import { ThemeConfig } from "@/theme/config"
import * as font from "@theme/text"

const zedDark: ThemeConfig = {
    name: "Zed Dark",
    appearance: "dark",
    author: {
        name: "Nate Butler",
        email: "iamnbutler@gmail.com",
        handle: "@iamnbutler",
    },
    license: "MIT",
    url: "https://zed.dev",
    colors: {
        neutral: ["#020202", "#D8D8CA"],
        accent: ["#ff6a00", "#ff8f00", "#c43e00"],
        error: ["#420904", "#60201A", "#9A2C1B", "#BA4734", "BD6755"],
        info: "#2196f3",
        warning: "#ffc107",
        success: "#4caf50",
    },
    syntax: {
        keyword: {
            color: "#c43e00",
            weight: font.weight.bold,
        },
        string: {
            color: "#00bcd4",
            italic: true,
        },
        comment: {
            color: "#757575",
            italic: true,
        },
        number: {
            color: "#c43e00",
        },
        boolean: {
            color: "#c43e00",
        },
    },
}

export default zedDark
