///////////////////////////////////////////////////////////////////////////////////////////////////
//
//                                        _            __ _____
//                                       | |          /_ | ____|
//                                     __| | __ _ _   _| | |__
//                                    / _` |/ _` | | | | |___ \
//                                   | (_| | (_| | |_| | |___) |
//                                    \__,_|\__,_|\__, |_|____/
//                                                 __/ |
//                                                |___/
//
// Jonathan Lowe
// github : https://github.com/jglowe
//
// The file for day15 advent of code 2022
///////////////////////////////////////////////////////////////////////////////////////////////////

use std::collections::HashSet;

use regex::Regex;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Range {
    min: i64,
    max: i64,
}

impl Range {
    fn can_merge(self: &Range, range: &Range) -> bool {
        (self.min <= range.max && self.max >= range.min)
            || (range.min <= self.max && range.max >= self.min)
            || (self.min <= range.min && self.max >= range.max)
            || (range.min <= self.min && range.max >= self.max)
            || (self.max + 1 == range.min)
            || (range.max + 1 == self.min)
    }

    fn merge(self: &Range, range: &Range) -> Option<Range> {
        if self.can_merge(range) {
            let min = if self.min < range.min {
                self.min
            } else {
                range.min
            };
            let max = if self.max > range.max {
                self.max
            } else {
                range.max
            };
            Some(Range { max, min })
        } else {
            None
        }
    }

    fn contains(self: &Range, x: i64) -> bool {
        self.min <= x && self.max >= x
    }
}

#[derive(Debug)]
struct Ranges {
    ranges: Vec<Range>,
}

impl Ranges {
    fn new() -> Ranges {
        Ranges { ranges: vec![] }
    }

    fn add(self: &mut Ranges, range: Range) {
        if self.ranges.len() == 0 {
            self.ranges.push(range);
            return;
        }

        let mut range = range.clone();
        let mut new_ranges: Vec<Range> = Vec::new();
        for r in &self.ranges {
            match r.merge(&range) {
                Some(ra) => range = ra,
                None => new_ranges.push(r.clone()),
            }
        }
        new_ranges.push(range);
        self.ranges = new_ranges;
    }

    fn has_break(self: &Ranges) -> bool {
        self.ranges.len() > 1
    }

    fn contains(self: &Ranges, x: i64) -> bool {
        for range in &self.ranges {
            if range.contains(x) {
                return true;
            }
        }
        false
    }

    fn count(self: &Ranges) -> i64 {
        self.ranges
            .iter()
            .map(|range| (range.max - range.min).abs() + 1)
            .sum()
    }
}

fn parse_line(line: &str) -> (Point, Point) {
    let regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let captures = regex.captures(line).unwrap();
    (
        Point {
            x: captures.get(1).unwrap().as_str().parse::<i64>().unwrap(),
            y: captures.get(2).unwrap().as_str().parse::<i64>().unwrap(),
        },
        Point {
            x: captures.get(3).unwrap().as_str().parse::<i64>().unwrap(),
            y: captures.get(4).unwrap().as_str().parse::<i64>().unwrap(),
        },
    )
}

fn parse_sensors_and_beacons(input: &str) -> Vec<(Point, Point)> {
    input.trim().lines().map(parse_line).collect()
}

fn beacon_cant_be_presesnt_on_y(sensors_with_beacons: &Vec<(Point, Point)>, y: i64) -> i64 {
    let mut ranges: Ranges = Ranges::new();
    let mut sensors: Vec<Point> = Vec::new();
    let mut beacons: HashSet<Point> = HashSet::new();

    for (sensor, beacon) in sensors_with_beacons {
        sensors.push(sensor.clone());
        beacons.insert(beacon.clone());
        let max_distence = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();

        if max_distence > (sensor.y - y).abs() {
            let min = sensor.x - (max_distence - (sensor.y - y).abs());
            let max = sensor.x + (max_distence - (sensor.y - y).abs());
            ranges.add(Range { min, max });
        }
    }

    let beacon_count: i64 = beacons
        .iter()
        .map(|b| {
            if b.y == y && ranges.contains(b.x) {
                1
            } else {
                0
            }
        })
        .sum();

    ranges.count() - beacon_count
}

fn find_tuning_number(min: i64, max: i64, sensors_with_beacons: &Vec<(Point, Point)>) -> i64 {
    for y in min..max + 1 {
        let mut ranges: Ranges = Ranges::new();
        let y = y as i64;
        for (sensor, beacon) in sensors_with_beacons {
            let max_distence = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();

            if max_distence > (sensor.y - y).abs() {
                let mi = sensor.x - (max_distence - (sensor.y - y).abs());
                let ma = sensor.x + (max_distence - (sensor.y - y).abs());
                let mi = if mi < min { min } else { mi };
                let ma = if ma > max { max } else { ma };
                ranges.add(Range { min: mi, max: ma });
            }
        }

        if ranges.has_break() {
            let x = if ranges.ranges[0].max > ranges.ranges[1].max {
                ranges.ranges[1].max + 1
            } else {
                ranges.ranges[0].max + 1
            };
            return y + (4000000 * x);
        }
    }

    panic!("Didn't find the break as promised would be there.");
}

fn main() {
    let input = advent::read_file("data/day15_input.txt");
    let sensors_with_beacons = parse_sensors_and_beacons(&input);
    let y = 2000000;
    println!(
        "On Line y={}, there are {} places where beacons can't be present.",
        y,
        beacon_cant_be_presesnt_on_y(&sensors_with_beacons, y)
    );
    println!(
        "Tuning number: {}",
        find_tuning_number(0, 4000000, &sensors_with_beacons)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scenario() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
                     Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
                     Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
                     Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
                     Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
                     Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
                     Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
                     Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
                     Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
                     Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
                     Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
                     Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
                     Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
                     Sensor at x=20, y=1: closest beacon is at x=15, y=3\n";
        let sensors_with_beacons = parse_sensors_and_beacons(&input);

        let y = 10;
        assert_eq!(26, beacon_cant_be_presesnt_on_y(&sensors_with_beacons, y));
        assert_eq!(56000011, find_tuning_number(0, 20, &sensors_with_beacons));
    }
}
