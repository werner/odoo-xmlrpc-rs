use std::collections::BTreeMap;
use xmlrpc::{Request, Value, Error};

pub struct BasicRequest<'a> {
  base_url: &'a str
}

impl<'a> BasicRequest<'a> {

  pub fn new(url: &'a str) -> BasicRequest {
    BasicRequest {
      base_url: url
    }
  }

  pub fn authenticate(&mut self, db: &str, username: &str, password: &str) -> Result<Value, Error> {
    let user_agent_env: BTreeMap<String, Value> = BTreeMap::new();
    let request = Request::new("authenticate").arg(db).arg(username).arg(password).arg(Value::Struct(user_agent_env));
    self.execute_method(request) 
  }

  fn execute_method(&self, request: Request) -> Result<Value, Error> {
    let full_url = format!("{}/xmlrpc/2/common", &self.base_url);
    request.call_url(&full_url)
  }
}