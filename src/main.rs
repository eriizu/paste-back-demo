#[macro_use]
extern crate rocket;
mod cors;
mod paste_id;
mod state;

use cors::CorsFairing;
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

use rocket::{serde::json::Json, tokio::fs};
use std::path::Path;

#[get("/pastes/<id>")]
async fn retrieve(id: PasteId<'_>, config: &rocket::State<Config>) -> Option<fs::File> {
    let filepath = Path::new(&config.uploads_dir).join(id.to_str());
    fs::File::open(filepath).await.ok()
}

#[post("/pastes", data = "<paste>")]
async fn upload(
    paste: rocket::Data<'_>,
    config: &rocket::State<Config>,
) -> std::io::Result<String> {
    let id = PasteId::new(5);
    let filepath = Path::new(&config.uploads_dir).join(id.to_str());
    let max_size = rocket::data::ToByteUnit::kibibytes(128);
    paste.open(max_size).into_file(filepath).await?;
    Ok(id.to_str().to_owned())
}

#[get("/pastes")]
async fn list_pastes(confif: &rocket::State<Config>) -> Json<Vec<String>> {
    let mut out = Vec::new();

    match fs::read_dir(&confif.uploads_dir).await {
        Ok(mut dir) => {
            while let Ok(Some(entry)) = dir.next_entry().await {
                if let Ok(path) = entry.file_name().into_string() {
                    out.push(path);
                }
            }
        }
        Err(err) => {
            eprintln!(
                "Failed opening upload directory, this should not be possible. {:?}",
                err
            );
        }
    }
    Json(out)
}

#[launch]
fn rocket() -> _ {
    let upload_dir = std::env::var("UPLOAD_DIR").expect("UPLOAD_DIR env variable needs to be set");

    // let aa = rocket_

    rocket::build()
        .manage(Config::new(upload_dir))
        .mount("/", routes![index, retrieve, upload, list_pastes])
        .attach(CorsFairing)
}
