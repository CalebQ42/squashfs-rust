#[cfg(test)]
mod Tests{
    use std::fs::File;

    use crate::Squashfs;

    #[test]
    fn test_superblock(){
        let sfs = Squashfs::from_file("test.sfs");
        println!("{:?}", sfs.superblock);
    }
}