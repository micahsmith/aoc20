pub fn start(input: &str) {
    let course = TobogganCourse::from_input(input);

    let first_slope_count = count_trees_hit(course.clone(), 3, 1);
    println!("{}", first_slope_count);

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product = slopes
        .iter()
        .map(|(x, y)| count_trees_hit(course.clone(), *x as usize, *y as usize))
        .fold(0, |acc, x| {
            if acc == 0 {
                return x;
            }
            return acc * x;
        });
    println!("{}", product);
}

#[derive(Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
struct TobogganCourse {
    grid: Vec<Vec<char>>,
    position: Point,
}

impl TobogganCourse {
    fn from_input(input: &str) -> Self {
        return TobogganCourse {
            grid: input.lines().map(|l| l.chars().collect()).collect(),
            position: Point { x: 0, y: 0 },
        };
    }

    fn get_height(&self) -> usize {
        return self.grid.len();
    }

    fn get_width(&self) -> usize {
        return self.grid[0].len();
    }

    fn ride(&mut self, x: usize, y: usize) {
        self.position = Point {
            x: (self.position.x + x) % self.get_width(),
            y: self.position.y + y,
        }
    }

    fn is_tree(&self) -> bool {
        let row = &self.grid[self.position.y];
        let point = row[self.position.x];
        return point == '#';
    }
}

fn count_trees_hit(mut course: TobogganCourse, x_delta: usize, y_delta: usize) -> i64 {
    let height = course.get_height();
    let mut counter = 0;

    while course.position.y < height {
        if course.is_tree() {
            counter += 1;
        }
        course.ride(x_delta, y_delta);
    }

    return counter;
}
