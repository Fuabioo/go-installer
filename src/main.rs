use env_logger::Env;
use flate2::read::GzDecoder;
use log::{debug, info, warn, LevelFilter};
use std::fs;
use std::fs::File;
use structopt::StructOpt;
use tar::Archive;

const DEFAULT_INSTALL_PATH: &str = "/usr/local";
const DRY_RUN_INSTALL_PATH: &str = "/tmp/go-installer";
const LOG_LEVEL: &str = "LOG_LEVEL";
const PROFILE_PATH: &str = "/etc/profile";

#[derive(StructOpt)]
struct Opt {
    /// Dry Run
    #[structopt(short = "d", long = "dry-run")]
    dry_run: bool,

    /// Architecture (e.g. amd64)
    #[structopt(short = "a", long = "arch")]
    arch: Option<String>,

    /// OS (e.g. linux)
    #[structopt(short = "o", long = "os")]
    os: Option<String>,

    /// Version number (e.g. 1.24.0)
    #[structopt(short = "v", long = "version")]
    version: Option<String>,

    /// Go installation path
    #[structopt(short = "p", long = "install-path")]
    install_path: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger()?;

    let opt = Opt::from_args();

    let arch = opt.arch.unwrap_or_else(|| "amd64".to_string());
    let version = opt.version.unwrap_or_else(|| "1.24.2".to_string());
    let os = opt.os.unwrap_or_else(|| "linux".to_string());
    let install_path = opt
        .install_path
        .unwrap_or_else(|| DEFAULT_INSTALL_PATH.to_string());

    info!("Architecture: {}", arch);
    info!("Version: {}", version);
    info!("OS: {}", os);
    info!("Install path: {}", install_path);

    if opt.dry_run {
        warn!("DRY RUN ⚠️ Execution will not actually install go at the specified path");
    }

    let filename = format!("go{}.{}-{}.tar.gz", version, os, arch);

    download_go(filename.clone())?;

    remove_and_extract_go(install_path.clone(), filename.clone(), opt.dry_run)?;

    add_go_bin_to_path(install_path)?;

    defer::defer! {
        if let Err(e) = remove_file(filename.clone()) {
            warn!("Failed to remove file: {}", e);
        }
    }

    info!("\x1b[32mGo has been successfully installed!\x1b[0m");
    Ok(())
}

fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default().filter_or(LOG_LEVEL, LevelFilter::Info.to_string());

    env_logger::init_from_env(env);

    Ok(())
}

fn remove_file(filename: String) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Removing downloaded file {}...", filename);
    std::fs::remove_file(&filename)?;
    debug!("Removed file: {}", filename);
    Ok(())
}

fn download_go(filename: String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://go.dev/dl/{}", filename);

    debug!("Downloading {url:?}...");

    let client = reqwest::blocking::Client::new();
    let mut res = client.get(&url).send()?;

    let mut out = File::create(&filename)?;
    std::io::copy(&mut res, &mut out)?;

    debug!("Downloaded to {}", filename);

    Ok(())
}

fn remove_and_extract_go(
    mut install_path: String,
    filename: String,
    dry_run: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if dry_run {
        // change the install path to a temporary directory
        // so that we can test the dry run
        let temp_dir = std::env::temp_dir();
        let temp_install_path = temp_dir.join(format!("{}{}", DRY_RUN_INSTALL_PATH, install_path));
        warn!(
            "DRY RUN ⚠️ Using temporary install path: {}",
            temp_install_path.display()
        );
        std::fs::create_dir_all(&temp_install_path)?;
        install_path = temp_install_path.to_string_lossy().into_owned();
    }

    let installation_internal_path = format!("{}/go", install_path);
    if std::path::Path::new(&installation_internal_path).exists() {
        usr_install_location_safety_check(&installation_internal_path)?;

        debug!(
            "Removing old Go installation {}",
            installation_internal_path
        );
        std::fs::remove_dir_all(&installation_internal_path)?;
    }

    debug!("Extracting Go...");

    let tar_gz = File::open(filename)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(&install_path)?;

    debug!("Go extracted to {}", install_path);

    Ok(())
}

fn usr_install_location_safety_check(install_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    debug!("Checking if installation path is safe: {}", install_path);

    if install_path.starts_with("/usr") && install_path.ends_with("/local") {
        return Err(format!(
            "Preventing the destruction of the whole {} directory.",
            install_path
        )
        .into());
    }

    debug!("Installation path is safe: {}", install_path);
    Ok(())
}

fn add_go_bin_to_path(install_path: String) -> Result<(), Box<dyn std::error::Error>> {
    let go_bin_path = format!("{}/go/bin", install_path);

    debug!("Adding Go bin path {} to /etc/profile", go_bin_path);

    let profile_path = PROFILE_PATH;
    let mut profile_content = fs::read_to_string(profile_path)?;

    if !profile_content.contains(&go_bin_path) {
        profile_content.push_str(&format!("\nexport PATH=$PATH:{}\n", go_bin_path));
        fs::write(profile_path, profile_content)?;
        info!("Added Go bin path to /etc/profile");
    } else {
        info!("Go bin path already exists in /etc/profile");
    }

    Ok(())
}
