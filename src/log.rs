#[macro_export]
macro_rules! log {
    // 格式: [模块名] 消息 (自动根据模块名着色)
    ($module:literal, $($arg:tt)*) => {{
        use colored::Colorize;
        use std::io::{stdout, Write};

        let module_lower = $module.to_lowercase();
        let is_important = matches!(module_lower.as_str(), "server" | "watcher" | "error");
        
        let colored_prefix = match module_lower.as_str() {
            "server" => format!("[{}]", $module).bright_blue().bold(),
            "watcher" => format!("[{}]", $module).bright_green().bold(),
            "error" => format!("[{}]", $module).bright_red().bold(),
            _ => format!("[{}]", $module).bright_yellow().bold(),
        };

        let log_message = format!($($arg)*);
        let mut stdout = stdout().lock();
        if is_important {
            writeln!(stdout, "{} {}", colored_prefix, log_message).ok();

        } else {
            write!(stdout, "{} {}\r", colored_prefix, log_message).ok();
        }
        stdout.flush().ok();
    }};
}
