use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;

use std::fs;

const MOV_DIR: &str = "./static/mov/";

#[derive(Debug, Clone, Serialize)]
struct MovEntry {
    value: String,
    text: String,
}

impl MovEntry {
    const fn new(value: String, text: String) -> Self {
        Self { value, text }
    }
}

#[get("/mov-files")]
fn get_mov_files() -> Json<Vec<MovEntry>> {
    let mut files = vec![];

    let paths = fs::read_dir(MOV_DIR).unwrap();
    for entry in paths {
        let path = entry.unwrap().path();
        let text: String = path.file_name().unwrap().to_str().unwrap().to_string();
        let value: String = path
            .to_str()
            .unwrap()
            .strip_prefix('.')
            .unwrap()
            .to_string();
        files.push(MovEntry::new(value, text));
    }

    Json(files)
}

pub fn start() {
    rocket::ignite()
        .mount("/", routes![get_mov_files])
        .mount("/static", StaticFiles::from("static"))
        .launch();
}
