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
    event::{self, Event, KeyCode, KeyModifiers},
    execute as cross_execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn display_results(cols: &[String], results: &[Vec<String>]) -> Result<(), Box<dyn Error>> {
    if results.is_empty() {
        println!("No records found.");
        return Ok(());
    }

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

    let mut header_line = String::new();
    let mut separator_line = String::new();
    for (i, col_name) in cols.iter().enumerate() {
        let w = col_widths[i];
        let cell = format!(" {:<w$} ", col_name);
        header_line.push_str(&cell);
        
        for _ in 0..(w + 2) {
            separator_line.push_str("─");
        }

        if i < cols.len() - 1 {
            header_line.push_str("│");
            separator_line.push_str("┼");
        }
    }

    let mut current_offset = 0;
    let total_rows = results.len();

    while current_offset < total_rows {
        let mut should_next_page = false;
        
        while !should_next_page {
            terminal.draw(|f| {
                let size = f.size();
                f.render_widget(Block::default().style(Style::default().bg(Color::Black)), size);

                let footer_height = 2;
                let reserved_height = 4;
                let available_height = size.height.saturating_sub(footer_height + reserved_height);
                let rows_per_page = available_height as usize;
                let rows_per_page = rows_per_page.max(1);

                let end_index = (current_offset + rows_per_page).min(total_rows);
                let page_rows = &results[current_offset..end_index];

                let mut content = String::new();
                content.push_str(&header_line);
                content.push_str("\n");
                content.push_str(&separator_line);
                content.push_str("\n");

                for row in page_rows {
                    let mut data_line = String::new();
                    for (i, _) in cols.iter().enumerate() {
                        let w = col_widths[i];
                        let val = row.get(i).map(|s| s.as_str()).unwrap_or("NULL");
                        data_line.push_str(&format!(" {:<w$} ", val));
                        if i < cols.len() - 1 {
                            data_line.push_str("│");
                        }
                    }
                    content.push_str(&data_line);
                    content.push_str("\n");
                }

                let box_width = header_line.len() as u16 + 2;
                let box_height = (page_rows.len() as u16) + 4;

                let area = Rect::new(0, 0, box_width.min(size.width), box_height.min(size.height));
                let block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::White));
                let paragraph = Paragraph::new(content).block(block);
                f.render_widget(paragraph, area);

                let footer_area = Rect::new(0, size.height.saturating_sub(1), size.width, 1);
                let footer_text = format!("[ Continue (Enter) ] [ Exit (Ctrl+C) ] - Rows {}-{} of {}", current_offset + 1, end_index, total_rows);
                let footer = Paragraph::new(footer_text).alignment(Alignment::Left);
                f.render_widget(footer, footer_area);
            })?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
                        cleanup(&mut terminal).ok();
                        std::process::exit(0);
                    }
                    
                    if key.code == KeyCode::Enter {
                        let size = terminal.size().unwrap_or(Rect::default());
                        let available_height = size.height.saturating_sub(6).max(1);
                        current_offset += available_height as usize;
                        should_next_page = true;
                    }
                }
            }
        }
    }

    cleanup(&mut terminal)?;
    Ok(())
}

fn cleanup(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    cross_execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
