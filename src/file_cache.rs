use std::io::Write;

pub fn create_file(namespace: String, name: String, content: &[u8]) {
    let namespace = filenamify::filenamify(namespace);
    let name = filenamify::filenamify(name);
    std::fs::create_dir_all(format!("/tmp/soundos/{namespace}")).unwrap();
    let mut file = std::fs::File::create(format!("/tmp/soundos/{namespace}/{name}")).unwrap();
    file.write_all(content).unwrap();
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
