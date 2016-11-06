use iron::prelude::*;
use iron::status;
use iron::{Handler};
use middleware::mysqlpool::*;
use super::user::User;

use mysql as my;

/// simple handle to test middleware
pub fn user_handler(request: &mut Request) -> IronResult<Response> {
    let pool = request.extensions.get::<DBPool>().unwrap();
    User::lookup_by_auth(&pool, "blah".to_string(), "blah".to_string());
    Ok(Response::with((status::Ok, "{\"users\":[]}")))
}

/// Authentication handler. This is the handler that takes care of
/// authenticating against the DB and creating an active session that
/// allows the user to access authenticated routes (almost all of them).
pub fn auth_handler(request: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Unauthorized, "{}")))
}
