import { snakeCase } from "case-anything";

// https://stackoverflow.com/questions/60269936/typescript-convert-generic-object-from-snake-to-camel-case

// Typescript magic to convert any string from camelCase to snake_case at compile time
type SnakeCase<S> =
  S extends string ?
  S extends `${infer T}${infer U}` ?
  `${T extends Capitalize<T> ? "_" : ""}${Lowercase<T>}${SnakeCase<U>}` :
  S :
  S;

type SnakeCased<Type> = {
  [Property in keyof Type as SnakeCase<Property>]: SnakeCased<Type[Property]>
}

export default function snakeCaseTree<T>(object: T): SnakeCased<T> {
  const snakeObject: any = {};
  for (const key in object) {
    snakeObject[snakeCase(key)] = snakeCaseValue(object[key]);
  }
  return snakeObject;
}

function snakeCaseValue(value: any): any {
  if (typeof value === "object") {
    if (Array.isArray(value)) {
      return value.map(snakeCaseValue);
    } else {
      return snakeCaseTree(value);
    }
  } else {
    return value;
  }
}
