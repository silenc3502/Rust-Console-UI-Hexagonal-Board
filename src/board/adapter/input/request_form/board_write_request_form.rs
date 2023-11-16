use std::fmt;

#[derive(Debug, PartialEq)]
pub struct BoardWriteRequestForm {
    board_title: String,
    board_writer: String,
    board_content: String,
}

impl BoardWriteRequestForm {
    pub fn new(title: String, writer: String, content: String) -> Self {
        BoardWriteRequestForm {
            board_title: title,
            board_writer: writer,
            board_content: content,
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

// Debug 트레이트를 이용하여 출력 포맷 지정
impl fmt::Display for BoardWriteRequestForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "BoardWriteRequestForm {{ board_title: {}, board_writer: {}, board_content: {} }}",
            self.board_title, self.board_writer, self.board_content
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_write_request_form() {
        let parameter = BoardWriteRequestForm::new(
            String::from("Title"),
            String::from("Writer"),
            String::from("Content"),
        );

        assert_eq!(parameter.get_board_title(), "Title");
        assert_eq!(parameter.get_board_writer(), "Writer");
        assert_eq!(parameter.get_board_content(), "Content");

        let expected_str = "BoardWriteRequestForm { board_title: Title, board_writer: Writer, board_content: Content }";
        assert_eq!(format!("{}", parameter), expected_str);
    }
}
