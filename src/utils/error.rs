pub enum ErrorType {
    UnknownCommand,
    InvalidArgument,
}
#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        {
            // println!($($arg)*);
        }
    };
}
