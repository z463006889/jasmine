use std::fs;
use std::io::Read;
use std::path::Path;

use anyhow::Ok;
use anyhow::Result;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use rand::rngs::OsRng;


use crate::get_reader;
use crate::TextSignFormat;
use ed25519_dalek::Signature;
use ed25519_dalek::Signer;
use ed25519_dalek::SigningKey;
use ed25519_dalek::{Verifier,VerifyingKey};

use super::gen_process::process_password;

pub trait TextSign{
    fn sign<T:Read>(&self,data:&mut T)->Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify<T:Read>(&self,data:&mut T,sign:&[u8]) -> Result<bool>;
}

pub trait KeyLoader {
    fn load(path:impl AsRef<Path>) -> Result<Self>
    where Self:Sized;
}
pub struct Blake3{
    key:[u8;32]
}
pub struct Ed25519Sign{
    key:SigningKey,
}

pub struct Ed25519Verify{
    key:VerifyingKey
}

pub trait KeyGeneateor{
    fn generate()->Result<Vec<Vec<u8>>>;
}


impl KeyLoader for Blake3{
    fn load(path:impl AsRef<Path>) -> Result<Self>{
        let path = fs::read(path)?;
        Self::try_new(&path)
    }
}

impl Blake3{
    fn new(key:[u8;32]) ->Self {
        Self {key: key}
    }
    fn try_new(key:&[u8])->Result<Self> {      
        let key = &key[..32];
        let key=key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
    }
}

impl KeyGeneateor for Blake3{
    fn generate()->Result<Vec<Vec<u8>>> {
        let opts =crate::GenPassOpts{
            length: 32,
            uppercase:true,
            lowercase:true,
            number:true,
            symbols:true,
        };
        let ret = process_password(&opts)?;
        let key = ret.0.as_bytes().to_vec();
        Ok(vec![key])
    }
}

impl KeyGeneateor for Ed25519Sign {
    fn generate()->Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        let pk = signing_key.verifying_key().as_bytes().to_vec();
        let sk=signing_key.as_bytes().to_vec();
        Ok(vec![sk,pk])
    }
}

impl TextSign for Blake3 {
    fn sign<T:Read>(&self,data:&mut T)->Result<Vec<u8>> {
        let mut buf=Vec::new();
        let _ = data.read_to_end(&mut buf);
        Ok(blake3::keyed_hash(&self.key,&buf).as_bytes().to_vec())
    }
}

impl KeyLoader for Ed25519Sign{
    fn load(path:impl AsRef<Path>) -> Result<Self>  {
        let path = fs::read(path)?;
        Self::try_new(&path)
    }
}

impl Ed25519Sign {
    fn new(key:SigningKey) ->Self {
        Self{key}
    }

    fn try_new(key:&[u8]) -> Result<Self>{
        let key = SigningKey::from_bytes(key.try_into()?);
        let ed =Self::new(key);
        Ok(ed)
    }
}


impl KeyLoader for Ed25519Verify{
    fn load(path:impl AsRef<Path>) -> Result<Self> {
        let path = fs::read(path)?;
        Self::try_new(&path)
    }
} 

impl  Ed25519Verify{
    fn new(key:VerifyingKey) ->Self{
        Self{key}
    }


    fn try_new(key:&[u8]) -> Result<Self>{
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        Ok(Self::new(key))
    }

}

impl TextSign for Ed25519Sign {
    fn sign<T:Read>(&self,data:&mut T)->Result<Vec<u8>> {
        let mut buf=Vec::new();
        let _ = data.read_to_end(&mut buf);
        let sig=self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify<T:Read>(&self,data:&mut T,sign:&[u8]) -> Result<bool> {
        let mut buf=Vec::new();
        let _ = data.read_to_end(&mut buf);
        let hashs =blake3::keyed_hash(&self.key,&buf);
        let r = hashs.as_bytes();
        Ok(r==sign)
    }
}

impl TextVerify for Ed25519Verify {
    fn verify<T:Read>(&self,data:&mut T,sign:&[u8]) -> Result<bool> {
        let mut buf=Vec::new();
        let _ = data.read_to_end(&mut buf);
        let sig = Signature::from_bytes(&sign.try_into()?);
        let ret =self.key.verify(&buf,&sig).is_ok();
        Ok(ret)
    }
}

pub fn process_sign(input:&str, key:&str,format:TextSignFormat)->anyhow::Result<()>{
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3=>{
            let b = Blake3::load(key)?;
            b.sign(&mut reader)?
        },
        TextSignFormat::Ed25519=>{
            let ed = Ed25519Sign::load(key)?;
            ed.sign(&mut reader)?   
        }
    };
    let signed = URL_SAFE_NO_PAD.encode(signed);
    println!("signed: {:?}",signed);
    Ok(())
}

pub fn process_verify(input:&str,key:&str,format:TextSignFormat,sign:&str)->anyhow::Result<()>{
    let mut reader = get_reader(input)?;
    let sign = URL_SAFE_NO_PAD.decode(sign)?;
    let verify = match format { 
        TextSignFormat::Blake3=>{
            let b =Blake3::load(key)?;
            b.verify(&mut reader,&sign)?
        },
        TextSignFormat::Ed25519=>{
            let ed =Ed25519Verify::load(key)?;
            ed.verify(&mut reader, &sign)?
        }
    };
    println!("verify: {:?}",verify);
    Ok(())
}

pub fn process_key_gen(format:TextSignFormat)->anyhow::Result<Vec<Vec<u8>>>{
    match format {
        TextSignFormat::Blake3=>Blake3::generate(),
        TextSignFormat::Ed25519=>Ed25519Sign::generate()
    }
}