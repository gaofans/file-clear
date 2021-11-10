use std::fs::File;
use std::io::prelude::*;
use zip::result::{ZipError, ZipResult};
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};
use std::time::Instant;
use lzma_rs::{lzma2_compress, lzma_compress};

pub fn bzip(target: &str,source: &str) -> Result<(),ZipError>{
    let source_path = std::path::Path::new(source);
    let target_file = File::create(target).unwrap();
    let mut writer = ZipWriter::new(target_file);
    let option = FileOptions::default().compression_method(CompressionMethod::Bzip2);
    if source_path.is_file() {
        writer.start_file(source_path.file_name().unwrap().to_str().unwrap(),option);
        let mut source_file = File::open(source_path).unwrap();
        let mut buffer = vec![0u8;1024*1024*4];
        let mut read_size:usize = 0;
        let file_len:usize = source_file.metadata().unwrap().len() as usize;
        let now = Instant::now();
        while let Ok(size) = source_file.read(&mut buffer) {
            if size == 0 { break; }
            writer.write_all(&buffer[0..size]);
            read_size = read_size + size;
            println!("compress percent {}%",(read_size * 100 / file_len));
        }
        let elapsed_time = now.elapsed();
        println!("compress took {} seconds.", elapsed_time.as_secs());
        writer.finish()?;
    }else {
        ZipError::InvalidArchive("");
    }
    Ok(())
}
use std::io::BufReader;
use std::io::BufWriter;

pub fn sevenz(target: &str,source: &str){
    let now = Instant::now();
    let mut reader = BufReader::new(File::open(source).unwrap());
    let mut writer = BufWriter::new(File::create(target).unwrap());
    lzma_compress(&mut reader,&mut writer);
    let elapsed_time = now.elapsed();
    println!("compress took {} seconds.", elapsed_time.as_secs());
}


