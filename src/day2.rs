// Completed at 9:33ams Dec 3

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<String> {
    input
        .lines() // Iterate over each line
        .map(|l| l.trim().to_string()) // Remove trailing whitespace and make the String Ref into a String
        .collect() // Collect into a Vec of Strings
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[String]) -> u32 {
    let mut depth: u32 = 0; // Depth Counter
    let mut horizontal: u32 = 0; // Horizontal Counter

    for x in input { // Loop through each input
        let separated: Vec<&str> = x.split(' ').collect(); // Separate our prefix from the val

        let prefix = separated[0];
        let val = separated[1].parse::<u32>().unwrap();

        match prefix { // Match the prefix
            "forward" =>  horizontal += val, // Increase the horizontal by the val
            "up" => depth -= val, // Decrease the depth by the val
            "down" => depth += val, // Increase the depth by the val (down means you are going deeper)
            _ => println!("error parsing") // If the prefix doesn't fit into any of these branches
        }

    }

    depth * horizontal // Return the depth multiplied by the horizontal
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[String]) -> u32 {
    let mut depth: u32 = 0; // Depth counter
    let mut horizontal: u32 = 0; // Horizontal counter
    let mut aim: u32 = 0; // Aim counter

    for x in input { // Loop through each input
        let separated: Vec<&str> = x.split(' ').collect(); // Separate our prefix from the val

        let prefix = separated[0]; // Get our prefix from the separated stuff
        let val = separated[1].parse::<u32>().unwrap(); // Get our val from the separated stuff

        match prefix { // Match the prefix
            "forward" =>  { 
                horizontal += val; // Increase the horizontal by the val
                depth += aim * val; // Increase depth by the aim multiplied by the val
            },
            "up" => aim -= val, // Subtract the val from aim
            "down" => aim += val, // Increase aim by val
            _ => println!("error parsing") // Doesn't fit any of the match branches
        }

    }

    depth * horizontal // Return our result.
}
/* Console Output:
AOC 2021
Day 2 - Part 1 : 1451208
        generator: 66.25µs,
        runner: 180.125µs

Day 2 - Part 2 : 1620141160
        generator: 66.208µs,
        runner: 176.833µs */