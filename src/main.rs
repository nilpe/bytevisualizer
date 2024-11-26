use image::{RgbImage, Rgb};
use clap::Parser;
use std::path::PathBuf;
#[derive(Parser, Debug)]
#[command(name = "bytevisualizer")]
#[command(about = "Visualize byte data")]
struct Args {
    #[clap(short, long, parse(from_os_str))]
    input: PathBuf,
    #[clap(short='o', long="output", parse(from_os_str))]
    output: PathBuf,
}

fn main() {

    let args = Args::parse();

    println!("入力ファイル: {:?}", args.input);
    println!("出力ファイル: {:?}", args.output);

    // ファイル操作や処理の実装例
    if !args.input.exists() {
        eprintln!("入力ファイルが存在しません: {:?}", args.input);
        std::process::exit(1);
    }
    let d=    load_file_to_u8_array(&args.input).unwrap();
    let width = 1024;
    let height = 256;

    // 新しい画像を作成
    let mut img = RgbImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            // グラデーションを生成
            img.put_pixel(x, y, Rgb([x as u8, y as u8, 128]));
        }
    }

    // ファイルに保存
    img.save("output.png").unwrap();
}
fn load_file_to_u8_array(file_path: &PathBuf) -> Result<Vec<u8>> {
    use std::fs::File;
    use std::io::Read;

    let mut file = File::open(file_path).ok()?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).ok()?;
    Some(buffer)
}