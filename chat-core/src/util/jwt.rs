use anyhow::Result;
use jwt_simple::prelude::*;

use crate::User;

const JWT_DURATION: u64 = 60 * 60 * 24 * 7;
const JWT_ISS: &str = "chat_server";
const JWT_AUD: &str = "chat_web";
pub struct EncodingKey(Ed25519KeyPair);

pub struct DecodingKey(Ed25519PublicKey);

#[allow(unused)]
impl EncodingKey {
    pub fn load(pem: &str) -> Result<Self, jwt_simple::Error> {
        Ok(Self(Ed25519KeyPair::from_pem(pem)?))
    }
    pub fn sign(&self, user: impl Into<User>) -> Result<String, jwt_simple::Error> {
        let cliams = Claims::with_custom_claims(user.into(), Duration::from_secs(JWT_DURATION));
        let claims = cliams.with_issuer(JWT_ISS).with_audience(JWT_AUD);
        self.0.sign(claims)
    }
}
#[allow(unused)]
impl DecodingKey {
    pub fn load(pem: &str) -> Result<Self, jwt_simple::Error> {
        Ok(Self(Ed25519PublicKey::from_pem(pem)?))
    }
    pub fn verify(&self, token: &str) -> Result<User, jwt_simple::Error> {
        let opt = VerificationOptions {
            allowed_audiences: Some(HashSet::from_strings(&[JWT_AUD])),
            allowed_issuers: Some(HashSet::from_strings(&[JWT_ISS])),
            ..Default::default()
        };
        let claims = self.0.verify_token::<User>(token, Some(opt))?;
        Ok(claims.custom)
    }
}
#[cfg(test)]
mod tests {

    use crate::{util::jwt::DecodingKey, User};

    use super::EncodingKey;

    #[test]
    fn pem_load_should_be_work() {
        let pem_path = include_str!("../../../fixtures/privkey.pem");
        let pem = EncodingKey::load(pem_path);
        assert!(pem.is_ok())
    }
    #[test]
    fn jwt_sign_should_be_work() {
        let pem_path = include_str!("../../../fixtures/privkey.pem");
        let pem = EncodingKey::load(pem_path);
        let user = User::new(1, "jackma".to_string(), "email@gmail.com".to_string());
        let token = match pem {
            Ok(encode) => encode.sign(user),
            Err(e) => Err(e),
        };
        println!("{}", token.unwrap())
    }

    #[tokio::test]
    async fn jwt_sign_verify_should_work() -> anyhow::Result<()> {
        let encoding_pem = include_str!("../../../fixtures/privkey.pem");
        let decoding_pem = include_str!("../../../fixtures/pubkey.pem");
        let ek = EncodingKey::load(encoding_pem)?;
        let dk = DecodingKey::load(decoding_pem)?;

        let user = User::new(1, "jackma".to_string(), "email@gmail.com".to_string());

        let token = ek.sign(user.clone())?;
        let user2 = dk.verify(&token)?;

        assert_eq!(user, user2);
        Ok(())
    }
}
