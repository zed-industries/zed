import { snakeCase } from "case-anything";

export default function decamelizeTree(object: { [key: string]: any }) {
  const snakeObject: { [key: string]: any } = {};
  for (const key in object) {
    snakeObject[snakeCase(key)] = decamelizeValue(object[key]);
  }
  return snakeObject;
}

function decamelizeValue(value: any): any {
  if (typeof value === "object") {
    if (Array.isArray(value)) {
      return value.map(decamelizeValue);
    } else {
      return decamelizeTree(value);
    }
  } else {
    return value;
  }
}
