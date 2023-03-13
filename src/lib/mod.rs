pub mod text {
    struct Lexer<'a> {
        content: &'a [char],
        cursor: usize,
    }
    #[derive(Debug, PartialEq, Eq, Hash)]
    enum Token {
        Word(String, usize),
        Number(String, usize),
        Other(String, usize),
    }
    trait ToToken {
        fn to_number_token(&mut self) -> Token;
        fn to_word_token(&mut self) -> Token;
        fn to_other_token(&mut self) -> Token;
    }
    impl<'a> ToToken for Lexer<'a> {
        fn to_number_token(&mut self) -> Token {
            let mut n = 0;
            while n < self.content.len() && self.content[n].is_numeric() {
                n += 1;
            }
            let token = self.consume(n).iter().collect::<String>();
            let len = token.len();
            Token::Number(token, self.cursor - len)
        }
        fn to_word_token(&mut self) -> Token {
            let mut n = 0;
            while n < self.content.len() && self.content[n].is_alphabetic() {
                n += 1;
            }
            let token = self.consume(n).iter().collect::<String>();
            let len = token.len();
            Token::Word(token, self.cursor - len)
        }
        fn to_other_token(&mut self) -> Token {
            let token = self.consume(1).iter().collect::<String>();
            let len = token.len();
            Token::Other(token, self.cursor - len)
        }
    }
    impl<'a> Lexer<'a> {
        fn new(content: &'a [char]) -> Self {
            Self { content, cursor: 0 }
        }
        fn trim_left(&mut self) -> &mut Self {
            let mut n = 0;
            while self.content[n].is_whitespace() {
                n += 1;
            }
            self.content = &self.content[n..];
            self.cursor += n;
            self
        }
        fn consume(&mut self, n: usize) -> &'a [char] {
            let token = &self.content[0..n];
            self.content = &self.content[n..];
            self.cursor += n;
            token
        }
        fn next_token(&mut self) -> Option<Token> {
            if self.content.len() == 0 {
                return None;
            }
            if self.content.len() > 0 {
                self.trim_left();
            }

            if self.content[0].is_alphabetic() {
                return Some(self.to_word_token());
            } else if self.content[0].is_numeric() {
                return Some(self.to_number_token());
            }

            Some(self.to_other_token())
        }
    }
    impl<'a> Iterator for Lexer<'a> {
        type Item = Token;
        fn next(&mut self) -> Option<Self::Item> {
            self.next_token()
        }
    }
    pub fn search_in_text(text: &str, keyword: &str) -> Vec<(String, usize)> {
        let mut matches = Vec::<(String, usize)>::new();
        let ct = text.to_lowercase().chars().collect::<Vec<char>>();
        let keyword = keyword.to_lowercase();
        for token in Lexer::new(&ct) {
            match token {
                Token::Word(w, loc) => {
                    if let Some(pos) = w.find(&keyword) {
                        matches.push((w, loc + pos))
                    }
                }
                _ => (),
            }
        }
        matches
    }
}
