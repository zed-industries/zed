//! Constructs a Dfa which picks the longest matching regular
//! expression from the input.

use crate::collections::Set;
use crate::kernel_set::{Kernel, KernelSet};
use crate::lexer::nfa::{self, Nfa, NfaConstructionError, NfaStateIndex, Test};
use crate::lexer::re;
use std::fmt::{Debug, Display, Error, Formatter};
use std::rc::Rc;

#[cfg(test)]
mod test;

#[cfg(test)]
pub mod interpret;

mod overlap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dfa {
    pub states: Vec<State>,
}

#[allow(clippy::upper_case_acronyms)]
#[deprecated(since = "1.0.0", note = "use `Dfa` instead")]
pub type DFA = Dfa;

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Precedence(pub usize);

#[derive(Debug)]
pub enum DfaConstructionError {
    NfaConstructionError {
        index: NfaIndex,
        error: NfaConstructionError,
    },

    /// Either of the two regexs listed could match, and they have equal
    /// priority.
    Ambiguity { match0: NfaIndex, match1: NfaIndex },
}

#[deprecated(since = "1.0.0", note = "use `DfaConstructionError` instead")]
pub type DFAConstructionError = DfaConstructionError;

pub fn build_dfa(
    regexs: &[re::Regex],
    precedences: &[Precedence],
) -> Result<Dfa, DfaConstructionError> {
    assert_eq!(regexs.len(), precedences.len());
    let nfas = regexs
        .iter()
        .enumerate()
        .map(|(i, r)| match Nfa::from_re(r) {
            Ok(nfa) => Ok(nfa),
            Err(e) => Err(DfaConstructionError::NfaConstructionError {
                index: NfaIndex(i),
                error: e,
            }),
        })
        .collect::<Result<Vec<_>, _>>()?;

    let builder = DfaBuilder {
        nfas: &nfas,
        precedences: precedences.to_vec(),
    };
    let dfa = builder.build()?;
    Ok(dfa)
}

struct DfaBuilder<'nfa> {
    nfas: &'nfa [Nfa],
    precedences: Vec<Precedence>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    item_set: DfaItemSet,
    pub kind: Kind,
    pub test_edges: Vec<(Test, DfaStateIndex)>,
    pub other_edge: DfaStateIndex,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Accepts(NfaIndex),
    Reject,
    Neither,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NfaIndex(usize);

#[deprecated(since = "1.0.0", note = "use `NfaIndex` instead")]
pub type NFAIndex = NfaIndex;

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct DfaStateIndex(usize);

#[deprecated(since = "1.0.0", note = "use `DfaStateIndex` instead")]
pub type DFAStateIndex = DfaStateIndex;

type DfaKernelSet = KernelSet<DfaItemSet>;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct DfaItemSet {
    items: Rc<Vec<Item>>,
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    // which regular expression?
    nfa_index: NfaIndex,

    // what state within the Nfa are we at?
    nfa_state: NfaStateIndex,
}

const START: DfaStateIndex = DfaStateIndex(0);

impl<'nfa> DfaBuilder<'nfa> {
    fn build(&self) -> Result<Dfa, DfaConstructionError> {
        let mut kernel_set = KernelSet::new();
        let mut states = vec![];

        let start_state_index = self.start_state(&mut kernel_set);
        assert_eq!(start_state_index, START);

        while let Some(item_set) = kernel_set.next() {
            // collect all the specific tests we expect from any of
            // the items in this state
            let tests: Set<Test> = item_set
                .items
                .iter()
                .flat_map(|&item| {
                    self.nfa(item)
                        .edges::<Test>(item.nfa_state)
                        .map(|edge| edge.label.clone())
                })
                .collect();
            let tests = overlap::remove_overlap(&tests);

            // if any Nfa is in an accepting state, that makes this
            // Dfa state an accepting state
            let mut all_accepts: Vec<(Precedence, NfaIndex)> = item_set
                .items
                .iter()
                .cloned()
                .filter(|&item| self.nfa(item).is_accepting_state(item.nfa_state))
                .map(|item| (self.precedences[item.nfa_index.0], item.nfa_index))
                .collect();

            // if all NFAs are in a rejecting state, that makes this
            // Dfa a rejecting state
            let all_rejects: bool = item_set
                .items
                .iter()
                .all(|&item| self.nfa(item).is_rejecting_state(item.nfa_state));

            let kind = if all_rejects || item_set.items.is_empty() {
                Kind::Reject
            } else if all_accepts.is_empty() {
                Kind::Neither
            } else if all_accepts.len() == 1 {
                // accepts just one Nfa, easy case
                Kind::Accepts(all_accepts[0].1)
            } else {
                all_accepts.sort(); // sort regex with higher precedence, well, higher
                let (best_priority, best_nfa) = all_accepts[all_accepts.len() - 1];
                let (next_priority, next_nfa) = all_accepts[all_accepts.len() - 2];
                if best_priority == next_priority {
                    return Err(DfaConstructionError::Ambiguity {
                        match0: best_nfa,
                        match1: next_nfa,
                    });
                }
                Kind::Accepts(best_nfa)
            };

            // for each specific test, find what happens if we see a
            // character matching that test
            let mut test_edges: Vec<(Test, DfaStateIndex)> = tests
                .into_iter()
                .map(|test| {
                    let items: Vec<_> = item_set
                        .items
                        .iter()
                        .filter_map(|&item| self.accept_test(item, &test))
                        .collect();

                    // at least one of those items should accept this test
                    assert!(!items.is_empty());

                    (test, kernel_set.add_state(self.transitive_closure(items)))
                })
                .collect();

            test_edges.sort();

            // Consider what there is some character that doesn't meet
            // any of the tests. In this case, we can just ignore all
            // the test edges for each of the items and just union all
            // the "other" edges -- because if it were one of those
            // test edges, then that transition is represented above.
            let other_transitions: Vec<_> = item_set
                .items
                .iter()
                .filter_map(|&item| self.accept_other(item))
                .collect();

            // we never know the full set
            assert!(item_set.items.is_empty() || !other_transitions.is_empty());

            let other_edge = kernel_set.add_state(self.transitive_closure(other_transitions));

            let state = State {
                item_set,
                kind,
                test_edges,
                other_edge,
            };

            states.push(state);
        }

        Ok(Dfa { states })
    }

    fn start_state(&self, kernel_set: &mut DfaKernelSet) -> DfaStateIndex {
        // starting state is at the beginning of all regular expressions
        let items: Vec<_> = (0..self.nfas.len())
            .map(|i| Item {
                nfa_index: NfaIndex(i),
                nfa_state: nfa::START,
            })
            .collect();
        let item_set = self.transitive_closure(items);
        kernel_set.add_state(item_set)
    }

    fn accept_test(&self, item: Item, test: &Test) -> Option<Item> {
        let nfa = self.nfa(item);

        let matching_test = nfa
            .edges::<Test>(item.nfa_state)
            .filter(|edge| edge.label.intersects(test))
            .map(|edge| item.to(edge.to));

        let matching_other = nfa
            .edges::<nfa::Other>(item.nfa_state)
            .map(|edge| item.to(edge.to));

        matching_test.chain(matching_other).next()
    }

    fn accept_other(&self, item: Item) -> Option<Item> {
        let nfa = self.nfa(item);
        nfa.edges::<nfa::Other>(item.nfa_state)
            .map(|edge| item.to(edge.to))
            .next()
    }

    fn transitive_closure(&self, mut items: Vec<Item>) -> DfaItemSet {
        let mut observed: Set<Item> = items.iter().cloned().collect();

        let mut counter = 0;
        while counter < items.len() {
            let item = items[counter];
            let derived_states = self
                .nfa(item)
                .edges::<nfa::Noop>(item.nfa_state)
                .map(|edge| item.to(edge.to))
                .filter(|&item| observed.insert(item));
            items.extend(derived_states);
            counter += 1;
        }

        items.sort();
        items.dedup();

        DfaItemSet {
            items: Rc::new(items),
        }
    }

    fn nfa(&self, item: Item) -> &Nfa {
        &self.nfas[item.nfa_index.0]
    }
}

impl Kernel for DfaItemSet {
    type Index = DfaStateIndex;

    fn index(c: usize) -> DfaStateIndex {
        DfaStateIndex(c)
    }
}

impl Dfa {
    fn state(&self, index: DfaStateIndex) -> &State {
        &self.states[index.0]
    }
}

impl Item {
    fn to(&self, s: NfaStateIndex) -> Item {
        Item {
            nfa_index: self.nfa_index,
            nfa_state: s,
        }
    }
}

impl Debug for DfaStateIndex {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "Dfa{}", self.0)
    }
}

impl Display for DfaStateIndex {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        Debug::fmt(self, fmt)
    }
}

impl NfaIndex {
    pub fn index(self) -> usize {
        self.0
    }
}

impl DfaStateIndex {
    pub fn index(self) -> usize {
        self.0
    }
}

impl Debug for Item {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "({:?}:{:?})", self.nfa_index, self.nfa_state)
    }
}
