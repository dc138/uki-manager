#[derive(serde::Deserialize)]
pub struct ConfigOpt {
    pub boot_dir: Option<String>,
    pub esp_dir: Option<String>,
    pub output_dir: Option<String>,
}

pub struct Config {
    pub boot_dir: String,
    pub esp_dir: String,
    pub output_dir: String,
}

#[derive(serde::Deserialize)]
pub struct KernelConfigOpt {
    pub output: Option<String>,
    pub stub_path: Option<String>,
    pub cmdline_path: Option<String>,
    pub initrd_paths: Option<Vec<String>>,
    pub vmlinuz_path: Option<String>,
    pub splash_path: Option<String>,
}

pub struct KernelConfig {
    pub output: String,
    pub stub_path: String,
    pub cmdline_path: String,
    pub initrd_paths: Vec<String>,
    pub vmlinuz_path: String,
}
