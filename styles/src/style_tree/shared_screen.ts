import { useTheme } from "../theme"
import { background } from "./components"

export default function sharedScreen() {
    const theme = useTheme()

    return {
        background: background(theme.highest),
    }
}
