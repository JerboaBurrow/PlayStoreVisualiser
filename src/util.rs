use std::{fs::File, io::{Read, Write}};

pub fn read_file_utf8(path: &str) -> Option<String>
{
    let mut file = match File::open(path) {
        Err(why) => 
        {
            crate::debug(format!("error reading file to utf8, {}", why), None);
            return None
        },
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => 
        {
            crate::debug(format!("error reading file to utf8, {}", why), None);
            None
        },
        Ok(_) => Some(s)
    }
}

pub fn write_file(path: &str, data: &[u8])
{
    let mut file = File::create(path).unwrap();
    file.write_all(data).unwrap();
}