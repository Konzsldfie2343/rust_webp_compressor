// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image::{ io::Reader as ImageReader};
use rayon::prelude::*;
use std::path::Path;
use std::{
    fs,
    path::{self, PathBuf},
};
use std::time::Instant;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![convert_to_webp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn convert_to_webp(
    path: String,
    ratio: u32,
    isReplace: bool,
    isRecursive: bool,
) -> Result<String, String> {
    let start_time = Instant::now();

    let file_paths: Vec<String> = get_paths(&path, &isRecursive)?;
    let file_len = file_paths.len();

    file_paths.par_iter().try_for_each(|file_path| {
        convert(file_path, &ratio, &isReplace)
    })?;

    let duration = start_time.elapsed();

    Ok(format!("{}個のファイルを{}秒で変換完了しました。", file_len, (duration.as_secs_f32() * 100.0).floor() / 100.0).to_string())
}

fn convert(filePath: &String, ratio: &u32, is_replace: &bool) -> Result<(), String> {
    // 画像を開く
    let img = ImageReader::open(filePath).map_err(|e| e.to_string())?;
    let img = img.decode().map_err(|e| e.to_string())?; // `DynamicImage` に変換

    // 出力パスを作成
    let mut output_path = PathBuf::from(filePath);
    output_path.set_extension("webp");

    // 画像を保存
    img.save_with_format(&output_path, image::ImageFormat::WebP)
        .map_err(|e| e.to_string())?;

    // 元のファイルサイズと変換後のファイルサイズを比較
    let original_size = fs::metadata(filePath).map_err(|e| e.to_string())?.len();
    let converted_size = fs::metadata(&output_path).map_err(|e| e.to_string())?.len();

    if converted_size >= original_size {
        // 変換後のファイルが元のファイルより大きい場合、変換を取り消す
        fs::remove_file(&output_path).map_err(|e| e.to_string())?;
        return Err("変換後のファイルサイズが元のファイルサイズより大きいため、変換を取り消しました。".to_string());
    }

    if *is_replace {
        fs::remove_file(filePath).map_err(|e| e.to_string())?;
    }

    Ok(()) // 正常終了を示す
}

fn get_paths(path: &String, is_recursive: &bool) -> Result<Vec<String>, String> {
    let new_path = Path::new(path);
    let mut file_paths: Vec<String> = Vec::new();

    if new_path.is_dir() {
        if *is_recursive {
            let mut folder_paths: Vec<String> = vec![path.clone()];

            while let Some(folder_path) = folder_paths.pop() {
                file_paths.extend(get_file_paths(&folder_path).map_err(|e| e.to_string())?);

                let new_folders = get_folder_paths(&folder_path).map_err(|e| e.to_string())?;
                folder_paths.extend(new_folders); // 後から追加する
            }

            Ok(file_paths)
        } else {
            file_paths = get_file_paths(&path)?;
            Ok(file_paths)
        }
    } else if new_path.is_file() {
        Ok(vec![path.clone()])
    } else {
        Err(format!("指定されたパス {} は存在しないか、通常のファイルまたはディレクトリのパスではありません。", path))
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
