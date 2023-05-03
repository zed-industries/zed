export type Padding = {
    top: number
    bottom: number
    left: number
    right: number
}

/**
 * Returns an object representing the padding for a box element, with values for the top, bottom,
 * left, and right sides. The values can be specified separately for each side, or for all sides
 * using a single value.
 *
 * @param padding - The padding value to apply to all sides.
 *
 * @overload x,y
 * @param x - The padding value to apply to the left and right sides.
 * @param y - The padding value to apply to the top and bottom sides.
 *
 * @overload top,right,bottom,left
 * @param top - The padding value to apply to the top side.
 * @param right - The padding value to apply to the right side.
 * @param bottom - The padding value to apply to the bottom side.
 * @param left - The padding value to apply to the left side.
 */
export function padding(padding: number): Padding
export function padding(x: number, y: number): Padding
export function padding(
    top: number,
    right: number,
    bottom: number,
    left: number
): Padding
export function padding(
    a: number,
    b?: number,
    c?: number,
    d?: number
): Padding {
    if (b === undefined && c === undefined && d === undefined) {
        // Only one argument, apply it to all sides
        return { top: a, bottom: a, left: a, right: a }
    } else if (c === undefined && d === undefined) {
        // Two arguments, apply the first to left/right, the second to top/bottom
        return { top: b!, bottom: b!, left: a, right: a }
    } else if (d === undefined) {
        // Three arguments, apply the first to top, the second to right/left, the third to bottom
        return { top: a, bottom: c!, left: b!, right: b! }
    } else {
        // Four arguments, apply each to the corresponding side
        return { top: a, bottom: c!, left: d!, right: b! }
    }
}
