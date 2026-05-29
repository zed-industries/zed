use gpui::{AsyncApp, Task};
use gpui_platform::application;

fn main() {
    application().run(|cx| {
        cx.spawn(keep_executor_going).detach();
    });
}

async fn keep_executor_going(cx: &mut AsyncApp) {
    // keep the executor nice and busy
    let mut in_progress = Vec::new();

    loop {
        in_progress.retain(|task: &Task<()>| !task.is_ready());
        while in_progress.len() < 10 {
            in_progress.push(cx.spawn(mimic_useful_work));
            futures_lite::future::yield_now().await;
        }

        futures_lite::future::yield_now().await;
    }
}

async fn mimic_useful_work(_cx: &mut AsyncApp) {
    let input = core::hint::black_box(10);
    let output = fibonacci().nth(input).unwrap();
    core::hint::black_box(output);
}

// take from rust by example:
struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.curr;

        self.curr = self.next;
        self.next = current + self.next;

        Some(current)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci { curr: 0, next: 1 }
}
