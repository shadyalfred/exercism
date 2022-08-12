// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_freq = HashMap::new();

    for word in magazine {
        *magazine_freq.entry(word).or_insert(0) += 1;
    }

    let mut note_freq = HashMap::new();

    for word in note {
        *note_freq.entry(word).or_insert(0) += 1;
    }

    for (word, freq) in note_freq {
        match magazine_freq.get(word) {
            Some(available_freq) => {
                if *available_freq < freq {
                    return false;
                }
            }
            None => return false,
        }
    }

    true
}
