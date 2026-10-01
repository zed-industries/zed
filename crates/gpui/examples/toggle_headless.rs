#![cfg_attr(target_family = "wasm", no_main)]

#[cfg(not(target_family = "wasm"))]
use std::{
    io::{self, BufRead as _},
    sync::mpsc,
    time::Duration,
};

#[cfg(not(target_family = "wasm"))]
use gpui::{
    AnyWindowHandle, App, AppContext as _, AsyncApp, Context, Entity, GraphicalEnvironment,
    QuitMode, Render, Subscription, TitlebarOptions, Window, WindowOptions, WindowingRequest, div,
    prelude::*,
};

#[cfg(not(target_family = "wasm"))]
struct Todo {
    message: String,
}

#[cfg(not(target_family = "wasm"))]
struct Todos {
    items: Vec<Entity<Todo>>,
    window: Option<AnyWindowHandle>,
}

#[cfg(not(target_family = "wasm"))]
struct TodoWindow {
    todos: Entity<Todos>,
    _todos_subscription: Subscription,
}

#[cfg(not(target_family = "wasm"))]
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
            .child("Use the terminal: create [message], ls, close, open, quit")
            .when(rows.is_empty(), |view| {
                view.child("No todos yet. Type `create` in the terminal.")
            })
            .children(rows)
    }
}

/// English, Chinese, Japanese and Korean for a word, then an emoji for it.
#[cfg(not(target_family = "wasm"))]
const SAMPLE_TODOS: [&str; 5] = [
    "milk / 牛奶 / 牛乳 / 우유 🥛",
    "apple / 苹果 / りんご / 사과 🍎",
    "cat / 猫 / ねこ / 고양이 🐱",
    "book / 书 / 本 / 책 📚",
    "rain / 雨 / あめ / 비 🌧️",
];

#[cfg(not(target_family = "wasm"))]
fn describe_todo(index: usize, todo: &Entity<Todo>, cx: &App) -> String {
    let todo_reference: &Todo = todo.read(cx);
    format!(
        "{index} - {} - {:?} @{:p}",
        todo_reference.message,
        todo.entity_id(),
        todo_reference
    )
}

#[cfg(not(target_family = "wasm"))]
fn print_todos(todos: &Entity<Todos>, cx: &App) {
    if todos.read(cx).items.is_empty() {
        println!("no todos");
    }
    for (index, todo) in todos.read(cx).items.iter().enumerate() {
        println!("{}", describe_todo(index, todo, cx));
    }
}

#[cfg(not(target_family = "wasm"))]
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
    // The window opens behind the terminal otherwise, because the command came from there.
    handle.update(cx, |_, window, _| window.activate_window())?;
    todos.update(cx, |todos, _| todos.window = Some(handle.into()));
    Ok(())
}

/// Builds the environment for `open`: the given `NAME=value` pairs, or this process's
/// environment when none are given.
#[cfg(target_os = "linux")]
fn display_environment(arguments: &str) -> anyhow::Result<GraphicalEnvironment> {
    if arguments.trim().is_empty() {
        return Ok(GraphicalEnvironment::detect());
    }
    let mut environment = GraphicalEnvironment::default();
    for assignment in arguments.split_whitespace() {
        let (name, value) = assignment
            .split_once('=')
            .ok_or_else(|| anyhow::anyhow!("expected NAME=value, got {assignment:?}"))?;
        match name {
            "WAYLAND_DISPLAY" => environment.wayland_display = Some(value.into()),
            "DISPLAY" => environment.x11_display = Some(value.into()),
            "XDG_RUNTIME_DIR" => environment.xdg_runtime_dir = Some(value.into()),
            "XDG_ACTIVATION_TOKEN" => environment.activation_token = Some(value.into()),
            _ => anyhow::bail!("unknown display variable {name}"),
        }
    }
    Ok(environment)
}

/// Builds the environment for `open`: `SESSION=id`, or this process's session when not given.
#[cfg(target_os = "windows")]
fn display_environment(arguments: &str) -> anyhow::Result<GraphicalEnvironment> {
    let mut environment = GraphicalEnvironment::detect();
    for assignment in arguments.split_whitespace() {
        let session = assignment
            .strip_prefix("SESSION=")
            .ok_or_else(|| anyhow::anyhow!("expected SESSION=id, got {assignment:?}"))?;
        environment.session_id = Some(session.parse()?);
    }
    Ok(environment)
}

/// Builds the environment for `open`, which carries nothing on this platform.
#[cfg(not(any(target_os = "linux", target_os = "windows", target_family = "wasm")))]
fn display_environment(_arguments: &str) -> anyhow::Result<GraphicalEnvironment> {
    Ok(GraphicalEnvironment::detect())
}

#[cfg(not(target_family = "wasm"))]
fn handle_command(
    command: &str,
    todos: &Entity<Todos>,
    cx: &mut App,
) -> anyhow::Result<Option<WindowingRequest>> {
    let (name, arguments) = command.split_once(' ').unwrap_or((command, ""));
    match name {
        "ls" => print_todos(todos, cx),
        "create" => {
            // A sample with CJK text and an emoji, so that drawing todos exercises color glyph
            // rasterization, which uses the GPU devices a switch to windowed mode creates.
            let count = todos.read(cx).items.len();
            let sample = SAMPLE_TODOS[count % SAMPLE_TODOS.len()];
            let message = if arguments.trim().is_empty() {
                sample.to_owned()
            } else {
                format!("{arguments} · {sample}")
            };
            let todo = cx.new(|_| Todo { message });
            todos.update(cx, |todos, cx| {
                todos.items.push(todo);
                println!("todo added, total {}", todos.items.len());
                cx.notify();
            });
        }
        "open" => {
            return Ok(Some(WindowingRequest::Windowed(display_environment(
                arguments,
            )?)));
        }
        "close" => {
            let window = todos.update(cx, |todos, _| todos.window.take());
            if let Some(window) = window {
                window.update(cx, |_, window, _| window.remove_window())?;
            }
            return Ok(Some(WindowingRequest::Headless));
        }
        "quit" => cx.quit(),
        "" => {}
        _ => println!("{USAGE}"),
    }
    Ok(None)
}

#[cfg(target_os = "linux")]
const USAGE: &str = "commands: ls | create [message] | open [DISPLAY=… WAYLAND_DISPLAY=… XDG_RUNTIME_DIR=… XDG_ACTIVATION_TOKEN=…] | close | quit";

#[cfg(target_os = "windows")]
const USAGE: &str = "commands: ls | create [message] | open [SESSION=…] | close | quit";

#[cfg(not(any(target_os = "linux", target_os = "windows", target_family = "wasm")))]
const USAGE: &str = "commands: ls | create [message] | open | close | quit";

#[cfg(not(target_family = "wasm"))]
async fn run_command(
    command: String,
    todos: &Entity<Todos>,
    cx: &mut AsyncApp,
) -> anyhow::Result<()> {
    match cx.update(|cx| handle_command(&command, todos, cx))? {
        None => return Ok(()),
        Some(request @ WindowingRequest::Headless) => {
            if cx.update(|cx| cx.graphical_environment()).is_some() {
                cx.update(|cx| cx.request_windowing(request)).await?;
            }
        }
        Some(request @ WindowingRequest::Windowed(_)) => {
            // Switching fails while windowed, so `open` then just opens the window.
            if cx.update(|cx| cx.graphical_environment()).is_none() {
                cx.update(|cx| cx.request_windowing(request)).await?;
            }
            cx.update(|cx| open_window(todos, cx))?;
        }
    }
    let mode = cx.update(|cx| match cx.graphical_environment() {
        None => "headless".to_owned(),
        Some(_) => match cx.compositor_name() {
            "" => "windowed".to_owned(),
            compositor => compositor.to_owned(),
        },
    });
    println!("mode: {mode}");
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    #[cfg(target_os = "linux")]
    let application = gpui_platform::linux(gpui::WindowingModes::all());
    #[cfg(not(target_os = "linux"))]
    let application = gpui_platform::application();
    application
        .with_windowing(WindowingRequest::Headless)
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
