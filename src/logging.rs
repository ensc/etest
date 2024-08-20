//

#[doc(hidden)]
#[macro_export]
macro_rules! trace {
    ($feature:literal, $($arg:tt)*) => {{
        if cfg!(feature = $feature) {
            println!($($arg)*);
        }
    }}
}

#[doc(hidden)]
#[macro_export]
macro_rules! trace_resources {
    ($($arg:tt)*) => {
        $crate::trace!("trace_resources", $($arg)*)
    }
}
