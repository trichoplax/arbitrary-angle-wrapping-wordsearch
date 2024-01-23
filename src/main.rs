use gcd::Gcd;
use serde::{Serialize, Deserialize};
use serde_json::from_str;
use std::fs::{OpenOptions, read_to_string};

#[derive(Serialize, Deserialize, Clone)]
struct Grid<'a> {
    content: &'a str,
    width: usize,
}

impl<'a> Grid<'a> {
    fn new(content: &'a String, width: usize) -> Option<Self> {
        if content.len() > 0 && content.len() % width == 0 {
            Some(Self {
                content,
                width,
            })
        } else {
            None
        }
    }

    fn rows(&self, place_to_put_rows: &String) -> Vec<&[char]> {
        self.clone().content.chars().collect::<Vec<char>>().chunks(self.width).collect::<Vec<&[char]>>().cloned()
    }

    fn tile_right(&self, n: usize, place_to_put_new_content: &String) -> Option<Self> {
        Self::new(&self.rows().iter().map(|r| r.repeat(n)).flatten().map(|c| c.to_string()).collect::<Vec<String>>().join(""), self.width * n)
    }

    fn tile_down(&self, n: usize, place_to_put_new_content: &String) -> Option<Self> {
        Self::new(&self.content.repeat(n), self.width)
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Wordsearch<'a> {
    grid: Grid<'a>,
    word: &'a str,
    expected: bool,
}

impl<'a> Wordsearch<'a> {
    fn new(grid: Grid<'a>, word: &'a str, expected: bool) -> Option<Self> {
        if word.len() >= 2 {
            Some(Self {
                grid,
                word,
                expected,
            })
        } else {
            None
        }
    }
}

fn main() {
    let json_string = read_to_string("./test_cases").unwrap();
    let test_cases: Vec<Wordsearch> = from_str(&json_string).unwrap();

    let output_file = OpenOptions::new().write(true).truncate(true).create(true).open("tiled_test_case_explanations");

    println!("[");

    for test in test_cases {
        let test_for_explanation = test.clone();
        let grid = test.grid;
        let content = grid.content;
        let width = grid.width;
        let length = content.len();
        let word = test.word;
        let expected = test.expected;
        let result = assess(content, width, word);

        if result {
            let tiled_explanation = highlighted_word_in_tiled_grids(&test_for_explanation);

        }

        println!("    [");
        println!("        [");

        for i in (0..length-width).step_by(width) {
            println!("            \"{}\",", &content[i..i+width]);
        }

        println!("            \"{}\"", &content[length-width..length]);
        println!("        ],");
        println!("        \"{}\",", word);
        println!("        \"{}\"", result);
        println!("    ],");

        if result != expected {
            println!("*******************************************************");
            println!("Expected {} but got {}", expected, result);
            println!("*******************************************************");
            panic!();
        }
    }

    println!("]");
}

fn assess(content: &str, width: usize, word: &str) -> bool {
    let length = content.len();

    for content_index in 0..length {
        if start_letter(content, width, word, content_index) {
            return true;
        }
    }

    false
}

fn start_letter(content: &str, width: usize, word: &str, content_index: usize) -> bool {
    let height = content.len() / width;
    let start_x = content_index % width;
    let start_y = content_index / width;

    for offset_y in 0..=height {
        for offset_x in 0..=width {
            if offset_valid(offset_x, offset_y, width, height) {
                let mut found = true;
                for word_index in 0..word.len() {
                    let x = (start_x + offset_x * word_index) % width;
                    let y = (start_y + offset_y * word_index) % height;
                    let index_to_check = x + y * width;
                    if content[index_to_check..=index_to_check] != word[word_index..=word_index] {
                        found = false;
                        break;
                    }
                }
                if found {
//                    let (best_x, best_y) = best_offsets(offset_x, offset_y);

//                    println!("{}, {}", best_x, best_y);
                    return true;
                }
            }
        }
    }

    false
}

fn offset_valid(x: usize, y: usize, width: usize, height: usize) -> bool {
    x + y == 1  // One is 0 and the other is 1
    || (  // One is 0 and the other is equivalent to -1
        (x == 0 && y > 0 && y == height - 1)
        || (x > 0 && x == width - 1 && y == 0)
    )
    || (  // Both non-zero and GCD is 1 for at least one combination of positive and negative equivalents
        x > 0 && y > 0
        && (
            x.gcd(y) == 1
            || (width - x).gcd(y) == 1
            || x.gcd(height - y) == 1
            || (width - x).gcd(height - y) == 1
        )
    )
}

fn highlighted_word_in_tiled_grids<'a>(test: &Wordsearch<'a>) -> Grid<'a> {
    Grid {
        content: "abcdefghi",
        width: 3,
    }
}

//fn best_offsets(x: usize, y: usize) -> (isize, isize) {
    // Find all offsets that work? May be infinitely many
    // Find smallest offsets that work? Could work up from 0,1 in a defined order until finding one that works
    // What to prefer?
    // Minimum smallest offset coordinate?
    // Minimum largest offset coordinate?
    // Minimum Euclidean step size?
    // Favour offsets with one zero?
    //
    // Whatever the order, work through all offsets in that order until a working one is found, and return it
    // Why doesn't the previous function check offsets in this order and make this function redundant?
    // Might be easier to leave the previous function as a simple check of a square grid, and then let this function go off on its on spiral search with the reassurance that there is something to be found, so it doesn't matter about restricting to any particular size or shape
//    (1, 1)
//}

//fn ordered_offsets() -> (isize, isize) {
    // Rewrite everything with iterators
    // Make a coordinates struct
    // Function that takes a width and height and returns an iterator of coordinates over that area
    // Don't worry about finding the perfect order to check offsets - just make an order and get it all working
    // This way it can easily be swapped out later for a better ordering
//}

//fn test_case_array(t: [((&'static str, usize), &'static str, &'static str)]) -> [((String, usize), String, String); 8] {

    // ADD TEST CASE FOR LARGE OFFSET THAT WRAPS THE BOARD MULTIPLE TIMES
    // OR ELSE PROVE THIS IS NEVER NECESSARY

//}
