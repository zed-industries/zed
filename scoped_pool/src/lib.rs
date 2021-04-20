use crossbeam_channel as chan;
use std::{marker::PhantomData, mem::transmute, thread};

#[derive(Clone)]
pub struct Pool {
    req_tx: chan::Sender<Request>,
    thread_count: usize,
}

pub struct Scope<'a> {
    req_count: usize,
    req_tx: chan::Sender<Request>,
    resp_tx: chan::Sender<()>,
    resp_rx: chan::Receiver<()>,
    phantom: PhantomData<&'a ()>,
}

struct Request {
    callback: Box<dyn FnOnce() + Send + 'static>,
    resp_tx: chan::Sender<()>,
}

impl Pool {
    pub fn new(thread_count: usize, name: &str) -> Self {
        let (req_tx, req_rx) = chan::unbounded();
        for i in 0..thread_count {
            thread::Builder::new()
                .name(format!("scoped_pool {} {}", name, i))
                .spawn({
                    let req_rx = req_rx.clone();
                    move || loop {
                        match req_rx.recv() {
                            Err(_) => break,
                            Ok(Request { callback, resp_tx }) => {
                                callback();
                                resp_tx.send(()).ok();
                            }
                        }
                    }
                })
                .expect("scoped_pool: failed to spawn thread");
        }
        Self {
            req_tx,
            thread_count,
        }
    }

    pub fn thread_count(&self) -> usize {
        self.thread_count
    }

    pub fn scoped<'scope, F, R>(&self, scheduler: F) -> R
    where
        F: FnOnce(&mut Scope<'scope>) -> R,
    {
        let (resp_tx, resp_rx) = chan::bounded(1);
        let mut scope = Scope {
            resp_tx,
            resp_rx,
            req_count: 0,
            phantom: PhantomData,
            req_tx: self.req_tx.clone(),
        };
        let result = scheduler(&mut scope);
        scope.wait();
        result
    }
}

impl<'scope> Scope<'scope> {
    pub fn execute<F>(&mut self, callback: F)
    where
        F: FnOnce() + Send + 'scope,
    {
        // Transmute the callback's lifetime to be 'static. This is safe because in ::wait,
        // we block until all the callbacks have been called and dropped.
        let callback = unsafe {
            transmute::<Box<dyn FnOnce() + Send + 'scope>, Box<dyn FnOnce() + Send + 'static>>(
                Box::new(callback),
            )
        };

        self.req_count += 1;
        self.req_tx
            .send(Request {
                callback,
                resp_tx: self.resp_tx.clone(),
            })
            .unwrap();
    }

    fn wait(&self) {
        for _ in 0..self.req_count {
            self.resp_rx.recv().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_execute() {
        let pool = Pool::new(3, "test");

        {
            let vec = Mutex::new(Vec::new());
            pool.scoped(|scope| {
                for _ in 0..3 {
                    scope.execute(|| {
                        for i in 0..5 {
                            vec.lock().unwrap().push(i);
                        }
                    });
                }
            });

            let mut vec = vec.into_inner().unwrap();
            vec.sort_unstable();
            assert_eq!(vec, [0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4])
        }
    }

    #[test]
    fn test_clone_send_and_execute() {
        let pool = Pool::new(3, "test");

        let mut threads = Vec::new();
        for _ in 0..3 {
            threads.push(thread::spawn({
                let pool = pool.clone();
                move || {
                    let vec = Mutex::new(Vec::new());
                    pool.scoped(|scope| {
                        for _ in 0..3 {
                            scope.execute(|| {
                                for i in 0..5 {
                                    vec.lock().unwrap().push(i);
                                }
                            });
                        }
                    });
                    let mut vec = vec.into_inner().unwrap();
                    vec.sort_unstable();
                    assert_eq!(vec, [0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4])
                }
            }));
        }

        for thread in threads {
            thread.join().unwrap();
        }
    }

    #[test]
    fn test_share_and_execute() {
        let pool = Arc::new(Pool::new(3, "test"));

        let mut threads = Vec::new();
        for _ in 0..3 {
            threads.push(thread::spawn({
                let pool = pool.clone();
                move || {
                    let vec = Mutex::new(Vec::new());
                    pool.scoped(|scope| {
                        for _ in 0..3 {
                            scope.execute(|| {
                                for i in 0..5 {
                                    vec.lock().unwrap().push(i);
                                }
                            });
                        }
                    });
                    let mut vec = vec.into_inner().unwrap();
                    vec.sort_unstable();
                    assert_eq!(vec, [0, 0, 0, 1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4])
                }
            }));
        }

        for thread in threads {
            thread.join().unwrap();
        }
    }
}
