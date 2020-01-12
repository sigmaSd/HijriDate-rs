#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {return Err(format!($($arg)*))}
}
