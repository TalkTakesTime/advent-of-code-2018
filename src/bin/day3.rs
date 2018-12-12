use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<Error>> {
    let data = read_to_string("./data/day3.txt")?;
    let data: Vec<Claim> = data.lines().map(Claim::new).collect();
    part1(&data);
    part2(&data);
    Ok(())
}

fn part1(claims: &[Claim]) {
    let mut fabric = Fabric::new(1000, 1000);
    for claim in claims.iter() {
        fabric.add_claim(&claim);
    }
    println!("Part 1: {}", fabric.count(|&&cell| cell > 1));
}

fn part2(claims: &[Claim]) {
    let mut fabric = Fabric::new(1000, 1000);
    for claim in claims.iter() {
        fabric.add_claim(&claim);
    }

    let valid_claim = claims
        .iter()
        .find(|&claim| fabric.check_claim(claim))
        .unwrap()
        .id;
    println!("Part 2: {}", valid_claim);
}

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

struct Fabric {
    grid: Vec<i32>,
    width: usize,
    height: usize,
}

impl Claim {
    fn new(claim_str: &str) -> Self {
        let mut pieces = claim_str.trim_left_matches('#').split_whitespace();
        let id = parse(pieces.next());
        pieces.next();

        let mut coords = pieces.next().unwrap().trim_right_matches(':').split(',');
        let x = parse(coords.next());
        let y = parse(coords.next());

        let mut size = pieces.next().unwrap().trim().split('x');
        let width = parse(size.next());
        let height = parse(size.next());

        Self {
            id,
            x,
            y,
            width,
            height,
        }
    }
}

fn parse(num_str: Option<&str>) -> usize {
    num_str.unwrap().parse::<usize>().unwrap()
}

impl Fabric {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![0; width * height],
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<i32> {
        let pos = y * self.width + x;
        self.grid.get(pos as usize).cloned()
    }

    fn check_claim(&self, claim: &Claim) -> bool {
        // hmm
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                if let Some(val) = self.get(x, y) {
                    if val > 1 {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn inc(&mut self, x: usize, y: usize) -> Option<i32> {
        let pos = y * self.width + x;
        if pos < self.width * self.height {
            self.grid[pos] += 1;
            Some(self.grid[pos])
        } else {
            None
        }
    }

    fn add_claim(&mut self, claim: &Claim) {
        for x in claim.x..(claim.x + claim.width) {
            for y in claim.y..(claim.y + claim.height) {
                self.inc(x, y);
            }
        }
    }

    fn count<P>(&self, func: P) -> usize
    where
        P: Fn(&&i32) -> bool,
    {
        self.grid.iter().filter(func).count()
    }
}
