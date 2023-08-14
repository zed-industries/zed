export const ButtonVariant = {
    Default: 'default',
    Ghost: 'ghost'
} as const

export type Variant = typeof ButtonVariant[keyof typeof ButtonVariant]
