pub fn start(input: &str) {
    let processed = process_input(input);
    let (one_valid, two_valid) = count_valid_passwords(&processed);
    println!("{}", one_valid);
    println!("{}", two_valid);
}

#[derive(Debug)]
struct Policy {
    character: char,
    large: i32,
    small: i32,
}

impl Policy {
    fn from_line(input: &str) -> Self {
        let v: Vec<&str> = input.split(|c| c == '-' || c == ' ').collect();

        return Policy {
            character: v[2].chars().nth(0).unwrap(),
            large: v[1].parse::<i32>().unwrap(),
            small: v[0].parse::<i32>().unwrap(),
        };
    }

    fn is_one_valid(&self, password: &str) -> bool {
        let mut counter = 0;
        for c in password.chars() {
            if c == self.character {
                counter += 1;
            }
        }

        if counter <= self.large && counter >= self.small {
            return true;
        }

        return false;
    }

    fn is_two_valid(&self, password: &str) -> bool {
        let bytes = password.as_bytes();

        let small = self.small as usize - 1;
        let in_small_index = bytes[small] as char == self.character;
        let large = self.large as usize - 1;
        let in_large_index = bytes[large] as char == self.character;

        if (in_small_index || in_large_index) && !(in_small_index && in_large_index) {
            return true;
        }

        return false;
    }
}

fn process_input(input: &str) -> Vec<(Policy, &str)> {
    let mut vec: Vec<(Policy, &str)> = Vec::new();
    
    for line in input.lines() {
        let mut split = line.split(':');
        let policy = Policy::from_line(split.nth(0).unwrap());
        let password = split.nth(0).unwrap().trim();
        vec.push((policy, password));
    }

    return vec;
}

fn count_valid_passwords(policy_vec: &Vec<(Policy, &str)>) -> (i32, i32) {
    let mut one_valid = 0;
    let mut two_valid = 0;

    for (k, v) in policy_vec.iter() {
        if k.is_one_valid(v) {
            one_valid += 1;
        }
        if k.is_two_valid(v) {
            two_valid += 1;
        }
    }

    return (one_valid, two_valid);
}
