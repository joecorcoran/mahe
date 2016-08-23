use hyper::server::{Server, Request, Response};
use hyper::status::StatusCode;
use reroute::{Captures, Router};
use std::io::Read;
use store::Store;

pub struct ServerOptions<'a> {
    pub ip: &'a str,
    pub port: &'a str
}

pub fn start(db: &str, options: ServerOptions) {
    let store = Store::shared(db);
    let keys_pattern = r"/keys/([^/]+)$";

    let read_store = store.clone();
    let read_handler = move |_: Request, mut res: Response, cap: Captures| {
        let locked_store = read_store.lock().unwrap();
        if let Some(key) = cap.unwrap().pop() {
            match locked_store.read(key) {
                Some(value) => {
                    *res.status_mut() = StatusCode::Ok;
                    res.send(value.as_bytes()).unwrap();
                },
                None => {
                    *res.status_mut() = StatusCode::NotFound;
                }
            };
        }
    };

    let write_store = store.clone();
    let write_handler = move |mut req: Request, mut res: Response, cap: Captures| {
        let mut body = String::new();
        match req.read_to_string(&mut body) {
            Ok(length) => {
                if let Some(key) = cap.unwrap().pop() {
                    if length > 0 {
                        let mut locked_store = write_store.lock().unwrap();
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
    };

    let delete_store = store.clone();
    let delete_handler = move |_: Request, mut res: Response, cap: Captures| {
        let mut locked_store = delete_store.lock().unwrap();
        if let Some(key) = cap.unwrap().pop() {
            match locked_store.delete(key) {
                Some(_) => {
                    *res.status_mut() = StatusCode::Ok;
                },
                None => {
                    *res.status_mut() = StatusCode::NotFound;
                }
            };
        }
    };

    let mut router = Router::new();
    router.get(keys_pattern, read_handler);
    router.post(keys_pattern, write_handler);
    router.delete(keys_pattern, delete_handler);
    router.finalize().unwrap();

    let address = format!("{}:{}", options.ip, options.port);
    println!("Mahé ({}) running at {}...", db, address);
    Server::http(address.as_str()).unwrap().handle(router).unwrap();
}
