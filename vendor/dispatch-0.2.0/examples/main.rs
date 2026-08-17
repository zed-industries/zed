extern crate dispatch;

use std::io;
use std::process::exit;
use dispatch::{Queue, QueuePriority};

/// Prompts for a number and adds it to the given sum.
///
/// Reading from stdin is done on the given queue.
/// All printing is performed on the main queue.
/// Repeats until the user stops entering numbers.
fn prompt(mut sum: i32, queue: Queue) {
    queue.clone().exec_async(move || {
        let main = Queue::main();
        // Print our prompt on the main thread and wait until it's complete
        main.exec_sync(|| {
            println!("Enter a number:");
        });

        // Read the number the user enters
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if let Ok(num) = input.trim().parse::<i32>() {
            sum += num;
            // Print the sum on the main thread and wait until it's complete
            main.exec_sync(|| {
                println!("Sum is {}\n", sum);
            });
            // Do it again!
            prompt(sum, queue);
        } else {
            // Bail if no number was entered
            main.exec_async(|| {
                println!("Not a number, exiting.");
                exit(0);
            });
        }
    });
}

fn main() {
    // Read from stdin on a background queue so that the main queue is free
    // to handle other events. All printing still occurs through the main
    // queue to avoid jumbled output.
    prompt(0, Queue::global(QueuePriority::Default));

    unsafe {
        dispatch::ffi::dispatch_main();
    }
}
