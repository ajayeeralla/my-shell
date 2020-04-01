#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};
#[allow(unused_imports)]
use std::process::{Child, Command, Stdio};
#[allow(unused_imports)]
use std::result::Result;

use std::path::Path;
use std::env;

fn main() {

    loop{
            print!("> ");
            stdout().flush();

            let mut user_input = String::new();
            stdin().read_line(&mut user_input).unwrap();

            let mut parts = user_input.trim().split_whitespace();
            let cmd = parts.next().unwrap();
            let args = parts;       
            

            match cmd {
                    "cd" => {
                        let new_dir = args.peekable().peek().map_or("~", |x| *x);
                        let home = Path::new(new_dir);
                        if let Err(e) = env::set_current_dir(&home){
                                eprintln!("{}", e);
                        }
                    }
            ,
             cmd  => {
           // let cmd = user_input.trim();
            //println!("user input is: {}", _cmd);
            let mut child = Command::new(cmd)
            .spawn()
            .unwrap();
            //println!("Hello, world!");
            child.wait();
    }
 }
} }