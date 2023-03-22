mod superblock;

pub struct Squash{
    sup: crate::superblock::Superblock,
}

pub fn read(rdr: &mut dyn std::io::Read) -> Squash{
    Squash { sup: superblock::read_from(rdr) }
}

#[cfg(test)]
mod tests{
    use std::{fs::File, io};

    use super::*;

    #[test]
    fn stuff() -> io::Result<()>{
        let mut test_file = File::open("test.sfs")?;
        let out = read(&mut test_file);
        println!("{:?}", out.sup);
        Ok(())
    }
}