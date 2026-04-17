use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::game::EstadoJogo;

pub fn renderizar(frame: &mut Frame, estado: &EstadoJogo) {
    let area = frame.area();

    // Layout principal: área de jogo + barra de input
    let blocos = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),    // área de output (expande)
            Constraint::Length(1), // separador visual
            Constraint::Length(3), // caixa de input
        ])
        .split(area);

    let area_output = blocos[0];
    let area_input = blocos[2];

    // Calcular quantas linhas cabem na área de output (menos bordas)
    let linhas_visiveis = (area_output.height as usize).saturating_sub(2);

    // Montar linhas estilizadas do output
    let linhas_texto: Vec<Line> = estado
        .mensagens
        .iter()
        .map(|m| estilizar_linha(m))
        .collect();

    // Rolar para o final
    let total = linhas_texto.len();
    let scroll = if total > linhas_visiveis {
        (total - linhas_visiveis) as u16
    } else {
        0
    };

    let titulo_principal = Line::from(vec![
        Span::styled(" ⚡ ", Style::default().fg(Color::Yellow)),
        Span::styled(
            "ZORK-ZED",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            " — O Editor como Dungeon ",
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    let area_saida = Paragraph::new(linhas_texto)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Blue))
                .title(titulo_principal)
                .title_alignment(Alignment::Center),
        )
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));

    frame.render_widget(area_saida, area_output);

    // Caixa de input
    let prompt = format!("> {}_", estado.entrada_atual);
    let estilo_input = Style::default().fg(Color::Green);

    let movimentos = estado.jogador.movimentos;
    let titulo_input = Line::from(vec![
        Span::styled(" Comando ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("[{} movimentos]", movimentos),
            Style::default().fg(Color::Yellow),
        ),
        Span::styled(" ", Style::default()),
    ]);

    let caixa_input = Paragraph::new(Span::styled(prompt, estilo_input)).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Green))
            .title(titulo_input),
    );

    frame.render_widget(caixa_input, area_input);
}

fn estilizar_linha(linha: &str) -> Line<'static> {
    let linha = linha.to_string();

    // Títulos de sala (começam com ━━━)
    if linha.starts_with("━━━") {
        return Line::from(Span::styled(
            linha,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ));
    }

    // Prompt do usuário (começam com >)
    if let Some(texto_cmd) = linha.strip_prefix("> ") {
        return Line::from(vec![
            Span::styled("> ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                texto_cmd.to_string(),
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]);
    }

    // Itens (começam com •)
    if linha.starts_with("  •") {
        return Line::from(Span::styled(linha, Style::default().fg(Color::Yellow)));
    }

    // Itens no inventário (começam com  -)
    if linha.starts_with("  -") {
        return Line::from(Span::styled(linha, Style::default().fg(Color::Green)));
    }

    // Banner e vitória (★)
    if linha.contains('★') || linha.contains('╔') || linha.contains('║') || linha.contains('╠')
    {
        return Line::from(Span::styled(
            linha,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ));
    }

    // Saídas
    if linha.starts_with("Saídas:") || linha.starts_with("Saida") {
        return Line::from(Span::styled(linha, Style::default().fg(Color::Magenta)));
    }

    // Dicas
    if linha.starts_with("Dica:") {
        return Line::from(Span::styled(
            linha,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::ITALIC),
        ));
    }

    // Separadores e banners ASCII
    if linha.contains("═══") || linha.contains("───") {
        return Line::from(Span::styled(linha, Style::default().fg(Color::Blue)));
    }

    // Linhas de código/diff
    if linha.trim_start().starts_with("- ") || linha.trim_start().starts_with("+ ") {
        let cor = if linha.trim_start().starts_with("- ") {
            Color::Red
        } else {
            Color::Green
        };
        return Line::from(Span::styled(linha, Style::default().fg(cor)));
    }

    // Texto padrão
    Line::from(Span::styled(linha, Style::default().fg(Color::Gray)))
}
