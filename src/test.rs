#[cfg(test)]
mod tests{

    use std::fs::File;

    use crate::Squashfs;

    #[test]
    fn test_superblock(){
        let sfs = Squashfs::new(File::open("test.sfs").unwrap());
        println!("{:?}", sfs.superblock);
    }
}