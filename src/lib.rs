#![cfg_attr(windows, feature(abi_vectorcall))]

use std::{cell::RefCell, io::Cursor, rc::Rc};

use ext_php_rs::types::Zval;
use ext_php_rs::prelude::*;
use zip::ZipArchive;
use std::io::Read;

#[php_class]
pub struct RustZip {
    zip: ZipArchive<Cursor<Vec<u8>>>,
}

#[php_impl]
impl RustZip {
    pub fn __construct(data: &mut Zval) -> PhpResult<RustZip> {
        let input_bytes = data
            .binary_slice()
            .expect("could not retrieve bytes of argument 1");

        let data_rc = Rc::new(RefCell::new(input_bytes.to_vec()));
        let cursor = std::io::Cursor::new(data_rc.borrow().clone());
        let zip = ZipArchive::new(cursor).map_err(|e| format!("Could not open zip file: {}", e))?;

        Ok(RustZip {
            zip
        })
    }

    #[php_impl]
    pub fn get(&mut self, filename: &str) -> PhpResult<Zval> {
        let mut file = self.zip.by_name(filename).map_err(|_| format!("File {} does not exists in Zip archive", filename))?;
        let mut buffer = Vec::new();
        let _ = file.read_to_end(&mut buffer).map_err(|_| format!("Cannot read file {} from Zip archive", filename));

        let mut val = Zval::new();
        val.set_binary(buffer);

        Ok(val)
    }

    #[php_impl]
    pub fn files(&mut self) -> Vec<String> {
        let mut files = Vec::new();
        for i in 0..self.zip.len() {
            let file = self.zip.by_index(i).expect("Failed to get file");
            files.push(file.name().to_string());
        }

        files
    }

    #[php_impl]
    pub fn has(&mut self, filename: &str) -> bool {
        self.zip.by_name(filename).is_ok()
    }
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
