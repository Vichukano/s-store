use std::{
    collections::hash_map::DefaultHasher,
    fs::File,
    hash::{Hash, Hasher},
    io::Write,
    io::{BufRead, BufReader, Error},
};

#[derive(Debug)]
pub struct Entity {
    uid: String,
    hash_code: u64,
    payload: String,
}

impl Entity {
    pub fn new(uid: String, payload: String) -> Self {
        let hash_code = Entity::calculate_hash(&uid);
        Entity {
            uid,
            hash_code,
            payload,
        }
    }

    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    fn get_uid(&self) -> String {
        self.uid.clone()
    }

    fn get_hash_code(&self) -> u64 {
        self.hash_code
    }

    fn get_payload(&self) -> String {
        self.payload.clone()
    }
}

pub struct EntityDao {
    root_path: String,
}

impl EntityDao {
    pub fn new(root_path: String) -> Self {
        EntityDao { root_path }
    }

    fn save(&self, t: Entity) -> Result<(), Error> {
        log::debug!("Start to save entity: {:?} to path: {}", t, self.root_path);
        let mut root_path = self.root_path.clone();
        root_path.push_str(t.uid.as_str());
        let mut to_store = File::create(root_path)?;
        write!(to_store, "{}", t.get_payload())?;
        Ok(())
    }

    fn get(&self, uid: &str) -> Option<Entity> {
        log::debug!("Start to find entity by uid: {}", uid);
        let mut path = self.root_path.clone();
        path.push_str(uid);
        let result = match File::open(path) {
            Ok(file) => {
                let buff = BufReader::new(file);
                let mut text = String::new();
                for line in buff.lines() {
                    text.push_str(line.unwrap().as_str());
                }
                Some(Entity::new(uid.to_string(), text))
            }
            Err(_) => None,
        };
        result
    }
}
