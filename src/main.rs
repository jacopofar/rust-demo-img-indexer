use hex;
use image::GenericImageView;
use rusqlite::{Connection, Result};
use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;

fn init_db() -> Result<Connection> {
    let conn = Connection::open("images.db")?;

    conn.execute(
        "create table if not exists images (
             path text primary key,
             height integer,
             width integer,
             color text,
             samples text
         )",
        [],
    )?;
    return Ok(conn);
}

fn process_img(path: &PathBuf, conn: &Connection) {
    println!("Got image at path {}", path.to_string_lossy());
    let img_res = image::open(path);
    if img_res.is_err(){
        println!("CANNOT OPEN IMAGE, SKIPPING...");
        return;
    }
    let img = img_res.unwrap();
    let (w, h) = img.dimensions();
    println!(
        "dimensions: {:?} color: {:?}",
        img.dimensions(),
        img.color()
    );

    let mut samples = Vec::new();
    for x in 1..10 {
        for y in 1..10 {
            let [r, g, b, _] = img.get_pixel(w * x / 10, h * y / 10).0;
            samples.push(r);
            samples.push(g);
            samples.push(b);
        }
    }
    let res = conn.execute(
        "
    insert into images (
        path,
        height,
        width,
        color,
        samples
    ) values (?1, ?2, ?3, ?4, ?5)
    ",
        (
            path.to_str(),
            h,
            w,
            format!("{:?}", img.color()),
            hex::encode(samples),
        ),
    );
    println!("{:?}", res);
}
fn main() {
    // fetch command line argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <folder>", args[0]);
        std::process::exit(1);
    }
    let conn = init_db().unwrap();
    // 2h 25m to process 29k files
    let folder_path = &args[1];
    let entries = fs::read_dir(folder_path).unwrap();
    // sort entries alphabetically
    let mut entries: Vec<_> = entries.map(|res| res.unwrap()).collect();
    entries.sort_by(|a, b| a.path().cmp(&b.path()));
    // iterate over them with an index to keep track of progress


    for (idx, entry) in entries.iter().enumerate() {
        let path = entry.path();
        println!("Processing file {} of {}", idx, entries.len());
        // println!("Found {}", path.display());
        match path.extension() {
            Some(ext) => {
                // println!("It had extension {}", ext.to_string_lossy());
                match ext.to_string_lossy() {
                    Cow::Borrowed("png") => process_img(&path, &conn),
                    Cow::Borrowed("jpg") => process_img(&path, &conn),
                    _ => (),
                }
            }
            None => println!("No filename extension found: {:?}", path),
        }
    }
}
