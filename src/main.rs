use clap::Parser;
use image::{Rgb, RgbImage};
use std::{io, path::PathBuf};
#[derive(Parser, Debug)]
#[command(name = "bytevisualizer")]
#[command(about = "Visualize byte data")]
struct Args {
    #[clap(short, long)]
    input: PathBuf,
    #[clap(short, long)]
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
    let width = 1024;

    let mut  d = load_file_to_u8_array(&args.input).unwrap();
    let paddinglen = width - d.len() % width;
    for _ in 0..paddinglen {
        d.push(0);
    }
    let height = d.len() / width;

    // 新しい画像を作成
    let mut img = RgbImage::new(width as u32, height as u32);

    for x in 0..width {
        for y in 0..height {
            // グラデーションを生成
            img.put_pixel(x as u32, y as u32, Rgb([d[y * width + x], 0, 0]));
        }
    }

    // ファイルに保存
    img.save("output.png").unwrap();
}
fn load_file_to_u8_array(file_path: &PathBuf) -> Result<Vec<u8>, io::Error> {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(file_path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}
