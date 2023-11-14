use super::WriteBoardService;
use super::StoreBoardPort;

impl WriteBoardService {
    pub fn new(store_board_port: Box<dyn StoreBoardPort>) -> Self {
        WriteBoardService { store_board_port }
    }

}