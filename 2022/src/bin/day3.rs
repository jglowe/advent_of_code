///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                               _             ____
//                              | |           |___ \
//                            __| | __ _ _   _  __) |
//                           / _` |/ _` | | | ||__ <
//                          | (_| | (_| | |_| |___) |
//                           \__,_|\__,_|\__, |____/
//                                        __/ |
//                                       |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day3 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

fn convert_letter_to_priority(letter: char) -> i64 {
    let ascii: i64 = letter as i64;

    if ascii >= 97 && ascii <= 122 {
        return ascii - 96;
    } else if ascii >= 65 && ascii <= 90 {
        return ascii - 65 + 27;
    } else {
        panic!("Invalid char {}", letter);
    }
}

fn common_item(backpack1: &str, backpack2: &str) -> char {
    for item in backpack1.chars() {
        if backpack2.contains(item) {
            return item;
        }
    }

    panic!(
        "No common item in the backpacks {} {}",
        backpack1, backpack2
    );
}

fn common_items(backpack1: &str, backpack2: &str) -> Vec<char> {
    let mut common_items = Vec::new();

    for item in backpack1.chars() {
        if backpack2.contains(item) {
            common_items.push(item);
        }
    }

    common_items
}

fn get_line_priority(line: &str) -> i64 {
    let len = line.len();

    assert!(
        len % 2 == 0,
        "The packs should contain an even number of items"
    );

    let backpack1 = &line[0..len / 2];
    let backpack2 = &line[len / 2..len];

    assert!(
        backpack1.len() == backpack2.len(),
        "Backpacks must have the same size"
    );

    let common_item = common_item(backpack1, backpack2);

    convert_letter_to_priority(common_item)
}

fn get_priority_sum(input: &String) -> i64 {
    input.trim().split("\n").map(get_line_priority).sum()
}

fn get_badge_priority_sum(input: &String) -> i64 {
    let mut party = Vec::new();
    let mut sum = 0;

    for line in input.trim().split("\n") {
        if party.len() < 2 {
            party.push(line);
            continue;
        }

        let first = party.pop().unwrap();
        let second = party.pop().unwrap();
        let third = line;

        let items: String = common_items(first, second).into_iter().collect();

        let letter = common_item(&items, third);
        sum = sum + convert_letter_to_priority(letter);
    }

    assert!(party.len() == 0, "A party doesn't have 3 people in it");

    sum
}

fn main() {
    let input = advent::read_file("data/day3_input.txt");

    println!("Priority sum {}", get_priority_sum(&input));
    println!("Badge priority sum {}", get_badge_priority_sum(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priorities() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
                     jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
                     PmmdzqPrVvPwwTWBwg\n\
                     wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
                     ttgJtRGJQctTZtZT\n\
                     CrZsJsPPZsGzwwsLwLmpwMDw"
            .to_string();

        assert_eq!(157, get_priority_sum(&input));
        assert_eq!(70, get_badge_priority_sum(&input));
    }
}
