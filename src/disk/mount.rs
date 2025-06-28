use std::fs;
use std::path::Path;

use crate::disk::model::Partition;
use crate::utils::callers::Cmd;
use crate::utils::error::{Error, ErrorKind, ErrorState};

pub fn mount(
    partition: &mut Partition,
    rw: bool,
    remount_ro: bool,
    defaults: bool,
    dir_name: String,
) -> Result<(), Error> {
    let mut cmd_text: String = String::from("mount ");
    if defaults {
        cmd_text.push_str("-o ");
        cmd_text.push_str("defaults");
    } else {
        cmd_text.push_str("-o ");
        if rw {
            cmd_text.push_str("rw,");
        } else {
            cmd_text.push_str("ro,");
        }
        if remount_ro {
            cmd_text.push_str("errors=remount-ro");
        }
    }
    cmd_text.push_str(format!(" -t {}", partition.fstype).as_str());

    let pth = Path::new(dir_name.as_str());

    if pth.exists() {
        if Vec::from_iter(pth.read_dir().iter()).len() > 1 {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                ErrorState::Return,
                String::from("directory is not empty"),
            ));
        }
    } else {
        match fs::create_dir(&dir_name) {
            Err(e) => {
                return Err(Error::new(
                    ErrorKind::Other,
                    ErrorState::Return,
                    format!("Cannot create directory {e}"),
                ));
            }
            Ok(_) => {}
        };
    }

    cmd_text.push_str(format!(" /dev/{} {dir_name}", partition.name).as_str());

    let cmd_obj = Cmd::new(cmd_text);

    match cmd_obj.run() {
        Err(e) => return Err(e.excec()),
        Ok(_) => {}
    };
    partition.mounted = true;
    partition.mountpoint = Option::Some(format!("/mnt/{dir_name}"));

    Ok(())
}

pub fn umount(partition: &mut Partition) -> Result<(), Error> {
    if !partition.mounted {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            ErrorState::Return,
            format!(
                "partition \"/dev/{}\" cannot be unmounted if its not mounted in the first place",
                partition.name
            ),
        ));
    }

    let cmd_obj = Cmd::new(format!("sudo umount /dev/{}", partition.name));
    match cmd_obj.run() {
        Err(e) => return Err(e.excec()),
        Ok(_) => {}
    }
    let pth = Path::new(format!("{}", partition.mountpoint.clone().unwrap()).as_str());
    partition.mountpoint = Option::None;
    partition.mounted = false;

    Ok(())
}
