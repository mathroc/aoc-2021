use std::{cmp::Ordering, collections::{HashSet, HashMap}};

use parse_display::{Display, FromStr};

#[derive(Debug, PartialEq, Clone)]
struct Input {
    scanners: Vec<Vec<Point>>,
}

#[derive(Display, FromStr, PartialEq, Eq, Debug, Clone, Copy, Hash)]
#[display("{x},{y},{z}")]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Display, FromStr, PartialEq, Eq, Debug, Clone, Copy, Hash)]
#[display("{x},{y},{z}")]
pub struct PointU {
    x: u32,
    y: u32,
    z: u32,
}

#[allow(unused_variables)]
#[aoc_generator(day19)]
fn input_generator(input: &str) -> Input {
    let mut scanners: Vec<Vec<Point>> = Vec::new();

    for line in input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        if line.starts_with("--- scanner") {
            scanners.push(Vec::new());
        } else {
            let scanner = scanners.last_mut().unwrap();
            scanner.push(line.parse::<Point>().unwrap())
        }
    }

    Input { scanners }
}

#[test]
fn part1_provided_exemple_input() {
    let i = input_generator(exemple_raw_input());

    assert_eq!(i.scanners[0][0], Point { x: 404, y: -588, z: -901 });
    assert_eq!(i.scanners.len(), 5);
}

pub fn rotate(p: Point, i: usize) -> Point {
    let p = match i >> 2 {
        0 => Point { x: p.x, y: p.y, z: p.z },
        1 => Point { x: p.x, y: p.z, z: p.y },
        2 => Point { x: p.y, y: p.z, z: p.x },
        3 => Point { x: p.y, y: p.x, z: p.z },
        4 => Point { x: p.z, y: p.x, z: p.y },
        5 => Point { x: p.z, y: p.y, z: p.x },
        _ => panic!(),
    };

    match i & 7 {
        0 => Point { x: p.x, y: p.y, z: p.z },
        1 => Point { x: -p.x, y: -p.y, z: p.z },
        2 => Point { x: -p.x, y: p.y, z: -p.z },
        3 => Point { x: p.x, y: -p.y, z: -p.z },
        4 => Point { x: p.x, y: p.y, z: -p.z },
        5 => Point { x: p.x, y: -p.y, z: p.z },
        6 => Point { x: -p.x, y: p.y, z: p.z },
        7 => Point { x: -p.x, y: -p.y, z: -p.z },
        _ => panic!(),
    }
}

fn point_add(a: Point, b: Point) -> Point {
    Point { x: a.x + b.x, y: a.y + b.y, z: a.z + b.z }
}

fn point_sub(a: Point, b: Point) -> Point {
    Point { x: a.x - b.x, y: a.y - b.y, z: a.z - b.z }
}

fn manhattan_dist(a: Point, b: Point) -> u32 {
    ((a.x - b.x).abs() + (a.y - b.y).abs() + (a.z - b.z).abs()) as u32
}

fn wrapping_sub(p: Point) -> PointU {
    PointU { 
        x: p.x.wrapping_sub(i32::MIN) as u32,
        y: p.y.wrapping_sub(i32::MIN) as u32,
        z: p.z.wrapping_sub(i32::MIN) as u32,
    }
}

impl PointU {
    pub fn get(&self, d: usize) -> u32 {
        match d {
            0 => self.x,
            1 => self.x,
            2 => self.x,
            _ => panic!(),
        }
    }
}

// Compares two points as per their position on the Z-order curve.
fn cmp_z_order(lhs: Point, rhs: Point) -> Ordering {
    let lhs = wrapping_sub(lhs);
    let rhs = wrapping_sub(rhs);
    let is_msb_less = |x, y| x < y && x < (x ^ y);
    let is_dim_less = |i, j| is_msb_less(lhs.get(i) ^ rhs.get(i), lhs.get(j) ^ rhs.get(j));
    let msd = if is_dim_less(0, 1) { 1 } else { 0 };
    let msd = if is_dim_less(msd, 2) { 2 } else { msd };
    lhs.get(msd).cmp(&rhs.get(msd))
}

// Computes differences between (i, i+1), ..., (i, i+k) for all i. If the
// scanner is sorted in Z-order this gives a good chance of overlap between
// scanners if they share common beacons.
fn window_diffs(scanner: &[Point], k: usize) -> impl Iterator<Item = (usize, Point)> + '_ {
    scanner
        .windows(k + 1)
        .enumerate()
        .flat_map(|(i, w)| w.iter().skip(1).map(move |p| (i, point_sub(*p, w[0]))))
}

fn res(input: &Input) -> (usize, u32) {
    let mut i = input.clone();
    let (refscan, scanners) = i.scanners.split_first_mut().unwrap();
    refscan.sort_unstable_by(|l, r| cmp_z_order(*l, *r));
    let mut scanner_positions = vec![Point { x: 0, y: 0, z: 0 }];
    let mut unknown_scanners: HashSet<usize> = (0..scanners.len()).collect();
    let mut current_rotation = 0;
    let mut rotations_since_overlap = 0;
    let mut known_points = HashSet::new();
    let mut known_diffs = HashMap::new();
    let mut rotated = Vec::new();

    while unknown_scanners.len() > 0 {
        let window_size = 1 + rotations_since_overlap / 24;
        for ui in unknown_scanners.clone() {
            if known_points.len() < refscan.len() {
                known_points.extend(refscan.iter().copied());
                known_diffs = window_diffs(&refscan, window_size).map(|(i, d)| (d, i)).collect();
            }

            rotated.clear();
            rotated.extend(scanners[ui].iter().map(|p| rotate(*p, current_rotation)));
            rotated.sort_unstable_by(|l, r| cmp_z_order(*l, *r));
            let common_diffs = window_diffs(&rotated, window_size).filter_map(|(u, d)| {
                known_diffs.get(&d).map(|k| point_sub(refscan[*k], rotated[u]))
            });

            for translation in common_diffs {
                let translated = rotated.iter().map(|p| point_add(*p, translation)).collect::<Vec<_>>();
                let overlaps = translated.iter().filter(|p| known_points.contains(*p));
                if overlaps.count() >= 12 {
                    scanner_positions.push(translation);
                    refscan.extend(translated);
                    refscan.sort_unstable_by(|l, r| cmp_z_order(*l, *r));
                    refscan.dedup();
                    rotations_since_overlap = 0;
                    unknown_scanners.remove(&ui);
                    break;
                }
            }
        }

        current_rotation = (current_rotation + 1) % 24;
        rotations_since_overlap += 1;

    }

    let mut max = 0;

    for p1 in &scanner_positions {
        for p2 in &scanner_positions {
            max = std::cmp::max(max, manhattan_dist(*p1, *p2));
        }
    }

    (refscan.len(), max)
}

#[allow(unused_variables)]
#[aoc(day19, part1)]

fn part1(input: &Input) -> usize {
    res(input).0
}

#[test]
fn part1_provided_exemple() {
    assert_eq!(part1(&input_generator(exemple_raw_input())), 79);
}

#[allow(unused_variables)]
#[aoc(day19, part2)]

fn part2(input: &Input) -> u32 {
    res(input).1
}

#[test]
fn part2_provided_exemple() {
    assert_eq!(part2(&input_generator(exemple_raw_input())), 15);
}

#[allow(unreachable_code)]
#[allow(dead_code)]
fn exemple_raw_input() -> &'static str {
    "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"
}
