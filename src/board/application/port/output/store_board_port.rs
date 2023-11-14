use crate::board::domain::model::board::Board;

pub trait StoreBoardPort {
    fn store(&self, free_board: Board);
}