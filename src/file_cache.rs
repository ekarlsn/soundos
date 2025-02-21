use std::io::Write;

pub fn create_file(namespace: String, name: String, content: &[u8]) -> String {
    let (dir_path, full_path) = get_path(namespace, name);

    std::fs::create_dir_all(dir_path.clone()).unwrap();
    let mut file = std::fs::File::create(full_path.clone()).unwrap();
    file.write_all(content).unwrap();
    full_path
}

pub fn read_file(namespace: String, name: String) -> Option<std::io::BufReader<std::fs::File>> {
    let (dir_path, full_path) = get_path(namespace, name);
    match std::fs::File::open(full_path) {
        Ok(file) => Some(std::io::BufReader::new(file)),
        Err(_) => None,
    }
}

fn get_path(namespace: String, name: String) -> (String, String) {
    let namespace = filenamify::filenamify(namespace);
    let name = filenamify::filenamify(name);

    #[cfg(target_os = "linux")]
    let dir_path = format!("/tmp/soundos/{namespace}");

    #[cfg(target_os = "android")]
    let dir_path = format!("/data/data/com.example.SoundOs/files/{namespace}");

    let full_path = format!("{dir_path}/{name}");

    (dir_path, full_path)
}
