use crate::{helpers::input::ProblemInput, problem::Problem};

use anyhow::Result;

pub struct Day7 {}

impl Default for Day7 {
    fn default() -> Self {
        Self {}
    }
}

impl Problem<usize, usize> for Day7 {
    fn part1(&self, input: &ProblemInput) -> Result<usize> {
        let mut stack: Vec<&FsObject> = Vec::new();
        let root = FsObject::Directory {
            name: "/".to_string(),
            children: Vec::new(),
        };

        let mut dirs : HashMap<&FsObject, Vec<FsObject>> = HashMap::new();

        stack.push(root);

        for l in input.value().lines() {
            if l.starts_with('$') {
                if l.starts_with("$ cd ") {
                    let dirname = &l[5..];

                    if dirname == ".." {
                        stack.pop();
                    } else if dirname == "/" {
                        // Do nothing
                    } else {
                        let head = stack.last().unwrap();

                        let child = dirs.get(head).unwrap().iter().filter(|x| match x { FsObject::Directory { name: name, .. } => name == dirname, _ => false }).next().unwrap();

                        match stack.last().unwrap() {
                            FsObject::Directory => {
                                stack.push(child);
                            }
                            _ => {}
                        }
                    }
                }

                continue;
            }

            let parts = l.split(' ').collect::<Vec<&str>>();

            if parts[0] == "dir" {
                match stack[0] {
                    FsObject::Directory { children: x, .. } => x.push(FsObject::Directory {
                        name: parts[1].to_string(),
                        children: Vec::new(),
                    }),
                    _ => {}
                }
            } else {
                match stack[0] {
                    FsObject::Directory { children: x, .. } => x.push(FsObject::File {
                        name: parts[1].to_string(),
                        size: parts[0].parse::<usize>().unwrap(),
                    }),
                    _ => {}
                }
            }
        }

        Ok(0)
    }

    fn part2(&self, input: &ProblemInput) -> Result<usize> {
        todo!()
    }
}

pub enum FsObject {
    Directory {
        name: String,
        children: Vec<FsObject>,
    },
    File {
        name: String,
        size: usize,
    },
}

impl FsObject {
    fn size(&self) -> usize {
        match self {
            FsObject::Directory { children: x, .. } => x.iter().map(|e| e.size()).sum(),
            FsObject::File { size: size, .. } => *size,
        }
    }
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
