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
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![index2])
        .register("/", catchers![not_found])
}