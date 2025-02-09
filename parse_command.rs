pub fn parse_command(input: String) -> (String, Vec<String>) {
  let mut chars = input.trim().chars().peekable();
  let mut command = String::new();
  let mut args = Vec::new();
  let mut current_arg = String::new();
  let mut in_quotes = false;

  while let Some(&c) = chars.peek() {
    match c {
      '"' | '\'' => {
        // Toggle quote mode
        in_quotes = !in_quotes;
        chars.next(); // Consume the quote
      }
      ' ' if !in_quotes => {
        // If we hit a space outside quotes, finalize the argument
        if !current_arg.is_empty() {
          if command.is_empty() {
            command = current_arg.clone(); // First argument is the command
          } else {
            args.push(current_arg.clone()); // Rest are arguments
          }
          current_arg.clear();
        }
        chars.next(); // Consume the space
      }
      _ => {
        // Append character to current argument
        current_arg.push(c);
        chars.next();
      }
    }
  }

  // Push last argument if it's not empty
  if !current_arg.is_empty() {
    if command.is_empty() {
      command = current_arg;
    } else {
      args.push(current_arg);
    }
  }

  (command, args)
}
