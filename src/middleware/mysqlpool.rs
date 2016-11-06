// Simple Iron middleware component to inject a MySQL Pool
// into the request lifecycle (otherwise lifetimes become a
// big issue).

use iron::{BeforeMiddleware, typemap};
use iron::prelude::{IronResult, Request};
use mysql::conn::pool::Pool;

/// Simple wrapper for a MySQL Pool object. Necessary so that
/// we can implement the `typemap::Key` trait for it which is
/// needed for Iron middleware'y stuff.
pub struct DBPool {
	pub pool: Pool,
}

/// Iron uses a typed map, here we're just specifying the value's type
impl typemap::Key for DBPool {
    type Value = Pool;
}

/// Before each request, inject the DBPool so we can query and stuff.
impl BeforeMiddleware for DBPool {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<DBPool>(self.pool.clone());
        Ok(())
    }
}
