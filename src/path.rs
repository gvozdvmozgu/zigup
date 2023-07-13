use std::path::PathBuf;

pub(crate) fn zig() -> PathBuf {
    home::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".zig")
}

pub(crate) fn toolchains() -> PathBuf {
    zig().join("toolchains")
}
