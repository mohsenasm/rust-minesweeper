use clap::Parser;
use crossterm::event::{
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::{
    event::{poll, read, DisableMouseCapture, EnableMouseCapture},
    execute, queue,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{stdout, Result, Stdout};
use std::thread;
use std::time::Duration;

mod board;
use board::{init_random_game, Board};

mod theme;
use theme::get_theme;

mod input;
use input::process_input;

use crate::board::GameCompletionStatus;

fn event_loop(mut game_board: Board, stdout: &Stdout) -> Result<()> {
    // first draw
    game_board.draw(&stdout)?;
    game_board.update();

    loop {
        if game_board.game_completion_status == GameCompletionStatus::GameIsOnGoing {
            process_input(&mut game_board)?;
        }

        if let Some(time) = game_board.delay_before_draw {
            thread::sleep(time);
        }

        game_board.draw(&stdout)?;
        game_board.update();

        if game_board.game_completion_status == GameCompletionStatus::CompletedAsLost
            || game_board.game_completion_status == GameCompletionStatus::CompletedAsWin
        {
            break;
        }
    }
    // read all before leave
    loop {
        if poll(Duration::from_millis(10))? {
            read()?;
        } else {
            break;
        }
    }
    if game_board.game_completion_status == GameCompletionStatus::CompletedAsLost {
        println!("Boom! You Lost :(\r");
    } else if game_board.game_completion_status == GameCompletionStatus::CompletedAsWin {
        println!("You Won :)\r");
    }

    Ok(())
}

#[derive(Parser)]
#[command(
    version,
    about = "Command-line minesweeper game with mouse support.",
    long_about = "Command-line minesweeper game with mouse support.
See https://github.com/mohsenasm/rust-minesweeper

Key                          | Action
---------------------------- | -----------
Mouse, Arrow keys            | Navigate the board
Left Click, Enter, Space     | Open the selected cell
Right Click, Middle Click, F | Flag the selected cell
Tab, T                       | Change theme
H                            | Show a hint
Ctrl+C, Q, Esc               | Exit the game
"
)]
struct Args {
    /// The board size
    #[arg(short, long, default_value = "12x8")]
    size: String,

    /// The bomb percentage
    #[arg(short, long, default_value_t = 0.2)]
    bomb_percentage: f32,

    /// The board theme (border, dark_border, borderless)
    #[arg(short, long, default_value = "dark_border")]
    theme: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let size: Option<(usize, usize)> = {
        let size_str = args.size.split('x').collect::<Vec<&str>>();
        if size_str.len() != 2 {
            println!("wrong size argument {}, enter it like 10x5\r", args.size);
            None
        } else {
            let width_parsed = size_str[0].parse::<usize>();
            let height_parsed = size_str[1].parse::<usize>();
            if let Err(ref e) = width_parsed {
                println!("wrong size argument {}, enter it like 10x5\r", args.size);
                println!("error detail: {}\r", e.to_string());
                None
            } else if let Err(ref e) = height_parsed {
                println!("wrong size argument {}, enter it like 10x5\r", args.size);
                println!("error detail: {}\r", e.to_string());
                None
            } else {
                let width = width_parsed.unwrap();
                let height = height_parsed.unwrap();

                if width == 0 || height == 0 {
                    println!("wrong size argument {}, enter it like 10x5\r", args.size);
                    println!("error detail: the number is zero\r");
                    None
                } else {
                    Some((width, height))
                }
            }
        }
    };

    if size == None {
        return Ok(());
    }
    let (width, height) = size.unwrap();

    let theme = get_theme(&args.theme);
    if theme == None {
        println!("not found theme {}\r", &args.theme);
        return Ok(());
    }
    let theme = theme.unwrap();

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
    let game_board = init_random_game((height, width), args.bomb_percentage, theme);

    // event_loop
    if let Err(e) = event_loop(game_board, &stdout) {
        println!("{}\r", e.to_string());
    }

    // terminal exit
    if supports_keyboard_enhancement {
        queue!(stdout, PopKeyboardEnhancementFlags)?;
    }
    execute!(stdout, PopKeyboardEnhancementFlags, DisableMouseCapture)?;
    disable_raw_mode()
}
