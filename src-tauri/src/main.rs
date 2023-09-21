// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

use image::ImageFormat;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::{PngEncoder, CompressionType, FilterType};
use tauri::Manager;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // 添加文件选择功能
            let main_window = app.get_window("main").unwrap();
			let main_window_clone = main_window.clone();

            main_window.listen("select_directory", move |_msg| {
				print!("11");
                // 处理选择目录的逻辑
                // let directory_path = msg.payload().unwrap();
                // let output_directory = format!("{}_compressed", &directory_path);
                let directory_path = "C:\\Users\\ihupoo\\Desktop\\node\\画面合成风格";
                let output_directory = "C:\\Users\\ihupoo\\Desktop\\node\\画面合成风格_compressed";

                if let Err(err) = compress_images(&directory_path, &output_directory, 100) {
                    main_window_clone.emit("directory_selected", Some(format!("Error: {}", err))).unwrap();
                } else {
                    main_window_clone.emit("directory_selected", Some("Success".to_string())).unwrap();
                }
            });

			Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn compress_image(input_file_path: &Path, output_file_path: &PathBuf, quality: u8) -> Result<(), Box<dyn std::error::Error>> {
	let img = image::open(input_file_path)?;

    // Determine the image format based on the file extension
    let format = match output_file_path.extension().and_then(|ext| ext.to_str()) {
        Some("jpg") | Some("jpeg") => ImageFormat::Jpeg,
        Some("png") => ImageFormat::Png,
        _ => return Err("Unsupported file format".into()),
    };

    // Compress and save the image with the specified quality and format
    match format {
        ImageFormat::Jpeg => {
            let mut output_file = File::create(output_file_path)?;
            let mut encoder = JpegEncoder::new_with_quality(&mut output_file, quality);
            encoder.encode_image(&img)?;
        }
        ImageFormat::Png => {
            let mut output_file = File::create(output_file_path)?;
			let encoder = PngEncoder::new_with_quality(&mut output_file, CompressionType::Best, FilterType::Adaptive);
			img.write_with_encoder(encoder)?;
        }
        _ => return Err("Unsupported file format".into()),
    }

    Ok(())
}

fn compress_images(input_directory: &str, output_directory: &str, _quality: u8) -> Result<(), Box<dyn std::error::Error>> {
    // 创建输出目录
    fs::create_dir_all(output_directory)?;

    // 递归遍历输入目录
    for entry in WalkDir::new(input_directory).follow_links(true) {
        let entry = entry?;

        if entry.file_type().is_file() {
            let file_path = entry.path();
            let relative_path = file_path.strip_prefix(input_directory)?;

            if let Some(extension) = file_path.extension() {
                if let Some(ext_str) = extension.to_str() {
                    if ext_str.eq_ignore_ascii_case("jpg") || ext_str.eq_ignore_ascii_case("jpeg") || ext_str.eq_ignore_ascii_case("png") {
						let mut output_file_path = PathBuf::from(output_directory);
                        output_file_path.push(relative_path);

                        if let Some(parent) = output_file_path.parent() {
                            fs::create_dir_all(parent)?;
                        }

                        // 压缩图片
                        compress_image(file_path, &output_file_path, _quality)?;
                    }
                }
            }
        }
    }

    Ok(())
}
