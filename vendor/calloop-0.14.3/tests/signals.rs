// These tests cannot run as a regular test because cargo would spawn a thread to run it,
// failing the signal masking. So we make our own, non-threaded harnessing

#[cfg(all(target_os = "linux", feature = "signals"))]
fn main() {
    for test in self::test::TESTS {
        test();
        // reset the signal mask between tests
        self::test::reset_mask();
    }
}

#[cfg(not(all(target_os = "linux", feature = "signals")))]
fn main() {}

#[cfg(all(target_os = "linux", feature = "signals"))]
mod test {
    extern crate calloop;
    extern crate nix;

    use std::time::Duration;

    use self::calloop::signals::{Signal, Signals};
    use self::calloop::{Dispatcher, EventLoop};

    use self::nix::sys::signal::{kill, SigSet};
    use self::nix::unistd::Pid;

    pub const TESTS: &[fn()] = &[single_usr1, usr2_added_afterwards, usr2_signal_removed];

    pub fn reset_mask() {
        SigSet::empty().thread_set_mask().unwrap();
    }

    fn single_usr1() {
        let mut event_loop = EventLoop::try_new().unwrap();

        let mut signal_received = false;

        let _signal_source = event_loop
            .handle()
            .insert_source(
                Signals::new(&[Signal::SIGUSR1]).unwrap(),
                move |evt, &mut (), rcv| {
                    assert!(evt.signal() == Signal::SIGUSR1);
                    *rcv = true;
                },
            )
            .unwrap();

        // send ourselves a SIGUSR1
        kill(Pid::this(), nix::sys::signal::Signal::SIGUSR1).unwrap();

        event_loop
            .dispatch(Some(Duration::from_millis(10)), &mut signal_received)
            .unwrap();

        assert!(signal_received);
    }

    fn usr2_added_afterwards() {
        let mut event_loop = EventLoop::try_new().unwrap();

        let mut signal_received = None;
        let dispatcher = Dispatcher::new(
            Signals::new(&[Signal::SIGUSR1]).unwrap(),
            move |evt, &mut (), rcv| {
                *rcv = Some(evt.signal());
            },
        );

        let _signal_token = event_loop
            .handle()
            .register_dispatcher(dispatcher.clone())
            .unwrap();
        dispatcher
            .as_source_mut()
            .add_signals(&[Signal::SIGUSR2])
            .unwrap();

        // send ourselves a SIGUSR2
        kill(Pid::this(), nix::sys::signal::Signal::SIGUSR2).unwrap();

        event_loop
            .dispatch(Some(Duration::from_millis(10)), &mut signal_received)
            .unwrap();

        assert_eq!(signal_received, Some(Signal::SIGUSR2));
    }

    fn usr2_signal_removed() {
        let mut event_loop = EventLoop::try_new().unwrap();

        let mut signal_received = None;
        let dispatcher = Dispatcher::new(
            Signals::new(&[Signal::SIGUSR1, Signal::SIGUSR2]).unwrap(),
            move |evt, &mut (), rcv| {
                *rcv = Some(evt.signal());
            },
        );

        let _signal_token = event_loop
            .handle()
            .register_dispatcher(dispatcher.clone())
            .unwrap();
        dispatcher
            .as_source_mut()
            .remove_signals(&[Signal::SIGUSR2])
            .unwrap();

        // block sigusr2 anyway, to not be killed by it
        let mut set = SigSet::empty();
        set.add(nix::sys::signal::Signal::SIGUSR2);
        set.thread_block().unwrap();

        // send ourselves a SIGUSR2
        kill(Pid::this(), nix::sys::signal::Signal::SIGUSR2).unwrap();

        event_loop
            .dispatch(Some(Duration::from_millis(10)), &mut signal_received)
            .unwrap();

        // we should not have received anything, as we don't listen to SIGUSR2 any more
        assert!(signal_received.is_none());

        // swap the signals from [SIGUSR1] to [SIGUSR2]
        dispatcher
            .as_source_mut()
            .set_signals(&[Signal::SIGUSR2])
            .unwrap();

        event_loop
            .dispatch(Some(Duration::from_millis(10)), &mut signal_received)
            .unwrap();

        // we should get back the pending SIGUSR2 now
        assert_eq!(signal_received, Some(Signal::SIGUSR2));
    }
}
