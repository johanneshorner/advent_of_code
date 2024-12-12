#[derive(Debug, Clone)]
pub struct Grid<T>(Vec<Vec<T>>);

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl From<(isize, isize)> for Point {
    fn from((x, y): (isize, isize)) -> Self {
        Point { x, y }
    }
}

impl From<&(isize, isize)> for Point {
    fn from((x, y): &(isize, isize)) -> Self {
        Point { x: *x, y: *y }
    }
}

impl<T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
        Self(grid)
    }

    pub fn all_points(&self) -> Vec<Point> {
        (0..self.width())
            .flat_map(|y| {
                (0..self.height())
                    .map(|x| Point {
                        x: x as isize,
                        y: y as isize,
                    })
                    .collect::<Vec<Point>>()
            })
            .collect()
    }

    pub fn flattened(&self) -> Vec<&T> {
        let mut vec = Vec::with_capacity(self.width() * self.height());
        for y in 0..self.height() {
            for x in 0..self.width() {
                vec.push(self.get(x as isize, y as isize).unwrap());
            }
        }

        vec
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn is_inbounds(&self, x: isize, y: isize) -> bool {
        x >= 0
            && (0..self.width()).contains(&(x as usize))
            && y >= 0
            && (0..self.height()).contains(&(y as usize))
    }

    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        if self.is_inbounds(x, y) {
            Some(&self.0[y as usize][x as usize])
        } else {
            None
        }
    }

    pub fn get_unchecked(&self, x: usize, y: usize) -> &T {
        &self.0[y][x]
    }

    pub fn neighbour_coords(&self, x: usize, y: usize) -> Vec<Point> {
        let x = x as isize;
        let y = y as isize;
        [
            (x - 1, y), // left
            (x, y - 1), // up
            (x + 1, y), // right
            (x, y + 1), // down
        ]
        .iter()
        .filter(|(x, y)| self.is_inbounds(*x, *y))
        .map(Into::into)
        .collect::<Vec<Point>>()
    }

    pub fn neighbour_coords_with_diagonal(&self, x: usize, y: usize) -> Vec<Point> {
        let x = x as isize;
        let y = y as isize;
        [
            (x - 1, y),     // left
            (x - 1, y - 1), // left_up
            (x, y - 1),     // up
            (x + 1, y - 1), // right_up
            (x + 1, y),     // right
            (x + 1, y + 1), // right_down
            (x, y + 1),     // down
            (x - 1, y + 1), // left_down
        ]
        .iter()
        .filter(|(x, y)| self.is_inbounds(*x, *y))
        .map(Into::into)
        .collect::<Vec<Point>>()
    }
}
