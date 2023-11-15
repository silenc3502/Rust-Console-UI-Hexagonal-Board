use crate::board::application::port::output::store_board_port::StoreBoardPort;
use crate::board::application::port::input::write_board_use_case::WriteBoardUseCase;
use crate::board::application::port::input::write_board_command::WriteBoardCommand;
use crate::board::application::port::input::write_board_result::WriteBoardResult;

use crate::board::domain::model::board::Board;
use crate::board::domain::model::board_info::BoardInfo;
use crate::board::domain::model::board_content::BoardContent;
use crate::board::domain::model::board_title::BoardTitle;
use crate::board::domain::model::board_writer::BoardWriter;

pub struct WriteBoardService {
    store_board_port: Box<dyn StoreBoardPort>,
}

// WriteFreeBoardService에 대한 생성자 및 WriteFreeBoardUseCase 트레이트 구현
impl WriteBoardService {
    pub fn new(store_board_port: Box<dyn StoreBoardPort>) -> Self {
        Self { store_board_port }
    }
}

impl WriteBoardUseCase for WriteBoardService {
    fn execute(&mut self, command: WriteBoardCommand) -> WriteBoardResult {
        // 더미 데이터를 사용하여 결과 생성
        let new_board = Board::create_new_board(BoardInfo::create_board_info(
            BoardTitle::new(&command.get_board_title()).unwrap(),
            BoardWriter::new(&command.get_board_writer()).unwrap(),
            BoardContent::new(&command.get_board_content()).unwrap(),
        ));

        // 생성된 보드를 저장
        self.store_board_port.store(new_board.clone());

        // 결과 생성
        WriteBoardResult::new(new_board.get_board_id().to_string())
    }
}

pub struct MockStoreBoardPort {
    pub stored_board: Option<Board>,
}

impl StoreBoardPort for MockStoreBoardPort {
    fn store(&mut self, board: Board) {
        self.stored_board = Some(board);
    }

    fn inspect_store(&self) -> Option<&Board> {
        self.stored_board.as_ref()
    }
}


// 테스트 코드
#[cfg(test)]
mod tests {
    use super::*;

    trait InspectStore {
        fn inspect_store(&self) -> Option<&Board>;
    }

    // Implement the trait for MockStoreBoardPort
    impl InspectStore for MockStoreBoardPort {
        fn inspect_store(&self) -> Option<&Board> {
            self.stored_board.as_ref()
        }
    }

    #[test]
    fn test_write_board_service_execute() {
        // Remove mutability from mock_store_port
        let mock_store_port = MockStoreBoardPort { stored_board: None };

        // Create WriteBoardService with mock_store_port inside the Box
        let mut write_service = WriteBoardService::new(Box::new(mock_store_port));

        let command = WriteBoardCommand::new(
            "Test Title".to_string(),
            "Test Writer".to_string(),
            "Test Content".to_string(),
        );

        // Act
        let result = write_service.execute(command);

        // Assert
        if let Some(inner_stored_board) = write_service.store_board_port.inspect_store() {
            assert_eq!(result.board_id(), inner_stored_board.get_board_id().to_string());
        } else {
            panic!("No board was stored in the mock!");
        }
    }
}