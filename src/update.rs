use self_update::cargo_crate_version;

const REPO_OWNER: &str = "fufliik";
const REPO_NAME: &str = "TeraCraft";
const BIN_NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub  fn chack() -> Result<(), Box<dyn std::error::Error>>{
    let chack = self_update::backends::github::ReleaseList::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .build()?
        .fetch()?;

    if let Some(chack) = chack.first() {
        if VERSION != chack.version{
            println!("Найдено новое обновление: {} -> {}", VERSION, chack.version);
            run();
        }
    }

    Ok(())
}

pub  fn run() -> Result<(), Box<dyn std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner(REPO_OWNER)
        .repo_name(REPO_NAME)
        .bin_name(BIN_NAME)
        .show_download_progress(true)
        .show_output(false)
        .no_confirm(true) //-------------------------------------
        .target("x86_64-pc-windows-msvc")
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    Ok(())
}