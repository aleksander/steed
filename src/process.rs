#![stable(feature = "steed", since = "1.0.0")]

use linux;

#[stable(feature = "steed", since = "1.0.0")]
pub fn exit(code: i32) -> ! {
    ::sys_common::cleanup();
    unsafe { linux::exit(code) }
}
