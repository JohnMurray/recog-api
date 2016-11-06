extern crate iron;
extern crate env_logger;
extern crate log;
extern crate logger;
#[macro_use]
extern crate mysql;

mod middleware;
mod user;
mod auth;

use iron::prelude::*;
use iron::status;
use iron::{Handler};
use logger::Logger;
use middleware::mysqlpool::*;
use mysql as my;
use std::collections::HashMap;
use user::User;

struct Router {
    routes: HashMap<String, Box<Handler>>,
}

impl Router {
    fn new() -> Self {
        Router { routes: HashMap::new() }
    }

    fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }
}

impl Handler for Router {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        match self.routes.get(&req.url.path().join("/")) {
            Some(handler) => handler.handle(req),
            None          => Ok(Response::with(status::NotFound))
        }
    }
}

fn main() {
    // Initialize logging
    env_logger::init().unwrap();

    // Initialize router and routes
    let mut router = Router::new();

    router.add_route("auth".to_string(), auth::auth_handler);
    router.add_route("user".to_string(), auth::user_handler);


    // Setup Middleware
    let db_middleware = DBPool {
        pool: my::Pool::new("mysql://root:root@localhost:3306").unwrap(),
    };

    let mut chain = Chain::new(router);
    let (logger_before, logger_after) = Logger::new(None);

    chain.link_before(logger_before);
    chain.link_before(db_middleware);
    chain.link_after(logger_after);



    // Configure and launch server
    // TODO: setup with https
    Iron::new(chain).http("localhost:3000").unwrap();
}
