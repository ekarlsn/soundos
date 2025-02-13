use std::io::Write;

pub fn create_file(namespace: String, name: String, content: &[u8]) -> String {
    let namespace = filenamify::filenamify(namespace);
    let name = filenamify::filenamify(name);
    let full_path = format!("/tmp/soundos/{namespace}");
    std::fs::create_dir_all(full_path.clone()).unwrap();
    let mut file = std::fs::File::create(format!("/tmp/soundos/{namespace}/{name}")).unwrap();
    file.write_all(content).unwrap();
    full_path
}

pub fn read_file(namespace: String, name: String) -> Option<std::io::BufReader<std::fs::File>> {
    let namespace = filenamify::filenamify(namespace);
    let name = filenamify::filenamify(name);
    let path = format!("/tmp/soundos/{namespace}/{name}");
    match std::fs::File::open(path) {
        Ok(file) => Some(std::io::BufReader::new(file)),
        Err(_) => None,
    }
}
