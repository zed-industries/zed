import { ThemeConfig } from "@/theme/config";

const zedDark: ThemeConfig = {
    name: "Zed Dark",
    appearance: "dark",
    author: {
        name: "John Doe",
        email: "johndoe@example.com",
        handle: "@johndoe",
    },
    license: "MIT",
    colors: {
        neutral: ["#262626", "#191919", "#333"],
        accent: ["#ff6a00", "#ff8f00", "#c43e00"],
        error: "#f44336",
        info: "#2196f3",
        warning: "#ffc107",
        success: "#4caf50",
    },
    url: "https://example.com/themes/zed-dark",
    syntax: {
        keyword: {
            color: "#c43e00",
            weight: "bold",
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
