use clap::Parser;
use image::io::Reader as ImageReader;
use std::path::{Path, PathBuf};
use std::ffi::{OsStr};
use image::GenericImageView;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(help="path to image file")]
    path: String,
}

fn construct_target_path(path: &Path) -> PathBuf {
    let mut result = path.to_path_buf();
    let stem = path.file_stem().unwrap_or(OsStr::new(""));
    let mut filename = stem.to_os_string();
    filename.push("_cropped.");
    filename.push(path.extension().unwrap_or(OsStr::new("")));
    result.set_file_name(filename);

    result
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.path);
    let target_path = construct_target_path(path);
    println!("source: {}", path.to_string_lossy());
    println!("destination: {}", target_path.to_string_lossy());

    let img = ImageReader::open(path).unwrap().decode().unwrap();
    let (w, h) = img.dimensions();
    println!("[{}, {}] x [{}, {}]", 0, w, 0, h);
    let rgba = img.clone().into_rgba8();

    let mut lo_x = w;
    let mut hi_x = 0;
    let mut lo_y = h;
    let mut hi_y = 0;
    for (x, y, pix) in rgba.enumerate_pixels() {
        let [_r, _g, _b, a] = pix.0;
        let cutoff = 3;
        if a > cutoff {
            lo_x = lo_x.min(x);
            hi_x = hi_x.max(x + 1);
            lo_y = lo_y.min(y);
            hi_y = hi_y.max(y + 1);
        }
    }
    println!("[{}, {}] x [{}, {}]", lo_x, hi_x, lo_y, hi_y);
    let cropped = img.crop_imm(lo_x, lo_y, hi_x - lo_x, hi_y - lo_y);

    cropped.save(target_path).expect("Failed to save image");
}
