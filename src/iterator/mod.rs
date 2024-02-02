#![allow(unused)]
mod chain;

struct Fibonacci {
    curr: u32,
    next: u32,
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.next;
        self.next = self.curr + self.next;
        self.curr = tmp;
        Some(self.curr)
    }
}

struct Grid {
    x_coords: Vec<u32>,
    y_coords: Vec<u32>,
}

// implement IntoIterator for &Grid and storing a reference to the Grid in GridIter
// 需要声明 lifetime
impl<'a> IntoIterator for &'a Grid {
    type Item = (u32, u32);
    type IntoIter = GridIter<'a>;
    fn into_iter(self) -> GridIter<'a> {
        GridIter {
            grid: self,
            i: 0,
            j: 0,
        }
    }
}

struct GridIter<'a> {
    grid: &'a Grid,
    i: usize,
    j: usize,
}

impl<'a> Iterator for GridIter<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.i >= self.grid.x_coords.len() {
            self.i = 0;
            self.j += 1;
            if self.j >= self.grid.y_coords.len() {
                return None;
            }
        }
        let res = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
        self.i += 1;
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let fib = Fibonacci { curr: 0, next: 1 };
        let res: Vec<u32> = fib.into_iter().take(5).collect();
        assert_eq!(res, [1, 1, 2, 3, 5]);
    }

    #[test]
    fn test_grid_iter() {
        let grid = Grid {
            x_coords: vec![3, 5],
            y_coords: vec![10, 20],
        };
        for (x, y) in &grid {
            println!("point = {x}, {y}");
        }
        for (x, y) in &grid {
            println!("point = {x}, {y}");
        }
    }
}
