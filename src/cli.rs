use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Whether to clear the screen before showing the picker.
    #[arg(short, long)]
    pub clear: bool,

    /// Whether to allow wrapping the current selection.
    #[arg(long)]
    pub no_wrap: bool,

    /// Disable ending the program as soon as there's only one option left.
    #[arg(long)]
    pub no_auto_accept: bool,

    /// Allow the user to type in anything, even if it doesn't match any menu entry
    /// anymore.
    #[arg(short, long)]
    pub unrestricted_input: bool,

    /// Character used to indicate the current selection.
    #[arg(short, long, default_value_t = '▌')]
    pub indicator: char,
}
