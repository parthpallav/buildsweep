use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use buildsweep_license::{
    generate_local_pro_license, verify_license, LicenseTier, EMBEDDED_PUBLIC_KEY_B64,
};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use rand::RngCore;

use buildsweep_license::{free_status, LicenseTier as LT};

#[test]
fn free_tier_by_default() {
    let status = free_status();
    assert_eq!(status.tier, LT::Free);
    assert!(status.valid);
}

#[test]
fn tampered_license_rejected() {
    let bad = r#"{"payload":{"tier":"pro","license_id":"x","issued_at":"2024-01-01"},"signature":"AAAA"}"#;
    let result = verify_license(bad, EMBEDDED_PUBLIC_KEY_B64);
    assert!(result.is_err());
}

#[test]
fn local_sign_and_verify_roundtrip() {
    let mut secret = [0u8; 32];
    OsRng.fill_bytes(&mut secret);
    let signing_key = SigningKey::from_bytes(&secret);
    let verifying_key: VerifyingKey = signing_key.verifying_key();
    let public_b64 = BASE64.encode(verifying_key.to_bytes());
    let private_b64 = BASE64.encode(signing_key.to_bytes());

    let json = generate_local_pro_license(&private_b64, "TEST-001").unwrap();
    let status = verify_license(&json, &public_b64).unwrap();
    assert_eq!(status.tier, LicenseTier::Pro);
    assert!(status.valid);
}

#[test]
fn embedded_public_key_accepts_signer_output() {
    let keys_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tools/license-signer/.keys/private.key");
    if !keys_dir.is_file() {
        return;
    }
    let private_b64 = std::fs::read_to_string(keys_dir).unwrap();
    let json = generate_local_pro_license(private_b64.trim(), "EMBED-TEST").unwrap();
    let status = verify_license(&json, EMBEDDED_PUBLIC_KEY_B64).unwrap();
    assert_eq!(status.tier, LicenseTier::Pro);
}
