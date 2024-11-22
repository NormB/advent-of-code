advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines() {
        // take line in the format of lxwxh and split it into a vector of u32
        let numbers: Vec<u32> = line
            .split('x')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let s1 = numbers[0] * numbers[1];
        let s2 = numbers[1] * numbers[2];
        let s3 = numbers[2] * numbers[0];

        // find the minimum of the three sides
        let min_side = s1.min(s2).min(s3);

        // calculate the total area of the box
        total += 2 * s1 + 2 * s2 + 2 * s3 + min_side;
    }
    println!("Total: {}", total);
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    for line in input.lines() {
        // take line in the format of lxwxh and split it into a vector of u32
        let mut numbers: Vec<u32> = line
            .split('x')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        // sort the numbers without having to clone the vector
        numbers.sort();

        // sort the numbers so that the smallest two are first
        //let mut sorted = numbers.clone();
        //sorted.sort();

        // calculate the volume of the box
        let volume = numbers.iter().product::<u32>();

        // calculate the total length of ribbon needed
        total += 2 * numbers[0] + 2 * numbers[1] + volume;
    }
    println!("Total: {}", total);
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(58));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
