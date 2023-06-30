import merge from "ts-deepmerge"
import { DeepPartial } from "utility-types"

export type InteractiveState =
    | "default"
    | "hovered"
    | "clicked"
    | "selected"
    | "disabled"

export type Interactive<T> = {
    default: T
    hovered?: T
    clicked?: T
    selected?: T
    disabled?: T
}

export const NO_DEFAULT_OR_BASE_ERROR =
    "An interactive object must have a default state, or a base property."
export const NOT_ENOUGH_STATES_ERROR =
    "An interactive object must have a default and at least one other state."

interface InteractiveProps<T> {
    base?: T
    state: Partial<Record<InteractiveState, DeepPartial<T>>>
}

/**
 * Helper function for creating Interactive<T> objects that works with Toggle<T>-like behavior.
 * It takes a default object to be used as the value for `default` field and fills out other fields
 * with fields from either `base` or from the `state` object which contains values for specific states.
 * Notably, it does not touch `hover`, `clicked`, `selected` and `disabled` states if there are no modifications for them.
 *
 * @param defaultObj Object to be used as the value for the `default` field.
 * @param base Optional object containing base fields to be included in the resulting object.
 * @param state Object containing optional modified fields to be included in the resulting object for each state.
 * @returns Interactive<T> object with fields from `base` and `state`.
 */
export function interactive<T extends object>({
    base,
    state,
}: InteractiveProps<T>): Interactive<T> {
    if (!base && !state.default) throw new Error(NO_DEFAULT_OR_BASE_ERROR)

    let default_state: T

    if (state.default && base) {
        default_state = merge(base, state.default) as T
    } else {
        default_state = base ? base : (state.default as T)
    }

    const interactive_obj: Interactive<T> = {
        default: default_state,
    }

    let state_count = 0

    if (state.hovered !== undefined) {
        interactive_obj.hovered = merge(
            interactive_obj.default,
            state.hovered
        ) as T
        state_count++
    }

    if (state.clicked !== undefined) {
        interactive_obj.clicked = merge(
            interactive_obj.default,
            state.clicked
        ) as T
        state_count++
    }

    if (state.selected !== undefined) {
        interactive_obj.selected = merge(
            interactive_obj.default,
            state.selected
        ) as T
        state_count++
    }

    if (state.disabled !== undefined) {
        interactive_obj.disabled = merge(
            interactive_obj.default,
            state.disabled
        ) as T
        state_count++
    }

    if (state_count < 1) {
        throw new Error(NOT_ENOUGH_STATES_ERROR)
    }

    return interactive_obj
}
