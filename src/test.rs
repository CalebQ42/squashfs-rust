#[cfg(test)]
mod tests{

    use std::fs::File;

    use crate::Squashfs;

    #[test]
    fn test_superblock(){
        let sfs = Squashfs::new(File::open("test.sfs").unwrap());
        println!("{:?}", sfs.superblock);
    }

    #[test]
    fn test_filenames(){
        let sfs = Squashfs::new(File::open("test.sfs").unwrap());
        for e in sfs.root.entries{
            println!("{:?}", e.name);
        }
    }
}