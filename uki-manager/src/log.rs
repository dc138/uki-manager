macro_rules! println_warn {
    ( $pat:expr, $($arg:tt),* ) => {
        println!(concat!("{} ", $pat), "warning:".yellow().bold(), $($arg),*)
    };
    ( $msg:expr ) => {
        println!("{} {}", "warning:".yellow().bold(), $msg)
    };
}

macro_rules! println_error {
    ( $pat:expr, $($arg:tt),* ) => {
        println!(concat!("{} ", $pat), "error:".red().bold(), $($arg),*)
    };
    ( $msg:expr ) => {
        println!("{} {}", "error:".red().bold(), $msg)
    };
}

pub(crate) use println_error;
pub(crate) use println_warn;
