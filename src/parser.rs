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
//     go into "("" and parser first sexpr
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

pub fn print_sexpr(sexpr: &Sexpr) {
    match sexpr {
        Sexpr::Atom(atom) => print_atom(atom),
        Sexpr::Cons(cons) => print_cons(cons),
    }
}

fn print_atom(atom: &Atom) {
    match atom {
        Atom::Identifier(id) => print_id(id),
        Atom::Str(str) => print!("{}", str),
        Atom::Number(num) => print!("{}", num),
        Atom::Boolean(bol) => print!("{}", bol),
        Atom::Nil => print!("NIL"),
    }
}

fn print_id(id: &Identifier) {
    match id {
        Identifier::Add => print!("+"),
        Identifier::Subtract => print!("-"),
        Identifier::Multiply => print!("*"),
        Identifier::Divide => print!("/"),
        Identifier::Greater => print!(">"),
        Identifier::GreaterOrEqual => print!(">="),
        Identifier::Listp => print!("listp"),
        Identifier::Atom => print!("atom"),
        Identifier::Null => print!("null"),
        Identifier::Eq => print!("eq"),
        Identifier::Equal => print!("equal"),
        Identifier::Cons => print!("cons"),
        Identifier::Car => print!("car"),
        Identifier::Cdr => print!("cdr"),
        Identifier::Append => print!("append"),
        Identifier::Defun => print!("defun"),
        Identifier::Eval => print!("eval"),
    }
}

fn print_cons(cons: &Cons) {
    print!("(");
    print_sexpr(&cons.car);
    print!(" . ");
    print_sexpr(&cons.cdr);
    print!(")");
}