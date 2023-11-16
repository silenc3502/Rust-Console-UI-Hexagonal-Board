use crate::board::application::port::input::write_board_use_case::WriteBoardUseCase;
use crate::board::application::port::input::write_board_command::WriteBoardCommand;
use crate::board::application::port::input::write_board_result::WriteBoardResult;

pub struct WriteBoardController<U: WriteBoardUseCase> {
    write_board_use_case: U,
}

impl<U: WriteBoardUseCase> WriteBoardController<U> {
    // 생성자
    pub fn new(write_board_use_case: U) -> Self {
        Self { write_board_use_case }
    }

    // 게시물 작성 메서드
    pub fn write_board(&mut self, command: WriteBoardCommand) -> WriteBoardResult {
        // 로깅 또는 기타 필요한 로직 수행
        // log::info!("게시물 작성 요청: {:?}", command);

        // WriteBoardUseCase를 통해 게시물 작성 요청 수행
        self.write_board_use_case.execute(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // MockWriteBoardUseCase 구조체 정의 (테스트용)
    pub struct MockWriteBoardUseCase;

    // WriteBoardUseCase 트레이트를 MockWriteBoardUseCase에 대해 구현
    impl WriteBoardUseCase for MockWriteBoardUseCase {
        fn execute(&mut self, _command: WriteBoardCommand) -> WriteBoardResult {
            // 여기에 테스트에 필요한 동작을 구현
            // 예를 들어, 더미 결과를 반환하거나 로깅을 수행할 수 있습니다.
            WriteBoardResult::new("mocked_board_id".to_string())
        }
    }

    #[test]
    fn test_write_board_controller() {
        // Arrange
        let mut controller = WriteBoardController::new(MockWriteBoardUseCase {});

        // Act
        let result = controller.write_board(WriteBoardCommand::new(
            "Test Title".to_string(),
            "Test Writer".to_string(),
            "Test Content".to_string(),
        ));

        // Assert
        assert_eq!(result.board_id(), "mocked_board_id");
    }
}