use crate::board::application::port::input::update_board_command::UpdateBoardCommand;

use crate::board::domain::model::board_content::BoardContent;
use crate::board::domain::model::board_title::BoardTitle;
use crate::board::domain::model::board_writer::BoardWriter;

#[derive(Debug, Clone, PartialEq)]
pub struct BoardInfo {
    board_title: BoardTitle,
    board_writer: BoardWriter,
    board_content: BoardContent,
}

impl BoardInfo {
    pub fn new(
        board_title: BoardTitle,
        board_writer: BoardWriter,
        board_content: BoardContent,
    ) -> Self {
        Self {
            board_title,
            board_writer,
            board_content,
        }
    }

    pub fn create_board_info(
        board_title: BoardTitle,
        board_writer: BoardWriter,
        board_content: BoardContent,
    ) -> Self {
        Self::new(board_title, board_writer, board_content)
    }

    pub fn get_board_title(&self) -> &BoardTitle {
        &self.board_title
    }

    pub fn get_board_content(&self) -> &BoardContent {
        &self.board_content
    }

    pub fn update(&mut self, command: &UpdateBoardCommand) -> Result<(), &'static str> {
        self.board_title.update_board_title(&command.board_title)?;
        self.board_content.update_board_content(&command.board_content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_board_info() {
        let title = BoardTitle::new("Sample Title").expect("Failed to create BoardTitle");
        let writer = BoardWriter::new("Sample Writer").expect("Failed to create BoardWriter");
        let content = BoardContent::new("Sample Content").expect("Failed to create BoardContent");

        let info = BoardInfo::create_board_info(title, writer, content);

        assert_eq!(info.board_title.title(), "Sample Title");
        assert_eq!(info.board_writer.writer(), "Sample Writer");
        assert_eq!(info.board_content.content(), "Sample Content");
    }

    #[test]
    fn test_update_board_info() {
        let mut info = create_sample_info();

        let update_command = UpdateBoardCommand {
            board_title: "Updated Title".to_string(),
            board_content: "Updated Content".to_string(),
        };

        let _ = info.update(&update_command);

        assert_eq!(info.board_title.title(), "Updated Title");
        assert_eq!(info.board_content.content(), "Updated Content");
    }

    fn create_sample_info() -> BoardInfo {
        let title = BoardTitle::new("Sample Title").expect("Failed to create BoardTitle");
        let writer = BoardWriter::new("Sample Writer").expect("Failed to create BoardWriter");
        let content = BoardContent::new("Sample Content").expect("Failed to create BoardContent");

        BoardInfo::create_board_info(title, writer, content)
    }
}
