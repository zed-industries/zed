import merge from "ts-deepmerge"
import { DeepPartial } from "utility-types"

type ToggleState = "inactive" | "active"

export type Toggleable<T> = Record<ToggleState, T>

export const NO_INACTIVE_OR_BASE_ERROR =
    "A toggleable object must have an inactive state, or a base property."
export const NO_ACTIVE_ERROR = "A toggleable object must have an active state."

interface ToggleableProps<T> {
    base?: T
    state: Partial<Record<ToggleState, DeepPartial<T>>>
}

/**
 * Helper function for creating Toggleable objects.
 * @template T The type of the object being toggled.
 * @param props Object containing the base (inactive) state and state modifications to create the active state.
 * @returns A Toggleable object containing both the inactive and active states.
 * @example
 * ```
 * toggleable({
 *   base: { background: "#000000", text: "#CCCCCC" },
 *   state: { active: { text: "#CCCCCC" } },
 * })
 * ```
 */
export function toggleable<T extends object>(
    props: ToggleableProps<T>
): Toggleable<T> {
    const { base, state } = props

    if (!base && !state.inactive) throw new Error(NO_INACTIVE_OR_BASE_ERROR)
    if (!state.active) throw new Error(NO_ACTIVE_ERROR)

    const inactive_state = base
        ? ((state.inactive ? merge(base, state.inactive) : base) as T)
        : (state.inactive as T)

    const toggle_obj: Toggleable<T> = {
        inactive: inactive_state,
        active: merge(base ?? {}, state.active) as T,
    }
    return toggle_obj
}
