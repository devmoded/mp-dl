use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::Frame;
use ratatui::style::Style;
use ratatui::layout::{Margin};
use ratatui::widgets::Block;
// use ratatui::widgets::Paragraph;

fn run(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame))?;
        if handle_events()? {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let main_area = frame.area().inner(Margin {
        horizontal: 3,
        vertical: 1,
    });

    // let not_bold = Style::new().not_bold();
    let bold = Style::new().bold();

    let version = env!("CARGO_PKG_VERSION");

    let main_block = Block::bordered().title(format!(" mp-dl {version} ")).title_bottom(" Закрыть: q ").style(bold);

    frame.render_widget(main_block, main_area);
}

fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            // handle other key events
            _ => {}
        },
        // handle other events
        _ => {}
    }
    Ok(false)
}

pub fn run_tui() {
    let mut terminal = ratatui::init();
    let result = run(&mut terminal).unwrap();
    ratatui::restore();
    result
}
