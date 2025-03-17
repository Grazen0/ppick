use clap::Parser;
use crossterm::style::Color;
use derive_more::{Display, Error, From};

#[derive(Debug, Clone, From, Display, Error, PartialEq, Eq)]
pub enum ParseHexError {
    #[display("invalid length")]
    InvalidLength,
    #[display("parse int error: {_0}")]
    ParseIntError(#[from] std::num::ParseIntError),
}

fn parse_hex(mut src: &str) -> Result<Color, ParseHexError> {
    if src.starts_with('#') {
        // #abcdef -> abcdef
        src = &src[1..];
    }

    if src.len() != 6 {
        return Err(ParseHexError::InvalidLength);
    }

    let r = u8::from_str_radix(&src[..2], 16)?;
    let g = u8::from_str_radix(&src[2..4], 16)?;
    let b = u8::from_str_radix(&src[4..], 16)?;
    Ok(Color::Rgb { r, g, b })
}

fn parse_highlight(src: &str) -> Result<Color, String> {
    match src {
        "black" => Ok(Color::Black),
        "bright-black" => Ok(Color::DarkGrey),
        "red" => Ok(Color::DarkRed),
        "bright-red" => Ok(Color::Red),
        "green" => Ok(Color::DarkGreen),
        "bright-green" => Ok(Color::Green),
        "yellow" => Ok(Color::DarkYellow),
        "bright-yellow" => Ok(Color::Yellow),
        "blue" => Ok(Color::DarkBlue),
        "bright-blue" => Ok(Color::Blue),
        "magenta" => Ok(Color::DarkMagenta),
        "bright-magenta" => Ok(Color::Magenta),
        "cyan" => Ok(Color::DarkCyan),
        "bright-cyan" => Ok(Color::Cyan),
        "white" => Ok(Color::Grey),
        "bright-white" => Ok(Color::White),
        _ => src
            .parse::<u8>()
            .map(Color::AnsiValue)
            .or_else(|_| parse_hex(src))
            .map_err(|_| "could not parse highlight".to_string()),
    }
}

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

    /// Whether to silence error messages.
    #[arg(short, long)]
    pub silent: bool,

    /// Highlight for each entry's prefix segment.
    #[arg(long, value_parser = parse_highlight, default_value = "blue")]
    pub hl_prefix: Color,

    /// Highlight for the input overlay over selectable entries.
    #[arg(long, value_parser = parse_highlight, default_value = "red")]
    pub hl_input_overlay: Color,

    /// Highlight for non-selectable entries.
    #[arg(long, value_parser = parse_highlight, default_value = "bright-black")]
    pub hl_disabled_entry: Color,

    /// Highlight for the indicator of the current selection.
    #[arg(long, value_parser = parse_highlight, default_value = "red")]
    pub hl_indicator: Color,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_valid() {
        assert_eq!(
            parse_hex("#80aec3"),
            Ok(Color::Rgb {
                r: 0x80,
                g: 0xAE,
                b: 0xC3
            })
        );
        assert_eq!(
            parse_hex("80aec3"),
            Ok(Color::Rgb {
                r: 0x80,
                g: 0xAE,
                b: 0xC3
            })
        );
        assert_eq!(
            parse_hex("4a00ff"),
            Ok(Color::Rgb {
                r: 0x4A,
                g: 0x00,
                b: 0xFF
            })
        );
        assert_eq!(
            parse_hex("#4ABCFF"),
            Ok(Color::Rgb {
                r: 0x4A,
                g: 0xBC,
                b: 0xFF
            })
        );
    }

    #[test]
    fn test_parse_hex_invalid() {
        assert_eq!(parse_hex(""), Err(ParseHexError::InvalidLength));
        assert_eq!(parse_hex("#12345"), Err(ParseHexError::InvalidLength));
        assert_eq!(parse_hex("12345"), Err(ParseHexError::InvalidLength));
        matches!(parse_hex("#reallylong"), Err(ParseHexError::InvalidLength));
        matches!(parse_hex("#foobar"), Err(ParseHexError::ParseIntError(_)));
    }
}
