
pub struct ClICommand {
    pub command: Vec<String>,
    pub flags: Vec<Flag>,
}

pub struct Flag {
    pub flag: String,
    pub value: String
}


impl ClICommand {
    pub fn new() -> ClICommand {
        ClICommand {
            command: Vec::new(),
            flags: Vec::new(),
        }
    }

    pub fn get_flag(&self, flag: &str) -> Option<&Flag> {
        for f in &self.flags {
            if f.flag == flag {
                return Some(f);
            }
        }

        return None;
    }

    pub fn get_first(&self) -> String {
        if self.command.is_empty() { return String::new(); }
        return self.command[0].clone();
    }

    pub fn remove_first(&mut self) {
        if self.command.is_empty() { return; }
        self.command.remove(0);
    }
}

// env::args().collect() does this for us
// pub fn string_reader(command: &str) -> Vec<String> {

//     let mut tokens: Vec<String> = Vec::new();
//     let mut token: String = String::new();
//     let mut in_string: bool = false;

//     for c in command.chars() {
//         match c {
//             ' ' => {
//                 if in_string {
//                     token.push(c);
//                 } else {
//                     if !token.is_empty() {
//                         tokens.push(token.clone());
//                         token.clear();
//                     }
//                 }
//             }
//             '"' => {
//                 in_string = !in_string;
//             }
//             _ => {
//                 token.push(c);
//             }
//         }
//     }

//     if !token.is_empty() {
//         tokens.push(token);
//     }

//     return tokens;
// }

// Get commands from a list of tokens (No longer needed since removing tokens from the list when getting flags)
// pub fn get_commands(tokens: &Vec<String>) -> Vec<String> {
//     let mut commands: Vec<String> = Vec::new();
//     let mut command: String = String::new();
//     let in_string: bool = false;

//     for token in tokens {
//         if token.starts_with("-") {
//             if !command.is_empty() {
//                 commands.push(command.clone());
//                 command.clear();
//             }
//         } else {
//             if in_string {
//                 command.push_str(token);
//             } else {
//                 command = token.clone();
//             }
//         }
//     }

//     if !command.is_empty() {
//         commands.push(command.clone());
//     }

//     return commands;
// }

fn remove_args(tokens: Vec<String>, args: &[String]) -> Vec<String> {
    let mut new_tokens: Vec<String> = Vec::new();

    for token in tokens {
        if !args.contains(&token) {
            new_tokens.push(token);
        }
    }

    return new_tokens;
}


// Some Logic examples for the parser
// --flag1 "value1" --flag2 "value2"
// --flag1 "value1" --flag2 --flag3 "value3" (flag2 value is null)
// a argument without a flag is considered a command
// run --flag1 "value1" --flag2 "value2" subcommand (run and subcommand are commands)

pub fn get_flags(tokens: &mut Vec<String>) -> Vec<Flag> {
    let mut flags: Vec<Flag> = Vec::new();
    let mut removed_tokens: Vec<String> = Vec::new();

    for (i, token) in tokens.iter().enumerate() {
        if token.starts_with("-") {
            if i + 1 < tokens.len() {
                if !tokens[i + 1].starts_with("-") {
                    flags.push(Flag {
                        flag: token.clone(),
                        value: tokens[i + 1].clone(),
                    });

                    removed_tokens.push(token.clone());
                    removed_tokens.push(tokens[i + 1].clone());
                } else {
                    flags.push(Flag {
                        flag: token.clone(),
                        value: String::new(),
                    });

                    removed_tokens.push(token.clone());
                }
            } else {
                flags.push(Flag {
                    flag: token.clone(),
                    value: String::new(),
                });

                removed_tokens.push(token.clone());
            }
        }
    }

    // Here we pass a reference to removed_tokens instead of moving it
    *tokens = remove_args(tokens.clone(), &removed_tokens);

    return flags;
}

pub fn main(commands: &[String]) -> ClICommand {
    let mut tokens: Vec<String> = Vec::from(commands); // string_reader(command);

    return ClICommand {
        flags: get_flags(&mut tokens),
        command: tokens
    };
}

