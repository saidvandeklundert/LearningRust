// pull types into scope of HTTP module:
pub use method::Method;
pub use request::ParseError;
pub use request::Request;
// use modules:
pub mod method;
pub mod request;
