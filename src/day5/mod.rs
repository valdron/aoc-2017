use std::str::FromStr;

pub fn run(input: &str) -> Result<usize, String> {
    let update = |x: &mut i64| *x += 1;

    let res = input.parse::<JumpMaze>()?.into_jump_iter(&update).count();

    Ok(res)
}

pub fn run2(input: &str) -> Result<usize, String> {
    let update = |x: &mut i64| if *x < 3 { *x += 1 } else { *x -= 1 };

    let res = input.parse::<JumpMaze>()?.into_jump_iter(&update).count();

    Ok(res)
}

struct JumpMaze(Vec<i64>);

impl FromStr for JumpMaze {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            0: s.lines()
                .map(|line| line.parse().map_err(|e| format!("{:?}", e)))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }
}

impl JumpMaze {
    fn get_mut(&mut self, i: i64) -> Option<&mut i64> {
        if i >= 0 && i < self.0.len() as i64 {
            Some(&mut self.0[i as usize])
        } else {
            None
        }
    }

    fn into_jump_iter(self, update: &Fn(&mut i64)) -> JumpMazeIter {
        JumpMazeIter {
            maze: self,
            update: update,
            counter: 0,
        }
    }
}

struct JumpMazeIter<'a> {
    maze: JumpMaze,
    update: &'a Fn(&mut i64),
    counter: i64,
}

impl<'a> Iterator for JumpMazeIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.maze.get_mut(self.counter) {
            self.counter += *item;
            (self.update)(item);
            Some(self.counter as usize)
        } else {
            None
        }
    }
}
