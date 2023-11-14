#[derive(Debug)]
pub struct UpdateBoardCommand {
    pub board_title: String,
    pub board_content: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_update_board_command() {
        let command = UpdateBoardCommand {
            board_title: "Sample Title".to_string(),
            board_content: "Sample Content".to_string(),
        };

        assert_eq!(command.board_title, "Sample Title");
        assert_eq!(command.board_content, "Sample Content");
    }
}
