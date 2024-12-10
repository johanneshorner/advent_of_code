#[derive(Debug, Clone)]
pub struct Grid<T>(Vec<Vec<T>>);

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point { x, y }
    }
}

impl From<&(usize, usize)> for Point {
    fn from((x, y): &(usize, usize)) -> Self {
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
                    .map(|x| Point { x, y })
                    .collect::<Vec<Point>>()
            })
            .collect()
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

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if self.is_inbounds(x as isize, y as isize) {
            Some(&self.0[y][x])
        } else {
            None
        }
    }

    pub fn get_unchecked(&self, x: usize, y: usize) -> &T {
        &self.0[y][x]
    }

    pub fn left(&self, x: usize, y: usize) -> Option<&T> {
        self.get(x - 1, y)
    }

    pub fn left_up(&self, x: usize, y: usize) -> Option<&T> {
        self.get(x - 1, y - 1)
    }

    pub fn up(&self, x: usize, y: usize) -> Option<&T> {
        self.get(x, y - 1)
    }

    pub fn right_up(&self, x: usize, y: usize) -> Option<&T> {
        self.get(x + 1, y - 1)
    }

    pub fn right(&self, x: usize, y: usize) -> Option<&T> {
        self.get(x + 1, y)
    }

    pub fn right_down(&self, x: usize, y: usize) -> Option<&T> {
        self.get(x + 1, y + 1)
    }

    pub fn down(&self, x: usize, y: usize) -> Option<&T> {
        self.get(x, y + 1)
    }

    pub fn left_down(&self, x: usize, y: usize) -> Option<&T> {
        self.get(x - 1, y + 1)
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
        .map(|(x, y)| (*x as usize, *y as usize).into())
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
        .map(|(x, y)| (*x as usize, *y as usize).into())
        .collect::<Vec<Point>>()
    }
}
