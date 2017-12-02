pub fn run(input: &str) -> Result<u64,String> {
    let mut v: Vec<u32> = parse_to_vec(input)?;
    
    let first = v[0];
    v.push(first);
    let res = v.as_slice().windows(2).fold(0_u64, |acc, curr| {
        if curr[0] == curr[1] {
            acc + (curr[0] as u64)
        } else {
            acc
        }
    });
    Ok(res)
}

pub fn run2(input: &str) -> Result<u64,String> {
    let v: Vec<u32> = parse_to_vec(input)?;
    let res = v.iter()
                .zip(v.iter().cycle().skip(v.len()/2))
                .fold(0_u64, |acc, curr| {
                    if curr.0 == curr.1 {
                        acc + (*curr.0 as u64)
                    } else {
                        acc
                    }
                });
    Ok(res)
}

fn parse_to_vec(s: &str) -> Result<Vec<u32>, String> {
    s.chars()
        .map(|x| x.to_digit(10)
                  .ok_or(format!("Parse error: {}", x))
            )
        .collect()
}

