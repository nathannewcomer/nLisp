mod scanner;
mod parser;

use std::{env, io::{self, stdout, Write}};

use crate::{parser::{parse, print_sexpr}, scanner::scan};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => repl(),
        2 => todo!(),
        _ => {
            println!("Usage: nlisp [filename]");
        }
    }
}

fn repl() {
    loop {
        print!("> ");
        let _ = stdout().flush();
        let mut line = String::new();
        let tokens = match std::io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => scan(&line),
            Err(_) => panic!("Could not read stdin")
        };

        let sexpr = parse(&tokens, &mut 0);
        print!("Parsed as: ");
        print_sexpr(&sexpr);
        println!();
    }
}
