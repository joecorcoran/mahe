pub struct Store<'a> {
    pub db: &'a str
}

impl<'a> Store<'a> {
    pub fn new(db: &'a str) -> Store {
        Store { db: db }
    }
}
