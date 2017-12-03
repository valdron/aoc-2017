pub fn run(input: &str) -> Result<u64, String> {
    let x: u64 = input.trim().parse().map_err(|e| format!("{:?}", e))?;

    let mut circle = SpiralMem::new();
    let (x,y) = circle.nth((x - 2) as usize).ok_or("IterError!".to_owned())?;
    let res = x.abs() +y.abs();
    Ok(res as u64)
}

pub fn run2(input: &str) -> Result<u64, String> {
    let xx: u64 = input.trim().parse().map_err(|e| format!("{:?}", e))?;
    let mut v = vec![((0,0),1)];
    let mut circle = SpiralMem::new();
    for (x,y) in circle {
        let value = v.iter().filter(|&&((x2,y2), _)| (x-x2).abs()<= 1 && (y-y2).abs() <= 1).map(|&(_,val)| val).sum();
        if value > xx as u64{
            return Ok(value);
        }
        v.push(((x,y),value));
    }


    Ok(0)
}

struct SpiralMem{
    circle_no: usize,
    current_side: usize,
    current_ele_in_side: usize,
    side_elements: usize,
    curr: (isize,isize)
}

impl Iterator for SpiralMem {
    type Item = (isize, isize);
    fn next(&mut self) -> Option<Self::Item> {
        
        match self.current_side{
            0 => {
                if self.current_ele_in_side +1 == self.side_elements {
                    self.current_side += 1;
                    self.current_ele_in_side = 0;
                    self.curr.0 -= 1;
                } else {
                    self.curr.1 += 1;
                    self.current_ele_in_side += 1
                }
            },
            1 => {
                if self.current_ele_in_side +1 == self.side_elements  {
                    self.current_side += 1;
                    self.current_ele_in_side = 0;
                    self.curr.1 -= 1;
                } else {
                    self.curr.0 -= 1;
                    self.current_ele_in_side += 1
                }
            },
            2 => {
                if self.current_ele_in_side +1 == self.side_elements {
                    self.current_side += 1;
                    self.current_ele_in_side = 0;
                    self.curr.0 += 1;
                } else {
                    self.curr.1 -= 1;
                    self.current_ele_in_side += 1
                }
            },
            3 => {
                if self.current_ele_in_side +1 >= self.side_elements  {
                    self.circle_no += 1;
                    self.current_side = 0;
                    self.current_ele_in_side = 0;
                    self.side_elements += 2;
                    self.curr = (self.curr.0 + 1, self.curr.1);
                } else {
                    self.current_ele_in_side += 1;
                    self.curr = (self.curr.0 + 1, self.curr.1);
                }
            },
            _ => {unreachable!()}
        }
        Some(self.curr)    
    }
}

impl SpiralMem {
    fn new() -> Self {
        Self{
            circle_no: 1,
            current_side: 3,
            current_ele_in_side: 0,
            side_elements: 0,
            curr: (0, 0)
        }
    }
}