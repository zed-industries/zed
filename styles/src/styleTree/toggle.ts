import { DeepPartial } from 'utility-types';
import merge from 'ts-deepmerge';

interface Toggleable<T> {
  inactive: T
  active: T,
}

/// Helper function for creating Toggleable objects; it takes a object of type T that is used as
/// `inactive` member of result Toggleable<T>. `active` member is created by applying `modifications` on top of `inactive` argument.
// Thus, the following call:
// ```
//   toggleable({day: 1, month: "January"}, {day: 3})
// ```
// To return the following object:
// ```
//    Toggleable<_>{
//      inactive: { day: 1, month: "January" },
//      active: { day: 3, month: "January" }
//    }
// ```
// Remarkably, it also works for nested structures:
// ```
//   toggleable({first_level: "foo", second_level: {nested_member: "nested"}}, {second_level: {nested_member: "another nested thing"}})
// ```
// ```
//   Toggleable<_> {
//     inactive: {first_level: "foo", second_level: {nested_member: "nested"}},
//     active: { first_level: "foo", second_level: {nested_member: "another nested thing"}}
//   }
// ```
export function toggleable<T extends Object>(inactive: T, modifications: DeepPartial<T>): Toggleable<T> {
  let active: T = merge(inactive, modifications) as T;
  return { active: active, inactive: inactive };
}
