#[derive(Debug, Clone)]
pub struct Key {
    pub id: String,
    totp: totp_rs::TOTP,
}

impl std::str::FromStr for Key {
    type Err = totp_rs::TotpUrlError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let totp = totp_rs::TOTP::from_url(s)?;
        let id = if let Some(issuer) = &totp.issuer {
            format!("{issuer}: {}", totp.account_name)
        } else {
            totp.account_name.clone()
        };
        Ok(Self { id, totp })
    }
}

impl Key {
    /// Generate current code
    ///
    /// # Errors
    /// - can't get current system time: [`std::time::SystemTimeError`]
    pub fn generate_now(&self) -> Result<String, std::time::SystemTimeError> {
        self.totp.generate_current()
    }
    pub(crate) fn populate_capnp_builder(&self, builder: &mut crate::locker_capnp::key::Builder) {
        builder.set_id(&self.id);
        builder.set_issuer(self.totp.issuer.as_deref().unwrap_or_default());
        builder.set_secret(&self.totp.secret);
        builder.set_account_name(&self.totp.account_name);
    }
}

impl<'a> TryFrom<super::locker_capnp::key::Reader<'a>> for Key {
    type Error = capnp::Error;

    fn try_from(value: super::locker_capnp::key::Reader<'a>) -> Result<Self, Self::Error> {
        let issuer = value.get_issuer()?.to_string()?;
        let issuer = if issuer.is_empty() {
            None
        } else {
            Some(issuer)
        };
        let totp = totp_rs::TOTP {
            algorithm: totp_rs::Algorithm::SHA1,
            digits: 6,
            skew: 1,
            step: 30,
            secret: value.get_secret()?.to_vec(),
            issuer,
            account_name: value.get_account_name()?.to_string()?,
        };
        Ok(Self {
            id: value.get_id()?.to_string()?,
            totp,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn key_gen_code() {
        //? https://github.com/google/google-authenticator/wiki/Key-Uri-Format#examples
        // funny fact: the example secret provided by google is actually *too short*
        let url = "otpauth://totp/Example:mado@fyralabs.com?secret=SAKANOBORUMONOYOHIIROTONARUHOSIHAMIETEIMASUKA&issuer=Example";
        let key = Key::from_str(url).unwrap();
        assert_eq!(&*key.id, "Example: mado@fyralabs.com");
        assert_eq!(&*key.totp.generate(316706482800), "377285");
    }
}
