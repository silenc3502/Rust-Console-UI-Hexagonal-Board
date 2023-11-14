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
    fn execute(&self, command: WriteBoardCommand) -> WriteBoardResult {
        let new_board = Board::create_new_board(BoardInfo::create_board_info(
            BoardTitle::new(&command.board_title).unwrap(),
            BoardWriter::new(&command.board_writer).unwrap(),
            BoardContent::new(&command.board_content).unwrap(),
        ));

        self.store_board_port.store(new_board.clone());

        WriteBoardResult::new(new_board.get_board_id().to_string())
    }
}

struct Board;
struct BoardInfo;
struct BoardTitle;
struct BoardWriter;
struct BoardContent;

// 테스트 코드
#[cfg(test)]
mod tests {
    use super::*;

    // Mock 구현을 사용하여 테스트
    struct MockStoreBoardPort;
    impl StoreBoardPort for MockStoreBoardPort {
        fn store(&self, _board: Board) {
            // 실제 저장을 수행하지 않음
        }
    }

    #[test]
    fn test_write_board_service_execute() {
        // Arrange
        let store_port = Box::new(MockStoreBoardPort {}) as Box<dyn StoreBoardPort>;
        let write_service = WriteBoardService::new(store_port);
        let command = WriteBoardCommand {
            board_title: "Test Title".to_string(),
            board_writer: "Test Writer".to_string(),
            board_content: "Test Content".to_string(),
        };

        // Act
        let result = write_service.execute(command);

        // Assert
        assert_eq!(result.board_id(), "mocked_id");
    }
}