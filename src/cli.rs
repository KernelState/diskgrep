use clap::Parser;

#[derive(Parser, Clone)]
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

    #[arg(long = "read-only")]
    pub ro: bool,

    #[arg(short = 'd', long = "default")]
    pub default: bool,

    #[arg(short = 'e', long = "ro-error")]
    pub remount_ro: bool,

    #[arg(long = "mountpoint")]
    pub directory: Option<String>,

    #[arg(short = 'f', long = "find")]
    pub find: Option<String>,

    #[arg(short = 'i', long = "inside")]
    pub inside: Option<String>,

    #[arg(long = "fstype")]
    pub fstype: Option<String>,

    #[arg(short = 'n', long = "not")]
    pub not: bool,

    #[arg(short = 'a', long = "any")]
    pub any: bool,

    #[arg(long = "debug")]
    pub debug: bool,

    #[arg(short = 't', long = "tag")]
    pub tag: Option<String>,

    #[arg(long = "read-tag")]
    pub read_tag: Option<String>,

    #[arg(long = "tag-dir")]
    pub tag_dir: Option<String>,

    #[arg(long = "tag-file")]
    pub tag_file: Option<String>,

    #[arg(long = "ctag-file")]
    pub ctag_file: Option<String>,

    #[arg(long = "list-tags")]
    pub list_tags: bool,
}
