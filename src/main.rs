#![deny(unreachable_pub, unused_qualifications)]

mod path;

use anyhow::Context as _;
use tar::Archive;
use xz2::read::XzDecoder;

const HOST_TRIPLE: &str = env!("HOST_TRIPLE");
const DOWNLOAD_INDEX: &str = "https://ziglang.org/download/index.json";

type DownloadIndex =
    indexmap::IndexMap<String, std::collections::HashMap<String, serde_json::Value>>;

fn main() -> anyhow::Result<()> {
    let download_index: DownloadIndex = ureq::get(DOWNLOAD_INDEX).call()?.into_json()?;

    let (version, releases) = download_index.into_iter().nth(1).unwrap();
    let release =
        releases.get(HOST_TRIPLE).with_context(|| format!("unsupported {HOST_TRIPLE} platform"))?;

    println!(
        "Current installation options:
    default host triple: {HOST_TRIPLE}
    default toolchain: {version}"
    );

    let reader = {
        let tarball = release["tarball"].as_str().unwrap();
        ureq::get(tarball).call()?.into_reader()
    };

    let mut archive = Archive::new(XzDecoder::new(reader));
    archive.unpack(path::toolchains())?;

    Ok(())
}
