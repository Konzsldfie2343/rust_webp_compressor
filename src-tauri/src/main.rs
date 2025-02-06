// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image::io::Reader as ImageReader;
use std::{fs, path};
use tauri::api::file;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![convert_folder_to_webp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn convert_folder_to_webp(folder_path: String) -> Result<String, String> {
    // フォルダ内のエントリを走査
    let entries = fs::read_dir(&folder_path).map_err(|e| e.to_string())?;
    let mut converted_count = 0;

    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        // ファイルのみ対象
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let ext_lower = ext.to_lowercase();
                if ext_lower == "png"
                    || ext_lower == "jpg"
                    || ext_lower == "jpeg"
                    || ext_lower == "bmp"
                {
                    // 画像ファイルを読み込み
                    let img = ImageReader::open(&path)
                        .map_err(|e| e.to_string())?
                        .decode()
                        .map_err(|e| e.to_string())?;
                    // 出力パスを同じファイル名で拡張子を .webp に変更
                    let mut output_path = path.clone();
                    output_path.set_extension("webp");
                    // 変換して保存 (品質はデフォルト設定)
                    img.save_with_format(&output_path, image::ImageFormat::WebP)
                        .map_err(|e| e.to_string())?;
                    converted_count += 1;
                }
            }
        }
    }
    Ok(format!(
        "{} 個の画像をWebP形式に変換しました。",
        converted_count
    ))
}

fn convert (filePath:String, ratio: u32, is_replace: bool) -> Result<String, String> {
    return Ok("".to_string());
}


fn convert_to_webp(path: String, ratio: u32, isReplace: bool, isRecursive: bool) -> Result<String, String> {
    return Ok("".to_string());
}
