use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct BuildInfo {
    pub flavor: &'static str,
    pub product_name: &'static str,
    pub allow_local_license: bool,
    pub purchase_url: String,
    pub pro_price_label: String,
}

pub fn get_build_info() -> BuildInfo {
    if cfg!(build_flavor_personal) {
        BuildInfo {
            flavor: "personal",
            product_name: "BuildSweep Personal",
            allow_local_license: true,
            purchase_url: String::new(),
            pro_price_label: String::new(),
        }
    } else {
        BuildInfo {
            flavor: "store",
            product_name: "BuildSweep",
            allow_local_license: false,
            purchase_url: env!("BUILDSWEEP_PURCHASE_URL").to_string(),
            pro_price_label: "Pro — $7.99 lifetime".to_string(),
        }
    }
}

pub fn allow_local_license() -> bool {
    cfg!(build_flavor_personal)
}
