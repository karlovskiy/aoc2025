#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        std::println!($($arg)*)
    }
}

#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {
        #[cfg(debug_assertions)]
        std::print!($($arg)*)
    }
}

pub mod secret_entrance;
