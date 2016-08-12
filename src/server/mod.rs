use hyper::server::{Server, Request, Response};

fn hello(req: Request, res: Response) {
    // handle things here
}

pub fn start() {
    println!("Running Mahé server...");
    Server::http("0.0.0.0:8989").unwrap().handle(hello).unwrap();
}
