#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

#[get("/esio")]
fn index2() -> &'static str {
  "Hello, world!"
}

#[catch(404)]
fn not_found() -> &'static str {
  "czterysta i cztery"
}


#[launch]
fn rocket() -> _ {
  let figment = rocket::Config::figment()
    .merge(("address", "0.0.0.0"));

  rocket::custom(figment)
    .mount("/", routes![index])
    .mount("/", routes![index2])
    .register("/", catchers![not_found])
}

#[cfg(test)]
mod test {
  use super::rocket;
  use ::rocket::local::blocking::Client;
  use ::rocket::http::Status;
  use rocket::uri;

  #[test]
  fn hello_world() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let mut response = client.get(uri!(super::index)).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Hello, world!");
  }
}
