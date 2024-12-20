use std::collections::HashSet;

#[derive(Debug)]
struct Trailhead {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Map {
    map: Vec<Vec<u32>>,
    x_max: usize,
    y_max: usize,
    trailheads: Vec<Trailhead>,
}

impl Map {
    pub fn new(input: &String) -> Map {
        let mut map: Vec<Vec<u32>> = Vec::new();
        let mut trailheads: Vec<Trailhead> = Vec::new();

        for (y, line) in input.lines().enumerate() {
            let row: Vec<u32> = line
                .chars()
                .enumerate()
                .map(|(x, c)| {
                    let height = c.to_digit(10).unwrap();
                    if height == 0 {
                        trailheads.push(Trailhead { x, y });
                    }

                    height
                })
                .collect();
            map.push(row);
        }
        let x_max = map[0].len() - 1;
        let y_max = map.len() - 1;
        Map {
            map,
            x_max,
            y_max,
            trailheads,
        }
    }

    pub fn score(&self) -> u32 {
        let trails = self.hike();
        let mut score = 0;
        for trail in trails {
            let unique_peaks: HashSet<(usize, usize)> = trail.into_iter().collect();
            score += unique_peaks.len();
        }
        score as u32
    }

    pub fn rating(&self) -> u32 {
        let trails = self.hike();
        trails.into_iter().map(|t| t.len() as u32).sum()
    }

    fn hike(&self) -> Vec<Vec<(usize, usize)>> {
        let mut trails: Vec<Vec<(usize, usize)>> = Vec::new();

        for trailhead in &self.trailheads {
            let mut found: Vec<(usize, usize)> = Vec::new();
            self.wander(0, trailhead.x, trailhead.y, &mut found);
            trails.push(found);
        }
        trails
    }

    fn wander(
        &self,
        current_step: u32,
        current_x: usize,
        current_y: usize,
        found: &mut Vec<(usize, usize)>,
    ) {
        // if we're at 9, we're done.
        if current_step == 9 {
            found.push((current_x, current_y));
            return;
        }

        // check all neighbours for current_step +1
        if current_y > 0 && self.map[current_y - 1][current_x] == current_step + 1 {
            self.wander(current_step + 1, current_x, current_y - 1, found);
        }

        if current_x < self.x_max && self.map[current_y][current_x + 1] == current_step + 1 {
            self.wander(current_step + 1, current_x + 1, current_y, found);
        }

        if current_y < self.y_max && self.map[current_y + 1][current_x] == current_step + 1 {
            self.wander(current_step + 1, current_x, current_y + 1, found);
        }

        if current_x > 0 && self.map[current_y][current_x - 1] == current_step + 1 {
            self.wander(current_step + 1, current_x - 1, current_y, found);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_new_map() {
        let input = "9870456
8761328
4562987";
        let map = Map::new(&input.to_string());
        assert_eq!(1, map.trailheads.len());
        assert_eq!(0, map.trailheads[0].y);
        assert_eq!(3, map.trailheads[0].x);
    }

    #[test]
    fn can_hike() {
        let input = "9990999
9991999
9992999
6543456
7999997
8111118
9222229
";
        let map = Map::new(&input.to_string());
        assert_eq!(2, map.score());
    }

    #[test]
    fn can_hike_more() {
        let input = "5590559
5551598
5592557
6543456
7651987
8761111
9871111
";
        let map = Map::new(&input.to_string());
        println!("{:?}", map);
        assert_eq!(4, map.score());
    }

    #[test]
    fn can_hike_even_more() {
        let input = "1022922
2555855
3555711
4567654
1118553
1119442
5555501
";
        let map = Map::new(&input.to_string());
        println!("{:?}", map);
        assert_eq!(2, map.trailheads.len());
        assert_eq!(3, map.score());
    }

    #[test]
    fn can_get_rating() {
        let input = "8888808
8843215
8858827
8865437
1171147
1187657
1191111
";
        let map = Map::new(&input.to_string());
        println!("{:?}", map);
        assert_eq!(1, map.trailheads.len());
        assert_eq!(1, map.score());
        assert_eq!(3, map.rating());
    }

    #[test]
    fn can_get_more_rating() {
        let input = "012345
123456
234567
345678
426789
567892
";
        let map = Map::new(&input.to_string());
        println!("{:?}", map);
        assert_eq!(1, map.trailheads.len());
        assert_eq!(227, map.rating());
    }
}
