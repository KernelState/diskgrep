extern crate diskgrep;
mod cli;

use clap::Parser;
use diskgrep::debug::Debug;
use diskgrep::disk::model::Partition;
use diskgrep::disk::mount;
use diskgrep::disk::scanner;
use diskgrep::identification::identification::find;
use diskgrep::identification::model::{Id, IdItem};
use diskgrep::identification::tag;
use diskgrep::utils::error::Error;
use diskgrep::utils::find::{find_part_in_root, DiskTypes};
use libc::getuid;
use std::env::home_dir;
use std::fs;
use std::io::Write;
use std::path::Path;

fn log(str_: String, debug: &Debug) {
    if debug.On {
        println!("{}", str_.as_str());
    }
}

fn main() -> Result<(), Error> {
    let args = cli::ArgV::parse();
    let mut debug = Debug::new(false);
    if args.listdisks {
        let mut disks = match scanner::load_list() {
            Err(e) => return Err(e.excec()),
            Ok(v) => v,
        };
        for i in disks {
            println!(
                "[{} /dev/{}] SIZE: {}",
                i.model.as_str(),
                i.name,
                i.size.to_str().as_str()
            );
            for p in i.children {
                println!(
                    "[{} > /dev/{}] SIZE: {}, FSTYPE: {}, MOUNTED: {}",
                    i.model,
                    p.name,
                    p.size.to_str(),
                    p.fstype,
                    p.mounted
                );
            }
        }
        return Ok(());
    }
    if args.mount {
        if unsafe { getuid() } != 0 {
            println!("The mount command can only be run as sudo");
            return Ok(());
        }
        match args.partition_name {
            None => {
                println!("no partition name was provided");
                return Ok(());
            }
            Some(ref v) => match find_part_in_root(v.clone()) {
                Err(e) => return Err(e.excec()),
                Ok(v_) => match v_ {
                    DiskTypes::Disk(_) => {
                        println!("\"/dev/{}\" is a disk not a partition", v.clone());
                        return Ok(());
                    }
                    DiskTypes::Partition(v) => match v.mounted {
                        true => {
                            println!("disk already mounted");
                            return Ok(());
                        }
                        false => {
                            let dname = match args.directory {
                                None => format!("/mnt/{}", v.name),
                                Some(ref v) => v.clone(),
                            };
                            let mut part = Partition { ..v };
                            match mount::mount(
                                &mut part,
                                !args.ro,
                                args.remount_ro,
                                args.default,
                                dname,
                            ) {
                                Err(e) => return Err(e.excec()),
                                Ok(_) => {
                                    println!("mounted \"/dev/{}\" to /mnt/{}", part.name, part.name)
                                }
                            }
                        }
                    },
                },
            },
        }
        return Ok(());
    }
    if args.umount {
        if unsafe { getuid() } != 0 {
            println!("The un-mount function can only be run in sudo mode");
            return Ok(());
        }
        let mut partition = match args.partition_name {
            None => {
                println!("A partition name has to be provided");
                return Ok(());
            }
            Some(ref z) => match find_part_in_root(z.clone()) {
                Err(_) => {
                    println!("partition {} does not exist", z.clone());
                    return Ok(());
                }
                Ok(b) => match b {
                    DiskTypes::Disk(_) => {
                        println!("this only works on partitions this is a disk");
                        return Ok(());
                    }
                    DiskTypes::Partition(a) => a,
                },
            },
        };
        match mount::umount(&mut partition) {
            Err(e) => {
                println!("Error unmounting partition {e}");
                return Ok(());
            }
            Ok(_) => println!("disk {} unmounted succefully", partition.name),
        };
        return Ok(());
    }
    if args.debug {
        debug.on();
    }
    if args.list_tags {
        for i in fs::read_dir(format!(
            "{}/.diskgrep",
            home_dir().expect("Cannot get home dir").display()
        ))
        .unwrap()
        .map(|e| {
            e.unwrap()
                .file_name()
                .to_str()
                .expect("cannot stringfy file name from OSStr to String (OS Error)")
                .to_string()
        }) {
            println!("[TAG] ~/.diskgrep/{i}");
        }
    }
    if let Option::Some(ref v) = args.find {
        log(format!("finding"), &debug);
        match (args.tag_file, args.ctag_file) {
            (None, None) => {
                println!("finding in normal mode");
                let mut id = Id::new(
                    IdItem::new(vec![v.clone()], args.fstype.clone(), args.inside.clone()),
                    args.not,
                    !args.any,
                    !args.any,
                );
                find(&mut id, &debug.On);

                for i in id.candidates {
                    println!("[CANDIDATE] /dev/{}", i.name);
                }
            }
            (Some(ref s), None) => {
                println!("finding using tag in ~/.diskgrep");
                let filepath = format!("$HOME/.diskgrep/{}.json", s.to_string());
                if !Path::new(filepath.as_str()).exists() {
                    println!("file does not exist in ~/.diskgrep");
                    return Ok(());
                }
                let mut id = tag::read(filepath)?;
                find(&mut id, &debug.On);
                for c in id.candidates {
                    println!("[CANDIDATE] /dev/{}", c.name);
                }
            }
            (None, Some(ref s)) => {
                println!("finding using absolute path");
                if !Path::new(s.as_str()).exists() {
                    println!("file does not exist");
                    return Ok(());
                }
                let mut id = tag::read(s.to_string())?;
                find(&mut id, &debug.On);
                for c in id.candidates {
                    println!("[CANDIDATE] /dev/{}", c.name);
                }
            }
            (Some(_), Some(_1)) => {
                println!("cannot use both custom tag path and default tag path");
                return Ok(());
            }
        }
        if let Option::None = args.tag {
            return Ok(());
        }
    }
    if let Option::Some(ref v) = args.tag {
        let mut id = Id::new(
            IdItem::new(Vec::new(), args.fstype, args.inside.clone()),
            args.not,
            !args.any,
            !args.any,
        );
        match args.find {
            None => {}
            Some(vf) => id.id.has_directory = vec![vf],
        }
        match tag::save(v.to_string(), args.tag_dir, id) {
            Err(e) => println!("got an error saving the tag\n{e}"),
            Ok(_) => println!("tag saved succefully"),
        }
    }
    Ok(())
}
