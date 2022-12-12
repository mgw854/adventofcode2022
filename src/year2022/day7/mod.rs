use std::collections::HashMap;

use crate::{helpers::input::ProblemInput, problem::Problem};

use anyhow::Result;
use daggy::Dag;

pub struct Day7 {}

impl Default for Day7 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<usize, usize> for Day7 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        let mut stack: Vec<&str> = Vec::new();

        let mut files: HashMap<&str, Vec<File>> = HashMap::new();

        // First pass puts files in folders
        for l in input.value().lines() {
            if l.starts_with('$') {
                if l.starts_with("$ cd ") {
                    let dirname = &l[5..];

                    if dirname == ".." {
                        stack.pop();
                    } else {
                        stack.push(dirname);
                    }
                }
            } else {
                let parts = l.split(' ').collect::<Vec<&str>>();
                let parent = stack[stack.len() - 1];

                if parts[0] != "dir" {
                    files
                        .entry(parent)
                        .and_modify(|e| {
                            e.push(File {
                                name: parts[1].to_string(),
                                size: parts[0].parse::<usize>().unwrap(),
                            })
                        })
                        .or_insert(vec![File {
                            name: parts[1].to_string(),
                            size: parts[0].parse::<usize>().unwrap(),
                        }]);
                }
            }
        }

        stack.clear();

        // Second pass puts folders in folders
        let mut dag = Dag::<&str, usize>::new();
        let mut head = dag.add_node("/");

        for l in input.value().lines() {
            if l.starts_with('$') {
                if l.starts_with("$ cd ") {
                    let dirname = &l[5..];

                    if dirname == ".." {
                        dag.
                    } else {
                        stack.push(dirname);
                    }
                }
            } else {
                let parts = l.split(' ').collect::<Vec<&str>>();
                let parent = stack[stack.len() - 1];

                if parts[0] != "dir" {
                    files
                        .entry(parent)
                        .and_modify(|e| {
                            e.push(File {
                                name: parts[1].to_string(),
                                size: parts[0].parse::<usize>().unwrap(),
                            })
                        })
                        .or_insert(vec![File {
                            name: parts[1].to_string(),
                            size: parts[0].parse::<usize>().unwrap(),
                        }]);
                }
            }
        }

        for l in input.value().lines() {

            if l.starts_with('$') {
                if l.starts_with("$ cd ") {
                    let dirname = &l[5..];

                    if dirname == ".." {
                        stack.pop();
                    } else if dirname == "/" {
                        // Do nothing
                    } else {
                        let parent = stack[stack.len() - 1];

                        let child = dirs
                            .entry(&parent.name)
                            .or_default()
                            .iter()
                            .filter(|x| x.name == dirname)
                            .next()
                            .unwrap();

                        stack.push(child);
                    }
                }

                continue;
            }

            let parts = l.split(' ').collect::<Vec<&str>>();
            let parent = stack[stack.len() - 1];
            if parts[0] == "dir" {
                dirs.entry(parent)
                    .and_modify(|e| {
                        e.push(Directory {
                            name: parts[1].to_string(),
                        })
                    })
                    .or_insert(vec![Directory {
                        name: parts[1].to_string(),
                    }]);
            } else {
                files
                    .entry(parent)
                    .and_modify(|e| {
                        e.push(File {
                            name: parts[1].to_string(),
                            size: parts[0].parse::<usize>().unwrap(),
                        })
                    })
                    .or_insert(vec![File {
                        name: parts[1].to_string(),
                        size: parts[0].parse::<usize>().unwrap(),
                    }]);
            }
        }

        Ok(0)
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        todo!()
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Directory {
    name: String,
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct File {
    name: String,
    size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> ProblemInput {
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

        crate::helpers::input::ProblemInput::from_sample(input)
    }

    #[test]
    fn sample1() {
        assert_eq!(0, Day7::default().part1(&sample()).unwrap())
    }

    #[test]
    fn sample2() {
        assert_eq!(0, Day7::default().part2(&sample()).unwrap())
    }
}
