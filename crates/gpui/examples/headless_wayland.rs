#[cfg(target_os = "linux")]
use std::{
    io::{self, BufRead as _},
    sync::mpsc,
    time::Duration,
};

#[cfg(target_os = "linux")]
use gpui::{
    AnyWindowHandle, App, AppContext as _, Context, Entity, QuitMode, Render, Subscription, Window,
    WindowOptions, div, prelude::*,
};

#[cfg(target_os = "linux")]
struct Todo {
    message: String,
}

#[cfg(target_os = "linux")]
struct Todos {
    items: Vec<Entity<Todo>>,
    window: Option<AnyWindowHandle>,
}

#[cfg(target_os = "linux")]
struct TodoWindow {
    todos: Entity<Todos>,
    _todos_subscription: Subscription,
}

#[cfg(target_os = "linux")]
impl Render for TodoWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let rows = self
            .todos
            .read(cx)
            .items
            .iter()
            .enumerate()
            .map(|(index, todo)| div().child(describe_todo(index, todo, cx)))
            .collect::<Vec<_>>();

        div()
            .size_full()
            .bg(gpui::rgb(0xffffff))
            .text_color(gpui::rgb(0x202020))
            .flex()
            .flex_col()
            .gap_2()
            .p_4()
            .child("Headless / Wayland todos")
            .child("Use the terminal: create <message>, ls, close, open, quit")
            .when(rows.is_empty(), |view| {
                view.child("No todos yet. Type `create buy milk` in the terminal.")
            })
            .children(rows)
    }
}

#[cfg(target_os = "linux")]
fn describe_todo(index: usize, todo: &Entity<Todo>, cx: &App) -> String {
    let todo_reference: &Todo = todo.read(cx);
    format!(
        "{index} - {} - {:?} @{:p}",
        todo_reference.message,
        todo.entity_id(),
        todo_reference
    )
}

#[cfg(target_os = "linux")]
fn print_todos(todos: &Entity<Todos>, cx: &App) {
    for (index, todo) in todos.read(cx).items.iter().enumerate() {
        println!("{}", describe_todo(index, todo, cx));
    }
}

#[cfg(target_os = "linux")]
fn open_window(todos: &Entity<Todos>, cx: &mut App) -> anyhow::Result<()> {
    if todos.read(cx).window.is_some() {
        return Ok(());
    }

    let handle = cx.open_window(WindowOptions::default(), |_, cx| {
        cx.new(|cx| TodoWindow {
            todos: todos.clone(),
            _todos_subscription: cx.observe(todos, |_, _, cx| cx.notify()),
        })
    })?;
    todos.update(cx, |todos, _| todos.window = Some(handle.into()));
    Ok(())
}

#[cfg(target_os = "linux")]
fn handle_command(
    command: String,
    todos: &Entity<Todos>,
    cx: &mut App,
) -> anyhow::Result<Option<bool>> {
    if command == "ls" {
        print_todos(todos, cx);
    } else if let Some(message) = command.strip_prefix("create ") {
        let todo = cx.new(|_| Todo {
            message: message.to_owned(),
        });
        todos.update(cx, |todos, cx| {
            todos.items.push(todo);
            println!("todo added, total {}", todos.items.len());
            cx.notify();
        });
    } else if matches!(command.as_str(), "open" | "windowed") {
        return Ok(Some(false));
    } else if matches!(command.as_str(), "close" | "headless") {
        let window = todos.update(cx, |todos, _| todos.window.take());
        if let Some(window) = window {
            window.update(cx, |_, window, _| window.remove_window())?;
        }
        return Ok(Some(true));
    } else if command == "quit" {
        cx.quit();
    } else if !command.is_empty() {
        println!("commands: ls | create <message> | open (windowed) | close (headless) | quit");
    }
    Ok(None)
}

#[cfg(target_os = "linux")]
fn main() {
    gpui_platform::switchable_wayland()
        .with_quit_mode(QuitMode::Explicit)
        .run(|cx| {
            let todos = cx.new(|_| Todos {
                items: Vec::new(),
                window: None,
            });
            let window_closed_subscription = cx.on_window_closed({
                let todos = todos.clone();
                move |cx, window_id| {
                    todos.update(cx, |todos, _| {
                        if todos
                            .window
                            .as_ref()
                            .is_some_and(|window| window.window_id() == window_id)
                        {
                            todos.window = None;
                        }
                    });
                }
            });
            let (command_sender, command_receiver) = mpsc::channel();
            std::thread::spawn(move || {
                for line in io::stdin().lock().lines() {
                    match line {
                        Ok(line) => {
                            if command_sender.send(line).is_err() {
                                break;
                            }
                        }
                        Err(error) => {
                            eprintln!("stdin error: {error}");
                            break;
                        }
                    }
                }
            });

            println!("commands: ls | create <message> | open (windowed) | close (headless) | quit");
            cx.spawn(async move |cx| {
                let _window_closed_subscription = window_closed_subscription;
                loop {
                    cx.background_executor()
                        .timer(Duration::from_millis(25))
                        .await;
                    while let Ok(command) = command_receiver.try_recv() {
                        if let Some(headless) =
                            cx.update(|cx| handle_command(command, &todos, cx))?
                        {
                            let transition = cx.update(|cx| cx.set_headless(headless));
                            transition.await?;
                            if !headless {
                                cx.update(|cx| open_window(&todos, cx))?;
                            }
                        }
                    }
                }
                #[allow(unreachable_code)]
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
        });
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("the headless_wayland example is only supported on Linux");
}
