use std::{
    env, fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
};

const FFMPEG_PATH_LINUX: &str = "/usr/bin/ffmpeg";

fn main() {
    // REGION: Getting directory

    print!("Type the folder path: ");
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading the input");

    let directory: String = {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            let exe_path: std::path::PathBuf =
                env::current_exe().expect("Failed to get current exe path");
            let exe_dir: &std::path::Path = exe_path.parent().expect("Failed to get exe directory");
            exe_dir.to_str().unwrap().to_string()
        } else {
            trimmed.to_string()
        }
    };
    // ENDREGION

    println!("📁 Path provided: \"{}\"", directory);

    // REGION: Getting files
    let mp3_files: Vec<PathBuf> = get_mp3_files(&directory);

    if mp3_files.is_empty() {
        println!("⚠️  No compatible files finded in directory.");
    } else {
        println!("🎶 Files finded to be equalized:");
        for file in &mp3_files {
            println!("- {}", file.display());
        }
    }
    // ENDREGION

    if !confirm("Confirm? [Y/n]: ", true) {
        return;
    };

    // REGION: Conversion
    let loudness_i: String = read_input_with_default("Integrated loudness target (I)", "-14");
    let tp: String = read_input_with_default("True peak limit (TP)", "-1.5");
    let lra: String = read_input_with_default("Loudness range (LRA)", "11");
    let quality: String = read_input_with_default("Audio Quality (q:a)", "0");

    let loudnorm_filter = format!("loudnorm=I={}:TP={}:LRA={}", loudness_i, tp, lra);

    let ffmpeg_path_windows: PathBuf;
    #[cfg(target_os = "windows")]
    {
        let exe_path: PathBuf = env::current_exe().expect("Failed to get current executable path");
        let exe_dir: &Path = exe_path
            .parent()
            .expect("Failed to get executable directory");
        ffmpeg_path_windows = exe_dir.join("library").join("ffmpeg.exe");
        if !ffmpeg_path_windows.exists() {
            eprintln!(
                "❌ ffmpeg.exe library not found in: {}",
                ffmpeg_path_windows.display()
            );
            return;
        }
    }

    for input_path in mp3_files {
        let file_name: String = input_path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let parent: &Path = input_path.parent().unwrap();
        let result_dir: PathBuf = parent.join("result");
        std::fs::create_dir_all(&result_dir).expect("Cannot create folder 'result'");
        let output_path: PathBuf = result_dir.join(format!("{}.mp3", file_name));

        println!("🔧 Processing: {}", input_path.display());

        #[cfg(target_os = "windows")]
        {
            println!(
                "{} -i {} -af {} -q:a {} -y {}",
                ffmpeg_path_windows.to_str().unwrap(),
                input_path.to_str().unwrap(),
                loudnorm_filter,
                quality,
                output_path.to_str().unwrap()
            );

            let status: std::process::ExitStatus = Command::new(&ffmpeg_path_windows)
                .args([
                    "-i",
                    input_path.to_str().unwrap(),
                    "-af",
                    &loudnorm_filter,
                    "-q:a",
                    &quality,
                    "-y",
                    output_path.to_str().unwrap(),
                ])
                .status()
                .expect("Failed to execute ffmpeg");

            if status.success() {
                println!("✅ Finish: {}", output_path.display());
            } else {
                println!("❌ Error while processing: {}", input_path.display());
            }
        }
        #[cfg(target_os = "linux")]
        {
            let status: std::process::ExitStatus = Command::new(FFMPEG_PATH_LINUX)
                .args([
                    "-i",
                    input_path.to_str().unwrap(),
                    "-af",
                    &loudnorm_filter,
                    "-q:a",
                    &quality,
                    "-y",
                    output_path.to_str().unwrap(),
                ])
                .status()
                .expect("Failed to execute ffmpeg");

            if status.success() {
                println!("✅ Finish: {}", output_path.display());
            } else {
                println!("❌ Error while processing: {}", input_path.display());
            }
        }
    }
    // ENDREGION
}

// UTILS
fn get_mp3_files(dir_path: &str) -> Vec<PathBuf> {
    let path: &Path = Path::new(dir_path);
    let mut mp3_files: Vec<PathBuf> = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path: PathBuf = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.eq_ignore_ascii_case("mp3") {
                        mp3_files.push(path);
                    }
                }
            }
        }
    } else {
        println!("❌ Error accessing directory: {}", dir_path);
    }

    mp3_files
}

fn confirm(prompt: &str, default_yes: bool) -> bool {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler entrada");
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "" => return default_yes,
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("Por favor, responda com 'y' ou 'n'.");
            }
        }
    }
}

fn read_input_with_default(prompt: &str, default: &str) -> String {
    print!("{} (default: {}): ", prompt, default);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let trimmed = input.trim();

    if trimmed.is_empty() {
        default.to_string()
    } else {
        trimmed.to_string()
    }
}
