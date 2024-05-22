#[warn(clippy::nursery)]
#[warn(clippy::pedantic)]
pub mod key;
pub mod locker;
pub use key::Key;
pub use locker::Locker;

pub mod locker_capnp {
    include!(concat!(env!("OUT_DIR"), "/locker_capnp.rs"));
}

pub enum AuthErr {
    Io(std::io::Error),
    Cocoon(cocoon::Error),
    Capnp(capnp::Error),
}
impl From<std::io::Error> for AuthErr {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}
impl From<cocoon::Error> for AuthErr {
    fn from(value: cocoon::Error) -> Self {
        Self::Cocoon(value)
    }
}
impl From<capnp::Error> for AuthErr {
    fn from(value: capnp::Error) -> Self {
        Self::Capnp(value)
    }
}
