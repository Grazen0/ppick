use crossterm::cursor;

#[derive(Debug, Clone, Copy)]
pub struct MoveUpExact(pub u16);

impl crossterm::Command for MoveUpExact {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        if self.0 > 0 {
            cursor::MoveUp(self.0).write_ansi(f)
        } else {
            Ok(())
        }
    }
}
