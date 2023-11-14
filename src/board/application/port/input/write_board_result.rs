use std::fmt;

#[derive(Debug, PartialEq)]
pub struct WriteBoardResult {
    board_id: String,
}

impl WriteBoardResult {
    pub fn new(board_id: String) -> Self {
        WriteBoardResult { board_id }
    }

    pub fn board_id(&self) -> &str {
        &self.board_id
    }
}

impl fmt::Display for WriteBoardResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "WriteBoardResult {{ board_id: {} }}", self.board_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_board_result() {
        let result = WriteBoardResult::new("123".to_string());

        assert_eq!(result.board_id(), "123");
        assert_eq!(format!("{}", result), "WriteBoardResult { board_id: 123 }");
    }
}
