use regex::Regex;

pub fn extract_lines(grid: &Vec<Vec<char>>) -> Vec<String> {
    let mut all: Vec<String> = Vec::new();
    let row_count = grid.len();
    let column_count = grid[0].len();

    let mut horizontal: Vec<String> = Vec::new();
    for row in grid {
        horizontal.push(row.iter().collect());
    }
    all.extend(horizontal);

    let mut vertical: Vec<String> = Vec::new();
    for column_index in 0..column_count {
        let mut string = String::new();
        for row in grid {
            string.push(row[column_index]);
        }
        vertical.push(string);
    }
    all.extend(vertical);

    let mut diagonal: Vec<String> = Vec::new();

    for start_column in 0..column_count {
        let mut string = String::new();
        let mut x = start_column;
        let mut y = 0;

        while x < column_count && y < row_count {
            string.push(grid[y][x]);
            x += 1;
            y += 1;
        }
        diagonal.push(string);
    }

    for start_row in 1..row_count {
        let mut string = String::new();
        let mut x = 0;
        let mut y = start_row;

        while x < column_count && y < column_count {
            string.push(grid[y][x]);
            x += 1;
            y += 1;
        }
        diagonal.push(string);
    }

    for start_column in 0..column_count {
        let mut string = String::new();
        let mut x = start_column;
        let mut y: i32 = (row_count - 1).try_into().unwrap(); // last iteration will be -1, so we can't have usize

        while x < column_count && y >= 0 {
            string.push(grid[y as usize][x]);
            x += 1;
            y -= 1;
        }
        diagonal.push(string);
    }

    for start_row in (0..row_count - 1).rev() {
        let mut string = String::new();
        let mut x = 0;
        let mut y: i32 = start_row.try_into().unwrap();

        while x < column_count && y >= 0 {
            string.push(grid[y as usize][x]); // last iteration will be -1, so we can't have usize
            x += 1;
            y -= 1;
        }
        diagonal.push(string);
    }

    all.extend(diagonal);

    all
}

pub fn count_xmas(line: &str) -> i32 {
    if line.len() < 4 {
        return 0;
    }

    let re = Regex::new(r"XMAS").unwrap();
    let forward_count = re.captures_iter(line).count();

    let rev_line: String = line.chars().rev().collect();
    let reverse_count = re.captures_iter(&rev_line).count();

    (forward_count + reverse_count).try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_extract_lines() {
        let grid = vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ];

        let extracted_strings = extract_lines(&grid);
        // horizontals
        assert!(extracted_strings.contains(&String::from("abc")));
        assert!(extracted_strings.contains(&String::from("def")));
        assert!(extracted_strings.contains(&String::from("ghi")));
        // verticals
        assert!(extracted_strings.contains(&String::from("adg")));
        assert!(extracted_strings.contains(&String::from("beh")));
        assert!(extracted_strings.contains(&String::from("cfi")));
        // diagonals
        assert!(extracted_strings.contains(&String::from("aei")));
        assert!(extracted_strings.contains(&String::from("bf")));
        assert!(extracted_strings.contains(&String::from("c")));
        assert!(extracted_strings.contains(&String::from("dh")));
        assert!(extracted_strings.contains(&String::from("g")));
        assert!(extracted_strings.contains(&String::from("gec")));
        assert!(extracted_strings.contains(&String::from("hf")));
        assert!(extracted_strings.contains(&String::from("i")));
        assert!(extracted_strings.contains(&String::from("db")));
        assert!(extracted_strings.contains(&String::from("a")));
    }

    #[test]
    fn can_count_xmas() {
        assert_eq!(2, count_xmas(&String::from("XMASAMX.MM")));
        assert_eq!(0, count_xmas(&String::from("XMA")));
        assert_eq!(3, count_xmas(&String::from("XMASAMX.XMASM")));
    }
}
