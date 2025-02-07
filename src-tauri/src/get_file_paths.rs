use std::path::Path;

fn get_paths(path: &str, is_recursive: bool) -> Result<Vec<String>, String> {
    let new_path = Path::new(path);

    if new_path.is_dir() {
        if is_recursive {
            let mut file_paths: Vec<String> = Vec::new();
            let mut folder_paths = vec![path.to_string()];

            while let Some(folder_path) = folder_paths.pop() {
                file_paths.extend(get_file_paths(&folder_path).map_err(|e| e.to_string())?);
                folder_paths.extend(get_folder_paths(&folder_path).map_err(|e| e.to_string())?);
            }

            Ok(file_paths)
        } else {
            get_file_paths(path).map_err(|e| e.to_string())
        }
    } else if new_path.is_file() {
        Ok(vec![path.to_string()])
    } else {
        Err(format!(
            "指定されたパス {} は存在しないか、通常のファイルまたはディレクトリのパスではありません。",
            path
        ))
    }
}

fn get_folder_paths(folder_path: &String) -> Result<Vec<String>, String> {
    let mut folder_paths: Vec<String> = Vec::new();
    let entries = fs::read_dir(&folder_path).map_err(|e| e.to_string())?;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_dir() {
            folder_paths.push(path.to_string_lossy().to_string());
        }
    }
    Ok(folder_paths)
}

fn get_file_paths(folder_path: &String) -> Result<Vec<String>, String> {
    let mut file_paths: Vec<String> = Vec::new();
    let entries = fs::read_dir(&folder_path).map_err(|e| e.to_string())?; // ここでエラー処理

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let ext_lower = ext.to_lowercase();
                if ext_lower == "png" || ext_lower == "jpg" || ext_lower == "jpeg" {
                    file_paths.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(file_paths) // Vec<String> を Result で返す
}