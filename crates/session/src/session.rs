use anyhow::Context as _;
use db::kvp::KeyValueStore;
use gpui::{App, AppContext as _, Context, Subscription, Task, WindowId};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};
use util::ResultExt;

pub struct Session {
    session_id: String,
    old_session_id: Option<String>,
    old_window_ids: Option<Vec<WindowId>>,
    max_window_id: u64,
}

const SESSION_ID_KEY: &str = "session_id";
const SESSION_WINDOW_STACK_KEY: &str = "session_window_stack";

impl Session {
    pub async fn new(
        launch_id: String,
        db: KeyValueStore,
        preserve_previous: bool,
        max_window_id: Option<u64>,
    ) -> anyhow::Result<Self> {
        let old_session_id = db
            .read_kvp(SESSION_ID_KEY)
            .context("reading recovery session ID")?;
        let old_window_ids = db
            .read_kvp(SESSION_WINDOW_STACK_KEY)
            .context("reading recovery window stack")?
            .and_then(|json| serde_json::from_str::<Vec<u64>>(&json).log_err())
            .map(|ids| ids.into_iter().map(WindowId::from).collect::<Vec<_>>());
        let max_window_id = max_window_id
            .into_iter()
            .chain(old_window_ids.iter().flatten().map(WindowId::as_u64))
            .max()
            .unwrap_or(0);
        anyhow::ensure!(max_window_id < u64::MAX, "serialized window IDs exhausted");
        let session_id = if preserve_previous {
            old_session_id.clone().unwrap_or(launch_id)
        } else {
            launch_id
        };

        if old_session_id.as_deref() != Some(session_id.as_str()) {
            db.write_kvp(SESSION_ID_KEY.to_string(), session_id.clone())
                .await
                .context("writing recovery session ID")?;
        }

        Ok(Self {
            session_id,
            old_session_id,
            old_window_ids,
            max_window_id,
        })
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            old_session_id: None,
            old_window_ids: None,
            max_window_id: 0,
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test_with_old_session(old_session_id: String) -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            old_session_id: Some(old_session_id),
            old_window_ids: None,
            max_window_id: 0,
        }
    }

    pub fn id(&self) -> &str {
        &self.session_id
    }
}

pub struct WindowIdReservation {
    window_id: WindowId,
    preferred: bool,
    reservations: Rc<RefCell<HashSet<WindowId>>>,
}

impl WindowIdReservation {
    pub fn window_id(&self) -> WindowId {
        self.window_id
    }

    pub fn is_preferred(&self) -> bool {
        self.preferred
    }
}

impl Drop for WindowIdReservation {
    fn drop(&mut self) {
        self.reservations.borrow_mut().remove(&self.window_id);
    }
}

pub struct AppSession {
    session: Session,
    window_ids: HashMap<WindowId, WindowId>,
    window_id_reservations: Rc<RefCell<HashSet<WindowId>>>,
    pending_window_ids: Vec<WindowId>,
    max_window_id: u64,
    _serialization_task: Task<()>,
    _subscriptions: Vec<Subscription>,
}

impl AppSession {
    pub fn new(session: Session, cx: &Context<Self>) -> Self {
        let session_handle = cx.weak_entity();
        let _subscriptions = vec![
            cx.on_app_quit(Self::app_will_quit),
            cx.on_window_closed(move |cx, window_id| {
                session_handle
                    .update(cx, |session, _| {
                        session.window_ids.remove(&window_id);
                    })
                    .log_err();
            }),
        ];

        let _serialization_task = if cfg!(not(any(test, feature = "test-support"))) {
            let db = KeyValueStore::global(cx);
            cx.spawn(async move |this, cx| {
                // Disabled in tests: the infinite loop bypasses "parking forbidden" checks,
                // causing tests to hang instead of panicking.
                {
                    let mut current_window_stack = Vec::new();
                    loop {
                        let Ok(windows) = this.read_with(cx, |this, cx| this.window_stack(cx))
                        else {
                            return;
                        };
                        if let Some(windows) = windows
                            && !windows.is_empty()
                            && windows != current_window_stack
                        {
                            store_window_stack(db.clone(), &windows).await;
                            current_window_stack = windows;
                        }

                        cx.background_executor()
                            .timer(std::time::Duration::from_millis(500))
                            .await;
                    }
                }
            })
        } else {
            Task::ready(())
        };

        let pending_window_ids = if session.old_session_id.as_deref() == Some(session.id()) {
            session.old_window_ids.clone().unwrap_or_default()
        } else {
            Vec::new()
        };
        Self {
            max_window_id: session.max_window_id,
            session,
            window_ids: HashMap::new(),
            window_id_reservations: Rc::default(),
            pending_window_ids,
            _subscriptions,
            _serialization_task,
        }
    }

    pub fn id(&self) -> &str {
        self.session.id()
    }

    pub fn last_session_id(&self) -> Option<&str> {
        self.session.old_session_id.as_deref()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn replace_session_for_test(&mut self, session: Session) {
        self.max_window_id = self.max_window_id.max(session.max_window_id);
        self.pending_window_ids = if session.old_session_id.as_deref() == Some(session.id()) {
            session
                .old_window_ids
                .iter()
                .flatten()
                .filter(|pending| {
                    !self
                        .window_ids
                        .values()
                        .any(|window_id| window_id == *pending)
                })
                .copied()
                .collect()
        } else {
            Vec::new()
        };
        self.session = session;
    }

    pub fn last_session_window_stack(&self) -> Option<Vec<WindowId>> {
        self.session.old_window_ids.clone()
    }

    pub fn reserve_window_id(
        &mut self,
        preferred_id: Option<WindowId>,
    ) -> anyhow::Result<WindowIdReservation> {
        let window_id = if let Some(preferred_id) = preferred_id.filter(|preferred| {
            !self
                .window_ids
                .values()
                .any(|window_id| window_id == preferred)
                && !self.window_id_reservations.borrow().contains(preferred)
        }) {
            preferred_id
        } else {
            WindowId::from(
                self.max_window_id
                    .checked_add(1)
                    .context("serialized window IDs exhausted")?,
            )
        };
        self.max_window_id = self.max_window_id.max(window_id.as_u64());
        self.window_id_reservations.borrow_mut().insert(window_id);
        Ok(WindowIdReservation {
            window_id,
            preferred: preferred_id == Some(window_id),
            reservations: self.window_id_reservations.clone(),
        })
    }

    pub fn bind_window(
        &mut self,
        runtime_id: WindowId,
        reservation: WindowIdReservation,
    ) -> WindowId {
        assert!(Rc::ptr_eq(
            &self.window_id_reservations,
            &reservation.reservations
        ));
        assert!(!self.window_ids.contains_key(&runtime_id));
        let window_id = reservation.window_id;
        self.pending_window_ids
            .retain(|pending| *pending != window_id);
        self.window_ids.insert(runtime_id, window_id);
        window_id
    }

    fn app_will_quit(&mut self, cx: &mut Context<Self>) -> Task<()> {
        if let Some(window_stack) = self.window_stack(cx)
            && !window_stack.is_empty()
        {
            let db = KeyValueStore::global(cx);
            cx.background_spawn(async move { store_window_stack(db, &window_stack).await })
        } else {
            Task::ready(())
        }
    }

    fn window_stack(&self, cx: &App) -> Option<Vec<u64>> {
        Some(
            self.serialized_window_stack(
                cx.window_stack()?
                    .into_iter()
                    .map(|window| window.window_id()),
            ),
        )
    }

    fn serialized_window_stack(&self, windows: impl IntoIterator<Item = WindowId>) -> Vec<u64> {
        let windows = windows
            .into_iter()
            .filter_map(|window| self.window_ids.get(&window))
            .map(WindowId::as_u64)
            .collect::<Vec<_>>();
        if self.pending_window_ids.is_empty() {
            return windows;
        }

        let current_window_ids = windows.iter().copied().collect::<HashSet<_>>();
        let mut available_window_ids = current_window_ids.clone();
        available_window_ids.extend(self.pending_window_ids.iter().map(WindowId::as_u64));
        let saved_windows = self
            .session
            .old_window_ids
            .iter()
            .flatten()
            .map(WindowId::as_u64)
            .filter(|window_id| available_window_ids.remove(window_id))
            .collect::<Vec<_>>();
        let saved_window_ids = saved_windows.iter().copied().collect::<HashSet<_>>();
        let mut saved_windows = saved_windows.into_iter();
        let mut stack = Vec::new();
        for window_id in windows {
            if saved_window_ids.contains(&window_id) {
                for saved_window_id in saved_windows.by_ref() {
                    stack.push(saved_window_id);
                    if current_window_ids.contains(&saved_window_id) {
                        break;
                    }
                }
            } else {
                stack.push(window_id);
            }
        }
        stack.extend(saved_windows);
        stack
    }
}

async fn store_window_stack(db: KeyValueStore, windows: &[u64]) {
    if let Ok(window_ids_json) = serde_json::to_string(windows) {
        db.write_kvp(SESSION_WINDOW_STACK_KEY.to_string(), window_ids_json)
            .await
            .log_err();
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AppSession, SESSION_ID_KEY, SESSION_WINDOW_STACK_KEY, Session, store_window_stack,
    };
    use db::kvp::KeyValueStore;
    use gpui::{AppContext as _, Empty, TestAppContext, WindowId};

    #[gpui::test]
    async fn test_recovery_session_survives_interrupted_startup(cx: &mut TestAppContext) {
        for (restored_count, database_name) in
            [(0, "interrupted-startup-0"), (1, "interrupted-startup-1")]
        {
            let saved_windows = [4_294_967_297, 4_294_967_298];
            let db = session_database(database_name, &saved_windows).await;

            let interrupted = Session::new(
                String::from("interrupted"),
                db.clone(),
                true,
                Some(saved_windows[1]),
            )
            .await
            .expect("failed to resume session");
            let session = cx.new(|cx| AppSession::new(interrupted, cx));
            let stack = session.update(cx, |session, _| {
                for index in 0..restored_count {
                    let reservation = session
                        .reserve_window_id(Some(WindowId::from(saved_windows[1 - index])))
                        .expect("reserve restored window ID");
                    assert_eq!(
                        session.bind_window(WindowId::from(saved_windows[index]), reservation),
                        WindowId::from(saved_windows[1 - index]),
                    );
                }
                session.serialized_window_stack(
                    saved_windows
                        .iter()
                        .take(restored_count)
                        .copied()
                        .map(WindowId::from),
                )
            });
            assert_eq!(stack, vec![4_294_967_297, 4_294_967_298]);
            store_window_stack(db.clone(), &stack).await;
            drop(session);

            let resumed = Session::new(
                String::from("resumed"),
                db.clone(),
                true,
                Some(saved_windows[1]),
            )
            .await
            .expect("failed to resume interrupted session");
            assert_eq!(resumed.id(), "original");
            assert_eq!(resumed.old_session_id.as_deref(), Some("original"));
            assert_eq!(
                resumed.old_window_ids,
                Some(vec![
                    WindowId::from(4_294_967_297),
                    WindowId::from(4_294_967_298),
                ]),
            );
            assert_eq!(
                db.read_kvp(SESSION_ID_KEY)
                    .expect("failed to read session ID"),
                Some(String::from("original")),
            );
        }
    }

    #[gpui::test]
    async fn test_explicit_new_session_preserves_window_stack(cx: &mut TestAppContext) {
        let db = session_database("explicit-new-session", &[4_294_967_300]).await;
        let session = Session::new(String::from("new"), db.clone(), false, None)
            .await
            .expect("failed to start new session");
        assert_eq!(session.id(), "new");
        assert_eq!(session.old_session_id.as_deref(), Some("original"));
        assert_eq!(session.max_window_id, 4_294_967_300);
        assert_eq!(
            db.read_kvp(SESSION_ID_KEY)
                .expect("failed to read session ID"),
            Some(String::from("new")),
        );
        assert_eq!(
            db.read_kvp(SESSION_WINDOW_STACK_KEY)
                .expect("failed to read window stack"),
            Some(String::from("[4294967300]")),
        );
        let session = cx.new(|cx| AppSession::new(session, cx));
        session.update(cx, |session, _| {
            assert_eq!(session.serialized_window_stack([]), Vec::<u64>::new());
            let reservation = session.reserve_window_id(None).expect("reserve fresh ID");
            assert_eq!(
                session.bind_window(WindowId::from(4_294_967_297), reservation),
                WindowId::from(4_294_967_301),
            );
        });
    }

    #[gpui::test]
    fn test_restored_window_mapping_and_fresh_ids(cx: &mut TestAppContext) {
        let session = cx.new(|cx| AppSession::new(Session::test(), cx));
        session.update(cx, |session, _| {
            let first = WindowId::from(4_294_967_297);
            let second = WindowId::from(4_294_967_298);
            let third = WindowId::from(4_294_967_299);
            let reservation = session
                .reserve_window_id(Some(second))
                .expect("reserve first restored ID");
            assert_eq!(
                session.bind_window(first, reservation),
                WindowId::from(4_294_967_298)
            );
            let reservation = session
                .reserve_window_id(Some(first))
                .expect("reserve second restored ID");
            assert_eq!(
                session.bind_window(second, reservation),
                WindowId::from(4_294_967_297)
            );
            assert_eq!(
                session.serialized_window_stack([first, second, third]),
                vec![4_294_967_298, 4_294_967_297],
            );
            let reservation = session.reserve_window_id(None).expect("reserve fresh ID");
            assert_eq!(
                session.bind_window(third, reservation),
                WindowId::from(4_294_967_299)
            );
            assert_eq!(session.serialized_window_stack([]), Vec::<u64>::new());
        });
    }

    #[gpui::test]
    fn test_window_id_reservations_exclude_overlapping_preferred_ids(cx: &mut TestAppContext) {
        for preferred_id in [
            4_294_967_296,
            4_294_967_297,
            0x8000_0001_0000_0000,
            u64::MAX - 2,
        ] {
            for bind_preferred_first in [false, true] {
                let session = cx.new(|cx| AppSession::new(Session::test(), cx));
                session.update(cx, |session, _| {
                    let preferred = WindowId::from(preferred_id);
                    let first = session
                        .reserve_window_id(Some(preferred))
                        .expect("reserve preferred ID");
                    let second = session
                        .reserve_window_id(Some(preferred))
                        .expect("reserve while preferred ID is held");
                    assert_eq!(first.window_id(), preferred);
                    assert_eq!(first.window_id().as_u64(), preferred_id);
                    assert!(first.is_preferred());
                    assert_eq!(second.window_id().as_u64(), preferred_id + 1);
                    assert!(!second.is_preferred());
                    assert_eq!(session.serialized_window_stack([]), Vec::<u64>::new());
                    let first_runtime = WindowId::from(100);
                    let second_runtime = WindowId::from(101);
                    if bind_preferred_first {
                        assert_eq!(session.bind_window(first_runtime, first), preferred);
                        assert_eq!(
                            session.bind_window(second_runtime, second).as_u64(),
                            preferred_id + 1,
                        );
                    } else {
                        assert_eq!(
                            session.bind_window(second_runtime, second).as_u64(),
                            preferred_id + 1,
                        );
                        assert_eq!(session.bind_window(first_runtime, first), preferred);
                    }
                    assert!(session.window_id_reservations.borrow().is_empty());
                    let third = session
                        .reserve_window_id(Some(preferred))
                        .expect("reserve while preferred ID is bound");
                    assert_eq!(third.window_id().as_u64(), preferred_id + 2);
                    assert!(!third.is_preferred());
                    let third_runtime = WindowId::from(102);
                    assert_eq!(
                        session.bind_window(third_runtime, third).as_u64(),
                        preferred_id + 2,
                    );
                    assert_eq!(
                        session.serialized_window_stack([
                            first_runtime,
                            second_runtime,
                            third_runtime,
                        ]),
                        vec![preferred_id, preferred_id + 1, preferred_id + 2],
                    );
                });
            }
        }
    }

    #[gpui::test]
    fn test_window_id_reservation_cancellation_allows_reuse(cx: &mut TestAppContext) {
        for preferred_id in [None, Some(WindowId::from(4_294_967_303))] {
            let session = cx.new(|cx| {
                let mut session = Session::test();
                session.old_session_id = Some(session.session_id.clone());
                session.old_window_ids = Some(vec![WindowId::from(4_294_967_303)]);
                session.max_window_id = 4_294_967_303;
                AppSession::new(session, cx)
            });
            session.update(cx, |session, _| {
                let expected = preferred_id.unwrap_or(WindowId::from(4_294_967_304));
                let reservation = session
                    .reserve_window_id(preferred_id)
                    .expect("reserve cancellable ID");
                assert_eq!(reservation.window_id(), expected);
                assert_eq!(session.serialized_window_stack([]), vec![4_294_967_303]);
                drop(reservation);
                assert!(session.window_id_reservations.borrow().is_empty());
                assert!(session.window_ids.is_empty());
                assert_eq!(
                    session.max_window_id,
                    if preferred_id.is_some() {
                        4_294_967_303
                    } else {
                        4_294_967_304
                    },
                );
                assert_eq!(
                    session.pending_window_ids,
                    vec![WindowId::from(4_294_967_303)]
                );
                let reused = session
                    .reserve_window_id(Some(expected))
                    .expect("reuse cancelled reservation");
                assert_eq!(reused.window_id(), expected);
                assert!(reused.is_preferred());
                assert_eq!(session.bind_window(WindowId::from(1), reused), expected);
                assert!(session.window_id_reservations.borrow().is_empty());
                assert_eq!(
                    session.serialized_window_stack([WindowId::from(1)]),
                    if preferred_id.is_some() {
                        vec![4_294_967_303]
                    } else {
                        vec![4_294_967_304, 4_294_967_303]
                    },
                );
            });
        }
    }

    #[gpui::test]
    fn test_binding_window_id_reservation_consumes_one_fresh_id(cx: &mut TestAppContext) {
        let session = cx.new(|cx| {
            let mut session = Session::test();
            session.max_window_id = 4_294_967_337;
            AppSession::new(session, cx)
        });
        let first = session.update(cx, |session, _| {
            session.reserve_window_id(None).expect("reserve first ID")
        });
        assert_eq!(first.window_id().as_u64(), 4_294_967_338);
        assert!(!first.is_preferred());
        let first_window = cx.add_window(|window, cx| {
            session.update(cx, |session, _| {
                assert_eq!(
                    session
                        .bind_window(window.window_handle().window_id(), first)
                        .as_u64(),
                    4_294_967_338,
                );
                assert_eq!(session.max_window_id, 4_294_967_338);
            });
            Empty
        });
        let second = session.update(cx, |session, _| {
            session.reserve_window_id(None).expect("reserve second ID")
        });
        assert_eq!(second.window_id().as_u64(), 4_294_967_339);
        let second_window = cx.add_window(|window, cx| {
            session.update(cx, |session, _| {
                assert_eq!(
                    session
                        .bind_window(window.window_handle().window_id(), second)
                        .as_u64(),
                    4_294_967_339,
                );
                assert_eq!(session.max_window_id, 4_294_967_339);
            });
            Empty
        });
        assert_eq!(
            session.read_with(cx, |session, _| {
                session
                    .serialized_window_stack([first_window.window_id(), second_window.window_id()])
            }),
            vec![4_294_967_338, 4_294_967_339],
        );
    }

    #[gpui::test]
    fn test_window_id_reservation_exhaustion_preserves_state(cx: &mut TestAppContext) {
        let session = cx.new(|cx| {
            let mut session = Session::test();
            session.old_session_id = Some(session.session_id.clone());
            session.old_window_ids = Some(vec![WindowId::from(4_294_967_303)]);
            session.max_window_id = u64::MAX - 1;
            AppSession::new(session, cx)
        });
        session.update(cx, |session, _| {
            let reservation = session
                .reserve_window_id(None)
                .expect("reserve final fresh ID");
            assert_eq!(reservation.window_id(), WindowId::from(u64::MAX));
            let reservations = session.window_id_reservations.borrow().clone();
            for preferred_id in [None, Some(WindowId::from(u64::MAX))] {
                let error = session
                    .reserve_window_id(preferred_id)
                    .err()
                    .expect("overlapping reservation must report exhaustion");
                assert_eq!(error.to_string(), "serialized window IDs exhausted");
                assert_eq!(*session.window_id_reservations.borrow(), reservations);
                assert!(session.window_ids.is_empty());
                assert_eq!(
                    session.pending_window_ids,
                    vec![WindowId::from(4_294_967_303)]
                );
                assert_eq!(session.max_window_id, u64::MAX);
            }
            drop(reservation);
            let reused = session
                .reserve_window_id(Some(WindowId::from(u64::MAX)))
                .expect("reuse cancelled final ID");
            assert!(reused.is_preferred());
            assert_eq!(
                session.bind_window(WindowId::from(4_294_967_297), reused),
                WindowId::from(u64::MAX),
            );
            for preferred_id in [None, Some(WindowId::from(u64::MAX))] {
                let error = session
                    .reserve_window_id(preferred_id)
                    .err()
                    .expect("bound final ID must report exhaustion");
                assert_eq!(error.to_string(), "serialized window IDs exhausted");
                assert!(session.window_id_reservations.borrow().is_empty());
                assert_eq!(session.max_window_id, u64::MAX);
                assert_eq!(session.window_ids.len(), 1);
                assert_eq!(
                    session.pending_window_ids,
                    vec![WindowId::from(4_294_967_303)]
                );
                assert_eq!(
                    session.serialized_window_stack([WindowId::from(4_294_967_297)]),
                    vec![u64::MAX, 4_294_967_303],
                );
            }
            let restored = session
                .reserve_window_id(Some(WindowId::from(4_294_967_303)))
                .expect("reserve unused preferred ID after exhaustion");
            assert_eq!(
                session.bind_window(WindowId::from(4_294_967_298), restored),
                WindowId::from(4_294_967_303),
            );
            assert_eq!(session.pending_window_ids, Vec::<WindowId>::new());
            assert_eq!(
                session.serialized_window_stack([
                    WindowId::from(4_294_967_297),
                    WindowId::from(4_294_967_298),
                ]),
                vec![u64::MAX, 4_294_967_303],
            );
        });
    }

    #[gpui::test]
    fn test_window_close_releases_ids_and_keeps_pending_windows(cx: &mut TestAppContext) {
        let saved = [
            WindowId::from(4_294_967_297),
            WindowId::from(4_294_967_298),
            WindowId::from(4_294_967_299),
        ];
        let session = cx.new(|cx| {
            let mut session = Session::test();
            session.old_session_id = Some(session.session_id.clone());
            session.old_window_ids = Some(saved.to_vec());
            session.max_window_id = saved[2].as_u64();
            AppSession::new(session, cx)
        });
        let [first, second] = [saved[2], saved[0]].map(|preferred_id| {
            let reservation = session.update(cx, |session, _| {
                session
                    .reserve_window_id(Some(preferred_id))
                    .expect("reserve restored ID")
            });
            cx.add_window(|window, cx| {
                session.update(cx, |session, _| {
                    session.bind_window(window.window_handle().window_id(), reservation)
                });
                Empty
            })
        });
        assert_eq!(
            session.read_with(cx, |session, _| {
                session.serialized_window_stack([first.window_id(), second.window_id()])
            }),
            vec![4_294_967_297, 4_294_967_298, 4_294_967_299],
        );
        first
            .update(cx, |_, window, _| window.remove_window())
            .expect("failed to close restored window");
        cx.run_until_parked();
        assert_eq!(
            session.read_with(cx, |session, _| session.window_ids.len()),
            1
        );
        assert_eq!(
            session.read_with(cx, |session, _| {
                session.serialized_window_stack([second.window_id()])
            }),
            vec![4_294_967_297, 4_294_967_298],
        );
        let reservation = session.update(cx, |session, _| {
            session
                .reserve_window_id(Some(saved[1]))
                .expect("reserve pending ID")
        });
        let third = cx.add_window(|window, cx| {
            session.update(cx, |session, _| {
                session.bind_window(window.window_handle().window_id(), reservation)
            });
            Empty
        });
        session.read_with(cx, |session, _| {
            assert_eq!(session.pending_window_ids, Vec::<WindowId>::new());
            assert_eq!(
                session.serialized_window_stack([third.window_id(), second.window_id()]),
                vec![4_294_967_298, 4_294_967_297],
            );
        });
        let reservation = session.update(cx, |session, _| {
            session
                .reserve_window_id(Some(saved[2]))
                .expect("reserve closed window ID")
        });
        cx.add_window(|window, cx| {
            session.update(cx, |session, _| {
                assert_eq!(
                    session.bind_window(window.window_handle().window_id(), reservation),
                    WindowId::from(4_294_967_299),
                );
            });
            Empty
        });
    }

    #[gpui::test]
    fn test_incomplete_restoration_merges_current_and_pending_windows(cx: &mut TestAppContext) {
        let session = cx.new(|cx| {
            let mut session = Session::test();
            session.old_session_id = Some(session.session_id.clone());
            session.old_window_ids = Some(vec![
                WindowId::from(4_294_967_307),
                WindowId::from(4_294_967_318),
                WindowId::from(4_294_967_329),
            ]);
            session.max_window_id = 4_294_967_329;
            AppSession::new(session, cx)
        });
        session.update(cx, |session, _| {
            let first = WindowId::from(4_294_967_297);
            let second = WindowId::from(4_294_967_298);
            let third = WindowId::from(4_294_967_299);
            let fourth = WindowId::from(4_294_967_300);
            let unknown = WindowId::from(4_294_967_301);
            for (runtime_id, preferred_id) in [
                (first, Some(4_294_967_329)),
                (second, Some(4_294_967_307)),
                (third, None),
                (fourth, None),
            ] {
                let reservation = session
                    .reserve_window_id(preferred_id.map(WindowId::from))
                    .expect("reserve window ID");
                session.bind_window(runtime_id, reservation);
            }
            for (current, expected) in [
                (Vec::new(), vec![4_294_967_318]),
                (vec![first], vec![4_294_967_318, 4_294_967_329]),
                (vec![second], vec![4_294_967_307, 4_294_967_318]),
                (
                    vec![first, second],
                    vec![4_294_967_307, 4_294_967_318, 4_294_967_329],
                ),
                (
                    vec![second, first],
                    vec![4_294_967_307, 4_294_967_318, 4_294_967_329],
                ),
                (
                    vec![third, first, fourth, second, unknown],
                    vec![
                        4_294_967_330,
                        4_294_967_307,
                        4_294_967_331,
                        4_294_967_318,
                        4_294_967_329,
                    ],
                ),
                (
                    vec![first, third, second, fourth],
                    vec![
                        4_294_967_307,
                        4_294_967_330,
                        4_294_967_318,
                        4_294_967_329,
                        4_294_967_331,
                    ],
                ),
                (
                    vec![first, third],
                    vec![4_294_967_318, 4_294_967_329, 4_294_967_330],
                ),
                (
                    vec![third, fourth],
                    vec![4_294_967_330, 4_294_967_331, 4_294_967_318],
                ),
                (vec![unknown], vec![4_294_967_318]),
            ] {
                assert_eq!(
                    session.serialized_window_stack(current.iter().copied()),
                    expected,
                    "current windows: {current:?}",
                );
            }
            let reservation = session
                .reserve_window_id(Some(WindowId::from(4_294_967_318)))
                .expect("reserve final pending ID");
            session.bind_window(unknown, reservation);
            assert_eq!(
                session.serialized_window_stack([first, second, unknown, third]),
                vec![4_294_967_329, 4_294_967_307, 4_294_967_318, 4_294_967_330],
            );
        });
    }

    #[gpui::test]
    async fn test_high_bit_window_ids_seed_allocator(cx: &mut TestAppContext) {
        for (from_stack, database_name) in [
            (false, "high-bit-window-ids-database"),
            (true, "high-bit-window-ids-stack"),
        ] {
            let db = session_database(
                database_name,
                if from_stack {
                    &[0x8000_0001_0000_0000]
                } else {
                    &[]
                },
            )
            .await;
            let session = Session::new(
                String::from("resumed"),
                db,
                true,
                (!from_stack).then_some(0x8000_0001_0000_0000),
            )
            .await
            .expect("resume high-bit session");
            let session = cx.new(|cx| AppSession::new(session, cx));
            session.update(cx, |session, _| {
                let reservation = session
                    .reserve_window_id(None)
                    .expect("reserve after high-bit ID");
                assert_eq!(
                    session.bind_window(WindowId::from(4_294_967_297), reservation),
                    WindowId::from(0x8000_0001_0000_0001),
                );
            });
        }
    }

    #[gpui::test]
    async fn test_exhausted_window_ids_do_not_publish_session() {
        for (from_stack, database_name) in [
            (false, "exhausted-window-ids-false"),
            (true, "exhausted-window-ids-true"),
        ] {
            let db =
                session_database(database_name, if from_stack { &[u64::MAX] } else { &[] }).await;
            let error = Session::new(
                String::from("new"),
                db.clone(),
                false,
                (!from_stack).then_some(u64::MAX),
            )
            .await
            .err()
            .expect("exhausted window IDs should fail startup");
            assert_eq!(error.to_string(), "serialized window IDs exhausted");
            assert_eq!(
                db.read_kvp(SESSION_ID_KEY)
                    .expect("failed to read session ID"),
                Some(String::from("original")),
            );
        }
    }

    #[gpui::test]
    async fn test_session_pointer_errors_propagate() {
        let db = KeyValueStore::open_test_db("session-pointer-errors").await;
        db.write(|connection| {
            connection.exec(
                "CREATE TRIGGER reject_session BEFORE INSERT ON kv_store
                 WHEN NEW.key = 'session_id'
                 BEGIN SELECT RAISE(FAIL, 'session write rejected'); END",
            )?()
        })
        .await
        .expect("failed to install write failure");
        let error = Session::new(String::from("new"), db.clone(), true, None)
            .await
            .err()
            .expect("session pointer write should fail");
        assert_eq!(error.to_string(), "writing recovery session ID");
        assert_eq!(
            db.read_kvp(SESSION_ID_KEY)
                .expect("failed to read session ID"),
            None
        );

        db.write(|connection| connection.exec("DROP TABLE kv_store")?())
            .await
            .expect("failed to install read failure");
        let error = Session::new(String::from("new"), db, true, None)
            .await
            .err()
            .expect("session pointer read should fail");
        assert_eq!(error.to_string(), "reading recovery session ID");
    }

    async fn session_database(name: &'static str, windows: &[u64]) -> KeyValueStore {
        let db = KeyValueStore::open_test_db(name).await;
        let session = Session::new(String::from("original"), db.clone(), true, None)
            .await
            .expect("failed to create original session");
        assert_eq!(session.id(), "original");
        assert_eq!(session.old_session_id, None);
        store_window_stack(db.clone(), windows).await;
        db
    }
}
