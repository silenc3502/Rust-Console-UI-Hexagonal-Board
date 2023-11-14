use crate::board::application::port::input::write_board_result::WriteBoardResult;
use crate::board::application::port::input::write_board_command::WriteBoardCommand;

pub trait WriteFreeBoardUseCase {
    fn execute(&self, command: WriteBoardCommand) -> WriteBoardResult;
}