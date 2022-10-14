struct Lexer {
    equation: Vec<char>,
    cursor: usize,
}

enum Token {
    Num(f64),
    Add,
    Sub,
    Mul,
    Div,
}

impl Lexer {
    fn new(equation: String) -> Self {
        Self {
            equation: equation.chars().collect(),
            cursor: 0,
        }
    }

    fn peek(&mut self) -> Option<char> {
        if self.cursor > self.equation.len() - 1 {
            return None;
        }
        Some(self.equation[self.cursor])
    }

    fn pop(&mut self) -> Option<char> {
        self.cursor += 1;
        if self.cursor > self.equation.len() {
            return None;
        }
        Some(self.equation[self.cursor - 1])
    }

    fn bump(&mut self, init: char) -> Result<Token, String> {
        let mut lit = String::new();
        lit.push(init);
        loop {
            if let Some(c @ '0'..='9') = self.pop() {
                lit.push(c);
            } else {
                let num = lit.parse::<f64>().unwrap();
                return Ok(Token::Num(num));
            }
        }
    }

    fn next(&mut self) -> Result<Token, String> {
        loop {
            match self.pop() {
                None => return Err(String::from("Token : no new token")),
                Some(c) => match c {
                    ' ' => continue,
                    '0'..='9' => return self.bump(c),
                    '+' => return Ok(Token::Add),
                    '-' => return Ok(Token::Sub),
                    '*' => return Ok(Token::Mul),
                    '/' => return Ok(Token::Div),
                    _ => return Err(String::from("unexpected token")),
                },
            }
        }
    }
}

//------

struct Node {}

enum Nodekind {
    Add,
    Sub,
    Mul,
    Div,
}
