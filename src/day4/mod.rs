use std::collections::{HashSet, HashMap};
use std::hash::Hash;
use std::str::FromStr;

pub fn run(input: &str) -> Result<u64, String> {
    let res = input
        .lines()
        .filter_map(|line| if has_duplicates(&mut line.split_whitespace()) {
            None
        } else {
            Some(())
        })
        .count();
    Ok(res as u64)
}

pub fn run2(input: &str) -> Result<u64, String> {
    let res = input
        .lines()
        .filter_map(|line| if has_duplicates(
            &mut line.split_whitespace().map(
                |w| sort_word(&w),
            ),
        )
        {
            None
        } else {
            Some(())
        })
        .count();
    Ok(res as u64)
}

fn sort_word(s: &str) -> String {
    let mut v: Vec<_> = s.chars().collect();
    v.sort();
    v.into_iter().collect()
}

fn has_duplicates<T>(iter: &mut Iterator<Item = T>) -> bool
where
    T: Hash + Eq,
{
    let mut set = HashSet::new();
    for item in iter {
        if set.contains(&item) {
            return true;
        }
        set.insert(item);
    }
    return false;
}



#[test]
fn test_has_duplicates() {
    assert!(has_duplicates(&mut vec![1, 2, 2].iter()));
    assert!(!has_duplicates(&mut vec![1, 2, 3].iter()));
}

#[test]
fn test_sort_word() {
    assert_eq!("abc".to_owned(), sort_word("bca"));
}
