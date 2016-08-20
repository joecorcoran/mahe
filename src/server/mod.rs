use hyper::server::{Server, Request, Response};
use reroute::{Captures, Router};
use store::Store;

pub struct ServerOptions<'a> {
    pub ip: &'a str,
    pub port: &'a str
}

pub fn start(db: String, options: ServerOptions) {
    let mut router = Router::new();

    let store = Store::shared(db);

    let get_store = store.clone();
    router.get(r"/keys/([^/]+)$", move |_, res, captures| {
        let s = get_store.lock().unwrap();
        println!("{:?}", captures.unwrap());
        res.send(b"hi");
    });

    let post_store = store.clone();
    router.post("/keys", move |_, res, _| {
        let s = post_store.lock().unwrap();    
        res.send(b"hi");
    });

    router.finalize().unwrap();

    let address = format!("{}:{}", options.ip, options.port);
    println!("Mahé running at {}...", address);
    Server::http(address.as_str()).unwrap().handle(router).unwrap();
}
