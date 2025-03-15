use std::io::{self, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{self, StyledContent, Stylize},
    terminal::{self, ClearType},
};
use derive_more::{Display, Error, IsVariant};

use crate::{cli::Args, ct_extra, numeric};

#[derive(Debug, Clone)]
pub struct Entry {
    pub body: String,
    pub prefix_len: usize,
    pub auto_accept: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum SearchDirection {
    Forwards,
    Backwards,
}

impl Entry {
    pub fn is_selectable(&self, input: &str) -> bool {
        self.body.starts_with(input)
    }

    pub fn stylize(&self, input: &str) -> Vec<StyledContent<impl std::fmt::Display>> {
        if !self.is_selectable(input) {
            vec![self.body.clone().dark_grey()]
        } else if input.len() >= self.prefix_len {
            vec![
                input.to_string().dark_red().bold(),
                self.body[input.len()..].to_string().stylize(),
            ]
        } else {
            vec![
                input.to_string().dark_red().bold(),
                self.body[input.len()..self.prefix_len]
                    .to_string()
                    .dark_blue()
                    .bold(),
                self.body[self.prefix_len..].to_string().stylize(),
            ]
        }
    }
}

#[derive(Debug, Display, Error, IsVariant)]
pub enum AppError {
    #[display("picker was interrupted")]
    Interrupted,
    #[display("{_0}")]
    Other(Box<dyn std::error::Error>),
}

impl AppError {
    pub fn code(&self) -> i32 {
        match self {
            Self::Interrupted => 2,
            Self::Other(_) => 130,
        }
    }
}

macro_rules! impl_error_from {
    ($t:ty) => {
        impl From<$t> for AppError {
            fn from(value: $t) -> Self {
                Self::Other(value.into())
            }
        }
    };
}

impl_error_from!(io::Error);

pub struct App {
    pub args: Args,
    pub entries: Vec<Entry>,
    pub selection: Option<usize>,
    pub input: String,
    pub exit_value: Option<String>,
    pub redraw: bool,
}

impl App {
    pub fn new(args: Args, entries: Vec<Entry>) -> Self {
        Self {
            args,
            entries,
            selection: Some(0),
            input: String::new(),
            exit_value: None,
            redraw: true,
        }
    }

    pub fn init(&self, tty: &mut impl Write) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(tty, cursor::Hide)?;
        Ok(())
    }

    pub fn deinit(&self, tty: &mut impl Write) -> io::Result<()> {
        execute!(
            tty,
            cursor::Show,
            terminal::Clear(ClearType::FromCursorDown)
        )?;

        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn run(&mut self, tty: &mut impl Write) -> Result<String, AppError> {
        loop {
            if self.redraw {
                self.draw(tty)?;
            }

            self.redraw = true;
            self.handle_events()?;

            if let Some(value) = &self.exit_value {
                break Ok(value.clone());
            }
        }
    }

    fn draw(&self, tty: &mut impl Write) -> Result<(), AppError> {
        for (i, entry) in self.entries.iter().enumerate() {
            queue!(
                tty,
                terminal::Clear(ClearType::CurrentLine),
                style::Print(" "),
                style::Print(" ")
            )?;

            for el in entry.stylize(&self.input) {
                queue!(tty, style::Print(el))?;
            }

            queue!(tty, cursor::MoveToColumn(0))?;

            if i < self.entries.len() - 1 {
                queue!(tty, style::Print("\n"))?;
            }
        }

        if let Some(selection) = self.selection {
            queue!(
                tty,
                ct_extra::MoveUpExact((self.entries.len() - selection - 1) as u16),
                style::PrintStyledContent(self.args.indicator.dark_red()),
                cursor::MoveToColumn(0),
                ct_extra::MoveUpExact(selection as u16),
            )?;
        } else {
            queue!(tty, ct_extra::MoveUpExact((self.entries.len() - 1) as u16))?;
        }

        tty.flush()?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<(), AppError> {
        if let Event::Key(
            key @ KeyEvent {
                kind: KeyEventKind::Press,
                ..
            },
        ) = event::read()?
        {
            match (key.modifiers, key.code) {
                (KeyModifiers::NONE, KeyCode::Esc)
                | (KeyModifiers::CONTROL, KeyCode::Char('c')) => return Err(AppError::Interrupted),
                (KeyModifiers::NONE, KeyCode::Char(ch)) => self.try_input_type(ch),
                (KeyModifiers::NONE, KeyCode::Enter) => self.try_accept(),
                (KeyModifiers::NONE, KeyCode::Backspace) => self.input_delete_char(),
                (KeyModifiers::CONTROL, KeyCode::Char('w')) => self.input_delete_word(),
                (KeyModifiers::CONTROL, KeyCode::Char('n'))
                | (KeyModifiers::NONE, KeyCode::Tab) => {
                    self.move_selection(SearchDirection::Forwards)
                }
                (KeyModifiers::CONTROL, KeyCode::Char('p'))
                | (KeyModifiers::SHIFT, KeyCode::BackTab) => {
                    self.move_selection(SearchDirection::Backwards)
                }
                _ => self.redraw = false,
            }
        }

        Ok(())
    }

    fn try_input_type(&mut self, ch: char) {
        let mut new_input = self.input.clone();
        new_input.push(ch);

        if !self.args.unrestricted_input
            && !self
                .entries
                .iter()
                .any(|entry| entry.is_selectable(&new_input))
        {
            self.redraw = false;
        } else {
            self.input = new_input;

            if !self.args.no_auto_accept {
                self.try_auto_accept();
            }

            self.update_selection();
        }
    }

    fn try_accept(&mut self) {
        if let Some(selection) = self.selection {
            self.exit_value = Some(self.entries[selection].body.clone());
        } else {
            self.redraw = false;
        }
    }

    fn try_auto_accept(&mut self) {
        let selections_left: Vec<_> = self
            .entries
            .iter()
            .filter(|entry| entry.is_selectable(&self.input))
            .collect();

        if selections_left.len() == 1 && selections_left[0].auto_accept {
            self.exit_value = Some(selections_left[0].body.clone());
        }
    }

    fn input_delete_char(&mut self) {
        if !self.input.is_empty() {
            self.input.pop();
        }
    }

    fn input_delete_word(&mut self) {
        self.input = self
            .input
            .rsplit_once(' ')
            .map(|(rem, _)| rem.to_string() + " ")
            .unwrap_or_default();
    }

    fn update_selection(&mut self) {
        let start = self.selection.unwrap_or(0);

        let search_result = self.entries[start..]
            .iter()
            .enumerate()
            .find(|(_, entry)| entry.is_selectable(&self.input))
            .map(|(i, _)| start + i)
            .or_else(|| {
                self.entries[..start]
                    .iter()
                    .enumerate()
                    .rfind(|(_, entry)| entry.is_selectable(&self.input))
                    .map(|(i, _)| i)
            });

        self.selection = search_result;
    }

    fn move_selection(&mut self, direction: SearchDirection) {
        let mut candidate = self.selection.unwrap_or(0);
        let mut did_wrap;

        loop {
            (candidate, did_wrap) = match direction {
                SearchDirection::Forwards => numeric::wrapping_inc(candidate, self.entries.len()),
                SearchDirection::Backwards => numeric::wrapping_dec(candidate, self.entries.len()),
            };

            if did_wrap && self.args.no_wrap {
                self.redraw = false;
                break;
            }

            if self.entries[candidate].is_selectable(&self.input) {
                self.selection = Some(candidate);
                break;
            }
        }
    }
}
