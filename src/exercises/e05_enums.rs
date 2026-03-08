/// Define a Message enum with the following variants:
/// - Quit: No data
/// - Move: Named fields x and y (both i32)
/// - Write: Contains a String
/// - ChangeColor: Three unnamed i32 fields (representing R, G, B)
pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

/// Implement a function that takes a Message and returns a String representation.
/// - Quit -> "Quit"
/// - Move { x, y } -> "Move to x=X, y=Y"
/// - Write(text) -> "Text message: TEXT"
/// - ChangeColor(r, g, b) -> "Change color to R, G, B"
pub fn process_message(msg: Message) -> String {
    todo!("Implement this function using pattern matching")
}

/// Define a custom configuration enum called Config with:
/// - Integer(i32)
/// - Boolean(bool)
/// - None
pub enum Config {
    Integer(i32),
    Boolean(bool),
    None,
}

/// Implement a function that tries to extract an i32 value from a Config.
/// If the Config is Integer, return its value.
/// If the Config is anything else, return the provided default value.
pub fn get_config_value(config: Config, default: i32) -> i32 {
    todo!("Implement this function using pattern matching")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_quit() {
        assert_eq!(process_message(Message::Quit), "Quit");
    }

    #[test]
    fn test_process_move() {
        assert_eq!(process_message(Message::Move { x: 10, y: 20 }), "Move to x=10, y=20");
    }

    #[test]
    fn test_process_write() {
        assert_eq!(process_message(Message::Write("Hello".to_string())), "Text message: Hello");
    }

    #[test]
    fn test_process_color() {
        assert_eq!(process_message(Message::ChangeColor(255, 0, 0)), "Change color to 255, 0, 0");
    }

    #[test]
    fn test_get_config_integer() {
        assert_eq!(get_config_value(Config::Integer(42), 0), 42);
    }

    #[test]
    fn test_get_config_boolean_default() {
        assert_eq!(get_config_value(Config::Boolean(true), 10), 10);
    }

    #[test]
    fn test_get_config_none_default() {
        assert_eq!(get_config_value(Config::None, -1), -1);
    }
}
