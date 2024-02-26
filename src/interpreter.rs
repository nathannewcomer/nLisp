use crate::{parser::{Cons, Sexpr}, scanner::{Atom, Identifier}};

pub enum Value {
    Str(String),
    Number(f32),
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
                    Identifier::Add => add(cons.cdr),
                    Identifier::Subtract => todo!(),
                    Identifier::Multiply => todo!(),
                    Identifier::Divide => todo!(),
                    Identifier::Greater => todo!(),
                    Identifier::GreaterOrEqual => todo!(),
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
        Atom::Identifier(id) => panic!("Not sure what to do about this..."),
        Atom::Str(str) => Value::Str(str),
        Atom::Number(num) => Value::Number(num),
        Atom::Nil => Value::Nil,
    }
}

fn evalute_id(id: Identifier, sexpr: Sexpr) -> Value {
    match id {
        Identifier::Add => add(sexpr),
        Identifier::Subtract => todo!(),
        Identifier::Multiply => todo!(),
        Identifier::Divide => todo!(),
        Identifier::Greater => todo!(),
        Identifier::GreaterOrEqual => todo!(),
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
}

fn add(sexpr: Sexpr) -> Value {
    match sexpr {
        Sexpr::Atom(atom) => evaluate_atom(atom),
        Sexpr::Cons(cons) => {
            let car = add(cons.car);
            let cdr = add(cons.cdr);

            match (car, cdr) {
                // Two numbers
                (Value::Number(x), Value::Number(y)) => Value::Number(x + y),
                // Number and NIL
                (x @ Value::Number(_), Value::Nil) => x,
                // Other: (Number, String), (Nil, Number), etc. We can't add these
                _ => panic!("Can't add car and cdr")
            }
        },
    }
}
