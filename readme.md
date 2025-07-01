# The `diskgrep` user manual
So getting pissed off by atrocious amounts of disks on your linux pc tagged with /dev/sda{num}, were on the same board

# Installation
currently there is no official installer for this app you just compile it with cargo and run it from `./target/debug/diskgrep`

# Usage
The app is really simple you can do 4 things with it
- list all disks: 
  this could be done by using the `-l` or `--list` flags and pressing enter (does not require sudo/root permissions) 

- mount a partition:
  this could be done by using the `-m` or `--mount` flags and passing a disk name (requires sudo/root permissions)
  **NOTE**: you should not use /dev/sda3 like how you would in the `mount` utility in linux, instead use sda3 without /dev/

- unmount a partition:
  this could be done by using the `-u` or `--umount` commmand (requires sudo permissions)
**NOTE**: you should use the format mentioned in the mount **NOTE**

- find a disk (the main feature):
this requires no genius just a `--find` (no short flag for it) and a directory name inside the disk's root, `--fstype` for the type like "ext4", "Fat32" or "ntfs" and `-i` or `--inside` to look for it in a certain disk
**NOTE**: the find feature does not work with the root that your using (your os root) yet
**NOTE**: the find feature only looks in the root of the disk and not through the entire disk
**NOTE**: the find feature can accept from 0 to 3 inputs and doesnt require inputting all of them
