pub fn colorize(input: &str) -> String {
    let mut colored: String = String::new();
    let mut chars: std::str::Chars<'_> = input.chars();
    while let Some(c) = chars.next() {
        if c == '&' {
            match chars.next() {
                Some('0') => colored.push_str("\x1b[30m"), // Black
                Some('1') => colored.push_str("\x1b[34m"), // Blue
                Some('2') => colored.push_str("\x1b[32m"), // Green
                Some('3') => colored.push_str("\x1b[36m"), // Cyan
                Some('4') => colored.push_str("\x1b[31m"), // Red
                Some('5') => colored.push_str("\x1b[35m"), // Purple
                Some('6') => colored.push_str("\x1b[33m"), // Yellow
                Some('7') => colored.push_str("\x1b[37m"), // White
                Some('8') => colored.push_str("\x1b[90m"), // Gray
                Some('9') => colored.push_str("\x1b[94m"), // Light Blue
                Some('a') => colored.push_str("\x1b[92m"), // Light Green
                Some('b') => colored.push_str("\x1b[96m"), // Light Cyan
                Some('c') => colored.push_str("\x1b[91m"), // Light Red 
                Some('d') => colored.push_str("\x1b[95m"), // Light Purple
                Some('e') => colored.push_str("\x1b[93m"), // Light Yellow
                Some('f') => colored.push_str("\x1b[97m"), // Bright White
                Some('r') => colored.push_str("\x1b[0m"), // Reset
                _ => colored.push(c),
            }
        } else {
            colored.push(c);
        }
    }
    colored.push_str("\x1b[0m");
    colored
}


pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn add_strings(args: &[String]) -> String {
    let mut result = String::new();
    for arg in args {
        result.push_str(arg);
    }
    result
}

// Custom print functions that use the colorize formatter
pub fn print_colored(message: &str) {
    println!("{}", colorize(message));
}

pub fn print_error(message: &str) {
    eprintln!("{}", colorize(&format!("&4Error: {}&r", message)));
}

pub fn print_success(message: &str) {
    println!("{}", colorize(&format!("&aSuccess: {}&r", message)));
}

pub fn print_info(message: &str) {
    println!("{}", colorize(&format!("&3Info: {}&r", message)));
}

pub fn print_warning(message: &str) {
    println!("{}", colorize(&format!("&eWarning: {}&r", message)));
}