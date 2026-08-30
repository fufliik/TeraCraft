use crate::auth::Data;
use std::io::Write;
use std::path::PathBuf;
use crate::dir;
use lyceris::minecraft::loader::neoforge::NeoForge;
use lyceris::minecraft::config::Memory;
use lyceris::minecraft::{
    config::ConfigBuilder,
    emitter::{Emitter, Event},
    install::install,
    launch::launch,
};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let emitter = Emitter::default();

    emitter
        .on(
            Event::MultipleDownloadProgress,
            |(path, current, total): (String, u64, u64)| {
                let percent = (current as f64 / total as f64) * 100.0;
                print!("\rЗагрузка: {:.1}%", percent);
                std::io::stdout().flush().unwrap();

            },
        )
        .await;

    emitter
        .on(Event::Console, |line: String| {
            println!("Line: {}", line);
        })
        .await;


    let dir = dir::launcher()?;
    let data = Data::load()?;
    let config = ConfigBuilder::new(
        &dir,
        "1.21.1".into(),
        lyceris::auth::AuthMethod::Offline {
            username: data.username,
            uuid: None,
        },
    )
        .memory(Memory::Gigabyte(data.memory))
        .loader(NeoForge("21.1.233".to_string()).into())
        .build();


    install(&config, Some(&emitter)).await?;

    let mut child = launch(&config, Some(&emitter)).await?;
    child.wait().await?;

    Ok(())
}