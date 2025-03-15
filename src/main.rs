use std::{
    fs::OpenOptions,
    io::{self, Read, Write},
};

use app::{App, AppError, Entry};
use clap::Parser;
use cli::Args;
use crossterm::{
    cursor, execute,
    terminal::{self, ClearType},
};

mod app;
mod cli;
mod ct_extra;
mod numeric;

fn parse_lines(input: &str) -> Vec<String> {
    let lines: Vec<_> = input.trim().lines().map(String::from).collect();
    if lines.is_empty() {
        vec![String::new()]
    } else {
        lines
    }
}

fn reset_screen(tty: &mut impl Write) -> io::Result<()> {
    execute!(tty, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))
}

fn into_entry(lines: &[String], ln_index: usize) -> Entry {
    assert!(ln_index < lines.len());

    let src_line = &lines[ln_index];
    let mut prefix_len = 0;
    let mut auto_accept = true;

    while lines
        .iter()
        .enumerate()
        .any(|(i, ln)| i != ln_index && ln.starts_with(&src_line[..prefix_len]))
    {
        if prefix_len == src_line.len() {
            auto_accept = false;
            break;
        } else {
            prefix_len += 1;
        }
    }

    Entry {
        body: src_line.to_string(),
        prefix_len,
        auto_accept,
    }
}

fn try_main() -> Result<(), AppError> {
    let cli = Args::parse();

    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf)?;
    let lines = parse_lines(&buf);
    let entries = (0..lines.len()).map(|i| into_entry(&lines, i)).collect();

    let mut tty = OpenOptions::new().write(true).open("/dev/tty")?;

    if cli.clear {
        let _ = reset_screen(&mut tty);
    }

    let mut app = App::new(cli, entries);
    app.init(&mut tty)?;
    let result = app.run(&mut tty);
    app.deinit(&mut tty)?;

    result.map(|selection| {
        println!("{}", selection);
    })
}

fn main() {
    let main_result = try_main();
    if let Err(err) = main_result {
        if err.is_other() {
            eprintln!("{}", err);
        }
        std::process::exit(err.code());
    }
}
