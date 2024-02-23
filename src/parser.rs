use crate::scanner::Token;

pub enum Sexpr {
    Atom(String),
    Cons(Box<Cons>),
}

struct Cons {
    car: Sexpr,
    cdr: Option<Box<Cons>>
}

pub fn parse(tokens: &Vec<Token>, current: &mut usize) -> Sexpr {
    match &tokens[*current] {
        Token::LeftParen => {
            *current += 1;
            Sexpr::Cons(Box::new(parse_cons(tokens, current)))
        },
        Token::Atom(atom) => {
            *current += 1;
            Sexpr::Atom(atom.to_string())
        },
        _ => panic!("Uh oh!")
    }
}

fn parse_cons(tokens: &Vec<Token>, current: &mut usize) -> Cons {
    let car = parse(tokens, current);
    let cdr = match tokens[*current] {
        Token::RightParen => {
            *current += 1;
            None
        },
        _ => {
            Some(Box::new(parse_cons(tokens, current)))
        }
    };

    Cons {
        car,
        cdr
    }
}

pub fn print_sexpr(sexpr: &Sexpr) {
    match sexpr {
        Sexpr::Atom(str) => print!("{}", str),
        Sexpr::Cons(cons) => print_cons(cons),
    }
}

fn print_cons(cons: &Cons) {
    print!("(");
    print_sexpr(&cons.car);
    print!(" . ");
    if let Some(cdr) = &cons.cdr {
        print_cons(&cdr);
    } else {
        print!("NIL");
    }
    print!(")");
}