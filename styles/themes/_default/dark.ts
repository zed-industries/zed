import { ThemeConfig } from "@/theme/config"
import { color } from "@theme/system"

const neutral = color.neutral.scale.values

const systemDark: ThemeConfig = {
    name: "System Default Dark",
    appearance: "dark",
    author: {
        name: "Zed Team",
        email: "hi@zed.dev",
    },
    license: "MIT",
    url: "https://zed.dev",
    colors: {
        neutral: [neutral[0], neutral[100]],
        accent: ["#ff6a00", "#ff8f00", "#c43e00"],
        error: ["#420904", "#60201A", "#9A2C1B", "#BA4734", "BD6755"],
        info: "#2196f3",
        warning: "#ffc107",
        success: "#4caf50",
    },
}

export default systemDark
