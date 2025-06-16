pub mod config;
pub mod driver;
mod registers;
pub mod status;

#[allow(unused_imports)]
pub use config::Config;
#[allow(unused_imports)]
pub use driver::BMP280;
