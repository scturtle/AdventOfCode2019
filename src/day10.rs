use itertools::iproduct;
use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(PartialEq, Eq, Debug)]
struct PolarCoord {
    x: i32,
    y: i32,
}

impl PolarCoord {
    fn gcd(a: u32, b: u32) -> u32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
    fn new(x: i32, y: i32) -> Self {
        let g = Self::gcd(x.abs() as u32, y.abs() as u32) as i32;
        let g = if g == 0 { x.abs().max(y.abs()) } else { g };
        Self { x: x / g, y: y / g }
    }
    fn quadrant(&self) -> u8 {
        if self.x >= 0 && self.y < 0 {
            0
        } else if self.x > 0 && self.y >= 0 {
            1
        } else if self.x <= 0 && self.y > 0 {
            2
        } else if self.x < 0 && self.y <= 0 {
            3
        } else {
            unreachable!();
        }
    }
    fn cross(&self, other: &Self) -> i32 {
        self.x * other.y - self.y * other.x
    }
    fn dis(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}

impl PartialOrd for PolarCoord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PolarCoord {
    fn cmp(&self, other: &Self) -> Ordering {
        // first by quadrant
        let qa = self.quadrant();
        let qb = other.quadrant();
        if qa != qb {
            return qa.cmp(&qb);
        }
        // then by cross product
        let c = self.cross(other);
        if c != 0 {
            return if c > 0 {
                Ordering::Less
            } else {
                Ordering::Greater
            };
        }
        // finally by distance
        self.dis().cmp(&other.dis())
    }
}

pub fn run() {
    let txt = crate::common::get_input(10).unwrap();
    let lines: Vec<_> = txt.lines().map(|l| l.as_bytes()).collect();

    let (height, weight) = (lines.len(), lines[0].len());
    let stars: Vec<_> = iproduct!(0..height, 0..weight)
        .filter(|&(i, j)| lines[i][j] == b'#')
        .map(|(i, j)| (i as i32, j as i32))
        .collect();

    let mut maxsaw = 0;
    let mut center = stars[0];
    let mut rayvec = BTreeMap::new();
    for i in 0..stars.len() {
        let mut r = BTreeMap::new();
        for j in 0..stars.len() {
            if j == i {
                continue;
            }
            let k = PolarCoord::new(stars[j].1 - stars[i].1, stars[j].0 - stars[i].0);
            let e = r.entry(k).or_insert_with(|| vec![]);
            e.push(stars[j]);
        }
        if r.len() > maxsaw {
            maxsaw = r.len();
            center = stars[i];
            rayvec = r;
        }
    }
    dbg!(maxsaw);

    let dis = |a: (i32, i32), b: (i32, i32)| (a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1);
    // should be ordered by polar angle
    let mut vs = rayvec.values().cloned().collect::<Vec<_>>();
    for v in &mut vs {
        v.sort_by_key(|&t| dis(center, t));
    }
    let mut vi = 0;
    let mut del_cnt = 0;
    let mut deleted = (0, 0);
    while del_cnt != 200 {
        let v = &mut vs[vi];
        if !v.is_empty() {
            deleted = v.remove(0);
            del_cnt += 1;
        }
        vi = (vi + 1) % vs.len();
    }
    dbg!(deleted.1 + 100 * deleted.0);
}
