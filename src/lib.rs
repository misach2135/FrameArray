#[cfg(test)]
mod tests;
mod error;

use error::ArrayListError;

const CAPACITY: usize = 1024;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Item(u128);

#[derive(Debug)]
pub enum Frame {
    Gap {
        prev: Option<usize>,
        next: Option<usize>,
    },
    Element(Item),
    None
}

#[derive(Debug)]
pub struct Array {
    data: Box<[Frame; CAPACITY]>,
    len: usize,
    last_gap: Option<usize>
}

impl Array {
    /// Returns index of inserted item
    pub fn insert(&mut self, item: Item) -> Result<usize, ArrayListError> {
        if let Some(last_gap) = self.last_gap {
            let frame = &mut self.data[last_gap];
            if let Frame::Gap {prev, next} = frame {
                assert!(next.is_none());
                let index = last_gap;
                self.last_gap = *prev;
                *frame = Frame::Element(item);
                return Ok(index);
            }
            return Err(ArrayListError::Internal);
        }
        if self.len == CAPACITY {
            return Err(ArrayListError::RemoveError);
        }
        let index = self.len;
        self.data[index] = Frame::Element(item);
        self.len += 1;
        Ok(index)
    }

    /// Returns Item by index
    pub fn get(&self, index: usize) -> Result<Item, ArrayListError> {
        let frame = self.data.get(index).ok_or(ArrayListError::OutOfRange)?;

        if let Frame::Element(x) = frame {
            return Ok(x.clone());
        }
        
        Err(ArrayListError::ElementNotFound)
    }
    /// Removes Item from list by index
    pub fn remove(&mut self, index: usize) -> Result<(), ArrayListError> {
        let frame = self.data.get_mut(index).ok_or(ArrayListError::OutOfRange)?;
        
        if index == self.len - 1 {
            *frame = Frame::None;
            self.len -= 1;
            return Ok(());
        }

        if let Frame::Element(_) = frame {
            *frame = Frame::Gap { prev: self.last_gap, next: None };
            self.last_gap = Some(index);
        }

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Default for Array {
    fn default() -> Self {
        Self {
            data: Box::new([const {Frame::None} ; CAPACITY]),
            last_gap: None,
            len: 0
        }
    }
}
