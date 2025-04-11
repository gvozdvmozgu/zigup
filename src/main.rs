#![deny(unreachable_pub, unused_qualifications, clippy::unused_trait_names)]

mod path;

use std::io::Seek as _;

use anyhow::Context as _;

const HOST_TRIPLE: &str = env!("HOST_TRIPLE");
const DOWNLOAD_INDEX: &str = "https://ziglang.org/download/index.json";

type DownloadIndex =
    indexmap::IndexMap<String, std::collections::HashMap<String, serde_json::Value>>;

fn main() -> anyhow::Result<()> {
    std::fs::create_dir_all(path::toolchains())
        .with_context(|| "failed to create toolchains directory")?;

    let download_index: DownloadIndex = ureq::get(DOWNLOAD_INDEX).call()?.body_mut().read_json()?;

    let (version, releases) = download_index.into_iter().nth(1).unwrap();
    let release =
        releases.get(HOST_TRIPLE).with_context(|| format!("unsupported {HOST_TRIPLE} platform"))?;

    println!(
        "Current installation options:
    default host triple: {HOST_TRIPLE}
    default toolchain: {version}"
    );

    let tarball = release["tarball"].as_str().unwrap();
    let mut response = ureq::get(tarball).call()?;

    if tarball.ends_with(".tar.xz") {
        use tar::Archive;
        use xz2::read::XzDecoder;

        let mut archive = Archive::new(XzDecoder::new(response.body_mut().as_reader()));
        archive.unpack(path::toolchains())?;
    } else if tarball.ends_with(".zip") {
        let mut archive = std::fs::File::options()
            .truncate(true)
            .write(true)
            .read(true)
            .open(path::toolchains().join("zig.zip"))?;

        download_file(response, &mut archive)?;
        archive.seek(std::io::SeekFrom::Start(0))?;

        let mut zip = zip::ZipArchive::new(archive)?;
        zip.extract(path::toolchains())?;
    } else {
        anyhow::bail!("unsupported archive format")
    };

    Ok(())
}

pub fn download_file(
    mut response: http::Response<ureq::Body>,
    writer: &mut impl std::io::Write,
) -> std::io::Result<()> {
    const DOWNLOAD_TEMPLATE: &str = "{msg} {spinner:.green} [{elapsed_precise}] \
                                     [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})";

    let length: u64 = response.headers()["Content-Length"].to_str().unwrap().parse().unwrap();

    let bar = indicatif::ProgressBar::new(!0);
    bar.set_message("Downloading");
    bar.set_style(
        indicatif::ProgressStyle::with_template(DOWNLOAD_TEMPLATE).unwrap().progress_chars("##-"),
    );
    bar.set_length(length);

    std::io::copy(&mut bar.wrap_read(response.body_mut().as_reader()), writer).unwrap();

    Ok(())
}
