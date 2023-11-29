// use chrono::{DateTime, Local};
// use uuid::Uuid;
//
// #[derive(Debug, Clone, PartialEq)]
// pub struct InMemoryBoard {
//     id: String,
//     title: String,
//     writer: String,
//     content: String,
//     created_at: DateTime<Local>,
//     updated_at: DateTime<Local>,
// }
//
// impl InMemoryBoard {
//     pub fn new(title: &str, writer: &str, content: &str) -> Self {
//         let now = Local::now();
//
//         Self {
//             id: Uuid::new_v4().to_string(),
//             title: title.to_string(),
//             writer: writer.to_string(),
//             content: content.to_string(),
//             created_at: now,
//             updated_at: now,
//         }
//     }
//
//     pub fn get_id(&self) -> &str {
//         &self.id
//     }
//
//     pub fn get_title(&self) -> &str {
//         &self.title
//     }
//
//     pub fn get_writer(&self) -> &str {
//         &self.writer
//     }
//
//     pub fn get_content(&self) -> &str {
//         &self.content
//     }
//
//     pub fn get_created_at(&self) -> &DateTime<Local> {
//         &self.created_at
//     }
//
//     pub fn get_updated_at(&self) -> &DateTime<Local> {
//         &self.updated_at
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_in_memory_board_creation() {
//         let in_memory_board =
//             InMemoryBoard::new("Test Title", "Test Writer", "Test Content");
//
//         assert_eq!(in_memory_board.get_title(), "Test Title");
//         assert_eq!(in_memory_board.get_writer(), "Test Writer");
//         assert_eq!(in_memory_board.get_content(), "Test Content");
//     }
// }

use chrono::{DateTime, Local};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub struct InMemoryBoard {
    id: String,
    title: String,
    writer: String,
    content: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

impl InMemoryBoard {
    pub fn new(id: &str, title: &str, writer: &str, content: &str) -> Self {
        let now = Local::now();

        Self {
            id: id.to_string(),
            title: title.to_string(),
            writer: writer.to_string(),
            content: content.to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_writer(&self) -> &str {
        &self.writer
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_created_at(&self) -> &DateTime<Local> {
        &self.created_at
    }

    pub fn get_updated_at(&self) -> &DateTime<Local> {
        &self.updated_at
    }
}

// Implementing Clone manually
impl Clone for InMemoryBoard {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            title: self.title.clone(),
            writer: self.writer.clone(),
            content: self.content.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_board_creation() {
        let in_memory_board = InMemoryBoard::new("Test Title", "Test Writer", "Test Content");

        assert_eq!(in_memory_board.get_title(), "Test Title");
        assert_eq!(in_memory_board.get_writer(), "Test Writer");
        assert_eq!(in_memory_board.get_content(), "Test Content");

        // Cloning the board
        let cloned_board = in_memory_board.clone();
        assert_eq!(in_memory_board, cloned_board);
    }
}
