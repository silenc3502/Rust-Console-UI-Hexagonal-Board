use std::fmt;

#[derive(Debug, PartialEq)]
pub struct WriteBoardCommand {
    board_title: String,
    board_writer: String,
    board_content: String,
}

impl WriteBoardCommand {
    pub fn new(
        board_title: String,
        board_writer: String,
        board_content: String,
    ) -> Self {
        Self {
            board_title,
            board_writer,
            board_content,
        }
    }

    pub fn get_board_title(&self) -> &str {
        &self.board_title
    }

    pub fn get_board_writer(&self) -> &str {
        &self.board_writer
    }

    pub fn get_board_content(&self) -> &str {
        &self.board_content
    }
}

impl fmt::Display for WriteBoardCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WriteBoardCommand {{ board_title: {}, board_writer: {}, board_content: {} }}",
            self.board_title, self.board_writer, self.board_content
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_board_command() {
        let command = WriteBoardCommand::new(
            String::from("Title"),
            String::from("Writer"),
            String::from("Content"),
        );

        assert_eq!(command.get_board_title(), "Title");
        assert_eq!(command.get_board_writer(), "Writer");
        assert_eq!(command.get_board_content(), "Content");

        let expected_str = "WriteBoardCommand { board_title: Title, board_writer: Writer, board_content: Content }";
        assert_eq!(format!("{}", command), expected_str);
    }
}
