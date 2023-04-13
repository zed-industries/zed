import { find } from "./editor/find"
import { Theme } from "@/theme"

const buildUI = (theme: Theme) => {
    return {
        find: find(theme)
    }
}

export default buildUI
