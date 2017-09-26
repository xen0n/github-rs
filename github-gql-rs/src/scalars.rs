pub use git2::Oid;
pub use hyper::Uri;

// There is also a `String` type but we'll use `String` directly since it needs
// to be UTF-8 valid and the standard library guarantees that with `String`.
pub struct Boolean(bool);
pub struct DateTime(String); // wrong
pub struct Float(f64);
pub struct GitObjectId(Oid);
pub struct GitTimeStamp(String); // wrong
pub struct Html(String);
pub struct Id(String); // wrong
pub struct Int(i32);
pub struct X509Certificate(String); // wrong
