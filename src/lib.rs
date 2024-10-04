pub enum Token {
    Plus,
    Minus,
    Eq,
}

pub struct Lexer<'src> {
    /// The original input string
    input: &'src str,
    /// The current postion of the lexer. This byte position is *not* lexed yet
    byte: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(input: &'src str) -> Self {
        Self { input, byte: 0 }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
