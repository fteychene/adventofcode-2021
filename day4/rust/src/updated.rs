use itertools::{Either, FoldWhile, Itertools};

#[derive(Debug, Clone)]
struct Grid {
    rows: Vec<Vec<(u32, bool)>>,
}

fn split_inputs(inputs: &str) -> Option<(&str, Vec<Vec<&str>>)> {
    let mut inputs = inputs.lines();
    let draw_numbers = inputs.next();
    let grids = inputs.batching(|it| {
        Some(
            it.skip_while(|x| x.is_empty())
                .take_while(|x| !x.is_empty())
                .collect::<Vec<&str>>(),
        )
        .filter(|x| !x.is_empty())
    });
    draw_numbers.map(|numbers| (numbers, grids.collect::<Vec<Vec<&str>>>()))
}

fn parse_numbers(line: &str) -> Result<Vec<u32>, String> {
    line.split(",")
        .map(|x| {
            x.parse::<u32>()
                .map_err(|_| format!("Error parsing draw number : {}", x))
        })
        .collect()
}

fn parse_grid(grid: Vec<&str>) -> Result<Grid, String> {
    grid.iter()
        .map(|line| {
            line.split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| {
                    x.parse::<u32>()
                        .map(|value| (value, false))
                        .map_err(|_| format!("Error parsing grid number : {}", x))
                })
                .collect::<Result<Vec<(u32, bool)>, String>>()
        })
        .collect::<Result<Vec<Vec<(u32, bool)>>, String>>()
        .map(|grids| Grid { rows: grids })
}

fn update_grid(grid: &Grid, number: u32) -> Grid {
    Grid {
        rows: grid
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|(value, marked)| (*value, *marked || *value == number))
                    .collect()
            })
            .collect(),
    }
}

fn is_grid_valid(grid: &Grid) -> bool {
    let column_size = grid.rows.first().map(|x| x.len()).unwrap_or(0);
    let row_valid = grid
        .rows
        .iter()
        .any(|row| row.iter().all(|(_, marked)| *marked));
    let column_valid = (0..column_size).any(|column| {
        grid.rows
            .iter()
            .map(|x| x.get(column).unwrap())
            .all(|(_, marked)| *marked)
    });
    row_valid || column_valid
}

pub(crate) fn run(inputs: &str) -> Result<u32, String> {
    let (draw_numbers, grids) =
        split_inputs(inputs).ok_or("Can't split inputs for draw and grids")?;
    let draw_numbers = parse_numbers(draw_numbers)?;
    let grids = grids
        .into_iter()
        .map(|grid| parse_grid(grid))
        .collect::<Result<Vec<Grid>, String>>()?;

    let (_, result) = draw_numbers
        .into_iter()
        .fold_while((grids, None), |(grids, _), number| {
            let (remaining_grids, valid_grids): (Vec<Grid>, Vec<Grid>) = grids
                .iter()
                .map(|grid| update_grid(grid, number))
                .partition_map(|grid| match is_grid_valid(&grid) {
                    true => Either::Right(grid),
                    false => Either::Left(grid),
                });
            match remaining_grids.is_empty() {
                true => FoldWhile::Done((
                    remaining_grids,
                    valid_grids.last().map(|g| (g.clone(), number)),
                )),
                false => FoldWhile::Continue((remaining_grids, None)),
            }
        })
        .into_inner();
    match result {
        None => Err(format!("Can't compute a winning board, that's weird")),
        Some((grid, number)) => Ok(grid
            .rows
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .filter(|(_, x)| !*x)
                    .map(|(v, _)| v)
                    .sum::<u32>()
            })
            .sum::<u32>()
            * number),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_grid_valid() {
        assert_eq!(
            is_grid_valid(&Grid {
                rows: vec![
                    vec![(4, true), (5, false), (6, false)],
                    vec![(4, true), (5, false), (6, true)],
                    vec![(4, false), (5, true), (6, true)]
                ]
            }),
            false
        );
        assert_eq!(
            is_grid_valid(&Grid {
                rows: vec![
                    vec![(4, true), (5, true), (6, true)],
                    vec![(4, true), (5, false), (6, true)],
                    vec![(4, false), (5, true), (6, true)]
                ]
            }),
            true
        );
        assert_eq!(
            is_grid_valid(&Grid {
                rows: vec![
                    vec![(4, true), (5, false), (6, false)],
                    vec![(4, true), (5, false), (6, true)],
                    vec![(4, true), (5, false), (6, true)]
                ]
            }),
            true
        );
        assert_eq!(
            is_grid_valid(&Grid {
                rows: vec![
                    vec![(4, true), (5, false), (6, false)],
                    vec![(4, true), (5, true), (6, true)],
                    vec![(4, false), (5, false), (6, true)]
                ]
            }),
            true
        );
        assert_eq!(
            is_grid_valid(&Grid {
                rows: vec![
                    vec![(4, true), (5, false), (6, true)],
                    vec![(4, true), (5, true), (6, true)],
                    vec![(4, false), (5, false), (6, true)]
                ]
            }),
            true
        );
    }
}
