use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use reroute::{Captures, Router};
use std::io::Read;
use store::Store;

pub struct ServerOptions<'a> {
    pub ip: &'a str,
    pub port: &'a str
}

pub fn start(db: String, options: ServerOptions) {
    let mut router = Router::new();
    let store = Store::shared(db);
    let keys_pattern = r"/keys/([^/]+)$";

    let get_store = store.clone();
    router.get(keys_pattern, move |_: Request, mut res: Response, cap: Captures| {
        let locked_store = get_store.lock().unwrap();
        if let Some(key) = cap.unwrap().pop() {
            match locked_store.read(key) {
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
    router.post(keys_pattern, move |mut req: Request, mut res: Response, cap: Captures| {
        let mut body = String::new();
        match req.read_to_string(&mut body) {
            Ok(length) => {
                if let Some(key) = cap.unwrap().pop() {
                    if length > 0 {
                        let mut locked_store = post_store.lock().unwrap();
                        locked_store.write(key, body);
                        *res.status_mut() = StatusCode::Accepted;
                    } else {
                        *res.status_mut() = StatusCode::BadRequest;
                    }
                }
            },
            _ => {
                *res.status_mut() = StatusCode::InternalServerError;
            }
        };
    });

    router.finalize().unwrap();

    let address = format!("{}:{}", options.ip, options.port);
    println!("Mahé running at {}...", address);
    Server::http(address.as_str()).unwrap().handle(router).unwrap();
}
