use std::{
    fmt::{Display, Formatter},
    thread,
};

use anyhow::Result;
use args::{Args, Commands};
use clap::Parser;
use colored::Colorize;
use duration_to_sleep_parser::ParseResult;
use enigo::{Button, Direction, Enigo, Mouse};
use inquire::{Confirm, Select, Text};
use terminal::{println_error, println_warning};

mod args;
mod duration_to_sleep_parser;
mod terminal;
mod utils;

fn main() {
    let args = Args::parse();
    if let Err(e) = start(args) {
        println_error!("{}", e);
        std::process::exit(1);
    }
}

fn start(args: Args) -> Result<()> {
    match args.command {
        None => {
            start_interactively(args)?;
        }
        Some(Commands::At { ref time }) => {
            let parse_result = duration_to_sleep_parser::parse_at(time)?;
            sleep_and_click(parse_result, args.double)?;
        }
        Some(Commands::In { ref duration }) => {
            let parse_result = duration_to_sleep_parser::parse_in(duration)?;
            sleep_and_click(parse_result, args.double)?;
        }
        Some(Commands::Now) => {
            click(args.double)?;
        }
    }

    Ok(())
}

fn start_interactively(args: Args) -> Result<()> {
    let help_command = format!("{} --help", utils::get_current_executable_name());

    println!(
        "{} {}
A command line tool to schedule a mouse click at a specific time.
GitHub: https://github.com/JasonWei512/schedule-mouse-click

You are currently in interactive mode. Press {} to exit.
You can also use this program in a non-interactive way.
Run {} to get more info.
",
        "schedule-mouse-click".bright_green(),
        env!("CARGO_PKG_VERSION"),
        "Ctrl+C".bright_yellow(),
        help_command.bright_yellow()
    );

    enum Mode {
        At,
        In,
        Now,
    }

    // Option description in "Select" control
    impl Display for Mode {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                Mode::At => write!(
                    f,
                    "Click at a specific time, such as {}, {} or {}",
                    "18:00:00".bright_green(),
                    "18:00".bright_green(),
                    "6:00pm".bright_green()
                ),
                Mode::In => write!(
                    f,
                    "Click after a specific amount of time, such as {}, {} or {}",
                    "2m30s".bright_green(),
                    "150s".bright_green(),
                    "150".bright_green()
                ),
                Mode::Now => write!(f, "Click now"),
            }
        }
    }

    let mode = Select::new(
        "Please select a mode:\n",
        vec![Mode::At, Mode::In, Mode::Now],
    )
    .with_help_message("↑↓ to move, Enter to select")
    .without_filtering()
    .prompt()?;

    println!();

    match mode {
        Mode::At | Mode::In => {
            let parse_result = loop {
                let result = match mode {
                    Mode::At => {
                        let input = Text::new(&format!(
                            "Input time to click, such as {}, {} or {}:\n",
                            "18:00:00".bright_green(),
                            "18:00".bright_green(),
                            "6:00pm".bright_green()
                        ))
                        .prompt()?;

                        duration_to_sleep_parser::parse_at(&input)
                    }
                    Mode::In => {
                        let input = Text::new(&format!(
                            "Input amount of time to wait before clicking, such as {}, {} or {}:\n",
                            "2m30s".bright_green(),
                            "150s".bright_green(),
                            "150".bright_green()
                        ))
                        .prompt()?;

                        duration_to_sleep_parser::parse_in(&input)
                    }
                    _ => unreachable!(),
                };

                println!();

                match result {
                    Ok(result) => {
                        break result;
                    }
                    Err(e) => {
                        println_error!("{}", e);
                        println!();
                    }
                }
            };

            let double_click = Confirm::new("Double-click?").with_default(false).prompt()?;
            println!();
            sleep_and_click(parse_result, double_click)?;
        }
        Mode::Now => {
            let double_click = Confirm::new("Double-click?").with_default(false).prompt()?;
            println!();
            click(double_click)?;
        }
    }

    Ok(())
}

fn sleep_and_click(parse_result: ParseResult, double_click: bool) -> Result<()> {
    println!(
        "Schedule a mouse {} at {} (in {}).",
        if double_click {
            "double-click"
        } else {
            "click"
        },
        parse_result.time_str.as_str().bright_yellow(),
        parse_result.duration_str.as_str().bright_yellow()
    );

    let keep_awake_result = keepawake::Builder::default().display(true).create();
    match keep_awake_result {
        Ok(_) => {
            println!("This program will keep your computer awake until then.");
        }
        Err(e) => {
            println_warning!(
                "This program tried to keep your computer awake, but failed.\n{}",
                e
            );
        }
    }

    println!("Press {} to cancel.", "Ctrl+C".bright_yellow());

    thread::sleep(parse_result.duration);
    click(double_click)
}

fn click(double_click: bool) -> Result<()> {
    println!(
        "{}",
        if double_click {
            "Double-clicking."
        } else {
            "Clicking."
        }
    );
    let mut enigo = Enigo::new(&enigo::Settings::default())?;
    enigo.button(Button::Left, Direction::Click)?;
    if double_click {
        enigo.button(Button::Left, Direction::Click)?;
    }
    Ok(())
}
