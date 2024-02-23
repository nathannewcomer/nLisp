pub enum Token {
    LeftParen,
    RightParen,
    Dot,
    Atom(String),
}

pub fn scan(line: &String) -> Vec<Token> {
    let chars: Vec<char> = line.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();

    let mut start: usize = 0;
    let mut current: usize = 0;

    while current < chars.len() {
        start = current;
        if let Some(token) = scan_token(&chars, &mut start, &mut current) {
            tokens.push(token);
        }
    }

    tokens
}

fn scan_token(chars: &Vec<char>, start: &mut usize, current: &mut usize) -> Option<Token> {
    let token = match chars[*current] {
        '(' => {
            *current += 1;
            Some(Token::LeftParen)
        },
        ')' => {
            *current += 1;
            Some(Token::RightParen)
        },
        '.' => {
            *current += 1;
            Some(Token::Dot)
        },
        c if c.is_ascii_alphanumeric() => Some(scan_atom(chars, start, current)),
        _ => {
            *current += 1;
            None
        }
    };

    token
}

fn scan_atom(chars: &Vec<char>, start: &mut usize, current: &mut usize) -> Token {
    while chars[*current].is_alphanumeric() {
        *current += 1;
    }

    let atom: String = chars[*start..*current].iter().collect();
    *start = *current;
    Token::Atom(atom)
}