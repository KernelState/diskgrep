use crate::disk::model::{Disk, Partition};
use crate::disk::mount;
use crate::disk::scanner::load_list;
use crate::utils::error::{Error, ErrorKind, ErrorState};
use std::path::Path;

pub fn find_id_dir(name: String) -> Result<Partition, Error> {
    let disk_list: Vec<Disk> = match load_list() {
        Err(e) => return Err(e.excec()),
        Ok(v) => v,
    };
    for d in disk_list {
        for i in d.children {
            let mut part = i.clone();
            match mount(&mut i, false, true, false, String::new()) {
                Err(e) => return Err(e.excec()),
                Ok(_) => println!("looking in disk /dev/{}", i.name),
            }
            let pth = Path::new(part.mountpoint.as_str());
            for p in pth.read_dir() {
                if p.to_str() == name {
                    return Ok(i);
                }
            }
        }
        Err(Error::new(
            ErrorKind::NotFound,
            ErrorState::Return,
            format!("The filename/dirname {name} was not found in any of the partition roots"),
        ))
    }
}
