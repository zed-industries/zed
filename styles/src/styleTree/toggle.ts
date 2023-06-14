interface Toggleable<T> {
  inactive: T
  active: T,
}

export function toggleable<T>(inactive: T, modifications: Partial<Toggleable<T>>): Toggleable<T> {
  let active: T = inactive;
  if (modifications.active !== undefined) {
    active = { ...inactive, ...modifications.active };
  }
  return {
    inactive: inactive,
    active: active
  };

  d
}
