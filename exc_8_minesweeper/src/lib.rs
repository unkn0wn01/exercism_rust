#![allow(unused)]
#![allow(non_snake_case)]
use std::collections::HashSet;

pub struct Minesweeper {
    Mine: HashSet<(i32, i32)>,
    line: Vec<String>,
}

impl Minesweeper {
    fn new() -> Self {
        Minesweeper {
            Mine: HashSet::new(),
            line: Vec::new(),
        }
    }
    fn push(&mut self, points: (i32, i32)) {
        self.Mine.insert(points);
    }
    fn push_in_vec(&mut self, value: &str) {
        self.line.push(value.to_string());
    }
    fn check_contains(&self, points: &(i32, i32)) -> bool {
        self.Mine.contains(points)
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut mine = Minesweeper::new();
    let mut row = 0;
    for line in minefield {
        let mut index = 0;
        for char in line.chars() {
            match char {
                '*' => mine.push((row, index)),
                _ => (),
            }
            index += 1;
        }
        row += 1;
    }
    let mut push_string = String::new();
    let mut row = 0;
    for line in minefield {
        let mut index = 0;
        for char in line.chars() {
            let mut count = 0;
            if char == ' ' {
                for r in row - 1..=row + 1 {
                    for i in index - 1..=index + 1 {
                        if mine.check_contains(&(r, i)) {
                            count += 1;
                        }
                    }
                }

                push_string.push_str(&count.to_string())
            } else {
                push_string.push('*');
            }
            index += 1;
        }
        mine.push_in_vec(&push_string);
        push_string = String::new();
        row += 1;
    }

    mine.line
}
