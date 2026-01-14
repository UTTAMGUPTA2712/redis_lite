use bytes::Bytes;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::time;
pub struct Db {
    entries: HashMap<String, Bytes>,
    persist: bool,
    expires: HashMap<String, Vec<String>>,
}

impl Db {
    pub fn new(persist: bool) -> Db {
        if persist {
            let mut db = Db {
                entries: HashMap::new(),
                persist,
                expires: HashMap::new(),
            };
            db.read_from_file();
            db.remove_expired_keys();
            return db;
        }

        Db {
            entries: HashMap::new(),
            persist,
            expires: HashMap::new(),
        }
    }

    fn save(&self) {
        if self.persist {
            self.write_to_file();
        }
    }

    fn remove_expired_keys(&mut self) {
        let cur_time = time::SystemTime::now()
            .duration_since(time::SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut keys_to_remove = Vec::new();
        for time in self.expires.keys() {
            if let Ok(t) = time.parse::<u64>() {
                if t < cur_time {
                    keys_to_remove.push(time.clone());
                }
            }
        }

        for time in keys_to_remove {
            self.remove_expired_key(time);
        }
        self.save();
    }

    pub fn remove_expired_key(&mut self, time: String) {
        let expired_keys = self.expires.get(&time);
        if let Some(keys) = expired_keys {
            for key in keys {
                self.entries.remove(key);
            }
        }
        self.expires.remove(&time);
        self.save();
    }

    fn add_expiration(&mut self, key: &String, time: &String) {
        // time should be after increasing that much in seconds in current time
        let time = time.parse::<u64>().unwrap()
            + time::SystemTime::now()
                .duration_since(time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        let time = time.to_string();

        self.expires
            .entry(time.clone())
            .or_insert_with(Vec::new)
            .push(key.clone());
        self.save();
    }

    fn write_to_file(&self) {
        let mut file = File::create("db.txt").unwrap();
        for (key, value) in &self.entries {
            file.write_all(format!("{}:{}\n", key, String::from_utf8_lossy(value)).as_bytes())
                .unwrap();
        }
        file.flush().unwrap();

        let mut file = File::create("expires.txt").unwrap();
        for (time, keys) in &self.expires {
            for key in keys {
                file.write_all(format!("{}:{}\n", time, key).as_bytes())
                    .unwrap();
            }
        }
        file.flush().unwrap();
    }

    fn read_from_file(&mut self) {
        let mut file = match File::open("db.txt") {
            Ok(f) => f,
            Err(_) => {
                File::create("db.txt").unwrap();
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let (key, value) = line.split_once(':').unwrap();
            self.entries
                .insert(String::from(key), Bytes::from(value.to_string()));
        }

        let mut file = match File::open("expires.txt") {
            Ok(f) => f,
            Err(_) => {
                File::create("expires.txt").unwrap();
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        for line in contents.lines() {
            let (time, key) = line.split_once(':').unwrap();
            self.expires
                .entry(String::from(time))
                .or_insert_with(Vec::new)
                .push(String::from(key));
        }
    }

    pub fn write(&mut self, arr: &[String]) -> Result<&str, &'static str> {
        if arr.len() < 4 {
            return Err("Insufficient arguments");
        }
        let key = &arr[1];
        let value = &arr[2];
        let time = &arr[3];

        let val = value.clone();
        let res: &Option<Bytes> = &self.entries.insert(String::from(key), Bytes::from(val));

        if time != "" {
            self.add_expiration(key, time);
        }
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
