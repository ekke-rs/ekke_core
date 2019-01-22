pub mod ekke;
pub mod messages;

pub use ekke::Ekke;

pub type Result<T> = core::result::Result<T, failure::Error>;
