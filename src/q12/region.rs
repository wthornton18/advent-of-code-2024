use crate::grid::Grid;

pub struct Region {
    pub plant_type: char,
    pub coords: Vec<(usize, usize)>,
}

impl Region {
    fn area(&self) -> usize {
        self.coords.len()
    }

    fn perimeter(&self) -> usize {
        let (mut min_x, mut min_y) = (usize::MAX, usize::MAX);
        let (mut max_x, mut max_y) = (0, 0);

        for (x, y) in &self.coords {
            min_x = min_x.min(*x);
            min_y = min_y.min(*y);
            max_x = max_x.max(*x);
            max_y = max_y.max(*y);
        }

        let mut grid = Grid::with_capacity_and_default(max_x - min_x + 3, max_y - min_y + 3, '.');
        for (x, y) in &self.coords {
            grid[(*x - min_x + 1, *y - min_y + 1)] = self.plant_type;
        }

        let mut perimeter = 0;

        for (x, y) in &self.coords {
            let x = *x - min_x + 1;
            let y = *y - min_y + 1;
            for adj in grid.adjacent_indices((x, y), false) {
                if grid[adj] == '.' {
                    perimeter += 1;
                }
            }
        }

        perimeter
    }

    fn corners(&self) -> usize {
        let (mut min_x, mut min_y) = (usize::MAX, usize::MAX);
        let (mut max_x, mut max_y) = (0, 0);

        for (x, y) in &self.coords {
            min_x = min_x.min(*x);
            min_y = min_y.min(*y);
            max_x = max_x.max(*x);
            max_y = max_y.max(*y);
        }

        let mut grid = Grid::with_capacity_and_default(max_x - min_x + 3, max_y - min_y + 3, '.');
        for (x, y) in &self.coords {
            grid[(*x - min_x + 1, *y - min_y + 1)] = self.plant_type;
        }

        let mut corners = 0;

        for coord in &self.coords {
            let x = coord.0 - min_x + 1;
            let y = coord.1 - min_y + 1;

            let north_empty = grid[(x, y - 1)] == '.';
            let south_empty = grid[(x, y + 1)] == '.';
            let west_empty = grid[(x - 1, y)] == '.';
            let east_empty = grid[(x + 1, y)] == '.';
            let north_east_empty = grid[(x + 1, y - 1)] == '.';
            let north_west_empty = grid[(x - 1, y - 1)] == '.';
            let south_east_empty = grid[(x + 1, y + 1)] == '.';
            let south_west_empty = grid[(x - 1, y + 1)] == '.';

            if north_empty && west_empty {
                corners += 1;
            }

            if north_empty && east_empty {
                corners += 1;
            }

            if south_empty && west_empty {
                corners += 1;
            }

            if south_empty && east_empty {
                corners += 1;
            }

            if !north_empty && !west_empty && north_west_empty {
                corners += 1;
            }

            if !north_empty && !east_empty && north_east_empty {
                corners += 1;
            }

            if !south_empty && !west_empty && south_west_empty {
                corners += 1;
            }

            if !south_empty && !east_empty && south_east_empty {
                corners += 1;
            }
        }

        corners
    }

    pub fn price(&self) -> usize {
        self.area() * self.perimeter()
    }

    pub fn discounted_price(&self) -> usize {
        self.area() * self.corners()
    }
}

impl std::fmt::Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (mut min_x, mut min_y) = (usize::MAX, usize::MAX);
        let (mut max_x, mut max_y) = (0, 0);

        for (x, y) in &self.coords {
            min_x = min_x.min(*x);
            min_y = min_y.min(*y);
            max_x = max_x.max(*x);
            max_y = max_y.max(*y);
        }

        let mut grid = Grid::with_capacity_and_default(max_x - min_x + 3, max_y - min_y + 3, '.');

        for (x, y) in &self.coords {
            grid[(*x - min_x + 1, *y - min_y + 1)] = self.plant_type;
        }

        write!(f, "{}", grid)
    }
}
