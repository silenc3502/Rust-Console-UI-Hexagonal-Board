use std::fmt;

#[derive(Debug, PartialEq)]
pub struct WriteBoardResponseForm {
    board_id: String,
}

impl WriteBoardResponseForm {
    pub fn new(board_id: String) -> Self {
        WriteBoardResponseForm { board_id }
    }

    pub fn get_board_id(&self) -> &str {
        &self.board_id
    }
}

impl fmt::Display for WriteBoardResponseForm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WriteBoardResponseForm {{ board_id: {} }}", self.board_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_board_response_form() {
        let response = WriteBoardResponseForm::new("123".to_string());

        assert_eq!(response.get_board_id(), "123");

        let expected_str = "WriteBoardResponseForm { board_id: 123 }";
        assert_eq!(format!("{}", response), expected_str);
    }
}
