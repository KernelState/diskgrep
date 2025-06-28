use clap::Parser;

#[derive(Parser)]
#[command(name = "diskgrep", version, about = "DISKGREP UTILITY (V0.1)")]
pub struct ArgV {
    #[arg(short = 'l', long = "list")]
    pub listdisks: bool,

    #[arg(short = 'm', long = "mount")]
    pub mount: bool,

    #[arg(short = 'u', long = "umount")]
    pub umount: bool,

    #[arg(short = 'p', long = "partition")]
    pub partition_name: Option<String>,

    #[arg(short = 'r', long = "read-only")]
    pub ro: bool,

    #[arg(short = 'd', long = "default")]
    pub default: bool,

    #[arg(short = 'e', long = "ro-error")]
    pub remount_ro: bool,

    #[arg(short = 'c', long = "containing-dir")]
    pub directory: Option<String>,

    #[arg(short = 'i', long = "identifier")]
    pub identifier: Option<String>,

    #[arg(short = 'f', long = "find")]
    pub find: bool,
}
