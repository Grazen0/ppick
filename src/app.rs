use std::io::{self, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{self, StyledContent, Stylize},
    terminal::{self, ClearType},
};

use crate::{
    cli::Args,
    ct_extra,
    menu::{Entry, Menu, SearchDirection},
    string,
};

pub struct App {
    pub args: Args,
    pub menu: Menu,
    pub input: String,
    pub exit_value: Option<Option<String>>,
}

impl App {
    pub fn new(args: Args, lines: &[String]) -> Self {
        Self {
            args,
            menu: Menu::from_lines(lines),
            input: String::new(),
            exit_value: None,
        }
    }

    pub fn init(tty: &mut impl Write) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(tty, cursor::Hide)?;
        Ok(())
    }

    pub fn deinit(tty: &mut impl Write) -> io::Result<()> {
        execute!(
            tty,
            cursor::Show,
            terminal::Clear(ClearType::FromCursorDown)
        )?;

        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn run(&mut self, tty: &mut impl Write) -> io::Result<Option<String>> {
        let mut redraw = true;

        loop {
            if redraw {
                self.draw(tty)?;
            }

            redraw = self.handle_events()?;

            if let Some(value) = self.exit_value.take() {
                break Ok(value);
            }
        }
    }

    fn draw(&self, tty: &mut impl Write) -> io::Result<()> {
        for (i, entry) in self.menu.entries().iter().enumerate() {
            queue!(
                tty,
                terminal::Clear(ClearType::CurrentLine),
                style::Print("  "),
            )?;

            for el in self.stylize_entry(entry) {
                queue!(tty, style::Print(el))?;
            }

            queue!(tty, cursor::MoveToColumn(0))?;

            if i < self.menu.len() - 1 {
                queue!(tty, style::Print("\n"))?;
            }
        }

        if let Some(&selection) = self.menu.selection() {
            ct_extra::queue_move_up_exact(tty, (self.menu.len() - selection - 1) as u16)?;
            queue!(
                tty,
                style::PrintStyledContent(self.args.indicator.with(self.args.hl_indicator)),
                cursor::MoveToColumn(0),
            )?;
            ct_extra::queue_move_up_exact(tty, selection as u16)?;
        } else {
            ct_extra::queue_move_up_exact(tty, (self.menu.len() - 1) as u16)?;
        }

        tty.flush()?;
        Ok(())
    }

    fn stylize_entry(&self, entry: &Entry) -> Vec<StyledContent<impl std::fmt::Display>> {
        if !entry.is_selectable(&self.input) {
            vec![entry.body.clone().with(self.args.hl_disabled_entry)]
        } else {
            let input_seg = self
                .input
                .to_string()
                .with(self.args.hl_input_overlay)
                .bold();

            if self.input.len() >= entry.prefix_len {
                vec![
                    input_seg,
                    entry.body[self.input.len()..].to_string().stylize(),
                ]
            } else {
                vec![
                    input_seg,
                    entry.body[self.input.len()..entry.prefix_len]
                        .to_string()
                        .with(self.args.hl_prefix)
                        .bold(),
                    entry.body[entry.prefix_len..].to_string().stylize(),
                ]
            }
        }
    }

    fn handle_events(&mut self) -> io::Result<bool> {
        if let Event::Key(KeyEvent {
            kind: KeyEventKind::Press,
            modifiers,
            code,
            ..
        }) = event::read()?
        {
            let redraw = match (modifiers, code) {
                (KeyModifiers::NONE, KeyCode::Esc)
                | (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                    self.exit_value = Some(None);
                    false
                }
                (KeyModifiers::NONE, KeyCode::Char(ch)) => self.input_type(ch),
                (KeyModifiers::NONE, KeyCode::Enter) => self.try_manual_accept(),
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
                _ => false,
            };
            Ok(redraw)
        } else {
            Ok(false)
        }
    }

    fn input_type(&mut self, ch: char) -> bool {
        let mut new_input = self.input.clone();
        new_input.push(ch);

        if !self.args.unrestricted_input && !self.menu.has_selectable(&new_input) {
            false
        } else {
            self.input = new_input;

            if !self.args.no_auto_accept {
                self.try_auto_accept();
            }

            self.menu.update_selection(&self.input);
            true
        }
    }

    fn input_delete_char(&mut self) -> bool {
        if self.input.is_empty() {
            false
        } else {
            self.input.pop();
            true
        }
    }

    fn input_delete_word(&mut self) -> bool {
        if self.input.is_empty() {
            false
        } else {
            self.input = string::delete_word(&self.input);
            true
        }
    }

    fn move_selection(&mut self, direction: SearchDirection) -> bool {
        self.menu
            .move_selection(&self.input, direction, !self.args.no_wrap)
    }

    fn try_manual_accept(&mut self) -> bool {
        if let Some(entry) = self.menu.manual_accept() {
            self.exit_value = Some(Some(entry.body.clone()));
            true
        } else {
            false
        }
    }

    fn try_auto_accept(&mut self) -> bool {
        if let Some(accepted_entry) = self.menu.find_acceptable(&self.input) {
            self.exit_value = Some(Some(accepted_entry.body.clone()));
            true
        } else {
            false
        }
    }
}
