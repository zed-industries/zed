import { DeepPartial } from 'utility-types';
import merge from 'ts-deepmerge';

interface Toggleable<T> {
    inactive: T
    active: T,
}

/**
 * Helper function for creating Toggleable objects.
 * @template T The type of the object being toggled.
 * @param inactive The initial state of the toggleable object.
 * @param modifications The modifications to be applied to the initial state to create the active state.
 * @returns A Toggleable object containing both the inactive and active states.
 * @example
 * ```
 * toggleable({day: 1, month: "January"}, {day: 3})
 * ```
 * This returns the following object:
 * ```
 *  Toggleable<_>{
 *    inactive: { day: 1, month: "January" },
 *    active: { day: 3, month: "January" }
 *  }
 * ```
 * The function also works for nested structures:
 * ```
 *   toggleable({first_level: "foo", second_level: {nested_member: "nested"}}, {second_level: {nested_member: "another nested thing"}})
 * ```
 * Which returns:
 * ```
 *   Toggleable<_> {
 *     inactive: {first_level: "foo", second_level: {nested_member: "nested"}},
 *     active: { first_level: "foo", second_level: {nested_member: "another nested thing"}}
 *   }
 * ```
 */
export function toggleable<T extends Object>(inactive: T, modifications: DeepPartial<T>): Toggleable<T> {
    let active: T = merge(inactive, modifications) as T;
    return { active: active, inactive: inactive };
}
