use crate::locker_capnp as lc;
use crate::AuthErr;
use std::path::Path;

pub struct Locker {
    pub name: String,
    pub keys: Vec<super::Key>,
}

/// Read an encrypted locker db file.
pub fn read_locker_file(p: &Path, pw: &[u8]) -> Result<Locker, AuthErr> {
    let cocoon = cocoon::Cocoon::new(pw);
    let data = cocoon.parse(&mut std::fs::File::open(p)?)?;
    let rd = capnp::serialize_packed::read_message(&*data, capnp::message::ReaderOptions::new())?;
    Ok(Locker::try_from(rd.get_root::<lc::locker::Reader>()?)?)
}

impl TryFrom<lc::locker::Reader<'_>> for Locker {
    type Error = capnp::Error;

    fn try_from(value: lc::locker::Reader) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.get_name()?.to_string()?,
            keys: (value.get_keys()?.into_iter().map(super::Key::try_from))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}
