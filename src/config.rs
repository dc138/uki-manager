pub struct GlobalConfig {
    pub default_kernel_config: KernelConfig,
}

impl GlobalConfig {
    pub fn new(config_path: String) -> Option<GlobalConfig> {
        todo!();
    }

    pub fn parse_instance(config_path: String) -> Option<KernelConfig> {
        todo!();
    }
}

pub struct KernelConfig {
    pub output_dir: String,
    pub output_name: String,
    pub stub_path: String,
    pub uname: String,
    pub cmdline_path: String,
    pub initrd_paths: Vec<String>,
    pub vmlinuz_path: String,
    pub splash_path: Option<String>,
}
