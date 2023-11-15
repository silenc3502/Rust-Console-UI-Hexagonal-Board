use crate::board::domain::model::board::Board;

pub trait StoreBoardPort {
    fn store(&mut self, board: Board);
    fn inspect_store(&self) -> Option<&Board>;
}