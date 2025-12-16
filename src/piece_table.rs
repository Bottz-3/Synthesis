#[derive(Clone, Debug, PartialEq)]
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
        if text.is_empty() {
            println!("empty!");
            return;
        }
        let start_pos = self.add_buffer.len();
        // idx is cursor position
        self.add_buffer.push_str(text);
        // now edit the pieces
        //
        if idx == self.document_len {
            self.pieces.push(PieceNode {
                start: idx,
                len: text.len(),
                source: Piece::New,
            });
        }
        let mut idx_accum = 0;
        let mut target_index: Option<usize> = None;
        // Handling the case where a piece is in the middle of an existing piece.
        for (index, piece) in self.pieces.iter_mut().enumerate() {
            println!("{} {}", idx_accum, piece.len);
            if idx_accum < piece.len {
                println!("running!");
                // shortens the left side by the size of 'text'
                // however here it's guaranteed to pretty much be 1
                // as you call it one char at a time for the core logic
                //piece.len -= text.len();
                //
                target_index = Some(index);
                break;
            }
            idx_accum += piece.len;
        }

        println!("{}", target_index.is_none());
        let index = match target_index {
            Some(val) => val,
            None => return,
        };
        let og_size = self.pieces[index].len;
        let piece_type = self.pieces[index].source.clone();
        self.pieces[index].len = idx;

        self.pieces.insert(
            index + 1,
            PieceNode {
                start: start_pos,
                len: text.len(),
                source: Piece::New,
            },
        );
        self.document_len += text.len();
        // add the right side of the piece
        self.pieces.insert(
            index + 2,
            PieceNode {
                start: idx,
                len: og_size - idx,
                source: piece_type,
            },
        );

        println!("size of pieces: {}", self.pieces.len());
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
                    &self.og_buffer[piece.start..(piece.start + (piece.len))]
                )
            } else if piece.source == Piece::New {
                // Print a slice of the new  buffer
                print!(
                    "{}",
                    &self.add_buffer[piece.start..(piece.start + (piece.len))]
                )
            }
        }
    }
    pub fn delete(start: usize, end: usize) {
        todo!();
    }

    pub fn replace(start: usize, end: usize, replacement: &str) {
        todo!();
    }
}
