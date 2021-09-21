mod error;
#[cfg(test)]
mod tests;

pub use error::*;

#[macro_export]
macro_rules! assert {
    ($cond:expr $(,)?) => {{
        if $cond {
            Ok(())
        } else {
            Err($crate::Error::without_message())
        }
    }};
    ($cond:expr, $($arg:tt)+) => {{
        if $cond {
            Ok(())
        } else {
            Err($crate::Error::with_message(format!($($arg)+)))
        }
    }};
}

#[macro_export]
macro_rules! assert_eq {
    ($left:expr, $right:expr $(,)?) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    Err($crate::Error::without_message())
                } else {
                    Ok(())
                }
            }
        }
    }};
    ($left:expr, $right:expr, $($arg:tt)+) => {{
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    Err($crate::Error::with_message(format!($($arg)+)))
                } else {
                    Ok(())
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! assert_ne {
    ($left:expr, $right:expr $(,)?) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if *left_val == *right_val  {
                    Err($crate::Error::without_message())
                } else {
                    Ok(())
                }
            }
        }
    });
    ($left:expr, $right:expr, $($arg:tt)+) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if *left_val == *right_val {
                    Err($crate::Error::with_message(format!($($arg)+)))
                } else {
                    Ok(())
                }
            }
        }
    });
}