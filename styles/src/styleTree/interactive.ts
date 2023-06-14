interface Interactive<T> {
  default: T,
  hover?: T,
  clicked?: T,
  disabled?: T,
}

export function interactive<T>(base: T, modifications: Partial<Interactive<T>>): Interactive<T> {
  const interactiveObj: Interactive<T> = {
    default: base,
  };

  if (modifications.hover !== undefined) {
    interactiveObj.hover = { ...base, ...modifications.hover };
  }

  if (modifications.clicked !== undefined) {
    interactiveObj.clicked = { ...base, ...modifications.clicked };
  }

  if (modifications.disabled !== undefined) {
    interactiveObj.disabled = { ...base, ...modifications.disabled };
  }

  return interactiveObj;
}
