use generator::*;

#[derive(Debug)]
enum Action {
    Play(&'static str),
    Stop,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Playing,
    Stopped,
}

use crate::Action::*;
use crate::State::*;

fn main() {
    let mut cd_player = Gn::new_scoped(|mut s| {
        let mut state = Stopped;
        loop {
            // println!("{:?}", *state);
            // in release mod without this there is bugs!!!!! (rustc 1.59.0 (9d1b2106e 2022-02-23))
            std::sync::atomic::compiler_fence(std::sync::atomic::Ordering::AcqRel);

            match state {
                Stopped => match s.get_yield() {
                    Some(Play(t)) => {
                        println!("I'm playing {t}");
                        state = Playing;
                    }
                    Some(Stop) => println!("I'm already stopped"),
                    _ => unreachable!("some thing wrong"),
                },

                Playing => match s.get_yield() {
                    Some(Stop) => {
                        println!("I'm stopped");
                        state = Stopped;
                    }
                    Some(Play(_)) => println!("should first stop"),
                    _ => unreachable!("some thing wrong"),
                },
            }

            s.yield_with(state);
        }
    });

    for _ in 0..1000 {
        let ret = cd_player.send(Play("hello world"));
        assert_eq!(ret, Playing);
        let ret = cd_player.send(Play("hello another day"));
        assert_eq!(ret, Playing);
        let ret = cd_player.send(Stop);
        assert_eq!(ret, Stopped);
        let ret = cd_player.send(Stop);
        assert_eq!(ret, Stopped);
        let ret = cd_player.send(Play("hello another day"));
        assert_eq!(ret, Playing);
        let ret = cd_player.send(Stop);
        assert_eq!(ret, Stopped);
    }
}
