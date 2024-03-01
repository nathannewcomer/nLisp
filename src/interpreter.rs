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
            if let Atom::Symbol(name) = atom {
                match name.as_str() {
                    // Arithmatic
                    "+" => arithmetic(|x, y| x + y, cons.cdr),
                    "-" => arithmetic(|x, y| x - y, cons.cdr),
                    "*" => arithmetic(|x, y| x * y, cons.cdr),
                    "/" => arithmetic(|x, y| x / y, cons.cdr),
                    ">" => conditional(|x, y| x > y, cons.cdr),
                    ">=" => conditional(|x, y| x >= y, cons.cdr),
            
                    // Predicate
                    "listp" => todo!(),
                    "atom" => todo!(),
                    "null" => todo!(),
                    "eq" => todo!(),
                    "equal" => todo!(),
            
                    // Lists
                    "cons" => todo!(),
                    "car" => car(cons.cdr),
                    "cdr" => cdr(cons.cdr),
                    "append" => todo!(),
            
                    // Functions
                    "defun" => todo!(),
                    "eval" => todo!(),
            
                    "quote" => quote(cons.cdr),
                    "length" => length(cons.cdr),
            
                    _ => panic!("Operator \"{name}\" is not a procedure")
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
        Atom::Symbol(str) => Sexpr::Atom(Atom::Symbol(str)),
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

fn quote(sexpr: Sexpr) -> Sexpr {
    match sexpr {
        atom @ Sexpr::Atom(_) => atom,
        Sexpr::Cons(cons) => cons.car,
    }
}

fn car(sexpr: Sexpr) -> Sexpr {
    match sexpr {
        Sexpr::Atom(_) => panic!("Error: s-expresson is atom"),
        Sexpr::Cons(cons) => evaluate(cons.car),
    }
}

fn cdr(sexpr: Sexpr) -> Sexpr {
    match sexpr {
        Sexpr::Atom(_) => panic!("Error: s-expresson is atom"),
        Sexpr::Cons(cons) => evaluate(cons.cdr),
    }
}

