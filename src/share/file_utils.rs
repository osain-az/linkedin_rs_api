use serde::{
    de::{self, DeserializeOwned},
    Deserialize, Deserializer, Serialize,
};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::os::unix::fs::FileExt;

pub struct FileChunking {
    file: File,
    postition: u64,
    remaining: u64,
    file_size: u64,
    is_completed: bool,
}

impl FileChunking {
    pub fn new(file: File) -> Self {
        let _file = file.try_clone().unwrap();
        FileChunking {
            file,
            postition: 0,
            is_completed: false,
            file_size: _file.metadata().unwrap().len(),
            remaining: _file.metadata().unwrap().len(),
        }
    }
    pub fn chunk_by_4mb(mut self) -> Vec<u8> {
        let mut chunk_size = 4194303;
        let mut buffer = [0u8; 4194303];
        let position = self.postition.clone();
        let remaining = self.remaining.clone();
        let file_size = self.file_size.clone();

        let mut file = self.file.try_clone().unwrap();

        if file_size - remaining > chunk_size {
            let chunk_position = if position < 2 { position } else { position - 1 };
            file.seek(SeekFrom::Start(chunk_position)).unwrap();
            self.remaining = file_size - chunk_position;
            self.postition = position + chunk_position;

            file.read_exact(&mut buffer);
            return buffer.to_vec();
        } else {
            file.seek(SeekFrom::Start(position - 1)).unwrap();
            let mut buf = vec![];
            self.file.read_to_end(&mut buf);
            self.is_completed = true;
            self.remaining = 0;
            self.postition = position + chunk_size;
            return buf.to_vec();
        }
    }
    pub fn chunk_by_u64(mut self, chunk_size: u64) -> Vec<u8> {
        let mut chunk_size = 4194303;
        let mut buffer = [0u8; 4194303];
        let position = self.postition.clone();
        let remaining = self.remaining.clone();
        let file_size = self.file_size.clone();

        let mut file = self.file.try_clone().unwrap();

        if file_size - remaining > chunk_size {
            let chunk_position = if position < 2 { position } else { position - 1 };
            file.seek(SeekFrom::Start(chunk_position)).unwrap();
            self.remaining = file_size - chunk_position;
            self.postition = position + chunk_position;
            file.read_exact(&mut buffer);
            return buffer.to_vec();
        } else {
            file.seek(SeekFrom::Start(position - 1)).unwrap();
            let mut buf = vec![];
            self.file.read_to_end(&mut buf);
            self.is_completed = true;
            self.remaining = 0;
            self.postition = position + chunk_size;
            return buf.to_vec();
        }
    }

    pub fn extract_by_size_and_offset(mut self, extract_to_size: u64, offset: u64) -> Vec<u8> {
        let mut chunk_size = 4194303;
        let mut buffer = [0u8; 4194303];
        //  let mut chunk = Vec::with_capacity(extract_to_size as usize);
        let position = self.postition.clone();
        let remaining = self.remaining.clone();
        let file_size = self.file_size.clone();

        let mut file = self.file.try_clone().unwrap();
        //file.read_exact_at(buffer, )
        if extract_to_size < file_size.try_into().unwrap() {
            let chunk_position = offset;
            let mut buffer = Vec::new();
            // let mut buffer = [0u8; 4194303];
            println!(
                "at seek, siz: {} offset : {}, file size :{}  ",
                extract_to_size,
                offset,
                file.metadata().unwrap().len()
            );

            //  file.seek(SeekFrom::Start(offset)).unwrap();
            // file.read_exact(buffer.as_mut()).unwrap();
            //  file.read_to_end(&mut buffer).unwrap();
            //let mut chunk = file.take(offset);
            println!("at seek, chunk siz: {} ", buffer.len());
            file.take(offset).read_exact(&mut buffer).unwrap();
            println!("at seek, after chunk siz: {} ", buffer.len());
            println!(" hrjururn ",);
            buffer.to_vec()
        } else {
            let chunk_offset = if offset < 2 { offset } else { offset - 1 };

            //let mut buffer = vec![0u8, extract_to_size as];
            println!("at seek, chunk siz: {} ", buffer.len());

            file.take(offset).read_exact(&mut buffer);
            //   file.read_exact_at(&mut buffer, offset);
            println!("at seek, chunk siz: {} ", buffer.len());
            // return buffer.to_vec();
            buffer.to_vec()
        }
    }
    pub fn extract_to_end(self) -> Vec<u8> {
        let mut buffer = vec![];
        let mut file = self.file.try_clone().unwrap();
        file.read_to_end(&mut buffer);
        buffer
    }

    pub fn postition(&self) -> u64 {
        self.postition
    }
    pub fn remaining(&self) -> u64 {
        self.remaining
    }
    pub fn is_completed(&self) -> bool {
        self.is_completed
    }
}
