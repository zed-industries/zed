export type Prettify<T> = {
    [K in keyof T]: T[K];
} & unknown;

/**
* Clean removes the [k: string]: unknown property from an object,
* and Prettifies it, providing better hover information for the type
*/
export type Clean<T> = {
    [K in keyof T as string extends K ? never : K]: T[K];
}

export type DeepClean<T> = {
    [K in keyof T as string extends K ? never : K]: T[K] extends object ? DeepClean<T[K]> : T[K];
}
