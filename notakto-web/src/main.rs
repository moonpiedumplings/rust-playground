use std::{cell::RefCell, io, rc::Rc};

use ratatui::{
    layout::{Layout, Alignment, Constraint, Rect},
    style::{Style, Color, Stylize},
    widgets::{Widget, Block, BorderType, Paragraph},
    buffer::Buffer,
    Frame, Terminal,
};

use ratzilla::{
    event::{KeyCode, KeyEvent},
    DomBackend, WebRenderer,
};

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let state = Rc::new(App::default());

    let event_state = Rc::clone(&state);
    terminal.on_key_event(move |key_event| {
        event_state.handle_events(key_event);
    });

    let render_state = Rc::clone(&state);
    terminal.draw_web(move |frame| {
        render_state.render(frame);
    });

    Ok(())
}

#[derive(Default)]
struct App {
    counter: RefCell<u8>,
}

struct Grid {
    cols: usize,
    rows: usize,
    cells: 
}

impl Grid {
    fn new(cols: usize, rows: usize) -> Self {
        let col_constraints = (0..cols).map(|_| Constraint::Length(9));
        let row_constraints = (0..rows).map(|_| Constraint::Length(3));
        let horizontal = Layout::horizontal(col_constraints).spacing(1);
        let vertical = Layout::vertical(row_constraints).spacing(1);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());
        
        Self {
            cols: cols,
            rows: rows,
            cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec()),
                            
        }
    }
}

impl Widget for Grid {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col_constraints = (0..self.cols).map(|_| Constraint::Length(9));
        let row_constraints = (0..self.rows).map(|_| Constraint::Length(3));
        let horizontal = Layout::horizontal(col_constraints).spacing(1);
        let vertical = Layout::vertical(row_constraints).spacing(1);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            Paragraph::new(format!("Area {:02}", i + 1))
                .block(Block::bordered())
                .render(cell, buf);
        }
    }
}

impl App {
    fn render(&self, frame: &mut Frame) {
        let counter = self.counter.borrow();

        let mut grid = Grid {
            cols: 3,
            rows: 3,
        };

        let block = Block::bordered()
            .title("Notakto Web")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text = format!(
            "This is a Ratzilla template.\n\
             Press left and right to increment and decrement the counter respectively.\n\
             Counter: {counter}",
        );

        let inner = block.inner(frame.area());

        frame.render_widget(grid, inner);

        frame.render_widget(block, frame.area());



        // let paragraph = Paragraph::new(text)
        //     .block(block)
        //     .fg(Color::White)
        //     .bg(Color::Black)
        //     .centered();
        // let mut grid = grid.block(block);

        //frame.render_widget(grid, frame.area());
    }

    fn handle_events(&self, key_event: KeyEvent) {
        let mut counter = self.counter.borrow_mut();
        match key_event.code {
            KeyCode::Left => *counter = counter.saturating_sub(1),
            KeyCode::Right => *counter = counter.saturating_add(1),
            _ => {}
        }
    }
}
