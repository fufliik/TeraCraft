mod minecraft;
mod update;
mod dir;
mod mods;
mod auth;

use std::io::Write;
use crate::auth::Data;

#[tokio::main]
async fn  main() {
    titel();
    match tokio::task::spawn_blocking(||{
        update::chack();
    }).await {
        Ok(info) => {println!("Обновление прошло успешно")}
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
            "username" | "u" | "г" | "гыуктфьу" => {
                data.username();
                match  data.save(){
                    Ok(_) => {println!("Сохранено")}
                    Err(e) => {println!("Ошибка: {e}")}
                }
            }
            "memory" | "m" | "ь" | "ьуьщкн" => {
                data.memory();
                match data.save() {
                    Ok(_) => {println!("Сохранено")}
                    Err(e) => {println!("{e}")}
                }
            }
            "clean" | "c" | "с" | "сдуфт" =>{
                clean();
                titel()
            }
            _ => {}
        }
    }
}


fn titel(){
    println!("GlavaLauncher");

}

pub fn clean() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}