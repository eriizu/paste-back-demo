#[macro_use]
extern crate rocket;
mod paste_id;
mod state;

use paste_id::PasteId;
use state::Config;

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

use rocket::tokio::fs;
use std::path::Path;

#[get("/paste/<id>")]
async fn retrieve(id: PasteId<'_>, config: &rocket::State<Config>) -> Option<fs::File> {
    let filepath = Path::new(&config.uploads_dir).join(id.to_str());
    fs::File::open(filepath).await.ok()
}

use rocket::data::ToByteUnit;

#[post("/paste", data = "<paste>")]
async fn upload(
    paste: rocket::Data<'_>,
    config: &rocket::State<Config>,
) -> std::io::Result<String> {
    let id = PasteId::new(5);
    let filepath = Path::new(&config.uploads_dir).join(id.to_str());
    paste.open(128.kibibytes()).into_file(filepath).await?;
    Ok(id.to_str().to_owned())
}

#[launch]
fn rocket() -> _ {
    let upload_dir = std::env::var("UPLOAD_DIR").expect("UPLOAD_DIR env variable needs to be set");

    rocket::build()
        .manage(Config::new(upload_dir))
        .mount("/", routes![index, retrieve, upload])
}
