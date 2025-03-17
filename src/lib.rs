#[cfg(test)]
mod tests;
mod error;

use core::borrow::BorrowMut;
use core::borrow::Borrow;

use error::ArrayListError;

const CAPACITY: usize = 1024;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Item(u128);

#[derive(Debug)]
pub enum Frame {
    Gap {
        prev: Option<usize>,
    },
    Element(Item)
}

#[derive(Debug)]
pub struct Array {
    data: Box<[Frame; CAPACITY]>,
    len: usize,
    last_gap: usize
}

impl Array {
    /// Returns index of inserted item
    pub fn insert(&mut self, item: Item) -> Result<usize, ArrayListError> {
        let frame = self.data.get_mut(self.last_gap).ok_or(ArrayListError::OutOfRange)?;
        if let Frame::Gap {prev} = *frame {
            let index = self.last_gap;
            *frame = Frame::Element(item);
            if let Some (prev) = prev {
                self.last_gap = prev;
            } else {
                self.last_gap += 1;
            }
            self.len += 1;
            return Ok(index);
        }
        Err(ArrayListError::Internal)
    }

    /// Check, if value exists at specified index
    pub fn exists(&self, index: usize) -> Result<bool, ArrayListError> {
        let frame = self.data.get(index).ok_or(ArrayListError::OutOfRange)?;
        match frame {
            Frame::Gap { prev: _ } => Ok(false),
            Frame::Element(_) => Ok(true),
        }
    }

    pub fn get_some(&self, index: usize) -> Result<Option<Item>, ArrayListError> {
        let frame = self.data.get(index).ok_or(ArrayListError::OutOfRange)?;

        if let Frame::Element(x) = frame {
            return Ok(Some(x.clone()));
        }
        
        Ok(None)
    }

    /// Returns Item by index
    pub fn get(&self, index: usize) -> Result<&Item, ArrayListError> {
        let frame = self.data.get(index).ok_or(ArrayListError::OutOfRange)?;

        if let Frame::Element(x) = frame {
            return Ok(x);
        }
        
        Err(ArrayListError::ElementNotFound)
    }
    /// Removes element on the index from the array. 
    /// It's weak remove, so it may not change the len of the array, in case index is not element.
    /// However, if it is element on index, it will remove it. May be used to guarentee, 
    /// that element doesn't exists at the index.
    pub fn remove(&mut self, index: usize) -> Result<Option<Item>, ArrayListError> {
        let frame = self.data.get_mut(index).ok_or(ArrayListError::RemoveError)?;
        if let Frame::Gap { prev } = frame {
            return Ok(None);
        }
        
        let prev = Some(self.last_gap);
        let frame = core::mem::replace(frame, Frame::Gap { prev });
        self.last_gap = index;
        self.len -= 1;
        let Frame::Element(item) = frame else {
            // Since it is not gap, it is element.
            // This branch must be unreachable.
            unreachable!()
        };

        Ok(Some(item))

    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl Default for Array {
    fn default() -> Self {
        Self {
            data: Box::new([const {Frame::Gap { prev: None }} ; CAPACITY]),
            last_gap: 0,
            len: 0
        }
    }
}
