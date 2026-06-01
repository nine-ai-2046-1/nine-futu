use sha1::{Digest, Sha1};

use crate::error::FutuError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PacketEncAlgo {
    FtaesEcb = 0,
    None = -1,
    AesEcb = 1,
    AesCbc = 2,
}

impl PacketEncAlgo {
    pub fn from_i32(v: i32) -> Self {
        match v {
            0 => Self::FtaesEcb,
            -1 => Self::None,
            1 => Self::AesEcb,
            2 => Self::AesCbc,
            _ => Self::None,
        }
    }
}

pub struct FutuEncryption {
    #[allow(dead_code)]
    aes_key: [u8; 16],
    algo: PacketEncAlgo,
}

impl FutuEncryption {
    pub fn new(aes_key: [u8; 16], algo: PacketEncAlgo) -> Self {
        Self { aes_key, algo }
    }

    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, FutuError> {
        match self.algo {
            PacketEncAlgo::None => Ok(data.to_vec()),
            _ => {
                // Placeholder for actual encryption implementation
                // TODO: Implement actual AES encryption
                Ok(data.to_vec())
            }
        }
    }

    pub fn decrypt(&self, data: &[u8]) -> Result<Vec<u8>, FutuError> {
        match self.algo {
            PacketEncAlgo::None => Ok(data.to_vec()),
            _ => {
                // Placeholder for actual decryption implementation
                // TODO: Implement actual AES decryption
                Ok(data.to_vec())
            }
        }
    }
}

pub fn compute_sha1(data: &[u8]) -> [u8; 20] {
    let mut hasher = Sha1::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; 20];
    hash.copy_from_slice(&result);
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha1_empty_data() {
        let data = b"";
        let hash = compute_sha1(data);
        assert_eq!(hash.len(), 20);
        // SHA1 of empty string is well-known
        assert_eq!(hash, [0xda, 0x39, 0xa3, 0xee, 0x5e, 0x6b, 0x4b, 0x0d, 0x32, 0x55, 0xbf, 0xef, 0x95, 0x60, 0x18, 0x90, 0xaf, 0xd8, 0x07, 0x09]);
    }

    #[test]
    fn test_sha1_hello_world() {
        let data = b"hello world";
        let hash = compute_sha1(data);
        assert_eq!(hash.len(), 20);
        // Just verify it's a valid 20-byte hash
        assert!(hash.iter().any(|&b| b != 0)); // Not all zeros
    }

    #[test]
    fn test_sha1_different_inputs_different_hashes() {
        let data1 = b"test data 1";
        let data2 = b"test data 2";
        
        let hash1 = compute_sha1(data1);
        let hash2 = compute_sha1(data2);
        
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_sha1_same_input_same_hash() {
        let data = b"consistent data";
        
        let hash1 = compute_sha1(data);
        let hash2 = compute_sha1(data);
        
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_packet_enc_algo_from_i32() {
        assert_eq!(PacketEncAlgo::from_i32(0), PacketEncAlgo::FtaesEcb);
        assert_eq!(PacketEncAlgo::from_i32(-1), PacketEncAlgo::None);
        assert_eq!(PacketEncAlgo::from_i32(1), PacketEncAlgo::AesEcb);
        assert_eq!(PacketEncAlgo::from_i32(2), PacketEncAlgo::AesCbc);
        assert_eq!(PacketEncAlgo::from_i32(100), PacketEncAlgo::None); // Default
    }

    #[test]
    fn test_no_encryption_passthrough() {
        let key = [0x01; 16];
        let enc = FutuEncryption::new(key, PacketEncAlgo::None);

        let plaintext = b"test data";
        let encrypted = enc.encrypt(plaintext).unwrap();
        let decrypted = enc.decrypt(&encrypted).unwrap();

        assert_eq!(encrypted, plaintext);
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_no_encryption_empty_data() {
        let key = [0x01; 16];
        let enc = FutuEncryption::new(key, PacketEncAlgo::None);

        let plaintext = b"";
        let encrypted = enc.encrypt(plaintext).unwrap();
        let decrypted = enc.decrypt(&encrypted).unwrap();

        assert_eq!(encrypted, plaintext);
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_no_encryption_large_data() {
        let key = [0x01; 16];
        let enc = FutuEncryption::new(key, PacketEncAlgo::None);

        let plaintext = vec![0u8; 1024 * 1024]; // 1MB
        let encrypted = enc.encrypt(&plaintext).unwrap();
        let decrypted = enc.decrypt(&encrypted).unwrap();

        assert_eq!(encrypted, plaintext);
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encryption_different_keys() {
        let key1 = [0x01; 16];
        let key2 = [0x02; 16];
        
        let enc1 = FutuEncryption::new(key1, PacketEncAlgo::None);
        let enc2 = FutuEncryption::new(key2, PacketEncAlgo::None);

        let plaintext = b"test";
        
        // With None encryption, key doesn't matter
        let enc1_result = enc1.encrypt(plaintext).unwrap();
        let enc2_result = enc2.encrypt(plaintext).unwrap();
        
        assert_eq!(enc1_result, enc2_result);
    }
}
