use aes_gcm_siv::{Aes256GcmSiv, KeyInit};
use aes_gcm_siv::aead::Aead;
use core_lib::model::crypto::{KeyEntry, KeyMap};
use generic_array::GenericArray;
use hkdf::Hkdf;
use sha2::Sha256;
use std::collections::HashMap;
use std::sync::Mutex;
use anyhow::anyhow;
use once_cell::sync::Lazy;
use crate::model::doc_type::DocumentType;
use crate::model::crypto::MasterKey;
use rand::{RngCore, SeedableRng};

const EXP_KEY_SIZE: usize = 32;
const EXP_NONCE_SIZE: usize = 12;
const EXP_BUFF_SIZE: usize = 44;

fn initialize_kdf() -> (String, Hkdf<Sha256>) {
    let salt = generate_random_seed();
    let ikm = generate_random_seed();
    let (master_key, kdf) = Hkdf::<Sha256>::extract(Some(&salt), &ikm);
    (hex::encode_upper(master_key), kdf)
}

/// Generates a random seed with 256 bytes.
pub fn generate_random_seed() -> Vec<u8>{
    // Init crypto RNG once lazy
    static RNG: Lazy<Mutex<rand::rngs::StdRng>> = Lazy::new(|| Mutex::new(rand::rngs::StdRng::from_entropy()));
    // Create a buffer to fill with random bytes
    let mut buf = [0u8; 256];

    // Fill buffer with random bytes in a block, so the mutex is locked for a short time.
    {
        RNG.lock()
            .expect("This mutex locking is fine, because it will be release immediately after use and this is the only place of usage ")
            .fill_bytes(&mut buf);
    }

    buf.to_vec()
}

fn derive_key_map(kdf: Hkdf<Sha256>, dt: DocumentType, enc: bool) -> HashMap<String, KeyEntry>{
    let mut key_map = HashMap::new();
    let mut okm = [0u8; EXP_BUFF_SIZE];
    let mut i = 0;
    dt.parts.iter()
        .for_each( |p| {
            if kdf.expand(p.name.clone().as_bytes(), &mut okm).is_ok() {
                let map_key = match enc{
                    true => p.name.clone(),
                    false => i.to_string()
                };
                key_map.insert(map_key, KeyEntry::new(i.to_string(), okm[..EXP_KEY_SIZE].to_vec(), okm[EXP_KEY_SIZE..].to_vec()));
            }
            i = i +1;
        });
    key_map
}

pub fn generate_key_map(mkey: MasterKey, dt: DocumentType) -> anyhow::Result<KeyMap>{
    debug!("generating encryption key_map for doc type: '{}'", &dt.id);
    let (secret, doc_kdf) = initialize_kdf();
    let key_map = derive_key_map(doc_kdf, dt, true);

    debug!("encrypting the key seed");
    let kdf = restore_kdf(&mkey.key)?;
    let mut okm = [0u8; EXP_BUFF_SIZE];
    if kdf.expand(hex::decode(mkey.salt)?.as_slice(), &mut okm).is_err(){
        return Err(anyhow!("Error while generating key"));
    }
    match encrypt_secret(&okm[..EXP_KEY_SIZE], &okm[EXP_KEY_SIZE..], secret){
        Ok(ct) => Ok(KeyMap::new(true, key_map, Some(ct))),
        Err(e) => {
            error!("Error while encrypting key seed: {:?}", e);
            Err(anyhow!("Error while encrypting key seed!"))
        }
    }
}

pub fn restore_key_map(mkey: MasterKey, dt: DocumentType, keys_ct: Vec<u8>) -> anyhow::Result<KeyMap>{
    debug!("decrypting the key seed");
    let kdf = restore_kdf(&mkey.key)?;
    let mut okm = [0u8; EXP_BUFF_SIZE];
    if kdf.expand(hex::decode(mkey.salt)?.as_slice(), &mut okm).is_err(){
        return Err(anyhow!("Error while generating key"));
    }

    match decrypt_secret(&okm[..EXP_KEY_SIZE], &okm[EXP_KEY_SIZE..], &keys_ct){
        Ok(key_seed) => {
            // generate new random key map
            restore_keys(&key_seed, dt)
        }
        Err(e) => {
            error!("Error while decrypting key ciphertext: {}", e);
            Err(anyhow!("Error while decrypting keys"))
        }
    }
}

pub fn restore_keys(secret: &String, dt: DocumentType) -> anyhow::Result<KeyMap>{
    debug!("restoring decryption key_map for doc type: '{}'", &dt.id);
    let kdf = restore_kdf(secret)?;
    let key_map = derive_key_map(kdf, dt, false);

    Ok(KeyMap::new(false, key_map, None))
}

fn restore_kdf(secret: &String) -> anyhow::Result<Hkdf<Sha256>>{
    debug!("restoring kdf from secret");
    let prk = match hex::decode(secret){
        Ok(key) => key,
        Err(e) => {
            error!("Error while decoding master key: {}", e);
            return Err(anyhow!("Error while encrypting key seed!"));
        }
    };

    match Hkdf::<Sha256>::from_prk(prk.as_slice()){
        Ok(kdf) => Ok(kdf),
        Err(e) => {
            error!("Error while instantiating hkdf: {}", e);
            Err(anyhow!("Error while encrypting key seed!"))
        }
    }
}

pub fn encrypt_secret(key: &[u8], nonce: &[u8], secret: String) -> anyhow::Result<Vec<u8>>{
    // check key size
    if key.len() != EXP_KEY_SIZE {
        error!("Given key has size {} but expected {} bytes", key.len(), EXP_KEY_SIZE);
        Err(anyhow!("Incorrect key size"))
    }
    // check nonce size
    else if nonce.len() != EXP_NONCE_SIZE {
        error!("Given nonce has size {} but expected {} bytes", nonce.len(), EXP_NONCE_SIZE);
        Err(anyhow!("Incorrect nonce size"))
    }
    else{
        let key = GenericArray::from_slice(key);
        let nonce = GenericArray::from_slice(nonce);
        let cipher = Aes256GcmSiv::new(key);

        match cipher.encrypt(nonce, secret.as_bytes()){
            Ok(ct) => {
                Ok(ct)
            }
            Err(e) => Err(anyhow!("Error while encrypting {}", e))
        }
    }
}

pub fn decrypt_secret(key: &[u8], nonce: &[u8], ct: &[u8]) -> anyhow::Result<String>{
    debug!("key len = {}", key.len());
    debug!("ct len = {}", ct.len());
    let key = GenericArray::from_slice(key);
    let nonce = GenericArray::from_slice(nonce);
    let cipher = Aes256GcmSiv::new(key);

    debug!("key: {}", hex::encode_upper(key));
    debug!("nonce: {}", hex::encode_upper(nonce));

    debug!("ct len = {}", ct.len());
    debug!("nonce len = {}", nonce.len());
    match cipher.decrypt(nonce, ct){
        Ok(pt) => {
            let pt = String::from_utf8(pt)?;
            Ok(pt)
        },
        Err(e) => {
            Err(anyhow!("Error while decrypting: {}", e))
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_generate_random_seed() {
        for _ in 1..100 {
            let seed = super::generate_random_seed();
            // Check length of seed
            assert_eq!(256, seed.len());
            // Check that the seed is not all zeros
            assert_ne!(0, seed.iter().map(|b| *b as usize).sum::<usize>());
        }
    }
}