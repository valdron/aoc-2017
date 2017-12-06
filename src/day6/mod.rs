use std::str::FromStr;
use std::mem;
use std::collections::HashSet;
use std::hash::Hash;

pub fn run(input: &str) -> Result<usize, String> {
    let banks: Banks = input.parse()?;
    banks.into_bankredis_iter().find_duplicate().and_then(|(i,_)| Some(i)).ok_or("None".into())
}

pub fn run2(input: &str) -> Result<usize, String> {
    let banks: Banks = input.parse()?;
    let mut iter = banks.into_bankredis_iter();
    let (_, dup) = iter.find_duplicate().ok_or("None".to_owned())?;
    iter.position(|x| x == dup).map(|x| x + 1 ).ok_or("None".into())
}

#[derive(Debug)]
struct Banks(Vec<usize>);

impl FromStr for Banks {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            0: s.split_whitespace()
                .map(|x| x.parse().map_err(|e| format!("{:?}", e)))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl Banks {
    fn choose(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .fold((0, 0), |(j, max), (i, &x)| if x > max {
                (i, x)
            } else {
                (j, max)
            })
            .0
    }

    fn redistribute(&mut self) {
        let mut bank_nr = self.choose();
        let mut blocks = 0;
        if let Some( b) = self.0.get_mut(bank_nr) {
            mem::swap( b, &mut blocks)
        }

        let (all, rem) = (blocks / self.0.len(), blocks % self.0.len());

        self.0.iter_mut().map(|x| *x += all).for_each(|_| {});
        for _ in 0..rem {
            bank_nr += 1;
            bank_nr %= self.0.len();
            self.0[bank_nr] += 1;
        }
    }

    fn into_bankredis_iter(self) -> BankRedisIter {
        BankRedisIter(self)
    }
}

struct BankRedisIter(Banks);

impl Iterator for BankRedisIter {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.redistribute();
        Some(format!("{:?}", self.0))
    }
}

trait IterFindDuplicate<T> {
    fn find_duplicate(&mut self) -> Option<(usize, T)>;
}

impl<T, I> IterFindDuplicate<T> for I 
where I: Iterator<Item=T>,
      T: Hash + Eq
{
    fn find_duplicate(&mut self) -> Option<(usize, T)> {
        let mut set: HashSet<T> = HashSet::new();
        for i in 1.. {
            if let Some(itm) = self.next() {
                if set.contains(&itm) {
                    return Some((i, itm))
                } else {
                    set.insert(itm);
                }
            } else {
                return None;
            }
        }
        unreachable!()
    }
}