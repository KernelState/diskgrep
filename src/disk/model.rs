use crate::disk::units::Size;

#[derive(Debug)]
pub struct PermList {
    pub read: bool,
    pub write: bool,
    pub format: bool,
}

impl PermList {
    pub fn new() -> Self {
        Self {
            read: false,
            write: false,
            format: false,
        }
    }
}

pub trait Partitionable {}

#[derive(Debug)]
pub struct Partition {
    pub name: String,
    pub size: Size,
    pub fstype: String,
    pub uuid: String,
    pub mountpoint: Option<String>,
    pub mounted: bool,
    pub perms: PermList,
}

impl Partitionable for Partition {}

impl Partition {
    pub fn new(name: String, size: Size) -> Self {
        Self {
            name: name,
            size: size,
            fstype: String::from("Unknown"),
            uuid: String::from("Unknown"),
            mountpoint: None,
            mounted: false,
            perms: PermList::new(),
        }
    }
}

#[derive(Debug)]
pub struct Disk {
    pub name: String,
    pub serial: String,
    pub model: String,
    pub size: Size,
    pub children: Vec<Partition>,
}

impl Partitionable for Disk {}

impl Disk {
    pub fn new(name: String, size: Size) -> Self {
        Self {
            name: name,
            serial: String::from("Unknown"),
            model: String::from("Unknown"),
            size: size,
            children: Vec::new(),
        }
    }
}
