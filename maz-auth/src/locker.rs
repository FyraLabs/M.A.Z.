use crate::locker_capnp as lc;
use crate::AuthErr;
use capnp::message as cm;

pub struct Locker {
    pub name: String,
    pub keys: Vec<super::Key>,
}

impl Locker {
    /// Read an encrypted locker db file.
    ///
    /// # Errors
    /// - fail to parse file
    pub fn from_file(f: &mut File, pw: &[u8]) -> Result<Locker, AuthErr> {
        let data = cocoon::Cocoon::new(pw).parse(f)?;
        let rd = capnp::serialize_packed::read_message(&*data, cm::ReaderOptions::new())?;
        Ok(Locker::try_from(rd.get_root::<lc::locker::Reader>()?)?)
    }

    /// Encrypt and write to locker db file.
    pub fn write(&self, f: &mut impl Write, pw: &[u8]) -> Result<(), AuthErr> {
        let mut msg = capnp::message::Builder::new_default();
        {
            let mut locker = msg.init_root::<lc::locker::Builder>();
            locker.set_name(&self.name);
            let mut keysbuilder = locker.init_keys(self.keys.len().try_into().unwrap());
            for (i, key) in self.keys.iter().enumerate() {
                key.populate_capnp_builder(&mut keysbuilder.reborrow().get(i.try_into().unwrap()))
            }
        }

        let mut writer = std::io::BufWriter::new(&mut vec![]);
        capnp::serialize_packed::write_message(&mut writer, &msg)?;
        let buf = std::mem::take(writer.into_inner().unwrap());
        cocoon::Cocoon::new(pw).dump(buf, f)?;
        Ok(())
    }
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
