use crossterm::event::{
    KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, KeyboardEnhancementFlags, MouseButton,
    MouseEventKind, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::{
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{stdout, Result, Stdout};
use theme::default_theme;

mod board;
use board::{init_random_game, Board};

mod theme;

const CTRL_C_KEY: KeyEvent = KeyEvent {
    code: KeyCode::Char('c'),
    modifiers: KeyModifiers::CONTROL,
    kind: KeyEventKind::Press,
    state: KeyEventState::NONE,
};
const Q_KEY: KeyEvent = KeyEvent {
    code: KeyCode::Char('q'),
    modifiers: KeyModifiers::NONE,
    kind: KeyEventKind::Press,
    state: KeyEventState::NONE,
};
const ESC_KEY: KeyEvent = KeyEvent {
    code: KeyCode::Esc,
    modifiers: KeyModifiers::NONE,
    kind: KeyEventKind::Press,
    state: KeyEventState::NONE,
};

fn event_loop(mut game_board: Board, stdout: &Stdout) -> Result<()> {
    // first draw
    if let Err(e) = game_board.draw(&stdout) {
        return Err(e);
    }

    loop {
        let event = read()?;

        if let Event::Mouse(mouse_event) = event {
            let row = mouse_event.row as usize; // TODO: usize::try_from(mouse_event.row);
            let column = mouse_event.column as usize; // TODO: usize::try_from(mouse_event.column);

            if mouse_event.kind == MouseEventKind::Moved {
                game_board.mouse_hover(row, column);
            }
            if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
                game_board.mouse_down(row, column);
            }
        }

        if let Event::Key(key_event) = event {
            // exit on CTRL_C, ESC, or Q
            if key_event == CTRL_C_KEY || key_event == ESC_KEY || key_event == Q_KEY {
                break;
            }
        }

        if let Err(e) = game_board.draw(&stdout) {
            return Err(e);
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    // terminal setup
    enable_raw_mode()?;
    let mut stdout = stdout();
    let supports_keyboard_enhancement = matches!(
        crossterm::terminal::supports_keyboard_enhancement(),
        Ok(true)
    );
    if supports_keyboard_enhancement {
        queue!(
            stdout,
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                    | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
            )
        )?;
    }
    execute!(stdout, EnableMouseCapture)?;

    // board setup
    let game_board = init_random_game((5, 10), default_theme());

    // event_loop
    if let Err(e) = event_loop(game_board, &stdout) {
        println!("{:?}\r", e);
    }

    // terminal exit
    if supports_keyboard_enhancement {
        queue!(stdout, PopKeyboardEnhancementFlags)?;
    }
    execute!(stdout, PopKeyboardEnhancementFlags, DisableMouseCapture)?;
    disable_raw_mode()
}
