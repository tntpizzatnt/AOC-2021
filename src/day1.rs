// Completed at 1:49. Comments completed at 2:00
// TO DO: Use iterators instead of for loops

use std::str::FromStr;

#[aoc_generator(day1)]
pub fn scanned_depths(input: &str) -> Vec<u64> {
    input
        .lines() // iterate through each line
        .map(|line| u64::from_str(line).unwrap()) // map each line to a number
        .collect::<Vec<_>>() // collect in a vec
}

#[aoc(day1, part1, for_loop)]
pub fn solve_part1_for_loop(input: &Vec<u64>) -> u64 {
    let mut increases = 0; // counter

    for (i, x) in input.iter().enumerate() { // loop through. use enumerate to have the index
        if i == input.len()-1 { // return if this is true to prevent out of bounds error
            break 
        }

        if input[i + 1] > *x { // this means that the next number is greater than the previous
            increases += 1; // increase counter
        }
    }

    increases // return our counter as solution
}

#[aoc(day1, part2, for_loop)]
pub fn solve_part2(input: &Vec<u64>) -> u64 {
    let mut increases = 0; // counter
    let mut condensed_vec = Vec::new(); // Init a new vec to hold our condensed inputs

    for (i, x) in input.iter().enumerate() { // Loop through. Enumerate so we can have the index
        if i == input.len()-2 { // ¯\_(ツ)_/¯ initially got it wrong because I used -3. honestly couldn't tell you why. Just went untill I got an out of bound error and then when one up to remove it
            break // leave this for loop
        }

        condensed_vec.insert(i, x + input[i + 1] + input[i + 2]) // insert our condensed val to the index that we are on
    }

    for (i, x) in condensed_vec.iter().enumerate() { // Same as up above in part 1 but using our new vec
        if i == condensed_vec.len()-1 { // Escape from out of bounds errors
            break 
        }

        if condensed_vec[i + 1] > *x { // determine if the next val is greater
            increases += 1; // increase counter
        }
    }

    increases // return counter
}