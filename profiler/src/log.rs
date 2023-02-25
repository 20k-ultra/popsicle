#[macro_export]
macro_rules! info {
    ($fmt:expr $(, $arg:expr)*) => {
        writeln!(
            &mut ::std::io::stdout(),
            "\x1b[{}m[INFO] \x1b[0m{}",
            "36",
            format!($fmt $(, $arg)*)
        )
        .unwrap();
    };
}

#[macro_export]
macro_rules! debug {
    ($fmt:expr $(, $arg:expr)*) => {
        writeln!(
            &mut ::std::io::stdout(),
            "\x1b[{}m[DEBUG] \x1b[0m{}",
            "35",
            format!($fmt $(, $arg)*)
        )
        .unwrap();
    };
}

#[macro_export]
macro_rules! warn {
    ($fmt:expr $(, $arg:expr)*) => {
        writeln!(
            &mut ::std::io::stdout(),
            "\x1b[{}m[WARN] \x1b[0m{}",
            "33",
            format!($fmt $(, $arg)*)
        )
        .unwrap();
    };
}

#[macro_export]
macro_rules! error {
    ($fmt:expr $(, $arg:expr)*) => {
        writeln!(
            &mut ::std::io::stdout(),
            "\x1b[{}m[ERROR] \x1b[0m{}",
            "31",
            format!($fmt $(, $arg)*)
        )
        .unwrap();
    };
}
