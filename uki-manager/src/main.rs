use std::fs;
use std::path;
use std::process as proc;

use anyhow::Context;
use colored::Colorize;

use crate::log::println_info;
use crate::log::println_warn;
use log::println_error;
use log::println_note;

use crate::cfg::KernelConfig;
use crate::cfg::KernelConfigOpt;

mod cfg;
mod log;
mod opts;
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

    let config_path = path::Path::new(&opts.config);
    let config_dir_path = path::Path::new(&opts.config_dir);

    if !config_path.is_file() {
        log::println_error!(
            "config ({}) must point to an existing, readable file",
            opts.config
        );
        proc::exit(1);
    }

    if !config_dir_path.is_dir() {
        log::println_error!(
            "config-dir ({}) must point to an existing, readable directory",
            opts.config_dir
        );
        proc::exit(1);
    }

    let config_str = fs::read_to_string(config_path).context("cannot read config file")?;
    let config_opt: cfg::ConfigOpt =
        toml::from_str(&config_str).context("cannot parse config file")?;

    let config = {
        let boot_dir = config_opt.boot_dir.unwrap_or_else(|| {
            let opts = vec!["/boot/"];

            for opt in &opts {
                if path::Path::new(&opt).is_dir() {
                    println_info!("using {} as boot directory", opt);
                    return opt.to_string();
                }
            }

            println_error!("cannot find the boot directory, and none was provided, aborting...");
            println_note!("tried the following directories: {:?}", &opts);
            proc::exit(1);
        });

        let esp_dir = config_opt.esp_dir.unwrap_or_else(|| {
            let opts = vec!["/efi/", "/esp/", "/boot/"];

            for opt in &opts {
                if path::Path::new(&opt).is_dir() {
                    println_info!("using {} as EFI system partition", opt);
                    return opt.to_string();
                }
            }

            println_error!(
                "cannot find the EFI system partition, and none was provided, aborting..."
            );
            println_note!("tried the following directories: {:?}", &opts);
            proc::exit(1);
        });

        let output_dir = config_opt.output_dir.unwrap_or_else(|| {
            let opts = vec!["EFI/Linux"];

            for opt in &opts {
                if path::Path::new(&opt).is_dir() {
                    println_info!("using {} as output directory", opt);
                    return opt.to_string();
                }
            }

            println_error!(
                "cannot find a suitable output directory, and none was provided, aborting..."
            );
            println_note!("tried the following directories: {:?}", &opts);
            proc::exit(1);
        });

        cfg::Config {
            boot_dir,
            esp_dir,
            output_dir,
        }
    };

    for entry in fs::read_dir(&config.boot_dir).context("cannot read boot directory")? {
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
        let kernel_config_str = fs::read_to_string(&kernel_config_path).unwrap_or_default();

        if kernel_config_path.is_file() {
            println_info!("using custom kernel config file");
        }

        let kernel_config_opt: KernelConfigOpt =
            toml::from_str(&kernel_config_str).unwrap_or_else(|_| {
                println_warn!(
                    "cannot parse custom kernel config file {}, ignoring it...",
                    kernel_config_str
                );
                KernelConfigOpt {
                    output: None,
                    stub_path: None,
                    cmdline_path: None,
                    initrd_paths: None,
                    vmlinuz_path: None,
                    splash_path: None,
                }
            });

        let kernel_config = KernelConfig {
            output: kernel_config_opt.output.unwrap_or_else(|| {
                let output_path = path::Path::new(&config.output_dir).join(&kernel_name);
                let output = output_path.to_string_lossy();

                println_info!("using {} as output name", output);
                output.to_string()
            }),
            stub_path: kernel_config_opt.stub_path.unwrap_or_else(|| {
                let opts = vec![
                    "/usr/lib/systemd/boot/efi/linuxx64.efi.stub",
                    "/usr/lib/systemd/boot/efi/linuxia32.efi.stub",
                    "/usr/lib/systemd/boot/efi/linuxaa64.efi.stub",
                ];

                for opt in &opts {
                    if path::Path::new(opt).is_file() {
                        println_info!("using {} as EFI stub", opt);
                        return opt.to_string();
                    }
                }

                println_error!("cannot find a suitable EFI stub, aborting...");
                println_note!("tried the following files: {:?}", &opts);
                proc::exit(1);
            }),
            cmdline_path: kernel_config_opt.cmdline_path.unwrap_or_else(|| {
                let opts = vec!["/etc/kernel/cmdline"];

                for opt in &opts {
                    if path::Path::new(opt).is_file() {
                        println_info!("using {} as cmdline file", opt);
                        return opt.to_string();
                    }
                }

                println_error!("cannot find a suitable kernel cmdline file, aborting...");
                println_note!("tried the following files: {:?}", &opts);
                proc::exit(1);
            }),
            initrd_paths: kernel_config_opt.initrd_paths.unwrap_or_else(|| {
                let kernel_initrd_name = "initramfs-".to_owned() + &kernel_name + ".img";

                let boot_dir_path = path::Path::new(&config.boot_dir);
                let kernel_initrd = boot_dir_path.join(kernel_initrd_name);

                if !kernel_initrd.is_file() {
                    println_error!("cannot find kernel initrd, aborting...");
                    println_note!("tried the following: {}", kernel_initrd.to_string_lossy());
                    proc::exit(1);
                }

                let kernel_initrd_str = kernel_initrd
                    .to_str()
                    .expect("name must be valid UTF-8")
                    .to_string();

                println_info!("using {} as kernel initrd", kernel_initrd_str);

                let mut initrds = vec![kernel_initrd_str];

                let opts = vec!["intel-ucode.img", "amd-ucode.img"];

                for opt in opts {
                    let path = boot_dir_path.join(opt);
                    if path.is_file() {
                        println_info!("also adding {}", path.to_string_lossy().to_string());
                        initrds.push(path.to_string_lossy().to_string());
                    }
                }

                initrds
            }),
            vmlinuz_path: kernel_config_opt.vmlinuz_path.unwrap_or({
                println_info!("using {} as kernel image", entry_path_str);
                entry_path_str.to_string()
            }),
        };

        let mut uki =
            match uki::UnifiedKernelImage::new(&kernel_config.stub_path, &kernel_config.output) {
                Ok(uki) => uki,
                Err(e) => {
                    log::println_warn!("cannot create uki instance: {}, skipping it...", e);
                    continue;
                }
            };

        // TODO: configurable osrel
        match uki.add_section_buf(".osrel", "/usr/lib/os-release") {
            Ok(()) => log::println_info!("added {} to .osrel", "/usr/lib/os-release"),
            Err(e) => log::println_warn!(
                "cannot add .osrel section to executable: {}, skipping it...",
                e
            ),
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
            Ok(()) => log::println_info!("wrote {}", kernel_config.output),
            Err(e) => log::println_error!(
                "cannot output efi executable ({}): {}, skipping it...",
                kernel_config.output,
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
