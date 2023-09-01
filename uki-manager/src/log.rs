macro_rules! println_warn {
    ( $pat:expr, $($arg:tt),* ) => {
        println!(concat!("{} ", $pat), "warning:".yellow().bold(), $($arg),*)
    };
}

macro_rules! println_error {
    ( $pat:expr, $($arg:tt),* ) => {
        println!(concat!("{} ", $pat), "error:".red().bold(), $($arg),*)
    };
}

pub(crate) use println_error;
pub(crate) use println_warn;
