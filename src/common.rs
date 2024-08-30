use sha2::{Digest, Sha256};

pub fn calculate_discriminator(account_type: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", "global", account_type);
    let mut hasher = Sha256::new();
    hasher.update(preimage.as_bytes());

    let result = hasher.finalize();

    let mut discriminator = [0u8; 8];
    discriminator.copy_from_slice(&result[..8]);

    discriminator
}