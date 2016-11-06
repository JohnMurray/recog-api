use iron::prelude::*;
use iron::status;
use middleware::mysqlpool::*;
use rustc_serialize::json;
use super::user::User;

/// simple handle to test middleware
pub fn user_handler(request: &mut Request) -> IronResult<Response> {
    let pool = request.extensions.get::<DBPool>().unwrap();
    let user: User = User::lookup_by_auth(&pool, "blah".to_string(), "blah".to_string()).unwrap();
    Ok(Response::with((status::Ok, json::encode(&user).unwrap())))
}

/// Authentication handler. This is the handler that takes care of
/// authenticating against the DB and creating an active session that
/// allows the user to access authenticated routes (almost all of them).
pub fn auth_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Unauthorized, "{}")))
}
