#[derive(Debug)]
pub struct Config {
    db_url: String,
}

pub fn get_config() -> Config {
    Config {
        db_url: String::new(),
    }
}
