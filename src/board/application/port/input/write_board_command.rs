pub struct WriteFreeBoardCommand {
    pub board_title: String,
    pub board_writer: String,
    pub board_content: String,
}

impl WriteFreeBoardCommand {
    pub fn new(board_title: String, board_writer: String, board_content: String) -> Self {
        WriteFreeBoardCommand {
            board_title,
            board_writer,
            board_content,
        }
    }
}

impl WriteFreeBoardCommand {
    pub fn get_board_title(&self) -> &String {
        &self.board_title
    }

    pub fn get_board_writer(&self) -> &String {
        &self.board_writer
    }

    pub fn get_board_content(&self) -> &String {
        &self.board_content
    }
}