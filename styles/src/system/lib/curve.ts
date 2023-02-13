import bezier from "bezier-easing";
import { Curve } from "../ref/curves";

/**
 * Formats our Curve data structure into a bezier easing function.
 * @param {Curve} curve - The curve to format.
 * @param {Boolean} inverted - Whether or not to invert the curve.
 * @returns {EasingFunction} The formatted easing function.
 */
export function curve(curve: Curve, inverted?: Boolean) {
  if (inverted) {
    return bezier(
      curve.value[3],
      curve.value[2],
      curve.value[1],
      curve.value[0]
    );
  }

  return bezier(curve.value[0], curve.value[1], curve.value[2], curve.value[3]);
}
