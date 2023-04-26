import { find } from "./editor/find"
import { Theme } from "@/theme"

export const buildUI = (theme: Theme) => {
    console.log(`Reminder: Single color scales are currently placeholders`)

    const ui = {
        find: find(theme),
    }

    return ui
}
