#![no_std]

pub trait Writerator {
    type Item: Copy;
    type ResultValue;
    type Error;

    fn write(&mut self, item: Self::Item) -> Result<Self::ResultValue, Self::Error>;

    fn write_slice(&mut self, items: &[Self::Item]) -> Result<(), Self::Error> {
        for item in items {
            self.write(*item)?;
        }
        Ok(())
    }

    fn write_iter(&mut self, items: impl Iterator<Item = Self::Item>) -> Result<(), Self::Error> {
        for item in items {
            self.write(item)?;
        }
        Ok(())
    }
}

pub trait SizedWriterator: Writerator {
    fn max_size(&self) -> usize;
}