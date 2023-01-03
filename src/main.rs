#[macro_use] extern crate rocket;

mod appconfig;
mod apiv1;

#[launch]
fn rocket() -> _ {
    appconfig::check_dbfile(appconfig::DATABASE_FILE);
    rocket::build().mount("/api/", routes![apiv1::sayhi, apiv1::query, apiv1::web_create, apiv1::query_all])
}