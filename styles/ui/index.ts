import { find } from "./editor/find"
import { Theme } from "@/theme"

export const buildUI = (theme: Theme) => {
    return {
        find: find(theme)
    }
}
