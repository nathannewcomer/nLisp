use crate::{parser::{Cons, Sexpr}, scanner::{Atom, Identifier}};

pub struct Function {
    name: String,
    args: Sexpr,
    body: Sexpr
}

pub fn evaluate(sexpr: Sexpr) -> Sexpr {
    match sexpr {
        Sexpr::Atom(atom) => evaluate_atom(atom),
        Sexpr::Cons(cons) => evalute_cons(*cons),
    }
}

fn evalute_cons(cons: Cons) -> Sexpr {
    match cons.car {
        Sexpr::Atom(atom) => {
            if let Atom::Identifier(id) = atom {
                match id {
                    Identifier::Add => arithmetic(|x, y| x + y, cons.cdr),
                    Identifier::Subtract => arithmetic(|x, y| x - y, cons.cdr),
                    Identifier::Multiply => arithmetic(|x, y| x * y, cons.cdr),
                    Identifier::Divide => arithmetic(|x, y| x / y, cons.cdr),
                    Identifier::Greater => conditional(|x, y| x > y, cons.cdr),
                    Identifier::GreaterOrEqual => conditional(|x, y| x >= y, cons.cdr),
                    Identifier::Listp => todo!(),
                    Identifier::Atom => todo!(),
                    Identifier::Null => todo!(),
                    Identifier::Eq => todo!(),
                    Identifier::Equal => todo!(),
                    Identifier::Cons => todo!(),
                    Identifier::Car => todo!(),
                    Identifier::Cdr => todo!(),
                    Identifier::Append => todo!(),
                    Identifier::Defun => todo!(),
                    Identifier::Eval => todo!(),
                    Identifier::Quote => todo!(),
                    Identifier::Length => length(cons.cdr)
                }
            } else {
                panic!("Error: Operator is not a procedure");
            }
        },
        Sexpr::Cons(cons) => evalute_cons(*cons),
    }
}

fn evaluate_atom(atom: Atom) -> Sexpr {
    match atom {
        Atom::Identifier(id) => panic!("Error: expected atom, found identifier {:?}", id),
        Atom::Str(str) => Sexpr::Atom(Atom::Str(str)),
        Atom::Number(num) => Sexpr::Atom(Atom::Number(num)),
        Atom::Boolean(bol) => Sexpr::Atom(Atom::Boolean(bol)),
        Atom::Nil => Sexpr::Atom(Atom::Nil),
    }
}

fn arithmetic(f: fn(f32, f32) -> f32, sexpr: Sexpr) -> Sexpr {
    match sexpr {
        Sexpr::Atom(atom) => evaluate_atom(atom),
        Sexpr::Cons(cons) => {
            let car = evaluate(cons.car);
            let cdr = arithmetic(f, cons.cdr);

            match (car, cdr) {
                // Two numbers
                (Sexpr::Atom(Atom::Number(x)), Sexpr::Atom(Atom::Number(y)))
                    => Sexpr::Atom(Atom::Number(f(x, y))),
                // Number and NIL
                (x @ Sexpr::Atom(Atom::Number(_)), Sexpr::Atom(Atom::Nil)) => x,
                // Other: (Number, String), (Nil, Number), etc. We can't evaluate these
                _ => panic!("Error: Either car or cdr is not a number")
            }
        },
    }
}

fn conditional(f: fn(f32, f32) -> bool, sexpr: Sexpr) -> Sexpr {
    match sexpr {
        Sexpr::Atom(atom) => evaluate_atom(atom),
        Sexpr::Cons(cons) => {
            let car = evaluate(cons.car);
            let cdr = conditional(f, cons.cdr);

            match (car, cdr) {
                // Two numbers
                (Sexpr::Atom(Atom::Number(x)), Sexpr::Atom(Atom::Number(y)))
                    => Sexpr::Atom(Atom::Boolean(f(x, y))),
                // Number and NIL
                (x @ Sexpr::Atom(Atom::Number(_)), Sexpr::Atom(Atom::Nil)) => x,
                // Other: (Number, String), (Nil, Number), etc. We can't evaluate these
                _ => panic!("Error: Either car or cdr is not a number")
            }
        },
    }
}

fn length(sexpr: Sexpr) -> Sexpr {
    match sexpr {
        Sexpr::Atom(_) => Sexpr::Atom(Atom::Number(1.0)),
        Sexpr::Cons(cons) => {
            if let Sexpr::Atom(Atom::Number(l1)) = length(cons.cdr) {
                Sexpr::Atom(Atom::Number(l1 + 1.0))
             } else {
                panic!("Error: Expected number")
             }
        },
    }
}