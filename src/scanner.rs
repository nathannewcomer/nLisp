pub enum Token {
    LeftParen,
    RightParen,
    Dot,
    Atom(Atom),
}

#[derive(Clone)]
pub enum Atom {
    Str(String),
    Identifier(Identifier),
    Number(f32),
    Boolean(bool),
    Nil
}

#[derive(Clone, Debug)]
pub enum Identifier {
    // Arithmatic
    Add,
    Subtract,
    Multiply,
    Divide,
    Greater,
    GreaterOrEqual,

    // Predicate
    Listp,
    Atom,
    Null,
    Eq,
    Equal,


    // Lists
    Cons,
    Car,
    Cdr,
    Append,

    // Functions
    Defun,
    Eval,
    Quote,
    Length
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
        c if is_atom_char(&c) => Some(scan_atom(chars, start, current)),
        _ => {
            *current += 1;
            None
        }
    };

    token
}

fn scan_atom(chars: &Vec<char>, start: &mut usize, current: &mut usize) -> Token {
    while is_atom_char(&chars[*current]) {
        *current += 1;
    }

    let atom_str: String = chars[*start..*current].iter().collect();
    *start = *current;

    // Number
    if let Ok(num) = atom_str.parse::<f32>() {
        return Token::Atom(Atom::Number(num))
    }
    // Boolean
    if let Ok(bol) = atom_str.parse::<bool>() {
        return Token::Atom(Atom::Boolean(bol));
    }
    
    // Identifier
    if let Some(id) = match_identifier(&atom_str) {
        return Token::Atom(Atom::Identifier(id))
    }
    
    // String
    Token::Atom(Atom::Str(atom_str))
}

fn is_atom_char(c: &char) -> bool {
    match c {
        '+' | '-' | '*' | '/' | '=' | '>' => true,
        c if c.is_alphanumeric() => true,
        _ => false
    }
}

fn match_identifier(id: &String) -> Option<Identifier> {
    match id.as_str() {
        // Arithmatic
        "+" => Some(Identifier::Add),
        "-" => Some(Identifier::Subtract),
        "*" => Some(Identifier::Multiply),
        "/" => Some(Identifier::Divide),
        ">" => Some(Identifier::Greater),
        ">=" => Some(Identifier::GreaterOrEqual),

        // Predicate
        "listp" => Some(Identifier::Listp),
        "atom" => Some(Identifier::Atom),
        "null" => Some(Identifier::Null),
        "eq" => Some(Identifier::Eq),
        "equal" => Some(Identifier::Equal),

        // Lists
        "cons" => Some(Identifier::Cons),
        "car" => Some(Identifier::Car),
        "cdr" => Some(Identifier::Cdr),
        "append" => Some(Identifier::Append),

        // Functions
        "defun" => Some(Identifier::Defun),
        "eval" => Some(Identifier::Eval),

        _ => None
    }
}