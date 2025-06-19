#[macro_export]
macro_rules! log {
    // 格式: [模块名] 消息 (自动根据模块名着色)
    ($module:literal, $($arg:tt)*) => {{
        use colored::Colorize;
        use std::io::Write;

        let colored_prefix = match $module.to_lowercase().as_str() {
            "server" => format!("[{}]", $module).bright_blue().bold(),
            "watcher" => format!("[{}]", $module).bright_green().bold(),
            "error" => format!("[{}]", $module).bright_red().bold(),
            _ => format!("[{}]", $module).bright_yellow().bold(),
        };

        let log_message = format!($($arg)*).replace("\n", "");
        print!("\r{} {}\r", colored_prefix, log_message);
        std::io::stdout().flush().unwrap();
    }};
}
