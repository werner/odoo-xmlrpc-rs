extern crate xmlrpc;

mod main_request;

use main_request::BasicRequest;

fn main() {
  let mut base_request = BasicRequest::new("http://localhost:8069");
  let auth = base_request.authenticate("odoo", "werner_a_e@yahoo.es", "12345678");
  match auth {
    Ok(value) => println!("{:?}", value),
    Err(error) => println!("{:?}", error)
  }
}
