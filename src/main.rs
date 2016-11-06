extern crate env_logger;
extern crate iron;
extern crate log;
extern crate logger;
#[macro_use]
extern crate mysql;
extern crate router;
extern crate rustc_serialize;

mod middleware;
mod user;
mod auth;

use iron::prelude::*;
use logger::Logger;
use middleware::mysqlpool::*;
use mysql as my;
use router::Router;

fn main() {
    // Initialize logging
    env_logger::init().unwrap();

    // Initialize router and routes
    let mut router = Router::new();

    router.get("/auth", auth::auth_handler, "auth");
    router.get("/user", auth::user_handler, "user");


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
