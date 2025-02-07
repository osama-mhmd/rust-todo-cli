use std::fs;
use std::io::{self, Write};

fn main() {
  let result = fs::OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("tasks.txt");

  if let Err(e) = result {
    println!("Error occured while opening file: {:#?}", e);
  }

  loop {
    print!("> ");

    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let mut parts = input.trim().split_whitespace();

    let c = parts.next().unwrap_or("");
    // let args: Vec<&str> = parts.collect();

    match c {
      "exit" => break,
      "create" => panic!("Not implemented yet"),
      _ => (),
    }
  }
}
