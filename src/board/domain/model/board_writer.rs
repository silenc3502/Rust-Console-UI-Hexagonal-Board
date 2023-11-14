#[derive(Debug, Clone, PartialEq)]
pub struct BoardWriter {
    writer: String,
}

impl BoardWriter {
    pub fn new(writer: &str) -> Result<Self, &'static str> {
        Self::check_writer_validation(writer)?;
        Ok(Self {
            writer: writer.to_string(),
        })
    }

    pub fn writer(&self) -> &str {
        &self.writer
    }

    fn check_writer_validation(writer: &str) -> Result<(), &'static str> {
        if writer.is_empty() {
            Err("작성자를 입력해야 합니다!")
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_writer() {
        let writer = BoardWriter::new("Sample Writer").expect("Failed to create BoardWriter");
        assert_eq!(writer.writer, "Sample Writer");
    }

    #[test]
    fn test_empty_writer() {
        let result = BoardWriter::new("");
        assert!(result.is_err());
    }
}
