use crate::{directory::Entry, inode::Inode};

pub(crate) struct Folder{
    pub(crate) inode: Inode,
    pub(crate) entries: Vec<Entry>,
}