use crate::board::adapter::output::in_memory::in_memory_board::InMemoryBoard;

use std::collections::HashMap;
use crate::board::application::port::output::store_board_port::StoreBoardPort;
use crate::board::domain::model::board::Board;


#[derive(Debug)]
pub struct BoardRepository {
    storage: HashMap<String, InMemoryBoard>,
}

impl BoardRepository {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn store(&mut self, in_memory_board: InMemoryBoard) {
        self.storage.insert(in_memory_board.get_id().to_string(), in_memory_board);
    }

    pub fn get(&self, id: &str) -> Option<&InMemoryBoard> {
        println!("Searching for board with ID: {}", id);

        let searched_lowercased_id = id.to_lowercase();

        for (stored_id, board) in self.storage.iter() {
            let stored_lowercased_id = stored_id.to_lowercase();
            println!("Stored Board ID: {}", stored_lowercased_id);
            // Print other board information if needed

            if stored_lowercased_id == searched_lowercased_id {
                println!("Board found!");
                return Some(board);
            }
        }

        None
    }

    pub fn remove(&mut self, id: &str) -> Option<InMemoryBoard> {
        self.storage.remove(id)
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_storage() {
        let mut in_memory_storage = BoardRepository::new();

        let sample_board = InMemoryBoard::new("Sample Title", "Sample Writer", "Sample Content");

        in_memory_storage.store(sample_board.clone());
        assert_eq!(in_memory_storage.get(sample_board.get_id()), Some(&sample_board));

        let removed_board = in_memory_storage.remove(sample_board.get_id());
        assert_eq!(removed_board, Some(sample_board.clone()));
        assert_eq!(in_memory_storage.get(sample_board.get_id()), None);
    }
}
