use std::fs;

fn main() {
  let result = fs::OpenOptions::new()
    .read(true)
    .write(true)
    .create(true)
    .open("tasks.txt");

  if let Err(e) = result {
    println!("Error occured while opening file: {:#?}", e);
  }
}
