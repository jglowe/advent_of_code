///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                        _            __  ___
//                                       | |          /_ |/ _ \
//                                     __| | __ _ _   _| | | | |
//                                    / _` |/ _` | | | | | | | |
//                                   | (_| | (_| | |_| | | |_| |
//                                    \__,_|\__,_|\__, |_|\___/
//                                                 __/ |
//                                                |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day10 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

fn get_signal_strength(input: &str) -> i64 {
    let mut signal_strength = 0;
    let mut strength_cycle = 20;
    let mut cycle = 1;
    let mut register = 1;
    let max_cycle = 220;

    for line in input.trim().split("\n") {
        let parts: Vec<&str> = line.split(" ").collect();

        if parts.len() == 1 && parts[0] == "noop" {
            cycle += 1;
            if cycle == strength_cycle {
                signal_strength += register * cycle;
                strength_cycle += 40;
            }

            if strength_cycle > max_cycle {
                return signal_strength;
            }
        } else if parts.len() == 2 && parts[0] == "addx" {
            let add: i64 = parts[1].parse().unwrap();

            cycle += 1;

            if cycle == strength_cycle {
                signal_strength += register * cycle;
                strength_cycle += 40;
            }
            if strength_cycle > max_cycle {
                return signal_strength;
            }

            cycle += 1;
            register += add;
            if cycle == strength_cycle {
                signal_strength += register * cycle;
                strength_cycle += 40;
            }
            if strength_cycle > 220 {
                return signal_strength;
            }
        } else {
            assert!(false, "Malformed line {}", line);
        }
    }

    signal_strength
}

fn format_pixel(output: String, cycle: &mut i64, register: i64) -> String {
    let mut screen = output.clone();

    if *cycle % 40 == register || *cycle % 40 == register - 1 || *cycle % 40 == register + 1 {
        screen = screen + "#";
    } else {
        screen = screen + ".";
    }
    *cycle += 1;

    if *cycle % 40 == 0 {
        screen = screen + "\n";
    }

    screen
}

fn produce_screen(input: &str) -> String {
    let mut cycle = 0;
    let mut register = 1;
    let mut output = String::new();

    for line in input.trim().split("\n") {
        let parts: Vec<&str> = line.split(" ").collect();

        if parts.len() == 1 && parts[0] == "noop" {
            output = format_pixel(output, &mut cycle, register);
            if cycle > 240 {
                break;
            }
        } else if parts.len() == 2 && parts[0] == "addx" {
            let add: i64 = parts[1].parse().unwrap();

            output = format_pixel(output, &mut cycle, register);
            if cycle > 240 {
                break;
            }

            output = format_pixel(output, &mut cycle, register);
            register += add;
            if cycle > 240 {
                break;
            }
        } else {
            assert!(false, "Malformed line {}", line);
        }
    }

    output
}

fn main() {
    let input = advent::read_file("data/day10_input.txt");

    println!("Signal Strength: {}", get_signal_strength(&input));
    println!("Screen : \n{}", produce_screen(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stage() {
        let input = "addx 15\n\
                     addx -11\n\
                     addx 6\n\
                     addx -3\n\
                     addx 5\n\
                     addx -1\n\
                     addx -8\n\
                     addx 13\n\
                     addx 4\n\
                     noop\n\
                     addx -1\n\
                     addx 5\n\
                     addx -1\n\
                     addx 5\n\
                     addx -1\n\
                     addx 5\n\
                     addx -1\n\
                     addx 5\n\
                     addx -1\n\
                     addx -35\n\
                     addx 1\n\
                     addx 24\n\
                     addx -19\n\
                     addx 1\n\
                     addx 16\n\
                     addx -11\n\
                     noop\n\
                     noop\n\
                     addx 21\n\
                     addx -15\n\
                     noop\n\
                     noop\n\
                     addx -3\n\
                     addx 9\n\
                     addx 1\n\
                     addx -3\n\
                     addx 8\n\
                     addx 1\n\
                     addx 5\n\
                     noop\n\
                     noop\n\
                     noop\n\
                     noop\n\
                     noop\n\
                     addx -36\n\
                     noop\n\
                     addx 1\n\
                     addx 7\n\
                     noop\n\
                     noop\n\
                     noop\n\
                     addx 2\n\
                     addx 6\n\
                     noop\n\
                     noop\n\
                     noop\n\
                     noop\n\
                     noop\n\
                     addx 1\n\
                     noop\n\
                     noop\n\
                     addx 7\n\
                     addx 1\n\
                     noop\n\
                     addx -13\n\
                     addx 13\n\
                     addx 7\n\
                     noop\n\
                     addx 1\n\
                     addx -33\n\
                     noop\n\
                     noop\n\
                     noop\n\
                     addx 2\n\
                     noop\n\
                     noop\n\
                     noop\n\
                     addx 8\n\
                     noop\n\
                     addx -1\n\
                     addx 2\n\
                     addx 1\n\
                     noop\n\
                     addx 17\n\
                     addx -9\n\
                     addx 1\n\
                     addx 1\n\
                     addx -3\n\
                     addx 11\n\
                     noop\n\
                     noop\n\
                     addx 1\n\
                     noop\n\
                     addx 1\n\
                     noop\n\
                     noop\n\
                     addx -13\n\
                     addx -19\n\
                     addx 1\n\
                     addx 3\n\
                     addx 26\n\
                     addx -30\n\
                     addx 12\n\
                     addx -1\n\
                     addx 3\n\
                     addx 1\n\
                     noop\n\
                     noop\n\
                     noop\n\
                     addx -9\n\
                     addx 18\n\
                     addx 1\n\
                     addx 2\n\
                     noop\n\
                     noop\n\
                     addx 9\n\
                     noop\n\
                     noop\n\
                     noop\n\
                     addx -1\n\
                     addx 2\n\
                     addx -37\n\
                     addx 1\n\
                     addx 3\n\
                     noop\n\
                     addx 15\n\
                     addx -21\n\
                     addx 22\n\
                     addx -6\n\
                     addx 1\n\
                     noop\n\
                     addx 2\n\
                     addx 1\n\
                     noop\n\
                     addx -10\n\
                     noop\n\
                     noop\n\
                     addx 20\n\
                     addx 1\n\
                     addx 2\n\
                     addx 2\n\
                     addx -6\n\
                     addx -11\n\
                     noop\n\
                     noop\n\
                     noop";

        assert_eq!(13140, get_signal_strength(&input));
        let output2 = "##..##..##..##..##..##..##..##..##..##..\n\
                       ###...###...###...###...###...###...###.\n\
                       ####....####....####....####....####....\n\
                       #####.....#####.....#####.....#####.....\n\
                       ######......######......######......####\n\
                       #######.......#######.......#######.....\n";

        assert_eq!(output2, produce_screen(&input));
    }
}
