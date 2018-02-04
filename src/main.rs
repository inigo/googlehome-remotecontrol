extern crate iron;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use std::process::Command;

fn main() {
    println!("Starting Rust listener");
    Iron::new(send_response).http("0.0.0.0:9000").unwrap();
}

fn send_response(_request: &mut Request) -> IronResult<Response> {
    println!("sending response");

    Command::new("/Users/inigosurguy/Code/Inigo/rmmini/BlackBeanControl/BlackBeanControl.py")
        .args(&["-c", "TvPower"])
        .output()
        .expect("failed to execute process");

    println!("Done that now");

    let mut response = Response::new();
    response.set_mut(status::Ok);
    response.set_mut(mime!(Application/Json));
    response.set_mut(r#"
        { "speech" : "This is improved Rust", "displayText" : "This is improved Rust" }
    "#);
    Ok(response)
}
