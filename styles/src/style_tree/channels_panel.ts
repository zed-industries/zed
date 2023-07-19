// import { with_opacity } from "../theme/color"
import {
    //     Border,
    //     TextStyle,
    // background,
    //     border,
    foreground,
    text,
} from "./components"
import { interactive, toggleable } from "../element"
// import merge from "ts-deepmerge"
import { useTheme } from "../theme"
export default function channels_panel(): any {
    const theme = useTheme()

    // const { is_light } = theme

    return {
        spacing: 10,
        channel_tree: {
            channel_indent: 10,
            channel_name: text(theme.middle, "sans", "variant", { size: "md" }),
            root_name: text(theme.middle, "sans", "variant", { size: "lg", weight: "bold" }),
            channel_icon: (() => {
                const base_icon = (asset: any, color: any) => {
                    return {
                        icon: {
                            color,
                            asset,
                            dimensions: {
                                width: 12,
                                height: 12,
                            }
                        },
                        container: {
                            corner_radius: 4,
                            padding: {
                                top: 4, bottom: 4, left: 4, right: 4
                            },
                            margin: {
                                right: 4,
                            },
                        }
                    }
                }

                return toggleable({
                    state: {
                        inactive: interactive({
                            state: {
                                default: base_icon("icons/chevron_right_8.svg", foreground(theme.middle, "variant")),
                                hovered: base_icon("icons/chevron_right_8.svg", foreground(theme.middle, "hovered")),
                                clicked: base_icon("icons/chevron_right_8.svg", foreground(theme.middle, "active")),
                            },
                        }),
                        active: interactive({
                            state: {
                                default: base_icon("icons/chevron_down_8.svg", foreground(theme.highest, "variant")),
                                hovered: base_icon("icons/chevron_down_8.svg", foreground(theme.highest, "hovered")),
                                clicked: base_icon("icons/chevron_down_8.svg", foreground(theme.highest, "active")),
                            },
                        }),
                    },
                })
            })(),
        }
    }
}
