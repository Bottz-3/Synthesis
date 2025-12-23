#[derive(Clone, Copy, Debug, PartialEq)]
enum Piece {
    Original,
    New,
}
#[derive(Debug)]
pub struct PieceNode {
    start: usize,
    len: usize,
    source: Piece,
}

pub struct PieceTable {
    og_buffer: String,
    pub add_buffer: String,
    pub pieces: Vec<PieceNode>,
    document_len: usize,
}

impl PieceTable {
    pub fn new(og_buf: &str, add_buf: &str) -> Self {
        // have to define the original buffer as a piece, this is because when removing
        // we still need it to exist but it must not be a piece
        Self {
            og_buffer: og_buf.to_string(),
            add_buffer: add_buf.to_string(),
            pieces: vec![PieceNode {
                start: 0,
                len: og_buf.len(),
                source: Piece::Original,
            }],
            document_len: og_buf.len(),
        }
    }
    pub fn insert(&mut self, idx: usize, text: &str) {
        // idx is cursor position
        if text.is_empty() {
            return;
        }
        let start_pos = self.add_buffer.len();
        let prior_add_buff_len = self.add_buffer.len();

        self.add_buffer.push_str(text);

        // If piece is at the end or start
        if idx == self.document_len {
            self.pieces.push(PieceNode {
                start: prior_add_buff_len,
                len: text.len(),
                source: Piece::New,
            });
        }
        let mut idx_accum = 0;
        let mut target_index: Option<usize> = None;

        // Saves last piece len val
        for (index, piece) in self.pieces.iter().enumerate() {
            if idx_accum > idx {
                // Saves index of found piece
                target_index = Some(index - 1);
                break;
            }
            idx_accum += piece.len;
        }
        let index = match target_index {
            Some(val) => val,
            None => self.pieces.len() - 1,
        };
        // Remove piece is len is 0 (redundant piece if len is 0)
        let og_size = self.pieces[index].len;
        let piece_type = self.pieces[index].source;
        let mut index1 = 1;
        let mut index2 = 2;
        if idx == 0 {
            self.pieces.remove(index);
            index1 = 0;
            index2 = 1;
        } else {
            self.pieces[index].len = idx - (idx_accum - self.pieces[index].len);
        }

        self.pieces.insert(
            index + index1,
            PieceNode {
                start: start_pos,
                len: text.len(),
                source: Piece::New,
            },
        );
        self.document_len += text.len();
        // add the right side of the piece
        self.pieces.insert(
            index + index2,
            PieceNode {
                start: self.pieces[index].start + self.pieces[index].len,
                len: og_size - self.pieces[index].len,
                source: piece_type,
            },
        );
    }
    pub fn print(&self) {
        if self.pieces.len() < 1 {
            return;
        }
        // Iterate through every piece
        for piece in &self.pieces {
            if piece.source == Piece::Original {
                // Print a slice of the original buffer
                print!(
                    "{}",
                    &self.og_buffer[piece.start..(piece.start + piece.len)]
                )
            } else if piece.source == Piece::New {
                // Print a slice of the new buffer
                print!(
                    "{}",
                    &self.add_buffer[piece.start..(piece.start + piece.len)]
                )
            }
        }
        println!();
    }
    pub fn delete(start: usize, end: usize) {
        todo!();
    }

    pub fn replace(start: usize, end: usize, replacement: &str) {
        todo!();
    }
}
