import merge from "ts-deepmerge"
import { DeepPartial } from "utility-types"

export type InteractiveState =
    | "default"
    | "hovered"
    | "clicked"
    | "selected"
    | "disabled"

type Interactive<T> = {
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
export function interactive<T extends Object>({
    base,
    state,
}: InteractiveProps<T>): Interactive<T> {
    if (!base && !state.default) throw new Error(NO_DEFAULT_OR_BASE_ERROR)

    let defaultState: T

    if (state.default && base) {
        defaultState = merge(base, state.default) as T
    } else {
        defaultState = base ? base : (state.default as T)
    }

    let interactiveObj: Interactive<T> = {
        default: defaultState,
    }

    let stateCount = 0

    if (state.hovered !== undefined) {
        interactiveObj.hovered = merge(
            interactiveObj.default,
            state.hovered
        ) as T
        stateCount++
    }

    if (state.clicked !== undefined) {
        interactiveObj.clicked = merge(
            interactiveObj.default,
            state.clicked
        ) as T
        stateCount++
    }

    if (state.selected !== undefined) {
        interactiveObj.selected = merge(
            interactiveObj.default,
            state.selected
        ) as T
        stateCount++
    }

    if (state.disabled !== undefined) {
        interactiveObj.disabled = merge(
            interactiveObj.default,
            state.disabled
        ) as T
        stateCount++
    }

    if (stateCount < 1) {
        throw new Error(NOT_ENOUGH_STATES_ERROR)
    }

    return interactiveObj
}
