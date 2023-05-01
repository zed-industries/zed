import { ContainerStyle } from "@theme/container"

export interface FlexStyle {
    spacing: number
    /** Cross-axis alignment of items inside the flex */
    alignItems:
    | "start"
    | "center"
    // GPUI doesn't currently support end alignment
    // | "end"
}

export type Flex = {
    container: ContainerStyle,
    flex: FlexStyle,
}

type FlexOptions = Partial<Omit<FlexStyle, 'spacing'>>

const DEFAULT_FLEX_OPTIONS: FlexOptions = {
    alignItems: "start",
}

export function flex(spacing: number, options: FlexOptions) {
    const mergedOptions = {
        ...DEFAULT_FLEX_OPTIONS,
        ...options,
    }

    return {
        spacing: spacing,
        alignItems: mergedOptions.alignItems,
    }
}
