use mysql as my;

pub struct User {
    id: u32,
    name: String,
    email: String,
}

impl User {
    /// Lookup the user information by the authentication information
    /// in the database.
    pub fn lookup_by_auth(pool: &my::Pool, email: String, password: String) -> Option<User> {

        let lookup_result = pool.prep_exec(
                "SELECT id, name, email
                 FROM `recog_api`.`user`
                 WHERE email = :email
                   AND password = :password",
                params!{
                    "email"    => email,
                    "password" => password,
                });
        // lookup result is a Result
        lookup_result.ok().and_then(|mut result| {
            // next returns Option<MyResult<Row>>
            result.next().and_then(|mut maybe_row| {
                // convert MyResult<Row> to Option<Row>
                maybe_row.ok().map(|mut row| {
                    User {
                        id: row.take("id").unwrap(),
                        name: row.take("name").unwrap(),
                        email: row.take("email").unwrap(),
                    }
                })
            })
        })
    }
}
