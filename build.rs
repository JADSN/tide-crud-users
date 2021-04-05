
fn main() {
    use std::path::Path;
    use std::fs;

    // TODO: Iterate over directories inside this folder
    let endpoints = Path::new("./src/api");

    let dest_path = Path::new("./src/banner_listen.rs");
    fs::write(
        &dest_path,
        format!("{:#?}",endpoints)
    ).unwrap();
    dbg!(dest_path);
}