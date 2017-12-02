pub fn run(input: &str) -> Result<u64,String> {
    let v: Vec<Vec<u64>> = parse_to_vec(input)?;

    let res: u64 = v.iter()
                .map(|inner_vec| inner_vec.iter().max().unwrap() - inner_vec.iter().min().unwrap())
                .sum();

    Ok(res)
}

pub fn run2(input: &str) -> Result<u64,String> {
    let v: Vec<Vec<u64>> = parse_to_vec(input)?;
    
    let mut res = vec![];

    for inner_vec in v.iter() {
        'inner: for x in inner_vec {
            if let Some(a) = inner_vec.iter().find(|&y| (x % y) == 0 && x != y) {
                res.push(x/a);
                break 'inner;
            }
        }
    };

    Ok(res.iter().sum())
}

fn parse_to_vec(s: &str) -> Result<Vec<Vec<u64>>, String> {
    s.lines()
        .map(|line| line.split_whitespace()
                        .map(|s| s.parse()
                                  .map_err(|e| format!("{:?}",e)))
                        .collect())
                        .collect::<Result<Vec<_>,_>>()
}
