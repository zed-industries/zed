/** Converts a percentage scale value (0-100) to normalized scale (0-1) value. */
export function percentageToNormalized(value: number) {
  const normalized = value / 100;
  return normalized;
}

/** Converts a normalized scale (0-1) value to a percentage scale (0-100) value. */
export function normalizedToPercetage(value: number) {
  const percentage = value * 100;
  return percentage;
}
