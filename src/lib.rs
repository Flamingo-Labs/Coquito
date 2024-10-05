#[derive(Debug, PartialEq, Eq)]
pub struct Token<'src> {
    pub lexeme: &'src str,
    pub byte_offset: usize,
    pub kind: TokenKind,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    Plus,
    Minus,
    Eq,
}

pub struct Lexer<'src> {
    /// The original input string
    input: &'src str,
    /// The remain input from `byte` onwards
    rest: &'src str,
    /// The current postion of the lexer. This byte position is *not* lexed yet
    byte: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(input: &'src str) -> Self {
        Self {
            input,
            rest: input,
            byte: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token<'src>> {
        let mut chars = self.rest.chars();
        let c = chars.next()?;
        let pos = self.byte;
        let slice = &self.rest[..c.len_utf8()];
        //let rest = self.rest;

        self.rest = chars.as_str();
        self.byte += c.len_utf8();

        let just = move |kind: TokenKind| {
            Some(Token {
                lexeme: slice,
                byte_offset: pos,
                kind,
            })
        };

        return match c {
            '+' => just(TokenKind::Plus),
            '-' => just(TokenKind::Minus),
            '=' => just(TokenKind::Eq),
            _ => None,
        };
    }
}

impl<'src> Iterator for Lexer<'src> {
    // TODO: Eventually want this to return a `Result` to denote various errors
    type Item = Token<'src>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_kinds() {
        let contents = "++--";
        let tokens: Vec<Token> = Lexer::new(&contents).collect();

        let kinds: Vec<&TokenKind> = tokens.iter().map(|token| &token.kind).collect();
        let expected = vec![
            &TokenKind::Plus,
            &TokenKind::Plus,
            &TokenKind::Minus,
            &TokenKind::Minus,
        ];

        assert_eq!(expected, kinds);
    }

    #[test]
    fn test_token_byte_offsets() {
        let contents = "++--";
        let tokens: Vec<Token> = Lexer::new(&contents).collect();

        let kinds: Vec<usize> = tokens.iter().map(|token| token.byte_offset).collect();
        let expected = vec![0, 1, 2, 3];

        assert_eq!(expected, kinds);
    }
}
