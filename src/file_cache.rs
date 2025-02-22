use std::io::Write;

pub fn create_file(namespace: String, name: String, content: &[u8]) -> String {
    let (dir_path, full_path) = get_path(namespace, name);

    std::fs::create_dir_all(dir_path.clone()).unwrap();
    let mut file = std::fs::File::create(full_path.clone()).unwrap();
    file.write_all(content).unwrap();
    full_path
}

pub fn read_file(namespace: String, name: String) -> Option<std::io::BufReader<std::fs::File>> {
    let (_, full_path) = get_path(namespace, name);
    match std::fs::File::open(full_path) {
        Ok(file) => Some(std::io::BufReader::new(file)),
        Err(_) => None,
    }
}

pub fn file_exists(namespace: String, name: String) -> bool {
    let (_, full_path) = get_path(namespace, name);
    std::path::Path::new(&full_path).exists()
}

pub fn make_namespace(namespace: &str) -> std::path::PathBuf {
    let dir_path_str = get_namespace_str(namespace);
    let dir_path = std::path::Path::new(&dir_path_str);

    if !dir_path.exists() {
        std::fs::create_dir_all(dir_path).unwrap();
    }

    dir_path.to_path_buf()
}

fn get_path(namespace: String, name: String) -> (String, String) {
    let name = filenamify::filenamify(name);

    let dir_path = get_namespace_str(&namespace);

    let full_path = format!("{dir_path}/{name}");

    (dir_path, full_path)
}

pub fn get_namespace_str(namespace: &str) -> String {
    let namespace = filenamify::filenamify(namespace);

    #[cfg(target_os = "linux")]
    let dir_path = format!("/tmp/soundos/{namespace}");

    #[cfg(target_os = "android")]
    let dir_path = format!("/data/data/com.example.SoundOs/files/{namespace}");

    dir_path
}

pub fn get_filename_str(namespace: &str, name: &str) -> String {
    let (_, filename) = get_path(namespace.to_owned(), name.to_owned());

    filename
}
