#[macro_use]
extern crate rocket;

use rocket::http::ContentType;
use rocket::response::content::RawHtml;
use rust_embed::Embed;

use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Embed)]
#[folder = "examples/public/"]
struct Asset;

#[get("/")]
fn index() -> Option<RawHtml<Cow<'static, [u8]>>> {
  let asset = Asset::get("index.html")?;
  Some(RawHtml(asset.data))
}

#[get("/dist/<file..>")]
fn dist(file: PathBuf) -> Option<(ContentType, Cow<'static, [u8]>)> {
  let filename = file.display().to_string();
  let asset = Asset::get(&filename)?;
  let content_type = file
    .extension()
    .and_then(OsStr::to_str)
    .and_then(ContentType::from_extension)
    .unwrap_or(ContentType::Bytes);

  Some((content_type, asset.data))
}

#[rocket::launch]
fn rocket() -> _ {
  rocket::build().mount("/", routes![index, dist])
}
