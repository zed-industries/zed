macro_rules! state_body {

    // Special version of a body that just skips to the given char
    ( | [ $self:tt, $ctx:tt, $input:ident] |> [ memchr($memchr:literal) => $($arms:tt)+]) => {
        let ch = if $self.consume_until($memchr, $input) { Some(()) } else { None };

        state_body!(@map_arms | [$self, $ctx, $input, ch] |> [ _ /* memchr match */ => $($arms)+], []);
    };

    // Regular byte-by-byte matching body
    ( | [ $self:tt, $ctx:tt, $input:ident] |> [$($arms:tt)+]) => {
        // NOTE: clippy complains about some states that break the loop in each match arm
        #[allow(clippy::never_loop)]
        loop {
            let ch = $self.consume_ch($input);

            state_body!(@map_arms | [$self, $ctx, $input, ch] |> [$($arms)+], []);
        }
    };


    // Recursively expand each arm's pattern
    //--------------------------------------------------------------------
    ( @map_arms
        | $scope_vars:tt |>
        [ $pat:tt => ( $($actions:tt)* ) $($rest:tt)* ], [ $($expanded:tt)* ]
    ) => {
        arm_pattern!(|[ $scope_vars, [$($rest)*], [$($expanded)*] ]|> $pat => ( $($actions)* ))
    };

    ( @map_arms
        | $scope_vars:tt |>
        [], [$($expanded:tt)*]
    ) => {
        state_body!(@match_block |$scope_vars|> $($expanded)*);
    };


    // Callback for the expand_arm_pattern
    //--------------------------------------------------------------------
    ( @callback
        | [ $scope_vars:tt, [$($pending:tt)*], [$($expanded:tt)*] ] |>
        $($expanded_arm:tt)*
    ) => {
        state_body!(@map_arms | $scope_vars |> [$($pending)*], [$($expanded)* $($expanded_arm)*])
    };


    // Character match block
    //--------------------------------------------------------------------
    ( @match_block
        | [ $self:tt, $ctx:tt, $input:ident, $ch:ident ] |>
        $( $pat:pat_param $(|$pat_cont:pat)* $(if $pat_expr:expr)* => ( $($actions:tt)* ) )*
    ) => {
        // NOTE: guard against unreachable patterns
        // (e.g. such may occur if `eof => ...` arm comes before `eoc => ...` arm.)
        #[deny(unreachable_patterns)]
        match $ch {
            $(
                $pat $(| $pat_cont)* $(if $pat_expr)* => {
                    action_list!(|$self, $ctx, $input|> $($actions)*);
                }
            )*
        }
    };
}
