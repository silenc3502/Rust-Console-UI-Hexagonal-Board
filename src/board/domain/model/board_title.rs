#[derive(Debug, Clone, PartialEq)]
pub struct BoardTitle {
    title: String,
}

impl BoardTitle {
    pub fn new(title: &str) -> Result<Self, &'static str> {
        Self::check_title_validation(title)?;
        Ok(Self {
            title: title.to_string(),
        })
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    fn check_title_validation(title: &str) -> Result<(), &'static str> {
        if title.is_empty() {
            Err("제목을 입력해야 합니다!")
        } else {
            Ok(())
        }
    }

    pub fn update_board_title(&mut self, title: &str) -> Result<(), &'static str> {
        Self::check_title_validation(title)?;
        self.title = title.to_string();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_title_creation() {
        let title = BoardTitle::new("Sample Title").expect("Failed to create BoardTitle");
        assert_eq!(title.title, "Sample Title");
    }

    #[test]
    fn test_empty_title_creation() {
        let result = BoardTitle::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_title_update() {
        let mut title = BoardTitle::new("Old Title").expect("Failed to create BoardTitle");
        title
            .update_board_title("New Title")
            .expect("Failed to update title");
        assert_eq!(title.title, "New Title");
    }

    #[test]
    fn test_empty_title_update() {
        let mut title = BoardTitle::new("Old Title").expect("Failed to create BoardTitle");
        let result = title.update_board_title("");
        assert!(result.is_err());
    }
}
