/**
* Utility type to improve the readability of types.
*
* Wrapping a type or interface in `Prettify` will make it so that the type or interface properties are properly displayed when hovering over them.
*/
export type Prettify<T> = {
    [K in keyof T]: T[K];
}
