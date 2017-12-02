use std::str::FromStr;

pub fn run(input: &str) -> Result<u32, String> {
    let n: Numbers = input.parse()?;
    let res = n.count_numbers();

    Ok(res)
}

pub fn run2(input: &str) -> Result<u32, String> {
    let  n: Numbers = input.parse()?;
    let res = n.count_numbers2();
    Ok(res)
}

struct Numbers(pub Vec<u32>);

impl FromStr for Numbers {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        Ok(Self {
            0: s.chars()
                .map(|x| x.to_digit(10).ok_or(format!("Parse error: {}", x)))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl Numbers {
    fn count_numbers(&self) -> u32 {
        let res = self.0.as_slice().windows(2).fold(
            0_u32,
            |acc, curr| if curr[0] ==
                curr[1]
            {
                acc + curr[0]
            } else {
                acc
            },
        );
        if self.0.first() == self.0.last() && self.0.first().is_some() {
            res + self.0.last().unwrap()
        } else {
            res
        }
    }

    fn count_numbers2(&self) -> u32 {
        self.0
            .iter()
            .zip(self.0.iter().cycle().skip(self.0.len() / 2))
            .fold(0_u32, |acc, curr| if curr.0 == curr.1 {
                acc + curr.0
            } else {
                acc
            })
    }
}
