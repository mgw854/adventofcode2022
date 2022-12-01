use std::{fs::File, io::Read, path::Path};

use anyhow::Result;

pub struct ProblemInput {
    day: u8,
    value: String,
}

pub fn get_input(day: u8) -> Result<ProblemInput> {
    let path = format!(".\\src\\day{}\\input.txt", day);
    let path = Path::new(&path);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    
    Ok(ProblemInput {
        day,
        value: contents.replace("\r", ""),
    })
}

#[cfg(test)]
pub fn from_sample(sample: &str) -> ProblemInput {
    ProblemInput { day: 0, value: sample.replace("\r", "") }
}

#[allow(dead_code)]
impl ProblemInput {
    pub fn parse_lines<T>(&self, parser: fn(&str) -> Result<T>) -> Result<Vec<T>> {
        self.value.lines().map(|s| parser(s.trim())).collect()
    }

    pub fn parse_numbers(&self) -> Result<Vec<usize>> {
        self.parse_lines(|x| x.parse::<usize>().map_err(|e| e.into()))
    }

    pub fn get_chars(&self) -> Vec<char> {
        self.value.trim().chars().collect()
    }

    pub fn parse_sections<T>(&self, parser: fn(&str) -> Result<T>) -> Result<Vec<Vec<T>>> {
        self.value
            .split("\n\n")
            .map(|s| s.lines().map(|x| parser(x.trim())).collect())
            .collect()
    }

    pub fn parse_csv<T>(&self, parser: fn(&str) -> Result<T>) -> Result<Vec<T>> {
        self.value.split(",").map(|s| parser(s.trim())).collect()
    }

    pub fn parse_csv_per_line<T>(&self, parser: fn(&str) -> Result<T>) -> Result<Vec<Vec<T>>> {
        self.value
            .lines()
            .map(|s| s.trim().split(",").map(|x| parser(x.trim())).collect())
            .collect()
    }

    pub fn day(&self) -> u8 {
        self.day
    }
}
