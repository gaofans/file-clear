pub fn test(){
}
use std::fs::File;
use std::io::prelude::*;
use zip::result::{ZipError, ZipResult};
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

pub fn bzip(target: &str,source: &str) -> Result<(),ZipError>{
    let source_path = std::path::Path::new(source);
    let target_file = File::create(target).unwrap();
    let mut writer = ZipWriter::new(target_file);
    let option = FileOptions::default().compression_method(CompressionMethod::Bzip2);
    if source_path.is_file() {
        writer.start_file(source_path.file_name().unwrap().to_str().unwrap(),option);
        let mut source_file = File::open(source_path).unwrap();
        let mut buffer = vec![0u8;1024*1024*4];
        while let Ok(size) = source_file.read(&mut buffer) {
            if size == 0 { break; }
            writer.write_all(&buffer[0..size]);
        }
        writer.finish()?;
    }else {
        ZipError::InvalidArchive("");
    }
    Ok(())
}


