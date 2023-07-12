use std::collections::HashMap;
use std::fs::File;

use anyhow::Context as _;

const HOST_TRIPLE: &str = env!("HOST_TRIPLE");
const DOWNLOAD_INDEX_URL: &str = "https://ziglang.org/download/index.json";

type DownloadIndex = indexmap::IndexMap<String, HashMap<String, serde_json::Value>>;

fn main() -> anyhow::Result<()> {
    let download_index: DownloadIndex = ureq::get(DOWNLOAD_INDEX_URL).call()?.into_json()?;

    let (version, releases) = download_index.into_iter().nth(1).unwrap();
    let release =
        releases.get(HOST_TRIPLE).with_context(|| format!("unsupported {HOST_TRIPLE} platform"))?;

    println!(
        "Current installation options:
    default host triple: {HOST_TRIPLE}
    default toolchain: {version}"
    );

    let tarball = release["tarball"].as_str().unwrap();
    let filename = tarball.split('/').last().unwrap();

    let mut reader = ureq::get(tarball).call()?.into_reader();
    let mut writer = File::create(filename).unwrap();

    std::io::copy(&mut reader, &mut writer)?;

    Ok(())
}
