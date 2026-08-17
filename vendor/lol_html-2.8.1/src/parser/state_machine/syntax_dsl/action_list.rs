macro_rules! action_list {
    ( | $self:tt, $ctx:tt, $input:ident |>
        if $cond:ident
            ( $($if_actions:tt)* )
        else
            ( $($else_actions:tt)* )
    ) => {
        if $self.$cond() {
            action_list!(| $self, $ctx, $input |> $($if_actions)*);
        } else {
            action_list!(| $self, $ctx, $input |> $($else_actions)*);
        }
    };

    ( | $self:tt, $ctx:tt, $input:ident |> { $($code_block:tt)* } ) => ( $($code_block)* );

    ( | $self:tt, $ctx:tt, $input:ident |> $action:ident $($args:expr),*; $($rest:tt)* ) => {
        trace!(@actions $action $($args:expr)*);
        action!(| $self, $ctx, $input |> $action $($args),*);
        action_list!(| $self, $ctx, $input |> $($rest)*);
    };

     ( | $self:tt, $ctx:tt, $input:ident |> $action:ident ? $($args:expr),*; $($rest:tt)* ) => {
        trace!(@actions $action $($args:expr)*);
        action!(| $self, $ctx, $input |> $action ? $($args),*);
        action_list!(| $self, $ctx, $input |> $($rest)*);
    };

    // NOTE: state transition should always be in the end of the action list
    ( | $self:tt, $ctx:tt, $input:ident|> $($transition:tt)+ ) => {
        trace!(@actions $($transition)+);
        action!(@state_transition | $self, $ctx, $input |> $($transition)+);
    };

    // NOTE: end of the action list
    ( | $self:tt, $ctx:tt, $input:ident |> ) => ();
}
