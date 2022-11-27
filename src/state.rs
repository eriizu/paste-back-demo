pub struct Config {
    pub uploads_dir: String,
}

impl Config {
    pub fn new(uploads_dir: String) -> Self {
        Self { uploads_dir }
    }
}
