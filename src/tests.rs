use std::error::Error;
use rand::{rng, seq::{IndexedRandom, IteratorRandom}, RngCore};

use crate::{Array, Item};

#[derive(thiserror::Error, Debug)]
enum TestError {
    #[error("Fail to choose")]
    FailToChoose
}

#[test]
fn insert_elements_in_array_list() -> Result<(), Box<dyn Error>> {
    let mut array = Array::default();
    let item = Item::default();
    
    let index = array.insert(item.clone())?;
    
    assert!(index == 0);
    let array_inner = array.get(index)?;
    assert!(*array_inner == item);

    Ok(())
}

#[test]

fn handle_array_length_after_remove_and_insert() -> Result<(), Box<dyn Error>> {
    let mut array = Array::default();
    let mut indexes = Vec::new();
    for _ in 0..100 {
        let item = Item(rng().next_u64() as u128);
        indexes.push(array.insert(item)?);
    }
    // Now, remove some elements
    let len_before_remove = array.len();

    while array.len() > 80 {
        let index_ind = (0..indexes.len()).choose(&mut rng()).ok_or(TestError::FailToChoose)?;
        let index = indexes.remove(index_ind);
        println!("Index: {index}");
        array.remove(index)?;
    }
    
    // Add elements
    for _ in 0..20 {
        let item = Item(rng().next_u64() as u128);
        indexes.push(array.insert(item)?);
    }
    
    let len_after_insert = array.len();
    assert_eq!(len_before_remove, len_after_insert);
    Ok(())
}