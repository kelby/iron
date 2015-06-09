extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    // If a static item is declared with the mut keyword, then it is allowed to be modified by the program.
    // mut 表示此变量可变。
    Iron::new(|_: &mut Request| {
        Ok(Response::with(status::NotFound))
    }).http("localhost:3000").unwrap();
}

