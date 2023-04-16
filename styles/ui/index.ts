import { find } from "./editor/find"
import { Theme } from "@/theme"

export const buildUI = (theme: Theme) => {
    const ui = {
        find: find(theme),
    }

    return ui
}
