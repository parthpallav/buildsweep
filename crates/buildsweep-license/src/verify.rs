use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Local dev public key — pair with tools/license-signer/.keys/private.key
// Run: cargo run -p license-signer -- install-dev
pub const EMBEDDED_PUBLIC_KEY_B64: &str = "E+xKfbjmtNc9AlOlBdG4DYKM4cRfdZQorHeiVNifGEw=";

#[derive(Debug, Error)]
pub enum LicenseError {
    #[error("invalid license format: {0}")]
    InvalidFormat(String),
    #[error("invalid signature")]
    InvalidSignature,
    #[error("license verification failed: {0}")]
    VerificationFailed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LicenseTier {
    Free,
    Pro,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicensePayload {
    pub tier: LicenseTier,
    pub license_id: String,
    pub issued_at: String,
    #[serde(default)]
    pub customer_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedLicense {
    pub payload: LicensePayload,
    pub signature: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LicenseStatus {
    pub tier: LicenseTier,
    pub license_id: Option<String>,
    pub valid: bool,
    pub message: String,
}

pub fn verify_license(license_json: &str, public_key_b64: &str) -> Result<LicenseStatus, LicenseError> {
    let signed: SignedLicense = serde_json::from_str(license_json)
        .map_err(|e| LicenseError::InvalidFormat(e.to_string()))?;

    let payload_bytes = serde_json::to_vec(&signed.payload)
        .map_err(|e| LicenseError::InvalidFormat(e.to_string()))?;

    let key_bytes = BASE64
        .decode(public_key_b64)
        .map_err(|e| LicenseError::InvalidFormat(e.to_string()))?;

    if key_bytes.len() != 32 {
        return Err(LicenseError::VerificationFailed(
            "invalid public key length".to_string(),
        ));
    }

    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&key_bytes);
    let verifying_key = VerifyingKey::from_bytes(&key_array)
        .map_err(|e| LicenseError::VerificationFailed(e.to_string()))?;

    let sig_bytes = BASE64
        .decode(&signed.signature)
        .map_err(|e| LicenseError::InvalidFormat(e.to_string()))?;

    let signature = Signature::from_slice(&sig_bytes)
        .map_err(|e| LicenseError::InvalidFormat(e.to_string()))?;

    verifying_key
        .verify(&payload_bytes, &signature)
        .map_err(|_| LicenseError::InvalidSignature)?;

    Ok(LicenseStatus {
        tier: signed.payload.tier.clone(),
        license_id: Some(signed.payload.license_id.clone()),
        valid: true,
        message: "License valid".to_string(),
    })
}

pub fn free_status() -> LicenseStatus {
    LicenseStatus {
        tier: LicenseTier::Free,
        license_id: None,
        valid: true,
        message: "Free tier".to_string(),
    }
}

pub fn sign_license(
    payload: LicensePayload,
    private_key_b64: &str,
) -> Result<String, LicenseError> {
    use ed25519_dalek::{Signer, SigningKey};

    let key_bytes = BASE64
        .decode(private_key_b64.trim())
        .map_err(|e| LicenseError::InvalidFormat(e.to_string()))?;

    if key_bytes.len() != 32 {
        return Err(LicenseError::InvalidFormat(
            "private key must be 32 bytes".to_string(),
        ));
    }

    let mut key_array = [0u8; 32];
    key_array.copy_from_slice(&key_bytes);
    let signing_key = SigningKey::from_bytes(&key_array);

    let payload_bytes = serde_json::to_vec(&payload)
        .map_err(|e| LicenseError::InvalidFormat(e.to_string()))?;
    let signature = signing_key.sign(&payload_bytes);

    let signed = SignedLicense {
        payload,
        signature: BASE64.encode(signature.to_bytes()),
    };

    serde_json::to_string(&signed).map_err(|e| LicenseError::InvalidFormat(e.to_string()))
}

pub fn generate_local_pro_license(
    private_key_b64: &str,
    license_id: &str,
) -> Result<String, LicenseError> {
    sign_license(
        LicensePayload {
            tier: LicenseTier::Pro,
            license_id: license_id.to_string(),
            issued_at: chrono::Utc::now().to_rfc3339(),
            customer_id: Some("local-dev".to_string()),
        },
        private_key_b64,
    )
}

pub fn current_status(stored_license: Option<&str>) -> LicenseStatus {
    if cfg!(debug_assertions) {
        if std::env::var("BUILDSWEEP_DEV_PRO").ok().as_deref() == Some("1") {
            return LicenseStatus {
                tier: LicenseTier::Pro,
                license_id: Some("dev-pro".to_string()),
                valid: true,
                message: "Dev Pro mode".to_string(),
            };
        }
    }

    if let Some(lic) = stored_license {
        match verify_license(lic, EMBEDDED_PUBLIC_KEY_B64) {
            Ok(status) if status.tier == LicenseTier::Pro => return status,
            _ => {}
        }
    }

    free_status()
}
