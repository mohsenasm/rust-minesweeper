use crossterm::event::{
    KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseButton,
    MouseEventKind,
};
use crossterm::{
    event::{read, Event, KeyCode},
};
use std::io::{Error, ErrorKind, Result};

use crate::board::Board;

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
const TAB_KEY: KeyEvent = KeyEvent {
    code: KeyCode::Tab,
    modifiers: KeyModifiers::NONE,
    kind: KeyEventKind::Press,
    state: KeyEventState::NONE,
};
const T_KEY: KeyEvent = KeyEvent {
    code: KeyCode::Char('t'),
    modifiers: KeyModifiers::NONE,
    kind: KeyEventKind::Press,
    state: KeyEventState::NONE,
};
const H_KEY: KeyEvent = KeyEvent {
    code: KeyCode::Char('h'),
    modifiers: KeyModifiers::NONE,
    kind: KeyEventKind::Press,
    state: KeyEventState::NONE,
};

pub fn process_input(game_board: &mut Board) -> Result<()>  {
    let event = read()?;

    if let Event::Mouse(mouse_event) = event {
        let row = mouse_event.row as usize; // TODO: usize::try_from(mouse_event.row);
        let column = mouse_event.column as usize; // TODO: usize::try_from(mouse_event.column);

        if mouse_event.kind == MouseEventKind::Moved {
            game_board.mouse_hover(row, column);
        }
        if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) {
            game_board.mouse_down(row, column, true);
        } else if mouse_event.kind == MouseEventKind::Down(MouseButton::Right)
            || mouse_event.kind == MouseEventKind::Down(MouseButton::Middle)
        {
            game_board.mouse_down(row, column, false);
        }
    }

    if let Event::Key(key_event) = event {
        // exit on CTRL_C, ESC, or Q
        if key_event == CTRL_C_KEY || key_event == ESC_KEY || key_event == Q_KEY {
            return Err(Error::new(ErrorKind::Interrupted, ""));
        }
        // change theme on TAB or T
        if key_event == TAB_KEY || key_event == T_KEY {
            game_board.change_theme();
        }
        // discover a non-bomb on H
        if key_event == H_KEY {
            game_board.hint();
        }

        // Keyboard navigation
        match key_event.code {
            KeyCode::Up => game_board.move_selection(-1, 0),
            KeyCode::Down => game_board.move_selection(1, 0),
            KeyCode::Left => game_board.move_selection(0, -1),
            KeyCode::Right => game_board.move_selection(0, 1),
            KeyCode::Char('f') | KeyCode::Char('F') => game_board.flag_selected(),
            KeyCode::Char('c') | KeyCode::Char('C') => game_board.change_theme_color(),
            KeyCode::Enter | KeyCode::Char(' ') => game_board.open_selected(),
            _ => {}
        }
    }

    Ok(())
}
