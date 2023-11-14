#[derive(Debug, Clone, PartialEq)]
pub struct BoardContent {
    content: String,
}

impl BoardContent {
    pub fn new(content: &str) -> Result<Self, &'static str> {
        Self::check_content_validation(content)?;
        Ok(Self {
            content: content.to_string(),
        })
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    fn check_content_validation(content: &str) -> Result<(), &'static str> {
        if content.is_empty() {
            Err("내용을 입력해야 합니다!")
        } else {
            Ok(())
        }
    }

    pub fn update_board_content(&mut self, content: &str) -> Result<(), &'static str> {
        Self::check_content_validation(content)?;
        self.content = content.to_string();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_content_creation() {
        let content = BoardContent::new("Sample Content").expect("Failed to create BoardContent");
        assert_eq!(content.content, "Sample Content");
    }

    #[test]
    fn test_empty_content_creation() {
        let result = BoardContent::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_content_update() {
        let mut content = BoardContent::new("Old Content").expect("Failed to create BoardContent");
        content
            .update_board_content("New Content")
            .expect("Failed to update content");
        assert_eq!(content.content, "New Content");
    }

    #[test]
    fn test_empty_content_update() {
        let mut content = BoardContent::new("Old Content").expect("Failed to create BoardContent");
        let result = content.update_board_content("");
        assert!(result.is_err());
    }
}
