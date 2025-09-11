use std::fs;
use std::io;
use std::io::Read;
use std::path::PathBuf;

pub struct Files {
    pub path: PathBuf,
    pub buf: Box<[u8]>,
    pub buf_arr: Vec<u8>,
}

impl Files {
    pub fn new(p: PathBuf) -> Self {
        Self {
            path: p,
            buf: Box::new([]),
            buf_arr: Vec::new(),
        }
    }

    pub fn content_files(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = fs::File::open(&self.path)?;

        file.read_to_end(&mut self.buf_arr)?;

        self.buf = self.buf_arr.clone().into_boxed_slice();

        println!("{:?}", self.buf);

        Ok(())
    }
    pub fn metadata(&mut self, name_file: &String) -> std::io::Result<()> {
        //let mut data: Vec<String> = Vec::new();

        let metadata = fs::metadata(name_file)?;

        println!("{}", metadata.len());

        Ok(())
    }
}
