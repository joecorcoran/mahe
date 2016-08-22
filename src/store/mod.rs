use std::collections::btree_map::BTreeMap;
use std::sync::{Arc, Mutex};

type Data = BTreeMap<String, String>;

pub struct Store {
    pub db: String,
    data: Data
}

impl Store {
    pub fn new<S: Into<String>>(db: S) -> Store {
        Store {
            db: db.into(),
            data: Data::new()
        }
    }

    pub fn shared<S: Into<String>>(db: S) -> Arc<Mutex<Store>> {
        Arc::new(Mutex::new(Store::new(db)))
    }

    pub fn read<S: Into<String>>(&self, key: S) -> Option<&String> {
        self.data.get(&key.into()) 
    }

    pub fn write<S: Into<String>>(&mut self, key: S, value: S) -> bool {
        self.data.insert(key.into(), value.into());
        true
    }

    pub fn delete<S: Into<String>>(&mut self, key: S) -> Option<String> {
        self.data.remove(&key.into())
    }
}

#[test]
fn test_write_read() {
    let mut store = Store::new("test");
    assert_eq!(store.write("one", "two"), true);
    assert_eq!(store.read("one"), Some(&String::from("two")));
}

#[test]
fn test_write_delete() {
    let mut store = Store::new("test");
    store.write("one", "two");
    assert_eq!(store.delete("one"), Some(String::from("two")));
    assert_eq!(store.read("one"), None);
}

