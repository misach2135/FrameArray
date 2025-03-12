use std::error::Error;
use rand::{rng, seq::IndexedRandom, RngCore};

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
    assert!(array_inner == item);

    Ok(())
}

#[test]

fn handle_gaps() -> Result<(), Box<dyn Error>> {
    let mut array = Array::default();
    let mut indexes = Vec::new();
    for _ in 0..100 {
        let item = Item(rng().next_u64() as u128);
        indexes.push(array.insert(item)?);
    }

    // Now, remove some elements

    for _ in 0..20 {
        let index = *indexes.choose(&mut rng()).ok_or(TestError::FailToChoose)?;
        let len = array.len();
        array.remove(index)?;
        if index == (len - 1) {
            println!("last index");
            assert_eq!(array.len(), len - 1);
        }
    }

    // Now lets try to add new elements
    let old_len = array.len();
    for _ in 0..20 {
        let item = Item(rng().next_u64() as u128);
        indexes.push(array.insert(item)?);
    }

    assert_eq!(old_len, array.len());

    Ok(())
}