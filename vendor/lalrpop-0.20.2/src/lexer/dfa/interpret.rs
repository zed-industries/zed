use crate::lexer::dfa::{Dfa, Kind, NfaIndex, START};

pub fn interpret<'text>(dfa: &Dfa, input: &'text str) -> Option<(NfaIndex, &'text str)> {
    let mut longest: Option<(NfaIndex, usize)> = None;
    let mut state_index = START;

    for (offset, ch) in input.char_indices() {
        let state = &dfa.states[state_index.0];

        let target = dfa
            .state(state_index)
            .test_edges
            .iter()
            .filter_map(|(test, target)| {
                if test.contains_char(ch) {
                    Some(target)
                } else {
                    None
                }
            })
            .next();

        if let Some(target) = target {
            state_index = *target;
        } else {
            state_index = state.other_edge;
        }

        match dfa.state(state_index).kind {
            Kind::Accepts(nfa) => {
                longest = Some((nfa, offset + ch.len_utf8()));
            }
            Kind::Reject => {
                break;
            }
            Kind::Neither => {}
        }
    }

    longest.map(|(index, offset)| (index, &input[..offset]))
}
