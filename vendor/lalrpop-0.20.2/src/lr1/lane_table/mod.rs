use crate::collections::Set;
use crate::grammar::repr::*;
use crate::lr1::core::*;
use crate::lr1::lookahead::Lookahead;

mod construct;
mod lane;
mod table;

#[cfg(test)]
mod test;

pub fn build_lane_table_states(grammar: &Grammar, start: NonterminalString) -> Lr1Result<'_> {
    construct::LaneTableConstruct::new(grammar, start).construct()
}

fn conflicting_actions<'grammar, L: Lookahead>(
    state: &State<'grammar, L>,
) -> Set<Action<'grammar>> {
    let conflicts = L::conflicts(state);
    let reductions = conflicts.iter().map(|c| Action::Reduce(c.production));
    let actions = conflicts.iter().map(|c| c.action.clone());
    reductions.chain(actions).collect()
}
