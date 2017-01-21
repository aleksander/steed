// Rust 1.14.0
pub mod ffi;
pub mod fs;

#[stable(feature = "steed", since = "1.0.0")]
pub mod prelude {
    #[doc(no_inline)] #[stable(feature = "rust1", since = "1.0.0")]
    pub use super::ffi::{OsStrExt, OsStringExt};
    #[doc(no_inline)] #[unstable(feature = "file_offset", issue = "35918")]
    pub use super::fs::FileExt;
}
