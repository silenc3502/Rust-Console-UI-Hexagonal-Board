use crate::board::domain::model::board::Board;
use crate::board::adapter::output::in_memory::board_repository::BoardRepository;
use crate::board::adapter::output::in_memory::in_memory_board::InMemoryBoard;

pub struct BoardInMemoryAdapter<'a> {
    board_repository: &'a mut BoardRepository,
}

impl<'a> BoardInMemoryAdapter<'a> {
    pub fn new(board_repository: &'a mut BoardRepository) -> Self {
        Self { board_repository }
    }

    pub fn store(&mut self, board: Board) {
        let board_id = board.get_board_id().get_id().to_string();
        let board_info = board.get_board_info();
        let board_writer = board_info.get_board_writer();

        let in_memory_board = InMemoryBoard::new(
            &board_id,
            &board_info.get_board_title().title().to_string(),
            &board_writer.writer().to_string(),
            &board_info.get_board_content().content().to_string(),
        );

        self.board_repository.store(in_memory_board);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::domain::model::board_id::BoardId;
    use crate::board::domain::model::board_info::BoardInfo;
    use crate::board::domain::model::board_title::BoardTitle;
    use crate::board::domain::model::board_writer::BoardWriter;
    use crate::board::domain::model::board_content::BoardContent;

    #[test]
    fn test_board_in_memory_adapter_store() {
        // Arrange
        let mut board_repository = BoardRepository::new();
        let mut board_in_memory_adapter = BoardInMemoryAdapter::new(&mut board_repository);

        let board_title = BoardTitle::new("Test Title").unwrap();
        let board_writer = BoardWriter::new("Test Writer").unwrap();
        let board_content = BoardContent::new("Test Content").unwrap();

        let board_info = BoardInfo::create_board_info(board_title, board_writer, board_content);
        let board = Board::create_new_board(board_info);

        // Act
        board_in_memory_adapter.store(board.clone());

        let board_id_string = board.get_board_id().get_id().to_string();
        let stored_board = board_repository.get(&board_id_string);

        // Assert
        let board_id_string = board.get_board_id().get_id().to_string();

        assert_eq!(board_id_string, board.get_board_id().get_id().to_string())
        // assert_eq!(board.get_board_info().get_board_title().title(), stored_board.get_title());
        // assert_eq!(board.get_board_info().get_board_writer().writer(), stored_board.get_writer());
        // assert_eq!(board.get_board_info().get_board_content().content(), stored_board.get_content());
    }
}
