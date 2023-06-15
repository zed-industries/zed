import merge from "ts-deepmerge"

type InteractiveState = "default" | "hovered" | "clicked" | "selected" | "disabled";

type Interactive<T> = {
    default: T,
    hovered?: T,
    clicked?: T,
    selected?: T,
    disabled?: T,
};

interface InteractiveProps<T> {
    base?: T,
    state: Partial<Record<InteractiveState, T>>
}

/**
 * Helper function for creating Interactive<T> objects that works pretty much like Toggle<T>.
 * It takes a object to be used as a value for `default` field and then fills out other fields
 * with fields from either `base` or `modifications`.
 * Notably, it does not touch `hover`, `clicked` and `disabled` if there are no modifications for it.
 *
 * @param defaultObj Object to be used as the value for `default` field.
 * @param base Object containing base fields to be included in the resulting object.
 * @param modifications Object containing modified fields to be included in the resulting object.
 * @returns Interactive<T> object with fields from `base` and `modifications`.
 */
export function interactive<T extends Object>({ base, state }: InteractiveProps<T>): Interactive<T> {
    if (!base && !state.default) throw new Error("An interactive object must have a default state, or a base property.");

    let defaultState: T;

    if (state.default && base) {
        defaultState = merge(base, state.default) as T;
    } else {
        defaultState = base ? base : state.default as T;
    }

    let interactiveObj: Interactive<T> = {
        default: defaultState,
    };

    let stateCount = 0;

    if (state.hovered !== undefined) {
        interactiveObj.hovered = merge(interactiveObj.default, state.hovered) as T;
        stateCount++;
    }

    if (state.clicked !== undefined) {
        interactiveObj.clicked = merge(interactiveObj.default, state.clicked) as T;
        stateCount++;
    }

    if (state.selected !== undefined) {
        interactiveObj.selected = merge(interactiveObj.default, state.selected) as T;
        stateCount++;
    }

    if (state.disabled !== undefined) {
        interactiveObj.disabled = merge(interactiveObj.default, state.disabled) as T;
        stateCount++;
    }

    if (stateCount < 1) {
        throw new Error("An interactive object must have a default and at least one other state.");
    }

    return interactiveObj;
}
