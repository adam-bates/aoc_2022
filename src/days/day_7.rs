use crate::{result::*, utils};

use std::{cell::RefCell, collections::HashMap, rc::Rc};

// https://adventofcode.com/2022/day/7

const INPUT_FILE: &'static str = "input_7.txt";

struct Dir {
    pub name: String,
    pub items: HashMap<String, Item>,
    pub size: Option<usize>,
    pub parent_dir: Option<Rc<RefCell<Dir>>>,
}

struct File {
    pub name: String,
    pub size: usize,
    pub parent_dir: Rc<RefCell<Dir>>,
}

enum Item {
    Dir(Rc<RefCell<Dir>>),
    File(File),
}

struct DirTree {
    pub root: Rc<RefCell<Dir>>,
    pub current: Rc<RefCell<Dir>>,
}

fn build_dir_tree(input: &str) -> DirTree {
    let root = Rc::new(RefCell::new(Dir {
        name: "/".to_string(),
        items: HashMap::new(),
        size: None,
        parent_dir: None,
    }));

    let mut current = Rc::clone(&root);

    let mut is_listing = false;

    for line in input.lines() {
        match line {
            "$ cd /" => {
                is_listing = false;
                current = Rc::clone(&root);
            }
            "$ cd .." => {
                is_listing = false;
                if let Some(parent) = &Rc::clone(&current).borrow().parent_dir {
                    current = Rc::clone(&parent);
                }
            }
            _ if line.starts_with("$ cd ") => {
                is_listing = false;
                let dir_name = line[5..].to_string();

                if let Some(item) = &Rc::clone(&current).borrow().items.get(&dir_name) {
                    match item {
                        Item::Dir(item) => {
                            current = Rc::clone(item);
                        }
                        _ => panic!("Cannot cd into a file"),
                    }
                } else {
                    let new_dir = Rc::new(RefCell::new(Dir {
                        name: dir_name.clone(),
                        items: HashMap::new(),
                        size: None,
                        parent_dir: Some(Rc::clone(&current)),
                    }));

                    current
                        .borrow_mut()
                        .items
                        .insert(dir_name, Item::Dir(Rc::clone(&new_dir)));

                    current = new_dir;
                }
            }
            "$ ls" => {
                is_listing = true;
            }
            _ if is_listing => {
                let (size, name) = line.split_once(" ").unwrap();

                if !current.borrow().items.contains_key(name) {
                    match size {
                        "dir" => {
                            current.borrow_mut().items.insert(
                                name.to_string(),
                                Item::Dir(Rc::new(RefCell::new(Dir {
                                    name: name.to_string(),
                                    items: HashMap::new(),
                                    size: None,
                                    parent_dir: Some(Rc::clone(&current)),
                                }))),
                            );
                        }
                        _ => {
                            let size = size.parse::<usize>().unwrap();

                            current.borrow_mut().items.insert(
                                name.to_string(),
                                Item::File(File {
                                    name: name.to_string(),
                                    size,
                                    parent_dir: Rc::clone(&current),
                                }),
                            );
                        }
                    }
                }
            }
            _ => panic!("Unexpected line: {:?}", line),
        }
    }

    return DirTree { root, current };
}

fn fill_dir_sizes(dir: Rc<RefCell<Dir>>) -> usize {
    let mut sum = 0;

    for item in dir.borrow().items.values() {
        match item {
            Item::File(file) => sum += file.size,
            Item::Dir(dir) => sum += fill_dir_sizes(Rc::clone(dir)),
        }
    }

    dir.borrow_mut().size = Some(sum);

    return sum;
}

fn flatten_dirs(dir: Rc<RefCell<Dir>>) -> Vec<Rc<RefCell<Dir>>> {
    let mut flat = vec![Rc::clone(&dir)];

    for item in dir.borrow().items.values() {
        if let Item::Dir(child_dir) = item {
            let inner = flatten_dirs(Rc::clone(child_dir));

            flat.extend(inner);
        }
    }

    return flat;
}

pub fn part_1() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let DirTree { root, .. } = build_dir_tree(&input);

    fill_dir_sizes(Rc::clone(&root));

    let sum: usize = flatten_dirs(root)
        .into_iter()
        .map(|dir| dir.borrow().size.unwrap())
        .filter(|size| *size <= 100_000)
        .sum();

    return Ok(sum.to_string());
}

pub fn part_2() -> Result<String> {
    let input = utils::read_input_file(INPUT_FILE)?;

    let DirTree { root, .. } = build_dir_tree(&input);

    fill_dir_sizes(Rc::clone(&root));

    const SYSTEM_SIZE: usize = 70_000_000;
    const UPDATE_SIZE: usize = 30_000_000;

    let unused_space = SYSTEM_SIZE - root.borrow().size.unwrap();
    let required_space = UPDATE_SIZE - unused_space;

    let min: usize = flatten_dirs(root)
        .into_iter()
        .map(|dir| dir.borrow().size.unwrap())
        .filter(|size| *size >= required_space)
        .min()
        .unwrap();

    return Ok(min.to_string());
}
