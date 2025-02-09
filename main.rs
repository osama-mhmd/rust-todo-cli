mod parse_command;

use std::fs;
use std::io::{self, Write};

fn main() {
  fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open("tasks.txt")
    .expect("Error ocurrent while opening the file");

  let file_vec = fs::read("tasks.txt").unwrap();
  let file_content = String::from_utf8(file_vec).unwrap();
  let mut tasks: Vec<String> = file_content.lines().map(String::from).collect();

  loop {
    print!("> ");

    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let (c, args) = parse_command::parse_command(input);

    match c.as_str() {
      "exit" => break,
      "list" => {
        for task in &tasks {
          println!("{}", task);
        }
      }
      "check" => {
        if args.len() < 1 {
          println!("Please provide the taks index");
        } else {
          for item_index in &args {
            let index = item_index.parse::<usize>().unwrap() - 1;

            if tasks.len() < index {
              println!("Item of index {} not present", item_index);
            } else {
              tasks[index] = tasks[index].replacen("[ ]", "[x]", 1);
            }
          }
        }

        save_tasks(&tasks);
      }
      "add" => {
        if args.len() < 1 {
          println!("Please provide the tasks you want to add");
        } else {
          for task in args {
            tasks.push(String::from(format!("[ ] {}", task)));
          }

          save_tasks(&tasks);
        }
      }
      "rem" => {
        if args.len() < 1 {
          println!("Please provide the taks index");
        } else {
          let placeholder = "--WILL-BE-REMOVED--";

          for item_index in &args {
            let index = item_index.parse::<usize>().unwrap() - 1;

            if tasks.len() < index {
              println!("Item of index {} not present", item_index);
            } else {
              tasks[index] = String::from(placeholder);
            }
          }

          tasks = tasks
            .iter()
            .filter(|&task| *task != String::from(placeholder))
            .cloned()
            .collect::<Vec<String>>();

          save_tasks(&tasks);
        }
      }
      _ => (),
    }
  }
}

fn save_tasks(tasks: &Vec<String>) {
  fs::write("tasks.txt", tasks.join("\n")).unwrap();
}
