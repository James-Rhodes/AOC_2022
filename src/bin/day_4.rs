fn main() {
    let input_text = std::fs::read_to_string("./inputs/input_day_4.txt")
        .expect("The file could not be found or read");

    let split_text: Vec<&str> = input_text
        .split("\r\n")
        .flat_map(|line| line.split(|c| c == ',' || c == '-').collect::<Vec<&str>>())
        .collect();

    let mut sum: u32 = 0;
    println!("Number of numbers: {}", split_text.len());
    for i in (0..split_text.len()).step_by(4) {
        let a: u32 = split_text[i].parse::<u32>().unwrap();
        let b: u32 = split_text[i + 1].parse::<u32>().unwrap();
        let c: u32 = split_text[i + 2].parse::<u32>().unwrap();
        let d: u32 = split_text[i + 3].parse::<u32>().unwrap();

        sum += ranges_contain_each_other(a, b, c, d) as u32;
    }

    println!("Part One: {}", sum);

    let mut sum: u32 = 0;
    for i in (0..split_text.len()).step_by(4) {
        let a: u32 = split_text[i].parse::<u32>().unwrap();
        let b: u32 = split_text[i + 1].parse::<u32>().unwrap();
        let c: u32 = split_text[i + 2].parse::<u32>().unwrap();
        let d: u32 = split_text[i + 3].parse::<u32>().unwrap();

        sum += ranges_overlap(a, b, c, d) as u32;
    }
    println!("Part Two: {}", sum);
}

fn ranges_contain_each_other(a: u32, b: u32, c: u32, d: u32) -> bool {
    if a < c && d > b {
        return false;
    } else if c < a && b > d {
        return false;
    } else {
        return true;
    }
}

fn ranges_overlap(a: u32, b: u32, c: u32, d: u32) -> bool {
    if a <= c && b >= d && a <= d && b >= c {
        // a cd b
        return true;
    } else if c <= a && b <= d && b >= c && a <= d {
        // c ab d
        return true;
    } else if a <= c && b <= d && b >= c {
        // a c b d
        return true;
    } else if c <= a && d <= b && a <= d {
        // c a d b
        return true;
    }
    return false;
}
