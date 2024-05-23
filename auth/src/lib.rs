#[warn(clippy::nursery)]
#[warn(clippy::pedantic)]
pub mod key;
pub mod locker;
pub use key::Key;
pub use locker::Locker;

pub mod locker_capnp {
    include!(concat!(env!("OUT_DIR"), "/locker_capnp.rs"));
}

#[derive(thiserror::Error, Debug)]
pub enum AuthErr {
    #[error("I/O Error")]
    Io(#[from] std::io::Error),
    #[error("Error during encryption/decryption")]
    Cocoon(cocoon::Error),
    #[error("Fail to decode capnp file")]
    Capnp(#[from] capnp::Error),
}
// somehow cocoon::Error does not impl std::error::Error
// => https://github.com/fadeevab/cocoon/pull/26
impl From<cocoon::Error> for AuthErr {
    fn from(value: cocoon::Error) -> Self {
        Self::Cocoon(value)
    }
}
