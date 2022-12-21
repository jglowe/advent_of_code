///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                        _            __ ____
//                                       | |          /_ |___ \
//                                     __| | __ _ _   _| | __) |
//                                    / _` |/ _` | | | | ||__ <
//                                   | (_| | (_| | |_| | |___) |
//                                    \__,_|\__,_|\__, |_|____/
//                                                 __/ |
//                                                |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day13 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, PartialEq, Eq)]
enum PacketPart {
    Integer(i64),
    List(Vec<PacketPart>),
}

fn parse_packet(input: &str) -> PacketPart {
    let chars: Vec<char> = input.chars().collect();
    let mut stack: Vec<PacketPart> = Vec::new();
    let mut number = String::new();

    for i in 0..chars.len() {
        match chars[i] {
            '[' => {
                let part = PacketPart::List(Vec::new());
                stack.push(part);
            }

            ']' => {
                if number != String::new() {
                    let n: i64 = number.parse().unwrap();

                    let last_index = stack.len() - 1;
                    match &mut stack[last_index] {
                        PacketPart::List(x) => x.push(PacketPart::Integer(n)),
                        PacketPart::Integer(_) => panic!("Invalid last packet part"),
                    }
                    number = String::new();
                }
                let finished_list = stack.pop().unwrap();

                if stack.len() == 0 {
                    return finished_list;
                } else {
                    let last_index = stack.len() - 1;
                    match &mut stack[last_index] {
                        PacketPart::List(x) => x.push(finished_list),
                        PacketPart::Integer(_) => panic!("Invalid last packet part"),
                    }
                }
            }
            ',' => {
                if number != String::new() {
                    let n: i64 = number.parse().unwrap();
                    let last_index = stack.len() - 1;
                    match &mut stack[last_index] {
                        PacketPart::List(x) => x.push(PacketPart::Integer(n)),
                        PacketPart::Integer(_) => panic!("Invalid last packet part"),
                    }
                    number = String::new();
                }
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                number += &chars[i].to_string();
            }
            _ => panic!("Invalid character: {}", chars[i]),
        }
    }

    panic!("Didn't finish arrays")
}

fn compare_packets(packet1: &PacketPart, packet2: &PacketPart) -> std::cmp::Ordering {
    match (packet1, packet2) {
        (PacketPart::Integer(x), PacketPart::Integer(y)) => {
            if x < y {
                std::cmp::Ordering::Greater
            } else if x == y {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Less
            }
        }
        (PacketPart::List(x), PacketPart::List(y)) => {
            if x.len() < y.len() {
                for i in 0..x.len() {
                    match compare_packets(&x[i], &y[i]) {
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                        std::cmp::Ordering::Equal => (),
                    }
                }
            } else {
                for i in 0..y.len() {
                    match compare_packets(&x[i], &y[i]) {
                        std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                        std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                        std::cmp::Ordering::Equal => (),
                    }
                }
            }

            if x.len() < y.len() {
                return std::cmp::Ordering::Greater;
            } else if y.len() < x.len() {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Equal;
            }
        }
        (PacketPart::List(x), PacketPart::Integer(y)) => {
            let same = PacketPart::List(x.to_vec());
            let changed = PacketPart::List(vec![PacketPart::Integer(*y)]);
            compare_packets(&same, &changed)
        }
        (PacketPart::Integer(x), PacketPart::List(y)) => {
            let changed = PacketPart::List(vec![PacketPart::Integer(*x)]);
            let same = PacketPart::List(y.to_vec());
            compare_packets(&changed, &same)
        }
    }
}

fn compare_all_packets(input: &str) -> i64 {
    let mut it = input.trim().lines();
    let mut index_sum = 0;
    let mut i = 1;
    while let Some(line1) = it.next() {
        let line2 = it.next().unwrap();

        let packet1 = parse_packet(line1);
        let packet2 = parse_packet(line2);

        match compare_packets(&packet1, &packet2) {
            std::cmp::Ordering::Greater => index_sum += i,
            std::cmp::Ordering::Less => (),
            std::cmp::Ordering::Equal => panic!(
                "Should compare, \n{}\n{}\n{:?}\n{:?}\n",
                line1, line2, packet1, packet2
            ),
        }

        it.next(); // Gets blank line
        i += 1;
    }
    index_sum
}

fn sort_all_packets(input: &str) -> usize {
    let mut it = input.trim().lines();
    let mut packets: Vec<PacketPart> = Vec::new();
    while let Some(line1) = it.next() {
        let line2 = it.next().unwrap();

        let packet1 = parse_packet(line1);
        let packet2 = parse_packet(line2);
        packets.push(packet1);
        packets.push(packet2);

        it.next(); // Gets blank line
    }

    packets.push(PacketPart::List(vec![PacketPart::List(vec![
        PacketPart::Integer(2),
    ])]));
    packets.push(PacketPart::List(vec![PacketPart::List(vec![
        PacketPart::Integer(6),
    ])]));

    packets.sort_by(|x, y| compare_packets(y, x));
    let pos1 = packets
        .iter()
        .position(|x| *x == PacketPart::List(vec![PacketPart::List(vec![PacketPart::Integer(2)])]))
        .unwrap()
        + 1;
    let pos2 = packets
        .iter()
        .position(|x| *x == PacketPart::List(vec![PacketPart::List(vec![PacketPart::Integer(6)])]))
        .unwrap()
        + 1;

    pos1 * pos2
}

fn main() {
    let input = advent::read_file("data/day13_input.txt");
    println!("Result: {}", compare_all_packets(&input));
    println!("Result2: {}", sort_all_packets(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case() {
        let input = "[1,1,3,1,1]\n\
                     [1,1,5,1,1]\n\
                     \n\
                     [[1],[2,3,4]]\n\
                     [[1],4]\n\
                     \n\
                     [9]\n\
                     [[8,7,6]]\n\
                     \n\
                     [[4,4],4,4]\n\
                     [[4,4],4,4,4]\n\
                     \n\
                     [7,7,7,7]\n\
                     [7,7,7]\n\
                     \n\
                     []\n\
                     [3]\n\
                     \n\
                     [[[]]]\n\
                     [[]]\n\
                     \n\
                     [1,[2,[3,[4,[5,6,7]]]],8,9]\n\
                     [1,[2,[3,[4,[5,6,0]]]],8,9]";

        assert_eq!(13, compare_all_packets(&input));
        assert_eq!(140, sort_all_packets(&input));
    }
}
