use crate::context::Ctx;

pub fn get_file_contents_or_empty_string(ctx: &Ctx, path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            if ctx.debug {
                eprintln!("Failed to load {}: {}", path, err);
            }
            "".to_string()
        }
    }
}

pub fn get_dirs_in_path(ctx: &Ctx, path: &str) -> Vec<String> {
    let mut dirs = Vec::new();
    match std::fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Ok(file_type) = entry.file_type() {
                        if file_type.is_dir() {
                            if let Some(name) = entry.file_name().to_str() {
                                dirs.push(name.to_string());
                            }
                        }
                    }
                }
            }
        }
        Err(err) => {
            if ctx.debug {
                eprintln!("Failed to read directory {}: {}", path, err);
            }
        }
    }
    dirs
}
