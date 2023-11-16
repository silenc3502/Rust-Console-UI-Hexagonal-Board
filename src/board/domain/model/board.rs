use crate::board::domain::model::board_id::BoardId;
use crate::board::domain::model::board_info::BoardInfo;
use crate::board::application::port::input::update_board_command::UpdateBoardCommand;

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Board {
    board_id: BoardId,
    board_info: BoardInfo,
    create_at: DateTime<Utc>,
    update_at: DateTime<Utc>,
}

impl Board {
    pub fn new(board_id: BoardId, board_info: BoardInfo, create_at: DateTime<Utc>, update_at: DateTime<Utc>) -> Self {
        Self {
            board_id,
            board_info,
            create_at,
            update_at,
        }
    }

    pub fn create_new_board(board_info: BoardInfo) -> Self {
        let now = Utc::now();
        let board_id = BoardId::new();
        Self::new(board_id, board_info, now, now)
    }

    pub fn update(&mut self, command: UpdateBoardCommand) {
        let _ = self.board_info.update(&command);
        self.update_at = Utc::now();
    }

    pub fn get_board_id(&self) -> &BoardId {
        &self.board_id
    }

    pub fn get_board_info(&self) -> &BoardInfo {
        &self.board_info
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::domain::model::board_content::BoardContent;
    use crate::board::domain::model::board_title::BoardTitle;
    use crate::board::domain::model::board_writer::BoardWriter;

    #[test]
    fn test_create_new_board() {
        let title = BoardTitle::new("Sample Title").expect("Failed to create BoardTitle");
        let writer = BoardWriter::new("Sample Writer").expect("Failed to create BoardWriter");
        let content = BoardContent::new("Sample Content").expect("Failed to create BoardContent");

        let info = BoardInfo::create_board_info(title, writer, content);
        let board = Board::create_new_board(info.clone());

        assert_eq!(board.get_board_info(), &info);
        assert_ne!(board.get_board_id(), &BoardId::nil());
        assert!(board.create_at <= Utc::now());
    }

    #[test]
    fn test_update_board() {
        let title = BoardTitle::new("Sample Title").expect("Failed to create BoardTitle");
        let writer = BoardWriter::new("Sample Writer").expect("Failed to create BoardWriter");
        let content = BoardContent::new("Sample Content").expect("Failed to create BoardContent");

        let mut board = Board::create_new_board(BoardInfo::create_board_info(title, writer, content.clone()));

        let update_command = UpdateBoardCommand {
            board_title: "Updated Title".to_string(),
            board_content: "Updated Content".to_string(),
        };

        board.update(update_command);

        assert_eq!(board.get_board_info().get_board_title().title(), "Updated Title");
        assert_eq!(board.get_board_info().get_board_content().content(), "Updated Content");
    }
}
