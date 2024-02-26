use crate::{parser::{Cons, Sexpr}, scanner::{Atom, Identifier}};

pub enum Value {
    Str(String),
    Number(f32),
    Boolean(bool),
    Nil
}

pub fn evaluate(sexpr: Sexpr) -> Value {
    match sexpr {
        Sexpr::Atom(atom) => evaluate_atom(atom),
        Sexpr::Cons(cons) => evalute_cons(*cons),
    }
}

fn evalute_cons(cons: Cons) -> Value {
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
                }
            } else {
                panic!("I don't think this should happen...");
            }
        },
        Sexpr::Cons(cons) => evalute_cons(*cons),
    }
}

fn evaluate_atom(atom: Atom) -> Value {
    match atom {
        Atom::Identifier(id) => panic!("Error: expected atom, found identifier {:?}", id),
        Atom::Str(str) => Value::Str(str),
        Atom::Number(num) => Value::Number(num),
        Atom::Boolean(bol) => Value::Boolean(bol),
        Atom::Nil => Value::Nil,
    }
}

fn arithmetic(f: fn(f32, f32) -> f32, sexpr: Sexpr) -> Value {
    match sexpr {
        Sexpr::Atom(atom) => evaluate_atom(atom),
        Sexpr::Cons(cons) => {
            let car = evaluate(cons.car);
            let cdr = arithmetic(f, cons.cdr);

            match (car, cdr) {
                // Two numbers
                (Value::Number(x), Value::Number(y)) => Value::Number(f(x, y)),
                // Number and NIL
                (x @ Value::Number(_), Value::Nil) => x,
                // Other: (Number, String), (Nil, Number), etc. We can't evaluate these
                _ => panic!("Error: Either car or cdr is not a number")
            }
        },
    }
}

fn conditional(f: fn(f32, f32) -> bool, sexpr: Sexpr) -> Value {
    match sexpr {
        Sexpr::Atom(atom) => evaluate_atom(atom),
        Sexpr::Cons(cons) => {
            let car = evaluate(cons.car);
            let cdr = conditional(f, cons.cdr);

            match (car, cdr) {
                // Two numbers
                (Value::Number(x), Value::Number(y)) => Value::Boolean(f(x, y)),
                // Number and NIL
                (x @ Value::Number(_), Value::Nil) => x,
                // Other: (Number, String), (Nil, Number), etc. We can't evaluate these
                _ => panic!("Error: Either car or cdr is not a number")
            }
        },
    }
}