use std::{fs::{self, File}, io::Write, path::{Path, PathBuf}};
mod specfic_errors;
use image::{self, DynamicImage, ImageFormat, EncodableLayout};
use specfic_errors::SpecificErrors;
use webp::{self, Encoder, WebPMemory};

use clap::Parser;
#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
pub struct MyArgs {
    /// Extension of the files that should be transformed
    #[arg(short, long, default_value_t = String::from(".png"))]
    pub input_file_extension: String,
    /// string to add to the end of the file names
    #[arg(short, long, default_value_t = String::from(".webp"))]
    pub output_file_suffix: String,
}

fn main() {
    let mut all_files_path: Vec<PathBuf> = vec![];

    let this_dir = std::env::current_dir().expect("Error reading current dir");

    read_dir_recursive(this_dir.as_path(), &mut all_files_path).expect("Something went wrong while reading the path of the files");

    all_files_path = all_files_path.into_iter().filter(|x| {
        !(x.to_str().expect("error filtering webp images").ends_with(".webp"))
    }).collect::<Vec<PathBuf>>();

    'outerloop: for file_path_buf in all_files_path  {
        let file_name = match file_path_buf
            .as_os_str()
            .to_str(){
                Some(a) => a,
                None => {
                    println!("Error getting file path to file from buffer");
                    continue;
                }
            }
        ;
        println!("Finished {}", file_name);
        let webp_buffer = match convert_png_to_webp(file_name) {
            Ok(x)=>x,
            Err(e)=> {
                println!("{}: {}", file_name, e);
                continue 'outerloop;
            }
        };
        
        match  save_web_img(file_name, webp_buffer) {
            Ok(_)=> {},
            Err(e)=> {
                println!("{}: {}", file_name, e);
                continue 'outerloop;
            }
        };
    }
}


fn read_dir_recursive(dir: &Path, all_files_path: &mut Vec<PathBuf> )-> Result<(), std::io::Error>{
    if dir.is_dir() {
        for content_of_dir in fs::read_dir(dir)?{
            let entry = content_of_dir?;
            let entry_path = entry.path();
            if entry_path.is_dir() {
                // println!("Dir Opened: {}", entry_path.display());
                read_dir_recursive(&entry_path, all_files_path)?;
            } else {
                all_files_path.push(entry_path.to_path_buf());
                // println!("{}", entry_path.display());
            }
        }
    }
    Ok(())
}


fn read_png(path_buf: &str) -> Result<DynamicImage, SpecificErrors>{
    let mut img = image::io::Reader::
        open(
            path_buf
        )
        .map_err(SpecificErrors::FileRead)?
    ;
    img.set_format(ImageFormat::Png);
    let img_decoded = img.decode()
        .map_err(SpecificErrors::Image)?
    ;

    Ok(img_decoded)
}



fn convert_png_to_webp(file_name: &str)-> Result<WebPMemory, SpecificErrors> {
    let img = read_png(file_name)?;
    let enco = Encoder::from_image(&img).map_err(|x| SpecificErrors::Webp(x.to_string().clone()));
    Ok(enco?.encode(100.0))
}

fn save_web_img(file_name: &str, encoded_webp: WebPMemory)-> Result<(), std::io::Error>{
    let file_name_webp = format!("{}{}", file_name, ".webp") ;
    let mut webp_image = File::create(file_name_webp)?;
    webp_image.write_all(encoded_webp.as_bytes())?;
    Ok(())
}


