use crate::file_reader::read_grid;

pub fn day4() {
    let grid = read_grid("input/day4_demo.txt");
    let search_word: Vec<char> = "XMAS".chars().collect();
    println!("{}", search_word.iter().collect::<String>());
    println!("{}", find_horizontal_forward(&grid, &search_word));
}

fn find_horizontal_forward(grid: &Vec<Vec<char>>, search_word: &Vec<char>) -> i32 {
    let mut finds = 0;
    for l in grid {
        let mut search_index = 0;
        for c in l {
            match c {
                _ if c == &search_word[search_index] && search_index >= search_word.len() - 1 => {
                    finds += 1;
                    search_index = 0
                }
                _ if c == &search_word[search_index] => search_index += 1,
                _ if c == &search_word[0] => search_index = 1,
                _ => search_index = 0,
            }
        }
    }
    finds
}
