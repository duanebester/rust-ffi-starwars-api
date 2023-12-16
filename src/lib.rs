uniffi::setup_scaffolding!("starwars");

use starwars_api::{apis::{default_api::get_character_by_id, configuration::Configuration}, models::Character};

#[derive(uniffi::Record)]
pub struct MyCharacter {
    pub name: String,
    pub height: String,
    pub mass: String,
}

#[tokio::main]
pub async fn internal_get_character(id: i64) -> Character {
    let config = Configuration::new();
    let character = get_character_by_id(&config, id).await.unwrap();
    character
}

#[uniffi::export]
pub async fn get_character(id: i64) -> MyCharacter {
    let character = internal_get_character(id);
    MyCharacter {
        name: character.name,
        height: character.height,
        mass: character.mass,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_character_works() {
        let character = internal_get_character(2);
        assert_eq!(character.name, "C-3PO");
    }
}
