use hyper::server::{Server, Request, Response};
use reroute::{Captures, Router};
use store::Store;

pub struct ServerOptions<'a> {
    pub ip: &'a str,
    pub port: &'a str
}

fn read_key(_: Request, _: Response, captures: Captures) {
    println!("Reading {:?}", captures);
}

fn save_key(_: Request, _: Response, captures: Captures) {
    println!("Saving {:?}", captures);
}

pub fn start(store: Store, options: ServerOptions) {
    let address = format!("{}:{}", options.ip, options.port);
    
    let mut router = Router::new();
    router.get(r"/keys/[^/]+$", read_key);
    router.post("/keys", save_key);
    router.finalize().unwrap();

    println!("Mahé database {} running at {}...", store.db, address);
    Server::http(address.as_str()).unwrap().handle(router).unwrap();
}
