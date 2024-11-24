use sled::Db;

pub struct Storage {
    db: Db,
}

impl Storage {
    pub fn init_with_path(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn init() -> Result<Self, Box<dyn std::error::Error>> {
        Self::init_with_path("functions_db")
    }

    pub fn save_function(&self, name: String, code: String) -> Result<(), sled::Error> {
        self.db.insert(name, code.as_bytes())?;
        Ok(())
    }

    pub fn load_function(&self, name: &str) -> Result<String, sled::Error> {
        if let Some(code) = self.db.get(name)? {
            Ok(String::from_utf8(code.to_vec()).unwrap())
        } else {
            Err(sled::Error::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Function not found",
            )))
        }
    }
}
