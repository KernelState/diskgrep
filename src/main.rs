extern crate diskgrep;
mod cli;

use clap::Parser;
use diskgrep::disk::model::Partition;
use diskgrep::disk::mount;
use diskgrep::disk::scanner;
use diskgrep::utils::error::Error;
use diskgrep::utils::find;
use libc::getuid;

fn main() -> Result<(), Error> {
    let args = cli::ArgV::parse();
    println!(
        "The arguments to access a certain disk will require using a non-/dev/ notation like -p sda2 instead of -p /dev/sda2"
    );
    if args.listdisks {
        let disks = match scanner::load_list() {
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
            Some(ref v) => match find::find_part_in_root(v.clone()) {
                Err(e) => return Err(e.excec()),
                Ok(v_) => match v_ {
                    find::DiskTypes::Disk(_) => {
                        println!("\"/dev/{}\" is a disk not a partition", v.clone());
                        return Ok(());
                    }
                    find::DiskTypes::Partition(v) => match v.mounted {
                        true => {
                            println!("disk already mounted");
                            return Ok(());
                        }
                        false => {
                            let dname = match args.directory {
                                None => format!("/mnt/{}", v.name),
                                Some(v) => v,
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
            Some(z) => match find::find_part_in_root(z.clone()) {
                Err(e) => {
                    println!("partition {} does not exist", z.clone());
                    return Ok(());
                }
                Ok(b) => match b {
                    find::DiskTypes::Disk(_) => {
                        println!("this only works on partitions this is a disk");
                        return Ok(());
                    }
                    find::DiskTypes::Partition(a) => a,
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
    }
    Ok(())
}
