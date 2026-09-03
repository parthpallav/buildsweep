//! Dev-only license signing tool. Private key must never be committed.
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use buildsweep_license::{
    generate_local_pro_license, LicensePayload, LicenseTier, EMBEDDED_PUBLIC_KEY_B64,
};
use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use rand::RngCore;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: license-signer <generate|install-dev|public-key|sign> [license_id]");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "generate" => generate_keys(),
        "install-dev" => install_dev_key(),
        "public-key" => print_public_key(),
        "sign" => {
            let license_id = args.get(2).cloned().unwrap_or_else(|| "LIC-LOCAL".to_string());
            sign_license(&license_id);
        }
        _ => {
            eprintln!("Unknown command");
            std::process::exit(1);
        }
    }
}

fn keys_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".keys")
}

fn home_dev_key_path() -> PathBuf {
    dirs::home_dir()
        .expect("home dir")
        .join(".buildsweep/dev-private.key")
}

fn generate_keys() {
    let dir = keys_dir();
    fs::create_dir_all(&dir).expect("create keys dir");
    let mut secret = [0u8; 32];
    OsRng.fill_bytes(&mut secret);
    let signing_key = SigningKey::from_bytes(&secret);
    let verifying_key: VerifyingKey = signing_key.verifying_key();

    fs::write(dir.join("private.key"), BASE64.encode(signing_key.to_bytes())).unwrap();
    fs::write(dir.join("public.key"), BASE64.encode(verifying_key.to_bytes())).unwrap();
    println!("Keys written to {:?}", dir);
    println!("Public key (embedded in app): {}", BASE64.encode(verifying_key.to_bytes()));
    println!("Run `cargo run -p license-signer -- install-dev` then rebuild the app if the public key changed.");
    println!("NEVER commit private.key");
}

fn install_dev_key() {
    let src = keys_dir().join("private.key");
    if !src.is_file() {
        eprintln!("No keys found. Run `cargo run -p license-signer -- generate` first.");
        std::process::exit(1);
    }

    let dest = home_dev_key_path();
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).expect("create ~/.buildsweep");
    }
    fs::copy(&src, &dest).expect("copy private key");
    println!("Installed dev signing key to {}", dest.display());
    println!("In BuildSweep Settings, click \"Generate local Pro license\" to activate.");
}

fn print_public_key() {
    let path = keys_dir().join("public.key");
    if path.is_file() {
        let key = fs::read_to_string(path).expect("read public key");
        println!("{}", key.trim());
    } else {
        println!("{}", EMBEDDED_PUBLIC_KEY_B64);
    }
}

fn sign_license(license_id: &str) {
    let dir = keys_dir();
    let private_b64 = fs::read_to_string(dir.join("private.key")).expect("read private key");
    let json = generate_local_pro_license(private_b64.trim(), license_id).expect("sign license");
    println!("{}", json);
}
