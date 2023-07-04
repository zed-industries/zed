import { create } from "zustand"
import { ColorScheme } from "./color_scheme"

type ThemeState = {
    theme: ColorScheme | undefined
    setTheme: (theme: ColorScheme) => void
}

export const useThemeStore = create<ThemeState>((set) => ({
    theme: undefined,
    setTheme: (theme) => set(() => ({ theme })),
}))

export const useTheme = (): ColorScheme => {
    const { theme } = useThemeStore.getState()

    if (!theme) throw new Error("Tried to use theme before it was loaded")

    return theme
}

export * from "./color_scheme"
export * from "./ramps"
export * from "./syntax"
export * from "./theme_config"
