use std::collections::HashSet;
use std::hash::Hash;
use std::str::FromStr;

pub fn run(input: &str) -> Result<usize, String> {
    let res = input
        .lines()
        .map(|line| line.parse::<Passphrase>().unwrap())
        .filter(|pass| pass.is_valid1())
        .count();
    Ok(res)
}

pub fn run2(input: &str) -> Result<usize, String> {
    let res = input
        .lines()
        .map(|line| line.parse::<Passphrase>().unwrap())
        .filter(|pass| pass.is_valid2())
        .count();
    Ok(res)
}

fn sort_word(s: &str) -> String {
    let mut v: Vec<_> = s.chars().collect();
    v.sort();
    v.into_iter().collect()
}

struct Passphrase(Vec<String>);

impl FromStr for Passphrase {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            0: s.split_whitespace().map(|word| word.to_owned()).collect(),
        })
    }
}

impl Passphrase {
    fn is_valid1(&self) -> bool {
        !self.0.iter().has_duplicates()
    }

    fn is_valid2(&self) -> bool {
        !self.0.iter().map(|w| sort_word(w)).has_duplicates()
    }
}

trait HasDuplicates {
    fn has_duplicates(&mut self) -> bool;
}

impl<I, T> HasDuplicates for I
where
    I: Iterator<Item = T>,
    T: Hash + Eq,
{
    fn has_duplicates(&mut self) -> bool {
        let mut set = HashSet::new();
        for item in self {
            if set.contains(&item) {
                return true;
            }
            set.insert(item);
        }
        false
    }
}

#[test]
fn test_has_duplicates() {
    assert!(vec![1, 2, 2].iter().has_duplicates());
    assert!(!vec![1, 2, 3].iter().has_duplicates());
}

#[test]
fn test_sort_word() {
    assert_eq!("abc".to_owned(), sort_word("bca"));
}
