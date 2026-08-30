mod minecraft;
mod update;
mod dir;
mod mods;
mod auth;
use crate::auth::Data;

#[tokio::main]
async fn  main() {
    titel();
    match tokio::task::spawn_blocking(||{
        update::chack();
    }).await {
        Ok(info) => {}
        Err(e) => {println!("Ошибка обновление: {}", e)}
    }

    let mut data = match Data::load() {
        Ok(data) => data,
        Err(e) => {println!("Ошибка: {e}"); return }
    };

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let temp = input.trim().to_lowercase();
        match temp.as_str() {
            "start" | "s" | "ыефке" | "ы" => {
                match minecraft::run().await {
                    Ok(n) => {println!("")}
                    Err(e) => {println!("Ошибка Minecraft: {}", e)}
                }
            }
            "info" | "i" | "ш" | "штащ" => {
                println!("ИНФОРМАЦИЯ");
                println!("Ник: {}", data.username);
                println!("ОЗУ: {}", data.memory);
                println!("Билд: v{}",update::VERSION)
            }
            "rename" | "r" | "к" | "кутфьу" => {
                data.username();
                match  data.save(){
                    Ok(_) => {println!("Сохранено: {}",data.username)}
                    Err(e) => {println!("Ошибка: {e}")}
                }
            }
            "memory" | "m" | "ь" | "ьуьщкн" => {
                data.memory();
                match data.save() {
                    Ok(_) => {println!("Сохранено: {}", data.memory)}
                    Err(e) => {println!("{e}")}
                }
            }
            "clean" | "c" | "с" | "сдуфт" =>{
                clear();
                titel()
            }
            _ => {}
        }
    }
}


fn titel(){
    println!("
▀▀█▀▀ ░█▀▀▀ ░█▀▀█ ─█▀▀█ ░█▀▀█ ░█▀▀█ ─█▀▀█ ░█▀▀▀ ▀▀█▀▀
─░█── ░█▀▀▀ ░█▄▄▀ ░█▄▄█ ░█─── ░█▄▄▀ ░█▄▄█ ░█▀▀▀ ─░█──
─░█── ░█▄▄▄ ░█─░█ ░█─░█ ░█▄▄█ ░█─░█ ░█─░█ ░█─── ─░█──");
    println!("\n[==================КОМАНДЫ======================]");
    println!("start  | s => Запустить игру. ");
    println!("rename | r => Изменить никнейм.");
    println!("memory | m => Изменить объем оперативной памяти.");
    println!("clean  | c => Очистить терминал.");
    println!("info   | i => Информация. ");
    println!("[===============================================]");

}

pub fn clear() {
    std::process::Command::new("cmd").args(["/c","cls"]).status().unwrap();

}