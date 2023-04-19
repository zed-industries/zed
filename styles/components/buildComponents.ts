import { Theme } from "@theme/config"
import { buildSurfaces } from "./surface"

export const buildComponents = (theme: Theme) => {
    buildSurfaces(theme)
}
