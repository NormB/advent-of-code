advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let mut _floor:i32  = 0;
    for line in input.lines() {

        //let first_50_chars = &line[0..50.min(line.len())];
        //println!("First 50 characters of input\n{}", first_50_chars);

        //let first_50_bytes = &line.as_bytes()[0..50.min(line.len())];
        //println!("First 50 bytes of input\n{}", first_50_bytes.iter().map(|b| *b as char).collect::<String>());

        // iterate over the bytes
        for byte in &line.as_bytes()[0..line.len()] {
            match byte {
                b'(' => _floor += 1,
                b')' => _floor -= 1,
                _ => (),
            }
        }
    }
    Some(_floor)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut floor:i32  = 0;
    let mut position:u32 = 0;

    for line in input.lines() {
        // iterate over the bytes
        for byte in &line.as_bytes()[0..line.len()] {
            position += 1;
            match byte {
                b'(' => floor += 1,
                b')' => floor -= 1,
                _ => (),
            }
            if floor < 0 {
                // println!("Position: {}", _position);
                break;
            }
        }
    }
    Some(position)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
