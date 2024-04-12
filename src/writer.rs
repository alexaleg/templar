pub fn write_template(temp_lines: std::vec::Vec<String>, out_path: &std::path::PathBuf) {
    for line in temp_lines {
        let path_buf = out_path.join(&line);
        if line.ends_with("/") {
            println!("Creating path: {}", path_buf.display());
            let _ =std::fs::create_dir_all(path_buf);
            continue;
        } else if path_buf.file_name().is_some() {
            println!("Creating path: {}", path_buf.parent().unwrap().display());
            let _ =std::fs::create_dir_all(path_buf.parent().unwrap());
            println!("Creating file: {}", path_buf.display());
            let _ =std::fs::write(path_buf, "");
        }
    }
}
