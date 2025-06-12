use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} [-n] [-h|--help] <message>", args[0]);
        std::process::exit(1);
    }

    // Check for help flag anywhere in arguments
    for arg in &args[1..] {
        if arg == "-h" || arg == "--help" {
            print_help(&args[0]);
            return;
        }
    }

    let (no_newline, message) = parse_args(&args);

    if message.is_empty() {
        eprintln!("Usage: {} [-n] [-h|--help] <message>", args[0]);
        std::process::exit(1);
    }

    print_message(&message, no_newline);
}

fn print_message(message: &str, no_newline: bool) {
    let formatted = format_text(message);
    if no_newline {
        print!("{}", formatted);
    } else {
        println!("{}", formatted);
    }
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

fn parse_args(args: &[String]) -> (bool, String) {
    let mut no_newline = false;
    let mut message_parts = Vec::new();
    let mut i = 1; // Skip program name

    while i < args.len() {
        if args[i] == "-n" {
            no_newline = true;
        } else if args[i] == "-h" || args[i] == "--help" {
            // Help flag should have been handled in main, but skip it here too
            // in case someone puts it after other flags
        } else {
            message_parts.push(args[i].clone());
        }
        i += 1;
    }

    let message = message_parts.join(" ");
    (no_newline, message)
}

fn print_help(program_name: &str) {
    println!("cecho - A command-line tool for printing colorized text");
    println!();
    println!("USAGE:");
    println!("    {} [OPTIONS] <message>", program_name);
    println!();
    println!("OPTIONS:");
    println!("    -n              Suppress the trailing newline");
    println!("    -h, --help      Show this help message and exit");
    println!();
    println!("FORMATTING:");
    println!("    Text can be formatted using {{format: text}} syntax where 'format' can contain:");
    println!();
    println!("    Colors:");
    println!("        black, blue, cyan, green, magenta, red, white, yellow");
    println!();
    println!("    Styles:");
    println!("        blink, bold, dim, italic, reverse, strikethrough, underline");
    println!();
    println!("    Multiple formats can be combined with spaces:");
    println!("        {{red bold: text}}    - Red and bold");
    println!("        {{blue underline: text}} - Blue and underlined");
    println!();
    println!("EXAMPLES:");
    println!("    Basic usage:");
    println!("        {} 'Hello world!'", program_name);
    println!();
    println!("    Colored text:");
    println!("        {} 'This is {{red: red text}}'", program_name);
    println!("        {} 'Status: {{green: SUCCESS}}'", program_name);
    println!();
    println!("    Styled text:");
    println!("        {} 'This is {{bold: important}}'", program_name);
    println!("        {} 'This is {{italic: emphasized}}'", program_name);
    println!(
        "        {} 'This is {{underline: underlined}}'",
        program_name
    );
    println!();
    println!("    Combined formatting:");
    println!(
        "        {} 'This is {{red bold: very important}}'",
        program_name
    );
    println!(
        "        {} 'Warning: {{yellow bold underline: critical issue}}'",
        program_name
    );
    println!();
    println!("    Multiple formatted sections:");
    println!(
        "        {} 'Hello {{cyan: world}}, this is {{red: amazing}}!'",
        program_name
    );
    println!(
        "        {} 'Status: {{green: PASSED}} - {{blue: 42 tests}} completed'",
        program_name
    );
    println!();
    println!("    Log-style output:");
    println!(
        "        {} '[{{green: INFO}}] Application started'",
        program_name
    );
    println!(
        "        {} '[{{yellow: WARN}}] Connection timeout'",
        program_name
    );
    println!(
        "        {} '[{{red: ERROR}}] {{red bold: Critical failure}}'",
        program_name
    );
    println!();
    println!("    Suppress newline:");
    println!("        {} -n 'Loading{{cyan: ...}}'", program_name);
    println!();
    println!("    Fun examples:");
    println!(
        "        {} 'My {{cyan: hovercraft}} is {{red underline: full of eels}}!'",
        program_name
    );
    println!(
        "        {} '🚀 {{green bold: Build Status:}} {{green: PASSED}}'",
        program_name
    );
    println!(
        "        {} '📊 Results: {{cyan: 45 passed}}, {{yellow: 2 skipped}}, {{red: 0 failed}}'",
        program_name
    );
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
        print_message("test message", false);
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

    #[test]
    fn test_parse_args_no_flags() {
        let args = vec!["program".to_string(), "hello world".to_string()];
        let (no_newline, message) = parse_args(&args);
        assert!(!no_newline);
        assert_eq!(message, "hello world");
    }

    #[test]
    fn test_parse_args_with_n_flag() {
        let args = vec![
            "program".to_string(),
            "-n".to_string(),
            "hello world".to_string(),
        ];
        let (no_newline, message) = parse_args(&args);
        assert!(no_newline);
        assert_eq!(message, "hello world");
    }

    #[test]
    fn test_parse_args_message_with_spaces() {
        let args = vec![
            "program".to_string(),
            "-n".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ];
        let (no_newline, message) = parse_args(&args);
        assert!(no_newline);
        assert_eq!(message, "hello world");
    }

    #[test]
    fn test_parse_args_ignores_help_flag() {
        let args = vec![
            "program".to_string(),
            "-h".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ];
        let (no_newline, message) = parse_args(&args);
        assert!(!no_newline);
        assert_eq!(message, "hello world");
    }

    #[test]
    fn test_parse_args_ignores_help_flag_long() {
        let args = vec![
            "program".to_string(),
            "--help".to_string(),
            "hello".to_string(),
            "world".to_string(),
        ];
        let (no_newline, message) = parse_args(&args);
        assert!(!no_newline);
        assert_eq!(message, "hello world");
    }

    #[test]
    fn test_parse_args_mixed_flags_with_help() {
        let args = vec![
            "program".to_string(),
            "-n".to_string(),
            "--help".to_string(),
            "hello".to_string(),
        ];
        let (no_newline, message) = parse_args(&args);
        assert!(no_newline);
        assert_eq!(message, "hello");
    }
}
