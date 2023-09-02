use std::fs;
use std::path;
use std::process as proc;

use anyhow::Context;
use colored::Colorize;

use crate::traits::ParseTemplate;

mod cfg;
mod log;
mod opts;
mod traits;
mod uki;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");

macro_rules! unwrap_or_continue {
    ($res:expr) => {
        match $res {
            Some(x) => x,
            None => {
                continue;
            }
        }
    };
}

fn main() -> Result<(), anyhow::Error> {
    let opts = opts::parse_opts()?;

    if opts.help {
        print_help(&opts);
        proc::exit(0);
    } else if opts.version {
        print_version();
        proc::exit(0);
    }

    let config_dir_path = path::Path::new(&opts.config_dir);
    let config_path = path::Path::new(&opts.config_file);

    if !config_dir_path.is_dir() {
        log::println_error!(
            "config-dir ({}) must point to an existing, readable directory",
            opts.config_dir
        );
        proc::exit(1);
    }

    if !config_path.is_file() {
        log::println_error!(
            "config file ({}) must point to an existing, readable file",
            opts.config_file
        );
        proc::exit(1);
    }

    let config_str = fs::read_to_string(config_path).context("cannot read config file")?;

    let config = cfg::Config::from_str_default(&config_str, cfg::Config::default())
        .context("cannot parse config file")?;

    for entry in fs::read_dir(config.vm_dir).context("cannot read vm_dir")? {
        let entry = unwrap_or_continue!(entry.ok());

        let entry_name = unwrap_or_continue!(entry.file_name().into_string().ok());
        let entry_path = entry.path();
        let entry_path_str = entry_path
            .to_str()
            .expect("directory entry should have a valid UTF-8 path");

        let (_, kernel_name) = unwrap_or_continue!(entry_name.split_once("vmlinuz-"));
        let kernel_name = kernel_name.to_string();

        log::println_info!("found installed kernel: {}", kernel_name);

        let kernel_config_path = config_dir_path.join(&format!("{}.toml", kernel_name));

        let kernel_config = {
            if let Ok(kernel_config_str) = fs::read_to_string(&kernel_config_path) {
                let parsed = cfg::KernelConfig::from_str_default(
                    &kernel_config_str,
                    config.default_kernel_config.clone(),
                )
                .unwrap_or(config.default_kernel_config.clone());

                log::println_info!(
                    "found custom config file: {}",
                    kernel_config_path.to_str().unwrap()
                );

                parsed
            } else {
                config.default_kernel_config.clone()
            }
        };

        let mut kernel_config =
            kernel_config.parse_template(&cfg::KernelConfigTemplate { kernel_name });

        let kernel_output_path = path::Path::new(&kernel_config.output_dir)
            .join(path::Path::new(&kernel_config.output_name));

        let kernel_output = kernel_output_path.to_str().unwrap();

        let mut uki = match uki::UnifiedKernelImage::new(&kernel_config.stub_path, &kernel_output) {
            Ok(uki) => uki,
            Err(e) => {
                log::println_warn!("cannot create uki instance: {}, skipping it...", e);
                continue;
            }
        };

        // TODO: configurable osrel
        match uki.add_section_buf(".osrel", "/usr/lib/os-release") {
            Ok(()) => log::println_info!("added {} to .osrel", "/usr/lib/os-release"),
            Err(e) => {
                log::println_warn!(
                    "cannot add .osrel section to executable: {}, skipping it...",
                    e
                );
            }
        };

        // TODO: detect this
        match uki.add_section_buf(".uname", "6.4.8-zen1-1-zen") {
            Ok(()) => log::println_info!("added {} to .uname", "6.4.8-zen1-1-zen"),
            Err(e) => log::println_warn!(
                "cannot add .uname section to executable: {}, skipping it...",
                e
            ),
        };

        match uki.add_section_path(".cmdline", &kernel_config.cmdline_path) {
            Ok(()) => log::println_info!("added {} to .cmdline", kernel_config.cmdline_path),
            Err(e) => log::println_warn!(
                "cannot add .cmdline section to executable: {}, skipping it...",
                e
            ),
        };

        kernel_config
            .initrd_paths
            .retain(|initrd| match path::Path::new(initrd).is_file() {
                true => {
                    log::println_info!("found {}", initrd);
                    true
                }
                false => {
                    log::println_warn!(
                        "initrd file {} is not present or it cannot be read, ignoring it...",
                        initrd
                    );
                    false
                }
            });

        match uki.add_section_paths(".initrd", kernel_config.initrd_paths) {
            Ok(()) => log::println_info!("added initrds to .initrd"),
            Err(e) => log::println_warn!(
                "cannot add .initrd section to executable: {}, skipping it...",
                e
            ),
        };

        match uki.add_section_path(".linux", &entry_path) {
            Ok(()) => log::println_info!("added {} to .linux", entry_path_str),
            Err(e) => log::println_warn!(
                "cannot add .initrd section to executable: {}, skipping it...",
                e
            ),
        };

        match uki.output() {
            Ok(()) => log::println_info!("wrote {}", kernel_output),
            Err(e) => log::println_error!(
                "cannot output efi executable ({}): {}, skipping it...",
                kernel_output,
                e
            ),
        };
    }

    Ok(())
}

fn print_help(opts: &opts::Opts) {
    println!("{}", opts.usage);
}

fn print_version() {
    println!("uki-manager v{}\n", VERSION);
    println!(
        "Copyright (C) 2023 Antonio de Haro. \n\
        This program is distributed under the MIT license, see the attatched LICENSE.txt file for terms and conditions. \n\
        This software is provided without any warranty of any kind. \n\
        Copyright atributions for any third party code included are provided in the attatched COPYRIGHT.md file."
    );
}
