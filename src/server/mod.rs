use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
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
    router.get(r"/keys/([^/]+)$", move |_: Request, mut res: Response, cap: Captures| {
        let s = get_store.lock().unwrap();
        if let Some(key) = cap.unwrap().pop() {
            match s.read(key) {
                Some(value) => {
                    *res.status_mut() = StatusCode::Ok;
                    res.send(value.as_bytes()).unwrap();
                },
                None => {
                    *res.status_mut() = StatusCode::NotFound;
                    res.send(b"Not found").unwrap();
                }
            };
        }
    });

    let post_store = store.clone();
    router.post("/keys", move |_, res, _| {
        let s = post_store.lock().unwrap();    
        res.send(b"hi").unwrap();
    });

    router.finalize().unwrap();

    let address = format!("{}:{}", options.ip, options.port);
    println!("Mahé running at {}...", address);
    Server::http(address.as_str()).unwrap().handle(router).unwrap();
}
