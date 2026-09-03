fn main() {
    let flavor = std::env::var("BUILDSWEEP_FLAVOR").unwrap_or_else(|_| "store".to_string());
    match flavor.as_str() {
        "personal" => {
            println!("cargo:rustc-cfg=build_flavor_personal");
        }
        _ => {
            println!("cargo:rustc-cfg=build_flavor_store");
        }
    }

    let purchase_url = std::env::var("BUILDSWEEP_PURCHASE_URL")
        .unwrap_or_else(|_| "https://buildsweep.app/buy".to_string());
    println!("cargo:rustc-env=BUILDSWEEP_PURCHASE_URL={purchase_url}");

    tauri_build::build()
}
