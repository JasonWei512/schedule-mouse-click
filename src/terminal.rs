macro_rules! println_error {
    ($($arg:tt)*) => {
        println!("{} {}", colored::Colorize::bright_red("Error:"), format!($($arg)*));
    }
}

macro_rules! println_warning {
    ($($arg:tt)*) => {
        println!("{} {}", colored::Colorize::bright_yellow("Warning:"), format!($($arg)*));
    }
}

pub(crate) use println_error;
pub(crate) use println_warning;
