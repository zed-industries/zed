use merman_core::diagrams::sequence::SequenceMessage;
use std::collections::BTreeMap;

pub(crate) fn sequence_activation_start_x(center_x: f64, stacked_size: usize, width: f64) -> f64 {
    center_x + (((stacked_size as f64) - 1.0) * width) / 2.0
}

pub(crate) fn sequence_activation_stack_bounds(
    starts: impl IntoIterator<Item = f64>,
    center_x: f64,
    width: f64,
) -> (f64, f64) {
    starts
        .into_iter()
        .fold((center_x - 1.0, center_x + 1.0), |(left, right), x| {
            (left.min(x), right.max(x + width))
        })
}

pub(super) struct SequenceActivationState<'a> {
    width: f64,
    stacks: BTreeMap<&'a str, Vec<f64>>,
}

impl<'a> SequenceActivationState<'a> {
    pub(super) fn new(width: f64) -> Self {
        Self {
            width,
            stacks: BTreeMap::new(),
        }
    }

    pub(super) fn handle_directive(
        &mut self,
        msg: &'a SequenceMessage,
        actor_index: &std::collections::HashMap<&'a str, usize>,
        actor_centers_x: &[f64],
    ) -> bool {
        match msg.message_type {
            // ACTIVE_START
            17 => {
                let Some(actor_id) = msg.from.as_deref() else {
                    return true;
                };
                let Some(&idx) = actor_index.get(actor_id) else {
                    return true;
                };
                let cx = actor_centers_x[idx];
                let stack = self.stacks.entry(actor_id).or_default();
                let stacked_size = stack.len();
                let startx = sequence_activation_start_x(cx, stacked_size, self.width);
                stack.push(startx);
                true
            }
            // ACTIVE_END
            18 => {
                let Some(actor_id) = msg.from.as_deref() else {
                    return true;
                };
                if let Some(stack) = self.stacks.get_mut(actor_id) {
                    let _ = stack.pop();
                }
                true
            }
            _ => false,
        }
    }

    pub(super) fn actor_bounds(&self, actor_id: &str, center_x: f64) -> (f64, f64) {
        sequence_activation_stack_bounds(
            self.stacks
                .get(actor_id)
                .into_iter()
                .flat_map(|stack| stack.iter().copied()),
            center_x,
            self.width,
        )
    }

    pub(super) fn width(&self) -> f64 {
        self.width
    }
}

#[cfg(test)]
mod tests {
    use super::{sequence_activation_stack_bounds, sequence_activation_start_x};

    #[test]
    fn activation_start_x_matches_mermaid_stack_offsets() {
        assert_eq!(sequence_activation_start_x(100.0, 0, 10.0), 95.0);
        assert_eq!(sequence_activation_start_x(100.0, 1, 10.0), 100.0);
        assert_eq!(sequence_activation_start_x(100.0, 2, 10.0), 105.0);
    }

    #[test]
    fn activation_stack_bounds_fold_full_active_stack() {
        assert_eq!(
            sequence_activation_stack_bounds(std::iter::empty::<f64>(), 100.0, 10.0),
            (99.0, 101.0)
        );
        assert_eq!(
            sequence_activation_stack_bounds([95.0], 100.0, 10.0),
            (95.0, 105.0)
        );
        assert_eq!(
            sequence_activation_stack_bounds([95.0, 100.0], 100.0, 10.0),
            (95.0, 110.0)
        );
    }
}
