mod game;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let resultado = executar_jogo(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(e) = resultado {
        eprintln!("Erro: {}", e);
    }

    Ok(())
}

fn executar_jogo(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut estado = game::EstadoJogo::novo();

    loop {
        terminal.draw(|frame| ui::renderizar(frame, &estado))?;

        if !estado.rodando {
            loop {
                if event::poll(std::time::Duration::from_millis(100))? {
                    if let Event::Key(_) = event::read()? {
                        break;
                    }
                }
            }
            break;
        }

        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(tecla) => {
                    if tecla.modifiers.contains(KeyModifiers::CONTROL) {
                        match tecla.code {
                            KeyCode::Char('c') | KeyCode::Char('d') => break,
                            _ => {}
                        }
                    }

                    match tecla.code {
                        KeyCode::Enter => {
                            estado.processar_entrada();
                        }
                        KeyCode::Char(c) => {
                            estado.entrada_atual.push(c);
                        }
                        KeyCode::Backspace => {
                            estado.entrada_atual.pop();
                        }
                        KeyCode::Esc => {
                            estado.entrada_atual.clear();
                        }
                        _ => {}
                    }
                }
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
    }

    Ok(())
}
