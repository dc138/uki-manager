macro_rules! println_warn {
    ($fmt:expr) => {
        println!("{} {}", "warning:".yellow().bold(), $fmt)
    };
    ($fmt:expr, $($args:expr)*) => {
        println!(concat!("{} ", $fmt), "warning:".yellow().bold(), $($args),*)
    };
}

macro_rules! println_error {
    ($fmt:expr) => {
        println!("{} {}", "error:".red().bold(), $fmt)
    };
    ($fmt:expr, $($args:expr)*) => {
        println!(concat!("{} ", $fmt), "error:".red().bold(), $($args),*)
    };
}

pub(crate) use println_error;
pub(crate) use println_warn;
