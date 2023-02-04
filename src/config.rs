pub struct Config {
    pub root: String
}

impl Config {
    pub fn new(config_path: String) -> Config{
        Config {
            root: String::from("./data")
        }
    }
}