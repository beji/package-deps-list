use std::{
    env, fs,
    io::{BufWriter, Write},
};

use toml::Value;

fn main() {
    let mut args = env::args();
    // Move past the command
    args.next();

    let path = args
        .next()
        .expect("This expects exactly one parameter, which should be a path to a Cargo.lock file");

    let contents = fs::read_to_string(path).expect("Failed to read the file given");

    let parsed = contents
        .parse::<Value>()
        .expect("The given file doesn't seem to be in the TOML format");
    let table = parsed.as_table().expect(
        "The given file seems to be in the TOML format but doesn't seem to be a Cargo.lock",
    );

    let package_array = table["package"]
        .as_array()
        .expect("The given file doesn't seem to contain any packages");

    let packagelines = package_array.iter().map(|package| {
        let name = &package["name"];
        let version = &package["version"];
        name.as_str().unwrap().to_owned() + "-" + version.as_str().unwrap()
    });

    let stdout = std::io::stdout();
    let lock = stdout.lock();
    let mut buf = BufWriter::new(lock);

    for line in packagelines {
        writeln!(buf, "{}", line).unwrap();
    }
}
