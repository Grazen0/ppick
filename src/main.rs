use std::{
    fs::OpenOptions,
    io::{self, Read},
    panic::{self, PanicHookInfo},
};

use app::App;
use clap::Parser;
use cli::{Args, CliError};

mod app;
mod cli;
mod ct_extra;
mod menu;
mod numeric;
mod string;

fn get_tty() -> io::Result<impl io::Write> {
    OpenOptions::new().write(true).open("/dev/tty")
}

fn add_panic_hook(hook: Box<dyn Fn(&PanicHookInfo<'_>) + 'static + Sync + Send>) {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        hook(panic_info);
        original_hook(panic_info);
    }));
}

fn try_main(args: Args) -> Result<(), CliError> {
    let mut buf = String::new();
    io::stdin().lock().read_to_string(&mut buf)?;
    let lines: Vec<_> = buf.lines().map(String::from).collect();

    if lines.is_empty() {
        return Err(CliError::NoInput);
    }

    let mut tty = get_tty()?;

    if args.clear {
        let _ = ct_extra::queue_clear_and_reset_cursor(&mut tty);
    }

    let mut app = App::new(args, &lines);
    App::init(&mut tty)?;

    add_panic_hook(Box::new(|_| {
        let _ = get_tty().and_then(|mut tty| App::deinit(&mut tty));
    }));

    let result = app.run(&mut tty);
    App::deinit(&mut tty)?;

    result
        .map_err(CliError::from)
        .and_then(|selection| selection.ok_or(CliError::Interrupted))
        .map(|selection| {
            println!("{selection}");
        })
}

fn main() {
    let args @ Args { silent, .. } = Args::parse();
    let main_result = try_main(args);

    if let Err(err) = main_result {
        if !err.is_interrupted() && !silent {
            eprintln!("{}", err);
        }
        std::process::exit(130);
    }
}
