use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/<path..>")]
fn all(path: PathBuf) -> Option<NamedFile> {
    let path = Path::new("static/").join(path);
    NamedFile::open(path).ok()
}

