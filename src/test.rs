#[cfg(test)]
mod tests{

    use crate::Squashfs;

    #[test]
    fn test_superblock(){
        let sfs = Squashfs::from_file("test.sfs");
        println!("{:?}", sfs.superblock);
    }
}