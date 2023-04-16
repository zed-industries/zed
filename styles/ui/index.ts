import { find } from "./editor/find"
import { Theme } from "@/theme"

export const buildUI = (theme: Theme) => {
    return {
        theme: theme,
        find: find(theme)
    }
}
