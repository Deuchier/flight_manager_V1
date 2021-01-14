

use std::io;
use std::fs::File;
use crate::domain::UserId;


#[get("/hello")]
fn hello() -> io::Result<File> {
    File::open("assets/hello.html")
}

#[get("/login<user_id>")]
fn login(user_id: UserId) -> io::Result<File> {
    File::open("assets/login.html")
}


/// Ignites the server for
pub fn ignite() {
    rocket::ignite().mount("/", routes![hello, login]).launch();
}