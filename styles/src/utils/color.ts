import chroma from "chroma-js";

export function withOpacity(color: string, opacity: number): string {
  return chroma(color).alpha(opacity).hex();
}
