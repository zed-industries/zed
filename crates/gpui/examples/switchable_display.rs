#[cfg(target_os = "linux")]
use std::{
    io::{self, BufRead as _},
    sync::mpsc,
    time::Duration,
};

#[cfg(target_os = "linux")]
use gpui::{
    AnyWindowHandle, App, AppContext as _, AsyncApp, Context, DisplayEnvironment, DisplayMode,
    Entity, QuitMode, Render, Subscription, TitlebarOptions, Window, WindowOptions, div,
    prelude::*,
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
            .child("Switchable display todos")
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

    let options = WindowOptions {
        titlebar: Some(TitlebarOptions {
            title: Some("Switchable display todos".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    let handle = cx.open_window(options, |_, cx| {
        cx.new(|cx| TodoWindow {
            todos: todos.clone(),
            _todos_subscription: cx.observe(todos, |_, _, cx| cx.notify()),
        })
    })?;
    todos.update(cx, |todos, _| todos.window = Some(handle.into()));
    Ok(())
}

/// Builds the environment for `open`: the given `NAME=value` pairs, or this process's
/// environment when none are given.
#[cfg(target_os = "linux")]
fn display_environment(arguments: &str) -> anyhow::Result<DisplayEnvironment> {
    if arguments.trim().is_empty() {
        return Ok(DisplayEnvironment::from_process_environment());
    }
    let mut environment = DisplayEnvironment::default();
    for assignment in arguments.split_whitespace() {
        let (name, value) = assignment
            .split_once('=')
            .ok_or_else(|| anyhow::anyhow!("expected NAME=value, got {assignment:?}"))?;
        let value = Some(value.into());
        match name {
            "WAYLAND_DISPLAY" => environment.wayland_display = value,
            "DISPLAY" => environment.x11_display = value,
            "XDG_RUNTIME_DIR" => environment.xdg_runtime_dir = value,
            _ => anyhow::bail!("unknown display variable {name}"),
        }
    }
    Ok(environment)
}

#[cfg(target_os = "linux")]
fn handle_command(
    command: &str,
    todos: &Entity<Todos>,
    cx: &mut App,
) -> anyhow::Result<Option<DisplayMode>> {
    let (name, arguments) = command.split_once(' ').unwrap_or((command, ""));
    match name {
        "ls" => print_todos(todos, cx),
        "create" => {
            let todo = cx.new(|_| Todo {
                message: arguments.to_owned(),
            });
            todos.update(cx, |todos, cx| {
                todos.items.push(todo);
                println!("todo added, total {}", todos.items.len());
                cx.notify();
            });
        }
        "open" => return Ok(Some(DisplayMode::Windowed(display_environment(arguments)?))),
        "close" => {
            let window = todos.update(cx, |todos, _| todos.window.take());
            if let Some(window) = window {
                window.update(cx, |_, window, _| window.remove_window())?;
            }
            return Ok(Some(DisplayMode::Headless));
        }
        "quit" => cx.quit(),
        "" => {}
        _ => println!("{USAGE}"),
    }
    Ok(None)
}

#[cfg(target_os = "linux")]
const USAGE: &str = "commands: ls | create <message> | open [DISPLAY=… WAYLAND_DISPLAY=… XDG_RUNTIME_DIR=…] | close | quit";

#[cfg(target_os = "linux")]
async fn run_command(
    command: String,
    todos: &Entity<Todos>,
    cx: &mut AsyncApp,
) -> anyhow::Result<()> {
    let Some(mode) = cx.update(|cx| handle_command(&command, todos, cx))? else {
        return Ok(());
    };
    let windowed = matches!(mode, DisplayMode::Windowed(_));
    cx.update(|cx| cx.set_display_mode(mode)).await?;
    if windowed {
        cx.update(|cx| open_window(todos, cx))?;
    }
    let compositor = cx.update(|cx| cx.compositor_name());
    println!("mode: {compositor}");
    Ok(())
}

#[cfg(target_os = "linux")]
fn main() {
    gpui_platform::linux(
        gpui_platform::LinuxDisplayModes::all(),
        DisplayMode::Headless,
    )
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

        println!("{USAGE}");
        cx.spawn(async move |cx| {
            let _window_closed_subscription = window_closed_subscription;
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(25))
                    .await;
                while let Ok(command) = command_receiver.try_recv() {
                    if let Err(error) = run_command(command, &todos, cx).await {
                        println!("error: {error:#}");
                    }
                }
            }
        })
        .detach();
    });
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!("the switchable_display example is only supported on Linux");
}
