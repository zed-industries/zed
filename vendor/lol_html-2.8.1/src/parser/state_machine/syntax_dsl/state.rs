macro_rules! state {
    // defining state with enter actions
    (
        $(#[$meta:meta])*
        $name:ident <-- ( $($enter_actions:tt)* ) {
            $($arms:tt)*
        }

        $($rest:tt)*
    ) => {
        #[allow(unused_variables)]
        $(#[$meta])*
        fn $name(&mut self, context: &mut Self::Context, input: &[u8]) -> StateResult {
            // consume_ch shouldn't be needed here, but the existing states are written to assume an off-by-one position
            let _ = self.consume_ch(input);
            action_list!(|self, context, input|> $($enter_actions)*);
            self.unconsume_ch();

            // use the state machine to remember that enter actions are done
            let entered: fn(&mut Self, &mut Self::Context, &[u8]) -> StateResult = |this, context, input| {
                state_body!(|[this, context, input]|> [$($arms)*]);
            };
            self.set_state(entered);
            return entered(self, context, input);
        }

        state!($($rest)*);
    };
    // defining state without enter actions
    (
        $(#[$meta:meta])*
        $name:ident {
            $($arms:tt)*
        }

        $($rest:tt)*
    ) => {
        #[allow(unused_variables)]
        $(#[$meta])*
        fn $name(&mut self, context: &mut Self::Context, input: &[u8]) -> StateResult {
            state_body!(|[self, context, input]|> [$($arms)*]);
        }

        state!($($rest)*);
    };

    // NOTE: end of the state list
    () => ();
}
