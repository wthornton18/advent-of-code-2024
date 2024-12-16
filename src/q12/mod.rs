use region::Region;

use crate::grid::Grid;
mod region;

fn parse_input(input: &str) -> Grid<char> {
    let mut grid = Grid::new();
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let chars = line.chars().collect::<Vec<_>>();
        grid.push(&chars);
    }
    grid
}

pub fn get_total_garden_price(input: &str) -> usize {
    let grid = parse_input(input);
    let regions = get_regions(&grid);
    regions.iter().map(|r| r.price()).sum()
}

pub fn get_total_garden_discounted_price(input: &str) -> usize {
    let grid = parse_input(input);
    let regions = get_regions(&grid);
    regions.iter().map(|r| r.discounted_price()).sum()
}

fn get_regions(grid: &Grid<char>) -> Vec<Region> {
    let mut regions = Vec::new();
    let mut seen = Grid::with_capacity_and_default(grid.rows, grid.cols, false);

    for (coord, c) in grid.clone().into_iter() {
        if (c == '.') || seen[coord] {
            continue;
        }

        regions.push(get_region(grid, coord, &mut seen));
    }
    regions
}

fn get_region(grid: &Grid<char>, (i, j): (usize, usize), seen: &mut Grid<bool>) -> Region {
    let mut coords = vec![(i, j)];
    let target_char = grid[(i, j)];
    let mut visited = Grid::with_capacity_and_default(grid.rows, grid.cols, false);

    while let Some(coord) = coords.pop() {
        if visited[coord] {
            continue;
        }

        visited[coord] = true;

        for adj in grid.adjacent_indices(coord, false) {
            if grid[adj] == target_char && !visited[adj] {
                coords.push(adj);
            }
        }
    }

    *seen |= visited.clone();

    let mut coords = Vec::new();

    for (coord, visited) in visited {
        if visited {
            coords.push(coord);
        }
    }

    Region {
        plant_type: grid[(i, j)],
        coords,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const TEST_INPUT: &str = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const TEST_NON_CONNECTED_REGIONS: &str = "
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const TEST_SIMPLE_REGION: &str = "
AAAA
BBCD
BBCC
EEEC";

    #[rstest]
    #[case(TEST_INPUT, 1930)]
    #[case(TEST_NON_CONNECTED_REGIONS, 772)]
    #[case(TEST_SIMPLE_REGION, 140)]
    fn test_get_total_garden_price(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(get_total_garden_price(input), expected);
    }
}
