macro_rules! println_warn {
    ($fmt:expr) => {
        println!("{} {}", "warn:".yellow().bold(), $fmt)
    };
    ($fmt:expr, $($args:tt)*) => {
        println!(concat!("{} ", $fmt), "warn:".yellow().bold(), $($args)*)
    };
}

macro_rules! println_error {
    ($fmt:expr) => {
        println!("{} {}", "error:".red().bold(), $fmt)
    };
    ($fmt:expr, $($args:tt)*) => {
        println!(concat!("{} ", $fmt), "error:".red().bold(), $($args)*)
    };
}

macro_rules! println_info {
    ($fmt:expr) => {
        println!("{} {}", "info:".blue().bold(), $fmt)
    };
    ($fmt:expr, $($args:tt)*) => {
        println!(concat!("{} ", $fmt), "info:".blue().bold(), $($args)*)
    };
}

pub(crate) use println_error;
pub(crate) use println_info;
pub(crate) use println_warn;
