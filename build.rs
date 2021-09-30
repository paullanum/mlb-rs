use http_req::request;
use std::env;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::path::Path;

const URL: &str = "https://statsapi.mlb.com/api/v1/teams";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("{}", out_dir);
    let dest_path = Path::new(&out_dir).join("teams.json");
    println!("{}", dest_path.to_str().unwrap());
    let file = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(dest_path)
        .unwrap();
    let mut f = BufWriter::new(file);
    request::get(URL, &mut f).unwrap();
}
