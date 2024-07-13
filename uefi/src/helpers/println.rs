use crate::table::system_table_boot;
use core::fmt::Write;

/// INTERNAL API! Helper for print macros.
#[doc(hidden)]
pub fn _print(args: core::fmt::Arguments) {
    if let Some(mut bs) = system_table_boot() {
        bs.stdout()
            .write_fmt(args)
            .expect("Failed to write to stdout");
    } else {
        // Ease debugging: Depending on logger, this might write to serial or
        // debugcon.
        log::debug!("You are using `print!` after the boot services have been exited.");
    }
}

/// Prints to the standard output of the UEFI boot service console.
///
/// # Usage
/// Use this similar to `print!` from the Rust standard library, but only
/// as long as boot services have not been exited.
///
/// You should never use this macro in a custom Logger ([`log::Log`] impl) to
/// prevent a circular runtime dependency.
///
/// # Panics
/// Will panic if `SYSTEM_TABLE` is `None` (Before [`uefi::helpers::init()`] and
/// after [`uefi::prelude::SystemTable::exit_boot_services()`]).
///
/// # Examples
/// ```
/// print!("");
/// print!("Hello World\n");
/// print!("Hello {}", "World");
/// ```
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::helpers::_print(core::format_args!($($arg)*)));
}

/// Prints to the standard output of the UEFI boot service console, but with a
/// newline.
///
/// # Usage
/// Use this similar to `println!` from the Rust standard library, but only
/// as long as boot services have not been exited.
///
/// You should never use this macro in a custom Logger ([`log::Log`] impl) to
/// prevent a circular runtime dependency.
///
/// # Panics
/// Will panic if `SYSTEM_TABLE` is `None` (Before [`uefi::helpers::init()`] and
/// after [`uefi::prelude::SystemTable::exit_boot_services()`]).
///
/// # Examples
/// ```
/// println!();
/// println!("Hello World");
/// println!("Hello {}", "World");
/// ```
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::helpers::_print(core::format_args!("{}{}", core::format_args!($($arg)*), "\n")));
}
