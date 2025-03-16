
pub fn string_reader(command: &str) {

    let mut tokens = Vec::new();
    let mut token = String::new();
    let mut in_string = false;

    for c in command.chars() {
        match c {
            ' ' => {
                if in_string {
                    token.push(c);
                } else {
                    if !token.is_empty() {
                        tokens.push(token.clone());
                        token.clear();
                    }
                }
            }
            '"' => {
                in_string = !in_string;
            }
            _ => {
                token.push(c);
            }
        }
    }

    if !token.is_empty() {
        tokens.push(token);
    }

    println!("{:?}", tokens);
}