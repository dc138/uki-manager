use rand::distributions::{Alphanumeric, DistString};
use std::fs;
use std::io;
use std::io::Read;
use std::io::Write;
use std::os::fd::AsRawFd;
use std::os::unix::prelude::FileExt;

#[derive(Debug)]
struct UnifiedKernelImageSection {
    lenght: u64,
    name: String,
    file: fs::File,
}

impl UnifiedKernelImageSection {
    fn from_file(path: &str, name: &str) -> io::Result<Self> {
        let file = fs::File::open(path)?;
        let lenght = file.metadata()?.len();

        Ok(Self {
            lenght,
            name: name.to_owned(),
            file,
        })
    }

    fn from_files(paths: Vec<&str>, name: &str) -> io::Result<Self> {
        let mut files: Vec<fs::File> = paths
            .iter()
            .map(|path| Ok(fs::File::open(path)?))
            .collect::<io::Result<_>>()?;

        let opts = memfd::MemfdOptions::new();
        let mut file = opts
            .create(Alphanumeric.sample_string(&mut rand::thread_rng(), 8))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
            .into_file();

        let mut lenght: u64 = 0;

        for f in &mut files {
            let mut buf = Vec::new();
            f.read_to_end(&mut buf)?;

            let mut pos = 0;

            while pos < buf.len() {
                let written = file.write_at(&buf[pos..], lenght + pos as u64)?;
                pos += written;
            }

            lenght += pos as u64;
        }

        file.flush()?;
        Ok(Self {
            lenght,
            name: name.to_owned(),
            file,
        })
    }

    fn get_path(&self) -> String {
        format!("/proc/self/fd/{}", self.file.as_raw_fd())
    }
}

struct UnifiedKernelImage {
    osrel: fs::File,
    cmdline: fs::File,
    splash: fs::File,
    linux: fs::File,
    initrd: fs::File,
    output: fs::File,
}

impl UnifiedKernelImage {
    fn new(
        osrel: &str,
        cmdline: &str,
        splash: &str,
        linux: &str,
        initrds: Vec<&str>,
        output: &str,
    ) -> Result<Self, io::Error> {
        Err(io::Error::new(io::ErrorKind::Other, "todo"))
    }
}

fn main() -> io::Result<()> {
    let sec0 = UnifiedKernelImageSection::from_file("/etc/kernel/cmdline", "cmdline")?;

    let mut sec1 = UnifiedKernelImageSection::from_files(
        vec!["/boot/initramfs-linux-zen.img", "/boot/intel-ucode.img"],
        "initrd",
    )?;

    dbg!(&sec0);
    dbg!(&sec1);

    let mut output = fs::File::create("output")?;
    io::copy(&mut sec1.file, &mut output)?;

    Ok(())
}
