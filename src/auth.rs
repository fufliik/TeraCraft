use serde::{Serialize, Deserialize};
use crate::dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct Data{
    pub username : String,
    pub memory: u16,
}

impl Data{
    pub fn username(&mut self)  {
        loop{
            println!("Введите никнейм: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let len = input.trim().len();
            match len {
                3..=16 => {
                    self.username = input.trim().to_string();
                    return;
                }
                _ => {println!("Никнейм должен содержать от 3 до 16 символов ")}
            }
        }
    }

    pub fn memory (&mut self) {
        loop{
            println!("Введите объем ОЗУ (ГБ): ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            match input.trim().parse::<u16>() {
                Ok(n) if (1..16).contains(&n) => {
                    self.memory = n;
                    return;
                }
                Ok(_) => {}
                Err(_) => {println!("Error memory");} //норм текст здеалть
            }
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(dir::config()?, json)?;
        Ok(())
    }

    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config = dir::config()?;
        if dir::config()?.exists() {
            let json = std::fs::read_to_string(&config)?;

            if !json.trim().is_empty() {
                let data: Self = serde_json::from_str(&json)?;
                return Ok(data);
            }
        }

        let mut data = Self{
            username: String::new(),
            memory: 0,
        };

        data.username();
        data.memory();
        data.save()?;
        Ok(data)
    }
}



