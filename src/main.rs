use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <message>", args[0]);
        std::process::exit(1);
    }

    print_message(&args[1]);
}

fn print_message(message: &str) {
    let formatted = format_text(message);
    println!("{}", formatted);
}

fn format_text(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '{' {
            // Find the closing brace
            let mut format_section = String::new();
            let mut found_closing = false;

            while let Some(inner_ch) = chars.next() {
                if inner_ch == '}' {
                    found_closing = true;
                    break;
                }
                format_section.push(inner_ch);
            }

            if found_closing {
                // Parse the format section
                if let Some(colon_pos) = format_section.find(':') {
                    let format_spec = format_section[..colon_pos].trim();
                    let text = format_section[colon_pos + 1..].trim();

                    // Apply formatting
                    let formatted_text = apply_formatting(format_spec, text);
                    result.push_str(&formatted_text);
                } else {
                    // No colon found, treat as regular text
                    result.push('{');
                    result.push_str(&format_section);
                    result.push('}');
                }
            } else {
                // No closing brace found, treat as regular text
                result.push('{');
                result.push_str(&format_section);
            }
        } else {
            result.push(ch);
        }
    }

    result
}

fn apply_formatting(format_spec: &str, text: &str) -> String {
    let mut ansi_codes = Vec::new();
    let formats: Vec<&str> = format_spec.split_whitespace().collect();

    for format in formats {
        match format.to_lowercase().as_str() {
            "red" => ansi_codes.push("31"),
            "green" => ansi_codes.push("32"),
            "yellow" => ansi_codes.push("33"),
            "blue" => ansi_codes.push("34"),
            "magenta" => ansi_codes.push("35"),
            "cyan" => ansi_codes.push("36"),
            "white" => ansi_codes.push("37"),
            "black" => ansi_codes.push("30"),
            "bold" => ansi_codes.push("1"),
            "dim" => ansi_codes.push("2"),
            "italic" => ansi_codes.push("3"),
            "underline" => ansi_codes.push("4"),
            "blink" => ansi_codes.push("5"),
            "reverse" => ansi_codes.push("7"),
            "strikethrough" => ansi_codes.push("9"),
            _ => {} // Ignore unknown formats
        }
    }

    if ansi_codes.is_empty() {
        text.to_string()
    } else {
        let mut result = String::new();
        for code in ansi_codes {
            result.push_str(&format!("\x1b[{}m", code));
        }
        result.push_str(text);
        result.push_str("\x1b[0m"); // Reset formatting
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_message() {
        // This test verifies that print_message works with a given string
        // Since println! doesn't return a value, we just call it and ensure no panic occurs
        print_message("test message");
    }

    #[test]
    fn test_format_text_no_formatting() {
        let input = "hello world";
        let expected = "hello world";
        assert_eq!(format_text(input), expected);
    }

    #[test]
    fn test_format_text_single_color() {
        let input = "hello {red: bob}";
        let expected = "hello \x1b[31mbob\x1b[0m";
        assert_eq!(format_text(input), expected);
    }

    #[test]
    fn test_format_text_color_with_style() {
        let input = "hello {red underline: bob}";
        let expected = "hello \x1b[31m\x1b[4mbob\x1b[0m";
        assert_eq!(format_text(input), expected);
    }

    #[test]
    fn test_format_text_multiple_formats() {
        let input = "hello {red underline: bob}, do you like {green: race cars}?";
        let expected = "hello \x1b[31m\x1b[4mbob\x1b[0m, do you like \x1b[32mrace cars\x1b[0m?";
        assert_eq!(format_text(input), expected);
    }

    #[test]
    fn test_format_text_bold_and_italic() {
        let input = "This is {bold: important} and {italic: emphasized}";
        let expected = "This is \x1b[1mimportant\x1b[0m and \x1b[3memphasized\x1b[0m";
        assert_eq!(format_text(input), expected);
    }

    #[test]
    fn test_format_text_malformed() {
        let input = "This has {no colon} and {unclosed brace and normal text";
        let expected = "This has {no colon} and {unclosed brace and normal text";
        assert_eq!(format_text(input), expected);
    }
}
