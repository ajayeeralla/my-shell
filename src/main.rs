#[allow(unused_imports)]
use std::io::{stdin, stdout, Write};
#[allow(unused_imports)]
use std::process::{Child, Command, Stdio};
#[allow(unused_imports)]
use std::result::Result;

fn main() {

    loop{
            print!(">_ ");
            stdout().flush();
            let mut user_input = String::new();
            stdin().read_line(&mut user_input).unwrap();
            let cmd = user_input.trim();
            //println!("user input is: {}", _cmd);
            let mut child = Command::new(cmd)
            .spawn()
            .unwrap();
            //println!("Hello, world!");
            child.wait();
    }
 }
