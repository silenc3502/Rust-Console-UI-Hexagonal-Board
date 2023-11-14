use super::StoreBoardPort;

pub struct WriteBoardService {
    store_free_board_port: Box<dyn StoreFreeBoardPort>,
}