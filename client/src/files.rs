use std::fs;
use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::path::PathBuf;

pub struct Files {
    pub path: PathBuf,
    pub buf: Box<[u8]>,
    pub buf_arr: [u8; 1024],
}

impl Files {
    pub fn new(p: PathBuf) -> Self {
        Self {
            path: p,
            buf: Box::new([]),
            buf_arr: [0; 1024],
        }
    }

    pub fn send_files(&mut self, mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = fs::File::open(&self.path)?;

        loop {
            let mut n = file.read(&mut self.buf_arr)?;

            if n == 0 {
                break;
            }

            stream.write_all(&self.buf_arr[..n]);
        }

        Ok(())
    }
    pub fn metadata(&mut self, name_file: &String) -> String {
        //let mut data: Vec<String> = Vec::new();

        let metadata = fs::metadata(name_file).expect("FALLO");

        let mut data: String = String::new();

        let data_type = metadata.file_type();

        // Formato a enviar al servidor name{string};size{u64};directorio{bool};file{bool}
        data = String::from(format!(
            "{};{};{};{}",
            self.path.file_name().expect("FALLO").to_string_lossy(),
            metadata.len(),
            data_type.is_dir(),
            data_type.is_file()
        ));

        return data;
    }
}
