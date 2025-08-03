

pub fn colorize(input: &str) -> String {
    let mut colored = String::with_capacity(input.len());
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if c == '&' {
            match chars.next() {
                Some(code @ '0'..='9') | Some(code @ 'a'..='f') | Some(code @ 'r') => {
                    let escape_code = match code {
                        '0' => "\x1b[30m", // Black
                        '1' => "\x1b[34m", // Blue
                        '2' => "\x1b[32m", // Green
                        '3' => "\x1b[36m", // Cyan
                        '4' => "\x1b[31m", // Red
                        '5' => "\x1b[35m", // Purple
                        '6' => "\x1b[33m", // Yellow
                        '7' => "\x1b[37m", // White
                        '8' => "\x1b[90m", // Gray
                        '9' => "\x1b[94m", // Light Blue
                        'a' => "\x1b[92m", // Light Green
                        'b' => "\x1b[96m", // Light Cyan
                        'c' => "\x1b[91m", // Light Red
                        'd' => "\x1b[95m", // Light Purple
                        'e' => "\x1b[93m", // Light Yellow
                        'f' => "\x1b[97m", // Bright White
                        'r' => "\x1b[0m",  // Reset
                        _ => unreachable!(),
                    };
                    colored.push_str(escape_code);
                }
                Some(other) => {
                    colored.push('&');
                    colored.push(other);
                }
                None => colored.push('&'),
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

pub enum MessageType<'a> {
    Success(&'a str),
    Error(&'a str),
    Info(&'a str),
    Warning(&'a str),
    None(&'a str),
}

pub fn print_message(message_type: MessageType) {
    let (prefix, color_code, message) = match message_type {
        MessageType::Success(m) => ("Success: ", 'a', m),
        MessageType::Error(m) => ("Error: ", 'c', m),
        MessageType::Info(m) => ("Info: ", 'b', m),
        MessageType::Warning(m) => ("Warning: ", 'e', m),
        MessageType::None(m) => ("", 'r', m),
    };

    let formatted_message = if !prefix.is_empty() {
        format!("&{}{}{}&r", color_code, prefix, message)
    } else {
        message.to_string()
    };

    println!("{}", colorize(&formatted_message));
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

#[derive(Clone, Copy)]
pub enum BoxAlignment {
    Left,
    Center,
}

pub struct BoxOptions<'a> {
    pub title: Option<&'a str>,
    pub title_color: char,
    pub border_color: char,
}

impl Default for BoxOptions<'_> {
    fn default() -> Self {
        BoxOptions {
            title: None,
            title_color: '3',
            border_color: 'f',
        }
    }
}

pub fn print_box(lines: &[(&str, BoxAlignment)], options: &BoxOptions) {
    print_box_top(options);
    for (line, alignment) in lines {
        print_box_line(line, *alignment, options.border_color);
    }
    print_box_bottom(options);
}

fn print_box_top(options: &BoxOptions) {
    let border_char = "─";
    if let Some(title) = options.title {
        let title_len = visible_length(title);
        let remaining = BOX_WIDTH.saturating_sub(title_len + 4);
        let border = border_char.repeat(remaining);
        println!("{}", colorize(&format!("&{0}┌─ &{1}{2}&r &{0}─{3}┐&r", options.border_color, options.title_color, title, border)));
    } else {
        let border = border_char.repeat(BOX_WIDTH);
        println!("{}", colorize(&format!("&{}┌{}┐&r", options.border_color, border)));
    }
}

fn print_box_bottom(options: &BoxOptions) {
    let border = "─".repeat(BOX_WIDTH);
    println!("{}", colorize(&format!("&{}└{}┘&r", options.border_color, border)));
}

fn print_box_line(content: &str, alignment: BoxAlignment, border_color: char) {
    let content_len = visible_length(content);
    let total_padding = BOX_WIDTH.saturating_sub(content_len);
    
    let (left_padding, right_padding) = match alignment {
        BoxAlignment::Left => {
            if total_padding == 0 {
                (0, 0)
            } else {
                (1, total_padding - 1)
            }
        },
        BoxAlignment::Center => (total_padding / 2, total_padding - (total_padding / 2)),
    };

    let left_spaces = " ".repeat(left_padding);
    let right_spaces = " ".repeat(right_padding);
    println!("{}", colorize(&format!("&{0}│{1}{2}{3}&{0}│&r", border_color, left_spaces, content, right_spaces)));
}

pub fn print_table_header(columns: &[(&str, usize)]) {
    let mut header = String::from("&8  ┌ ");
    let mut separator = String::from("&8  └─");
    
    for (i, (title, width)) in columns.iter().enumerate() {
        if i > 0 {
            header.push_str("&8─┬─");
            separator.push_str("┴─");
        }
        
        header.push_str(&format!("&f{}&r &8─", title));
        let padding = width.saturating_sub(title.len() + 1);
        header.push_str(&"─".repeat(padding));
        separator.push_str(&"─".repeat(width + 1));
    }
    
    header.push_str("&8┐&r");
    separator.push_str("&8┘&r");
    
    println!("{}", colorize(&header));
}

pub fn print_table_row(columns: &[(&str, usize)], values: &[&str]) {
    let mut row = String::from("&8  │&r ");

    for (i, ((_, width), value)) in columns.iter().zip(values.iter()).enumerate() {
        if i > 0 {
            row.push_str(" &8│&r ");
        }

        let value_len = visible_length(value);
        let padding = if i < columns.len() - 1 {
            (width + 1).saturating_sub(value_len)
        } else {
            width.saturating_sub(value_len)
        };
        row.push_str(&format!("{}{}", value, " ".repeat(padding)));
    }

    row.push_str(" &8│&r");
    println!("{}", colorize(&row));
}

pub fn print_table_footer(columns: &[(&str, usize)]) {
    let mut footer = String::from("&8  └─");
    
    for (i, (_, width)) in columns.iter().enumerate() {
        if i > 0 {
            footer.push_str("─┴─");
        }
        footer.push_str(&"─".repeat(width + 1));
    }
    
    footer.push_str("&8┘&r");
    println!("{}", colorize(&footer));
}
