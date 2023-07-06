import chroma from "chroma-js"

export function with_opacity(color: string, opacity: number): string {
    return chroma(color).alpha(opacity).hex()
}
