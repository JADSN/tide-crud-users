use std::io;
use std::path::PathBuf;
fn main() -> io::Result<()> {
    let endpoints = get_endpoint_modules()?;
    print_endpoints(endpoints)?;
    Ok(())
}

fn print_endpoints(endpoints: Vec<PathBuf>) -> io::Result<()> {
    use regex::Regex;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::Write;
    use std::path::Path;
    let base_path = env!("CARGO_MANIFEST_DIR");
    let output_file_path = Path::new(base_path).join("endpoints_found.txt");
    dbg!(&output_file_path);
    let mut output_file_fd = File::create(&output_file_path)?;
    let re = Regex::new(r"pub struct (\w+);").unwrap();
    for endpoint in endpoints {
        let filename = endpoint.join("mod.rs");

        let file = File::open(filename)?;
        let lines = io::BufReader::new(file).lines();
        for line in lines {
            if let Ok(value) = line {
                let line_str = value.as_str();

                for cap in re.captures_iter(line_str) {
                    let endpoint_str = endpoint.as_os_str().to_str().unwrap_or("");
                    let endpoint_vec = endpoint_str.split("/").collect::<Vec<&str>>();

                    let endpoint_str_line =
                        format!("    /api/{} - {}\n", endpoint_vec[5], &cap[1]);
                    output_file_fd.write_all(endpoint_str_line.as_bytes())?;
                }
            }
        }
    }
    Ok(())
}

fn get_endpoint_modules() -> io::Result<Vec<PathBuf>> {
    use std::fs::read_dir;
    use std::path::Path;
    let base_path = env!("CARGO_MANIFEST_DIR");
    let endpoints_dir = Path::new(base_path).join("src/api");
    dbg!(&endpoints_dir);
    let mut endpoints: Vec<PathBuf> = Vec::new();
    read_dir(endpoints_dir)?
        .map(|res| {
            res.map(|e| {
                if e.path().is_dir() {
                    endpoints.push(e.path())
                }
            })
        })
        .collect::<Result<Vec<_>, io::Error>>()?;
    endpoints.sort();
    Ok(endpoints)
}
