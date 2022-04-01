import { colorRamp } from "../utils/color";

export default {
    fontFamily: {
        sans: "Zed Sans",
        mono: "Zed Mono",
    },
    fontSize: {
        "3xs": {
            value: "8",
            type: "fontSizes",
        },
        "2xs": {
            value: "10",
            type: "fontSizes",
        },
        xs: {
            value: "12",
            type: "fontSizes",
        },
        sm: {
            value: "14",
            type: "fontSizes",
        },
        md: {
            value: "16",
            type: "fontSizes",
        },
        lg: {
            value: "18",
            type: "fontSizes",
        },
        xl: {
            value: "20",
            type: "fontSizes",
        },
    },
    color: {
        neutral: colorRamp(["black", "white"], { steps: 21, increment: 50 }),
        rose: colorRamp("#F43F5EFF"),
        red: colorRamp("#EF4444FF"),
        orange: colorRamp("#F97316FF"),
        amber: colorRamp("#F59E0BFF"),
        yellow: colorRamp("#EAB308FF"),
        lime: colorRamp("#84CC16FF"),
        green: colorRamp("#22C55EFF"),
        emerald: colorRamp("#10B981FF"),
        teal: colorRamp("#14B8A6FF"),
        cyan: colorRamp("#06BBD4FF"),
        sky: colorRamp("#0EA5E9FF"),
        blue: colorRamp("#3B82F6FF"),
        indigo: colorRamp("#6366F1FF"),
        violet: colorRamp("#8B5CF6FF"),
        purple: colorRamp("#A855F7FF"),
        fuschia: colorRamp("#D946E4FF"),
        pink: colorRamp("#EC4899FF"),
    },
};
