import { ThemeConfig } from "@/theme/config";
import * as font from "@/theme/font"

const zedDark: ThemeConfig = {
    name: "Zed Dark",
    appearance: "dark",
    author: {
        name: "John Doe",
        email: "johndoe@example.com",
        handle: "@johndoe",
    },
    license: "MIT",
    url: "https://example.com/themes/zed-dark",
    colors: {
        neutral: ["#07363C", "#fef3e4"],
        accent: ["#ff6a00", "#ff8f00", "#c43e00"],
        error: "#f44336",
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
};

export default zedDark;
