import {
    text,
} from "./components"
import { useTheme } from "../theme"
export default function channels_panel(): any {
    const theme = useTheme()


    return {
        contacts_header: text(theme.middle, "sans", "variant", { size: "lg" }),
    }
}
