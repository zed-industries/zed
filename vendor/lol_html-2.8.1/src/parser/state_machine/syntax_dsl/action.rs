macro_rules! action {
    (| $self:tt, $ctx:tt, $input:ident | > $action_fn:ident ? $($args:expr),* ) => {
        $self.$action_fn($ctx, $input $(,$args),*)?;
    };

    (| $self:tt, $ctx:tt, $input:ident | > $action_fn:ident $($args:expr),* ) => {
        $self.$action_fn($ctx, $input $(,$args),*);
    };

    ( @state_transition | $self:tt, $ctx:tt, $input:ident | > reconsume in $state:ident) => {
        $self.unconsume_ch();
        action!(@state_transition | $self, $ctx, $input | > --> $state);
    };

    ( @state_transition | $self:tt, $ctx:tt, $input:ident | > - -> $state:ident) => {
        $self.set_state(Self::$state);
        return Ok(());
    };

    ( @state_transition | $self:tt, $ctx:tt, $input:ident | > - -> #[inline] $state:ident) => {
        // Jumps directly to the state function, which allows easy inlining,
        // but the calls using #[inline] must never create a loop
        $self.set_state(Self::$state);
        return Self::$state($self, $ctx, $input);
    };

    ( @state_transition | $self:tt, $ctx:tt, $input:ident | > - -> dyn $state_getter:ident) => {
        {
            let state = $self.$state_getter();
            $self.set_state(state);
        }

        return Ok(());
    };
}
