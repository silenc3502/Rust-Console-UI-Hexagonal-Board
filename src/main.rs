mod board;

use crate::board::domain::model::board::Board;
use crate::board::domain::model::board_info::BoardInfo;
use crate::board::domain::model::board_title::BoardTitle;
use crate::board::adapter::output::in_memory::board_repository::BoardRepository;
use crate::board::adapter::output::in_memory::board_in_memory_adapter::BoardInMemoryAdapter;
use crate::board::application::port::input::write_board_command::WriteBoardCommand;
use crate::board::application::port::input::write_board_result::WriteBoardResult;
use crate::board::application::port::input::write_board_use_case::WriteBoardUseCase;
use crate::board::application::services::write_board_service::WriteBoardService;
use crate::board::domain::model::board_content::BoardContent;
use crate::board::domain::model::board_writer::BoardWriter;

fn main() {
    let mut board_repository = BoardRepository::new();
    let mut board_in_memory_adapter = BoardInMemoryAdapter::new(&mut board_repository);

    let board_title = BoardTitle::new("Sample Title").expect("Failed to create BoardTitle");
    let board_writer = BoardWriter::new("Sample Writer").expect("Failed to create BoardWriter");
    let board_content = BoardContent::new("Sample Content").expect("Failed to create BoardContent");

    let info = BoardInfo::create_board_info(board_title, board_writer, board_content);
    let board = Board::create_new_board(info.clone());

    board_in_memory_adapter.store(board.clone());

    let board_id_string = board.get_board_id().get_id().to_string();

    println!("Board ID for retrieval: {}", board_id_string);

    let stored_board = board_repository.get(&board_id_string);

    match stored_board {
        Some(board) => {
            println!("Stored Board ID: {}", board.get_id());
            println!("Stored Board Title: {}", board.get_title());
            println!("Stored Board Writer: {}", board.get_writer());
            println!("Stored Board Content: {}", board.get_content());
        }
        None => println!("Board not found in the repository."),
    }
}
