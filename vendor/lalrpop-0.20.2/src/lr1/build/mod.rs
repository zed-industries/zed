//! LR(1) state construction algorithm.

use crate::collections::{map, Multimap};
use crate::grammar::repr::*;
use crate::kernel_set;
use crate::lr1::core::*;
use crate::lr1::first;
use crate::lr1::lane_table::*;
use crate::lr1::lookahead::*;
use crate::tls::Tls;
use std::env;

#[cfg(test)]
mod test;

fn build_lr1_states_legacy<'grammar>(
    grammar: &'grammar Grammar,
    start: NonterminalString,
) -> Lr1Result<'grammar> {
    let eof = TokenSet::eof();
    let mut lr1: Lr<'grammar, TokenSet> = Lr::new(grammar, start, eof);
    lr1.set_permit_early_stop(true);
    lr1.build_states()
}

type ConstructionFunction<'grammar> =
    fn(&'grammar Grammar, NonterminalString) -> Lr1Result<'grammar>;

pub fn use_lane_table() -> bool {
    match env::var("LALRPOP_LANE_TABLE") {
        Ok(ref s) => s != "disabled",
        _ => true,
    }
}

pub fn build_lr1_states(grammar: &Grammar, start: NonterminalString) -> Lr1Result<'_> {
    let (method_name, method_fn) = if use_lane_table() {
        ("lane", build_lane_table_states as ConstructionFunction)
    } else {
        ("legacy", build_lr1_states_legacy as ConstructionFunction)
    };

    profile! {
        &Tls::session(),
        format!("LR(1) state construction ({})", method_name),
        {
            method_fn(grammar, start)
        }
    }
}

pub fn build_lr0_states(
    grammar: &Grammar,
    start: NonterminalString,
) -> Result<Vec<Lr0State<'_>>, Lr0TableConstructionError<'_>> {
    let lr1 = Lr::new(grammar, start, Nil);
    lr1.build_states()
}

pub struct Lr<'grammar, L: LookaheadBuild> {
    grammar: &'grammar Grammar,
    first_sets: first::FirstSets,
    start_nt: NonterminalString,
    start_lookahead: L,
    permit_early_stop: bool,
}

impl<'grammar, L: LookaheadBuild> Lr<'grammar, L> {
    fn new(grammar: &'grammar Grammar, start_nt: NonterminalString, start_lookahead: L) -> Self {
        Lr {
            grammar,
            first_sets: first::FirstSets::new(grammar),
            start_nt,
            start_lookahead,
            permit_early_stop: false,
        }
    }

    fn set_permit_early_stop(&mut self, v: bool) {
        self.permit_early_stop = v;
    }

    fn build_states(&self) -> Result<Vec<State<'grammar, L>>, TableConstructionError<'grammar, L>> {
        let session = Tls::session();
        let mut kernel_set = kernel_set::KernelSet::new();
        let mut states = vec![];
        let mut conflicts = vec![];

        // create the starting state
        kernel_set.add_state(Kernel::start(self.items(
            &self.start_nt,
            0,
            &self.start_lookahead,
        )));

        while let Some(Kernel { items: seed_items }) = kernel_set.next() {
            let items = self.transitive_closure(seed_items);
            let index = StateIndex(states.len());

            if index.0 % 5000 == 0 && index.0 > 0 {
                log!(session, Verbose, "{} states created so far.", index.0);
            }

            let mut this_state = State {
                index,
                items: items.clone(),
                shifts: map(),
                reductions: vec![],
                gotos: map(),
            };

            // group the items that we can transition into by shifting
            // over a term or nonterm
            let transitions: Multimap<Symbol, Multimap<Lr0Item<'grammar>, L>> = items
                .vec
                .iter()
                .filter_map(Item::shifted_item)
                .map(
                    |(
                        symbol,
                        Item {
                            production,
                            index,
                            lookahead,
                        },
                    )| {
                        (symbol.clone(), (Item::lr0(production, index), lookahead))
                    },
                )
                .collect();

            for (symbol, shifted_items) in transitions.into_iter() {
                let shifted_items: Vec<Item<'grammar, L>> = shifted_items
                    .into_iter()
                    .map(|(lr0_item, lookahead)| lr0_item.with_lookahead(lookahead))
                    .collect();

                // Not entirely obvious: if the original set of items
                // is sorted to begin with (and it is), then this new
                // set of shifted items is *also* sorted. This is
                // because it is produced from the old items by simply
                // incrementing the index by 1.
                let next_state = kernel_set.add_state(Kernel::shifted(shifted_items));

                match symbol {
                    Symbol::Terminal(s) => {
                        let prev = this_state.shifts.insert(s, next_state);
                        assert!(prev.is_none()); // cannot have a shift/shift conflict
                    }

                    Symbol::Nonterminal(s) => {
                        let prev = this_state.gotos.insert(s, next_state);
                        assert!(prev.is_none());
                    }
                }
            }

            // finally, consider the reductions
            for item in items.vec.iter().filter(|i| i.can_reduce()) {
                this_state
                    .reductions
                    .push((item.lookahead.clone(), item.production));
            }

            // check for shift-reduce conflicts (reduce-reduce detected above)
            conflicts.extend(L::conflicts(&this_state));

            // extract a new state
            states.push(this_state);

            if self.permit_early_stop && session.stop_after(conflicts.len()) {
                log!(
                    session,
                    Verbose,
                    "{} conflicts encountered, stopping.",
                    conflicts.len()
                );
                break;
            }
        }

        if !conflicts.is_empty() {
            Err(TableConstructionError { states, conflicts })
        } else {
            Ok(states)
        }
    }

    fn items(&self, id: &NonterminalString, index: usize, lookahead: &L) -> Vec<Item<'grammar, L>> {
        self.grammar
            .productions_for(id)
            .iter()
            .map(|production| {
                debug_assert!(index <= production.symbols.len());
                Item {
                    production,
                    index,
                    lookahead: lookahead.clone(),
                }
            })
            .collect()
    }

    // expands `state` with epsilon moves
    fn transitive_closure(&self, items: Vec<Item<'grammar, L>>) -> Items<'grammar, L> {
        let mut stack: Vec<Lr0Item<'grammar>> = items.iter().map(Item::to_lr0).collect();
        let mut map: Multimap<Lr0Item<'grammar>, L> = items
            .into_iter()
            .map(|item| (item.to_lr0(), item.lookahead))
            .collect();

        while let Some(item) = stack.pop() {
            let lookahead = map.get(&item).unwrap().clone();

            let shift_symbol = item.shift_symbol();

            // Check whether this is an item where the cursor
            // is resting on a non-terminal:
            //
            // I = ... (*) X z... [lookahead]
            //
            // The `nt` will be X and the `remainder` will be `z...`.
            let (nt, remainder) = match shift_symbol {
                None => continue, // requires a reduce
                Some((Symbol::Terminal(_), _)) => {
                    continue; // requires a shift
                }
                Some((Symbol::Nonterminal(nt), remainder)) => (nt, remainder),
            };

            // In that case, for each production of `X`, we are also
            // in a state where the cursor rests at the start of that production:
            //
            // X = (*) a... [lookahead']
            // X = (*) b... [lookahead']
            //
            // Here `lookahead'` is computed based on the `remainder` and our
            // `lookahead`. In LR1 at least, it is the union of:
            //
            //   (a) FIRST(remainder)
            //   (b) if remainder may match epsilon, also our lookahead.
            for new_item in L::epsilon_moves(self, nt, remainder, &lookahead) {
                let new_item0 = new_item.to_lr0();
                if map.push(new_item0, new_item.lookahead) {
                    stack.push(new_item0);
                }
            }
        }

        let final_items = map
            .into_iter()
            .map(|(lr0_item, lookahead)| lr0_item.with_lookahead(lookahead))
            .collect();

        Items { vec: final_items }
    }
}

/// Except for the initial state, the kernel sets always contain
/// a set of "seed" items where something has been pushed (that is,
/// index > 0). In other words, items like this:
///
///    A = ...p (*) ...
///
/// where ...p is non-empty. We now have to expand to include any
/// epsilon moves:
///
///    A = ... (*) B ...
///    B = (*) ...        // added by transitive_closure algorithm
///
/// But note that the state is completely identified by its
/// kernel set: the same kernel sets always expand to the
/// same transitive closures, and different kernel sets
/// always expand to different transitive closures. The
/// first point is obvious, but the latter point follows
/// because the transitive closure algorithm only adds
/// items where `index == 0`, and hence it can never add an
/// item found in a kernel set.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Kernel<'grammar, L: LookaheadBuild> {
    items: Vec<Item<'grammar, L>>,
}

impl<'grammar, L: LookaheadBuild> Kernel<'grammar, L> {
    pub fn start(items: Vec<Item<'grammar, L>>) -> Kernel<'grammar, L> {
        // In start state, kernel should have only items with `index == 0`.
        debug_assert!(items.iter().all(|item| item.index == 0));
        Kernel { items }
    }

    pub fn shifted(items: Vec<Item<'grammar, L>>) -> Kernel<'grammar, L> {
        // Assert that this kernel consists only of shifted items
        // where `index > 0`. This assertion could cost real time to
        // check so only do it in debug mode.
        debug_assert!(items.iter().all(|item| item.index > 0));
        Kernel { items }
    }
}

impl<'grammar, L: LookaheadBuild> kernel_set::Kernel for Kernel<'grammar, L> {
    type Index = StateIndex;

    fn index(c: usize) -> StateIndex {
        StateIndex(c)
    }
}

pub trait LookaheadBuild: Lookahead {
    // Given that there exists an item
    //
    //     X = ... (*) Y ...s [L]
    //
    // where `nt` is `Y`, `remainder` is `...s`, and `lookahead` is
    // `L`, computes the new items resulting from epsilon moves (if
    // any). The technique of doing this will depend on the amount of
    // lookahead.
    //
    // For example, if we have an LR0 item, then for each `Y = ...`
    // production, we just add an `Y = (*) ...` item. But for LR1
    // items, we have to add multiple items where we consider the
    // lookahead from `FIRST(...s, L)`.
    fn epsilon_moves<'grammar>(
        lr: &Lr<'grammar, Self>,
        nt: &NonterminalString,
        remainder: &[Symbol],
        lookahead: &Self,
    ) -> Vec<Item<'grammar, Self>>;
}

impl LookaheadBuild for Nil {
    fn epsilon_moves<'grammar>(
        lr: &Lr<'grammar, Self>,
        nt: &NonterminalString,
        _remainder: &[Symbol],
        lookahead: &Nil,
    ) -> Vec<Lr0Item<'grammar>> {
        lr.items(nt, 0, lookahead)
    }
}

impl LookaheadBuild for TokenSet {
    fn epsilon_moves<'grammar>(
        lr: &Lr<'grammar, Self>,
        nt: &NonterminalString,
        remainder: &[Symbol],
        lookahead: &Self,
    ) -> Vec<Lr1Item<'grammar>> {
        let first_set = lr.first_sets.first1(remainder, lookahead);
        lr.items(nt, 0, &first_set)
    }
}
