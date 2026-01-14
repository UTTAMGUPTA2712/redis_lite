use bytes::Bytes;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;

pub struct Db {
    entries: HashMap<String, Bytes>,
    persist: bool,
}

impl Db {
    pub fn new(persist: bool) -> Db {
        if persist {
            let mut db = Db {
                entries: HashMap::new(),
                persist,
            };
            db.read_from_file();
            return db;
        }
        Db {
            entries: HashMap::new(),
            persist,
        }
    }

    fn save(&self) {
        if self.persist {
            self.write_to_file();
        }
    }

    fn make_file(&self) -> File {
        File::create("db.txt").unwrap()
    }

    fn write_to_file(&self) {
        // save the entries in a .txt file
        let mut file = File::create("db.txt").unwrap();
        for (key, value) in &self.entries {
            file.write_all(format!("{}:{}\n", key, String::from_utf8_lossy(value)).as_bytes()).unwrap();
        }
        file.flush().unwrap();
    }

    fn read_from_file(&mut self) {
        let mut file = match File::open("db.txt") {
            Ok(f) => f,
            Err(_) => {
                self.make_file();
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let (key, value) = line.split_once(':').unwrap();
            self.entries.insert(String::from(key), Bytes::from(value.to_string()));
        }
    }

    pub fn write(&mut self, arr: &[String]) -> Result<&str, &'static str> {
        if arr.len() < 4 {
            return Err("Insufficient arguments");
        }
        let key = &arr[1];
        let value = &arr[2];

        let val = value.clone();
        let res: &Option<Bytes> = &self.entries.insert(String::from(key), Bytes::from(val));

        self.save();

        match res {
            Some(_res) => Ok("r OK"),
            None => Ok("OK"),
        }
    }

    pub fn read(&mut self, arr: &[String]) -> Result<&str, &'static str> {
        if arr.len() < 3 {
            return Err("Insufficient arguments");
        }
        let key = &arr[1];
        let query_result = self.entries.get(key);

        if let Some(value) = query_result {
            match str::from_utf8(value) {
                Ok(v) => Ok(v),
                Err(_) => Err("No such key found"),
            }
        } else {
            return Err("No such key found");
        }
    }

    pub fn delete(&mut self, arr: &[String]) -> Result<&str, &'static str> {
        if arr.len() < 3 {
            return Err("Insufficient arguments");
        }
        let key = &arr[1];
        let res = self.entries.remove(key);

        self.save();

        match res {
            Some(_res) => Ok("OK"),
            None => Ok("OK"),
        }
    }
}
