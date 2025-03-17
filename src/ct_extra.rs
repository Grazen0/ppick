use std::io::{self, Write};

use crossterm::{
    cursor, queue,
    terminal::{self, ClearType},
};

pub fn queue_move_up_exact(file: &mut impl Write, lines: u16) -> io::Result<()> {
    if lines > 0 {
        queue!(file, cursor::MoveUp(lines))
    } else {
        Ok(())
    }
}

pub fn queue_clear_and_reset_cursor(file: &mut impl Write) -> io::Result<()> {
    queue!(file, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))
}
