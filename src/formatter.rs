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

const BOX_WIDTH: usize = 85;

fn visible_length(text: &str) -> usize {
    let mut length = 0;
    let mut chars = text.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '&' {
            if let Some(next) = chars.peek() {
                if matches!(next, '0'..='9' | 'a'..='f' | 'r') {
                    chars.next(); // consume the color code
                    continue;
                }
            }
        }
        length += 1;
    }
    length
}

pub fn print_box_top() {
    let border = "─".repeat(BOX_WIDTH);
    print_colored(&format!("&f┌{}┐&r", border));
}

pub fn print_box_bottom() {
    let border = "─".repeat(BOX_WIDTH);
    print_colored(&format!("&f└{}┘&r", border));
}

pub fn print_box_line(content: &str) {
    let content_len = visible_length(content);
    let padding = if content_len < BOX_WIDTH { BOX_WIDTH - content_len } else { 0 };
    let spaces = " ".repeat(padding);
    print_colored(&format!("&f│{}{}&f│&r", content, spaces));
}

pub fn print_box_line_centered(content: &str) {
    let content_len = visible_length(content);
    let total_padding = if content_len < BOX_WIDTH { BOX_WIDTH - content_len } else { 0 };
    let left_padding = total_padding / 2;
    let right_padding = total_padding - left_padding;
    
    let left_spaces = " ".repeat(left_padding);
    let right_spaces = " ".repeat(right_padding);
    print_colored(&format!("&f│{}{}{}&f│&r", left_spaces, content, right_spaces));
}

pub fn print_usage_box_top(title: &str) {
    let title_len = visible_length(title);
    let remaining = BOX_WIDTH.saturating_sub(title_len + 4);
    let border = "─".repeat(remaining);
    print_colored(&format!("&8┌─ &3{}&r &8─{}┐&r", title, border));
}

pub fn print_usage_box_bottom() {
    let border = "─".repeat(BOX_WIDTH);
    print_colored(&format!("&8└{}┘&r", border));
}

pub fn print_usage_box_line(content: &str) {
    let content_len = visible_length(content);
    let padding = if content_len < BOX_WIDTH { BOX_WIDTH - content_len } else { 0 };
    let spaces = " ".repeat(padding);
    print_colored(&format!("&8│&r{}{}&8│&r", content, spaces));
}

pub fn print_table_header(columns: &[(&str, usize)]) {
    let mut header = String::from("&8  ┌─ ");
    let mut separator = String::from("&8  └─");
    
    for (i, (title, width)) in columns.iter().enumerate() {
        if i > 0 {
            header.push_str("&8┬─ ");
            separator.push_str("┴─");
        }
        
        header.push_str(&format!("&f{}&r &8─", title));
        let padding = width.saturating_sub(title.len() + 1);
        header.push_str(&"─".repeat(padding));
        separator.push_str(&"─".repeat(width + 1));
    }
    
    header.push_str("&8┐&r");
    separator.push_str("&8┘&r");
    
    print_colored(&header);
}

pub fn print_table_row(columns: &[(&str, usize)], values: &[&str]) {
    let mut row = String::from("&8  │&r ");
    
    for (i, ((_, width), value)) in columns.iter().zip(values.iter()).enumerate() {
        if i > 0 {
            row.push_str(" &8│&r ");
        }
        
        let value_len = visible_length(value);
        let padding = width.saturating_sub(value_len);
        row.push_str(&format!("{}{}", value, " ".repeat(padding)));
    }
    
    row.push_str(" &8│&r");
    print_colored(&row);
}

pub fn print_table_footer(columns: &[(&str, usize)]) {
    let mut footer = String::from("&8  └─");
    
    for (i, (_, width)) in columns.iter().enumerate() {
        if i > 0 {
            footer.push_str("┴─");
        }
        footer.push_str(&"─".repeat(width + 1));
    }
    
    footer.push_str("&8┘&r");
    print_colored(&footer);
}