use sp_core::{crypto::Ss58Codec, sr25519, ByteArray};
use sp_io::crypto::sr25519_verify;

use crate::{error::ApiError, types::PingMessage};

pub fn verify_sr25519_signature(
    ss58_public_key: &str,
    message: &PingMessage,
    signature: Vec<u8>,
) -> Result<bool, ApiError> {
    let message_bytes =
        serde_json::to_vec(message).map_err(|e| ApiError::SerializationError(e.to_string()))?;
    let public_key =
        sr25519::Public::from_ss58check(ss58_public_key).map_err(|_| ApiError::InvalidSignature)?;

    let signature =
        sr25519::Signature::from_slice(&signature).map_err(|_| ApiError::InvalidSignature)?;

    Ok(sr25519_verify(&signature, &message_bytes, &public_key))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sp_core::sr25519::Pair as Sr25519Pair;
    use sp_core::Pair;

    fn create_test_message() -> PingMessage {
        PingMessage {
            timestamp: 1234567890,
            multiaddr: "/ip4/127.0.0.1/tcp/30333".to_string(),
            peer_id: "12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2".to_string(),
            block_number: "123456".to_string(),
        }
    }

    #[test]
    fn test_valid_signature() {
        // Generate a keypair for testing
        let pair = Sr25519Pair::generate().0;
        let public_key = pair.public();
        let ss58_public_key = public_key.to_ss58check();

        // Create and sign a message
        let message = create_test_message();
        let message_bytes = serde_json::to_vec(&message).unwrap();
        let signature = pair.sign(&message_bytes);

        // Verify the signature
        let result = verify_sr25519_signature(&ss58_public_key, &message, signature.to_vec());

        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_invalid_signature() {
        // Generate two different keypairs
        let pair1 = Sr25519Pair::generate().0;
        let pair2 = Sr25519Pair::generate().0;
        let public_key1 = pair1.public();
        let ss58_public_key1 = public_key1.to_ss58check();

        // Sign with pair2 but try to verify with public_key1
        let message = create_test_message();
        let message_bytes = serde_json::to_vec(&message).unwrap();
        let signature = pair2.sign(&message_bytes);

        let result = verify_sr25519_signature(&ss58_public_key1, &message, signature.to_vec());

        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_invalid_public_key_format() {
        let message = create_test_message();
        let invalid_public_key = "invalid_public_key";
        let dummy_signature = vec![0u8; 64];

        let result = verify_sr25519_signature(invalid_public_key, &message, dummy_signature);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::InvalidSignature));
    }

    #[test]
    fn test_invalid_signature_length() {
        let pair = Sr25519Pair::generate().0;
        let public_key = pair.public();
        let ss58_public_key = public_key.to_ss58check();

        let message = create_test_message();
        let invalid_signature = vec![0u8; 32]; // Invalid length

        let result = verify_sr25519_signature(&ss58_public_key, &message, invalid_signature);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ApiError::InvalidSignature));
    }

    // #[test]
    // fn test_tampered_message() {
    //     let pair = Sr25519Pair::generate().0;
    //     let public_key = pair.public();
    //     let ss58_public_key = public_key.to_ss58check();

    //     // Sign original message
    //     let original_message = create_test_message();
    //     let message_bytes = serde_json::to_vec(&original_message).unwrap();
    //     let signature = pair.sign(&message_bytes);

    //     // Create tampered message
    //     let mut tampered_message = original_message.clone();
    //     tampered_message.peer_id =
    //         "12D3KooWHDNG8W9q4oVqfJ8CG7G6XwzHhp3QbKSU4LGKrH9NVsJN".to_string();

    //     // Verify with tampered message
    //     let result = verify_sr25519_signature(
    //         &ss58_public_key,
    //         &tampered_message,
    //         signature.as_ref().to_vec(),
    //     );

    //     assert!(result.is_ok());
    //     assert!(!result.unwrap());
    // }

    // #[test]
    // fn test_modified_timestamp() {
    //     let pair = Sr25519Pair::generate().0;
    //     let public_key = pair.public();
    //     let ss58_public_key = public_key.to_ss58check();

    //     // Sign original message
    //     let original_message = create_test_message();
    //     let message_bytes = serde_json::to_vec(&original_message).unwrap();
    //     let signature = pair.sign(&message_bytes);

    //     // Create message with modified timestamp
    //     let mut modified_message = original_message.clone();
    //     modified_message.timestamp = 9876543210;

    //     // Verify with modified message
    //     let result = verify_sr25519_signature(
    //         &ss58_public_key,
    //         &modified_message,
    //         signature.as_ref().to_vec(),
    //     );

    //     assert!(result.is_ok());
    //     assert!(!result.unwrap());
    // }
}
