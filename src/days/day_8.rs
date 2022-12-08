use crate::{result::*, utils};

// https://adventofcode.com/2022/day/8

const INPUT_FILE: &'static str = "input_8.txt";

fn parse_trees(input: &str) -> Vec<Vec<(usize, bool)>> {
    let mut trees = Vec::new();
    let mut row = Vec::new();

    for c in input.chars() {
        if c == '\n' {
            trees.push(row);
            row = Vec::new();
        } else {
            const OFFSET: usize = '0' as usize;

            let height = c as usize - OFFSET;

            row.push((height, false));
        }
    }

    return trees;
}

pub fn part_1() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let mut trees = parse_trees(&input);

    let rows = trees.len();
    let cols = trees[0].len();

    let mut top_to_bottom = |reverse: bool| {
        let mut iter: Box<dyn DoubleEndedIterator<Item = usize>> = Box::new(1..rows - 1);

        if reverse {
            iter = Box::new(iter.rev());
        }

        for row_idx in iter {
            let row = &mut trees[row_idx];
            let mut row_max = row[if reverse { row.len() - 1 } else { 0 }].0;

            let mut iter: Box<dyn DoubleEndedIterator<Item = usize>> = Box::new(1..cols - 1);

            if reverse {
                iter = Box::new(iter.rev());
            }

            for col_idx in iter {
                let (height, visible) = &mut row[col_idx];

                if *height > row_max {
                    *visible = true;
                    row_max = *height;
                }
            }
        }
    };
    top_to_bottom(false);
    top_to_bottom(true);

    let mut left_to_right = |reverse: bool| {
        let mut iter: Box<dyn DoubleEndedIterator<Item = usize>> = Box::new(1..cols - 1);

        if reverse {
            iter = Box::new(iter.rev());
        }

        for col_idx in iter {
            let mut col_max = trees[if reverse { trees.len() - 1 } else { 0 }][col_idx].0;

            let mut iter: Box<dyn DoubleEndedIterator<Item = usize>> = Box::new(1..rows - 1);

            if reverse {
                iter = Box::new(iter.rev());
            }

            for row_idx in iter {
                let row = &mut trees[row_idx];
                let (height, visible) = &mut row[col_idx];

                if *height > col_max {
                    *visible = true;
                    col_max = *height;
                }
            }
        }
    };
    left_to_right(false);
    left_to_right(true);

    let mut visible = trees
        .into_iter()
        .flat_map(|row| row)
        .filter(|(_, visible)| *visible)
        .count();

    visible += (cols * 2) + ((rows - 2) * 2); // outer edges

    return Ok(visible.to_string());
}

fn calc_scenic_score(trees: &Vec<Vec<(usize, bool)>>, row_idx: usize, col_idx: usize) -> usize {
    let src_height = trees[row_idx][col_idx].0;

    let up_score = {
        let mut score = 0;
        for i in (0..=row_idx - 1).rev() {
            let (height, _) = trees[i][col_idx];

            score += 1;

            if height >= src_height {
                break;
            }
        }
        score
    };

    let down_score = {
        let mut score = 0;
        for i in row_idx + 1..trees.len() {
            let (height, _) = trees[i][col_idx];

            score += 1;

            if height >= src_height {
                break;
            }
        }
        score
    };

    let left_score = {
        let mut score = 0;
        for j in (0..=col_idx - 1).rev() {
            let (height, _) = trees[row_idx][j];

            score += 1;

            if height >= src_height {
                break;
            }
        }
        score
    };

    let right_score = {
        let mut score = 0;
        for j in col_idx + 1..trees[row_idx].len() {
            let (height, _) = trees[row_idx][j];

            score += 1;

            if height >= src_height {
                break;
            }
        }
        score
    };
    
    return up_score * down_score * left_score * right_score;
}

pub fn part_2() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let trees = parse_trees(&input);

    let rows = trees.len();
    let cols = trees[0].len();

    let mut max_score = 0;

    for row_idx in 1..rows - 1 {
        for col_idx in 1..cols - 1 {
            let score = calc_scenic_score(&trees, row_idx, col_idx);

            if score > max_score {
                max_score = score;
            }
        }
    }

    return Ok(max_score.to_string());
}
