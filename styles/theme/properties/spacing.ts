export type Spacing = {
    top: number
    bottom: number
    left: number
    right: number
}

/**
 * Returns an object representing the padding or margin for a box element.
 *
 * @param all - The value to apply to all sides.
 * @returns An object representing the padding or margin for the box element.
 *
 * @param x - The value to apply to the left and right sides.
 * @param y - The value to apply to the top and bottom sides.
 * @returns An object representing the padding or margin for the box element.
 *
 * @param top - The value to apply to the top side.
 * @param right - The value to apply to the right side.
 * @param bottom - The value to apply to the bottom side.
 * @param left - The value to apply to the left side.
 * @returns An object representing the padding or margin for the box element.
 */
export function spacing(all: number): Padding | Margin
export function spacing(x: number, y: number): Padding | Margin
export function spacing(
    top: number,
    right: number,
    bottom: number,
    left: number
): Padding | Margin
export function spacing(
    a: number,
    b?: number,
    c?: never,
    d?: number
): Padding | Margin {
    if (b === undefined && c === undefined && d === undefined) {
        // One argument, apply it to all sides
        return { top: a, bottom: a, left: a, right: a }
    } else if (c === undefined && d === undefined) {
        // Two arguments, apply the first to left/right, the second to top/bottom
        return { top: b!, bottom: b!, left: a, right: a }
    } else if (d === undefined) {
        // Three arguments not allowed, throw an error
        throw new Error("Invalid number of arguments")
    } else {
        // Four arguments, apply each to the corresponding side
        return { top: a, bottom: c!, left: d!, right: b! }
    }
}

export type Padding = Spacing
export const padding = spacing

export type Margin = Spacing
export const margin = spacing
