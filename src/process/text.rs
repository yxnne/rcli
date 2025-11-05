use std::{fs, io::Read, path::Path};

use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

use crate::{get_reader, TextSignFormat};

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized; // Sized 是为了限制 Self 必须是具体类型，而不是 trait 对象
}

pub trait TextSign {
    fn sign(&self, read: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    // fn verify(&self, read: impl Read, signature: &[u8]) -> Result<bool>;
    fn verify<R: Read>(&self, read: R, signature: &[u8]) -> Result<bool>;
}

pub struct Black3 {
    pub key: [u8; 32],
}

pub struct Ed25519Signer {
    pub key: SigningKey,
}

pub struct Ed25519Verifier {
    pub key: VerifyingKey,
}

impl Black3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Black3::new(key);
        Ok(signer)
    }
}

impl KeyLoader for Black3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl TextSign for Black3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(self.key.sign(&buf).to_bytes().to_vec())
    }
}

impl TextVerify for Black3 {
    fn verify<R: Read>(&self, mut reader: R, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == signature)
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify<R: Read>(&self, mut reader: R, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(signature.try_into()?);
        // let signature = signature.try_into()?;
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        let signer = Ed25519Signer::new(key);
        Ok(signer)
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        let verifier = Ed25519Verifier::new(key);
        Ok(verifier)
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }

    // pub fn verify(&self, buf: &[u8], signature: &[u8]) -> Result<bool> {
    //     let sig = Signature::from_bytes(signature.try_into()?);
    //     Ok(self.key.verify(buf, &sig).is_ok())
    // }
}

impl TextVerify for Ed25519Signer {
    fn verify<R: Read>(&self, mut reader: R, signature: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(signature.try_into()?);
        // let signature = signature.try_into()?;
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<()> {
    let mut reader = get_reader(input)?;

    let mut buf = Vec::new();

    reader.read_to_end(&mut buf)?;

    let signed = match format {
        TextSignFormat::Black3 => {
            let signer = Black3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    println!("{}", signed);

    Ok(())
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    format: TextSignFormat,
    signature: &str,
) -> Result<()> {
    let mut reader = get_reader(input)?;
    let signature = URL_SAFE_NO_PAD.decode(signature)?;
    let verify = match format {
        TextSignFormat::Black3 => {
            let verifier = Black3::load(key)?;
            verifier.verify(&mut reader, &signature)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &signature)?
        }
    };
    println!("{}", verify);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*; // 移除未使用的导入

    #[test]
    fn test_black3_text_sign_verify() -> Result<()> {
        let black3 = Black3::load("fixtures/black3.txt")?;
        let data = b"hello world";

        let signature = black3.sign(&mut data.as_slice())?;
        let verify = black3.verify(&mut data.as_slice(), &signature)?;
        assert!(verify);
        Ok(())
    }
}
