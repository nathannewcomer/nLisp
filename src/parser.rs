use crate::scanner::{Atom, Identifier, Token};

pub enum Sexpr {
    Atom(Atom),
    Cons(Box<Cons>),
}

pub struct Cons {
    pub car: Sexpr,
    pub cdr: Sexpr
}

pub fn parse(tokens: &Vec<Token>, current: &mut usize) -> Sexpr {
    match &tokens[*current] {
        Token::LeftParen => {
            *current += 1;
            Sexpr::Cons(Box::new(parse_cons(tokens, current)))
        },
        Token::Atom(atom) => {
            *current += 1;
            Sexpr::Atom(atom.clone())
        },
        _ => panic!("Uh oh!")
    }
}

// parse list vs cons:
//     go into "(" and parser first sexpr
//     if next token is ".", then parse next token as sexpr, then put into cons
//     otherwise, recursively parse rest of list

fn parse_cons(tokens: &Vec<Token>, current: &mut usize) -> Cons {
    let car = parse(tokens, current);
    if matches!(tokens[*current], Token::RightParen) {
        *current += 1;
        Cons {
            car,
            cdr: Sexpr::Atom(Atom::Nil)
        }
    } else {
        // list
        let cdr = parse_cons(tokens, current);
        Cons {
            car,
            cdr: Sexpr::Cons(Box::new(cdr))
        }
    }
}

pub fn print_sexpr(sexpr: &Sexpr) -> String {
    match sexpr {
        Sexpr::Atom(atom) => print_atom(atom),
        Sexpr::Cons(cons) => print_cons(cons),
    }
}

fn print_atom(atom: &Atom) -> String {
    match atom {
        Atom::Identifier(id) => print_id(id),
        Atom::Str(str) => format!("{}", str),
        Atom::Number(num) => format!("{}", num),
        Atom::Boolean(bol) => match bol {
            true => "#t".to_string(),
            false => "#f".to_string(),
        },
        Atom::Nil => "NIL".to_string(),
    }
}

fn print_id(id: &Identifier) -> String{
    match id {
        Identifier::Add => format!("+"),
        Identifier::Subtract => format!("-"),
        Identifier::Multiply => format!("*"),
        Identifier::Divide => format!("/"),
        Identifier::Greater => format!(">"),
        Identifier::GreaterOrEqual => format!(">="),
        Identifier::Listp => format!("listp"),
        Identifier::Atom => format!("atom"),
        Identifier::Null => format!("null"),
        Identifier::Eq => format!("eq"),
        Identifier::Equal => format!("equal"),
        Identifier::Cons => format!("cons"),
        Identifier::Car => format!("car"),
        Identifier::Cdr => format!("cdr"),
        Identifier::Append => format!("append"),
        Identifier::Defun => format!("defun"),
        Identifier::Eval => format!("eval"),
        Identifier::Quote => format!("quote"),
        Identifier::Length => format!("length")
    }
}

fn print_cons(cons: &Cons) -> String {
    format!("({} . {})", print_sexpr(&cons.car), print_sexpr(&cons.cdr))
}