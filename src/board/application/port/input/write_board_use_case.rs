use crate::board::application::port::input::write_board_result::WriteBoardResult;
use crate::board::application::port::input::write_board_command::WriteBoardCommand;
pub trait WriteBoardUseCase {
    fn execute(&mut self, command: WriteBoardCommand) -> WriteBoardResult;
}