use crate::scanner::Token;

pub enum Sexpr {
    Atom(String),
    Cons(Box<Cons>),
    Nil
}

pub struct Cons {
    car: Sexpr,
    cdr: Sexpr
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

// parse list vs cons:
//     go into "("" and parser first sexpr
//     if next token is ".", then parse next token as sexpr, then put into cons
//     otherwise, recursively parse rest of list

fn parse_cons(tokens: &Vec<Token>, current: &mut usize) -> Cons {
    let car = parse(tokens, current);
    if matches!(tokens[*current], Token::Dot) {
        // Regular cons (x . y)
        
        *current += 1;   // .
        let cdr = parse(tokens, current);
        *current += 1;  // )

        Cons {
            car,
            cdr
        }
    } else if matches!(tokens[*current], Token::RightParen) {
        *current += 1;
        Cons {
            car,
            cdr: Sexpr::Nil
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
        Sexpr::Atom(str) => print!("{}", str),
        Sexpr::Cons(cons) => print_cons(cons),
        Sexpr::Nil => print!("NIL")
    }
}

fn print_cons(cons: &Cons) {
    print!("(");
    print_sexpr(&cons.car);
    print!(" . ");
    print_sexpr(&cons.cdr);
    print!(")");
}