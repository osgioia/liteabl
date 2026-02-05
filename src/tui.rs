use std::io;
use std::error::Error;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    layout::{Alignment, Rect},
    Terminal,
    style::{Style, Color},
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute as cross_execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn display_results(cols: &[String], results: &[Vec<String>]) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    cross_execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut col_widths = vec![0; cols.len()];
    for (i, col) in cols.iter().enumerate() {
        col_widths[i] = col.len();
    }
    for row in results {
        for (i, val) in row.iter().enumerate() {
            if i < col_widths.len() {
                col_widths[i] = col_widths[i].max(val.len());
            }
        }
    }
    for w in col_widths.iter_mut() {
        *w += 2;
    }

    for row in results {
        let mut should_continue = false;
        while !should_continue {
            terminal.draw(|f| {
                let size = f.size();
                f.render_widget(Block::default().style(Style::default().bg(Color::Black)), size);

                let mut header_line = String::new();
                let mut separator_line = String::new();
                let mut data_line = String::new();

                for (i, col_name) in cols.iter().enumerate() {
                    let w = col_widths[i];
                    header_line.push_str(&format!(" {:<width$} │", col_name, width = w - 1));
                    separator_line.push_str(&format!("{:-<width$}─┼", "", width = w - 1));
                    let val = row.get(i).map(|s| s.as_str()).unwrap_or("NULL");
                    data_line.push_str(&format!(" {:<width$} │", val, width = w - 1));
                }

                if !header_line.is_empty() {
                    header_line.pop();
                    separator_line.pop();
                    data_line.pop();
                }

                let content = format!("{}\n{}\n{}", header_line, separator_line, data_line);
                let box_width = header_line.len() as u16 + 2;
                let box_height = 5;

                let area = Rect::new(0, 0, box_width.min(size.width), box_height.min(size.height));
                let block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::White));
                let paragraph = Paragraph::new(content).block(block);
                f.render_widget(paragraph, area);

                if area.y + area.height < size.height {
                    let footer = Paragraph::new("\n[ Continue (Enter) ]").alignment(Alignment::Left);
                    f.render_widget(footer, Rect::new(0, area.y + area.height, size.width, 2));
                }
            })?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Enter {
                        should_continue = true;
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    cross_execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
