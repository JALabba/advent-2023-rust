use std::{ fmt::Error, str::FromStr };

advent_of_code::solution!(24);

pub fn part_one(_input: &str) -> Option<u64> {
    let min = 200_000_000_000_000;
    let max = 400_000_000_000_000;
    let hail = _input
        .lines()
        .map(|line| Hail::from_str(line).unwrap())
        .collect::<Vec<_>>();
    let intersections = intersections_xy(&hail);
    let result = intersections
        .iter()
        .flatten()
        .filter(|c| c.contained_xy(min, max))
        .count();
    Some(result as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    // https://github.com/sebastianotronto/aoc/blob/master/2023/24/24b.c

    let hailstones = _input
        .lines()
        .map(|line| Hail::from_str(line).unwrap())
        .collect::<Vec<_>>();
    if hailstones.len() < 3 {
        return None; // Not enough hail
    }

    /* To figure the correct starting point + velocity, we solve some
	 * systems of linear equations. Write your starting point and velocity
	 * as unknowns x, y, z, Vx, Vy, Vz. Equating the position of the
	 * rock at time t1 (another unknown parameter) with the position
	 * of one of the hailstones at the same time t1, we get a system
	 * of 3 equations and 7 unknowns. Unfortunately, these equations
	 * have degree 2 - this is not a linear system!
	 * However, manipulating these equations a bit we can get a linear
	 * equation of the type:
	 *     (Vy1-Vy2)x - (Vx1-Vx2)y + - (y1-y2)Vx + (x1-x2)Vy =
	 *         y2Vx2 + x2Vy2 - y1Vx1 - x1Vy1
	 * Where x1, y1, z2, Vx1, Vy1, Vz1 and x2, y2, z2, Vx2, Vy2, Vz2
	 * are the starting points and velocities of two of the hailstones.
	 * So with 2 lines we can get a linear equation. Similarly, we can
	 * get equations involving the unknowns z and Vz.
	 * We can use the myriad of hailstones we have to generate as many
	 * equations as we like. The system is going to be overdetermined,
	 * but the problem statement seems to ensure that there is going to
	 * be a solution. On the other hand it can happen that we make a
	 * bad choice of lines and the equation we use are underdetermined.
	 * This last problem is not accounted for in the code - if it happens,
	 * one can shuffle the input file until it works.
	 */

    let d = 6;
    let mut a = vec![vec![0.0; d]; d];
    let mut c = vec![0.0; d];
    // Construct the system of equations
    a[0] = vec![
        hailstones[0].v.y - hailstones[1].v.y,
        hailstones[1].v.x - hailstones[0].v.x,
        0.0,
        hailstones[1].p.y - hailstones[0].p.y,
        hailstones[0].p.x - hailstones[1].p.x,
        0.0,
    ];
    c[0] = hailstones[1].p.y * hailstones[1].v.x
        - hailstones[1].p.x * hailstones[1].v.y
        - hailstones[0].p.y * hailstones[0].v.x
        + hailstones[0].p.x * hailstones[0].v.y;

    a[1] = vec![
        hailstones[0].v.y - hailstones[2].v.y,
        hailstones[2].v.x - hailstones[0].v.x,
        0.0,
        hailstones[2].p.y - hailstones[0].p.y,
        hailstones[0].p.x - hailstones[2].p.x,
        0.0,
    ];
    c[1] = hailstones[2].p.y * hailstones[2].v.x
        - hailstones[2].p.x * hailstones[2].v.y
        - hailstones[0].p.y * hailstones[0].v.x
        + hailstones[0].p.x * hailstones[0].v.y;

    a[2] = vec![
        hailstones[0].v.z - hailstones[1].v.z,
        0.0,
        hailstones[1].v.x - hailstones[0].v.x,
        hailstones[1].p.z - hailstones[0].p.z,
        0.0,
        hailstones[0].p.x - hailstones[1].p.x,
    ];
    c[2] = hailstones[1].p.z * hailstones[1].v.x
        - hailstones[1].p.x * hailstones[1].v.z
        - hailstones[0].p.z * hailstones[0].v.x
        + hailstones[0].p.x * hailstones[0].v.z;

    a[3] = vec![
        hailstones[0].v.z - hailstones[2].v.z,
        0.0,
        hailstones[2].v.x - hailstones[0].v.x,
        hailstones[2].p.z - hailstones[0].p.z,
        0.0,
        hailstones[0].p.x - hailstones[2].p.x,
    ];
    c[3] = hailstones[2].p.z * hailstones[2].v.x
        - hailstones[2].p.x * hailstones[2].v.z
        - hailstones[0].p.z * hailstones[0].v.x
        + hailstones[0].p.x * hailstones[0].v.z;

    a[4] = vec![
        0.0,
        hailstones[0].v.z - hailstones[1].v.z,
        hailstones[1].v.y - hailstones[0].v.y,
        0.0,
        hailstones[1].p.z - hailstones[0].p.z,
        hailstones[0].p.y - hailstones[1].p.y,
    ];
    c[4] = hailstones[1].p.z * hailstones[1].v.y
        - hailstones[1].p.y * hailstones[1].v.z
        - hailstones[0].p.z * hailstones[0].v.y
        + hailstones[0].p.y * hailstones[0].v.z;

    a[5] = vec![
        0.0,
        hailstones[0].v.z - hailstones[2].v.z,
        hailstones[2].v.y - hailstones[0].v.y,
        0.0,
        hailstones[2].p.z - hailstones[0].p.z,
        hailstones[0].p.y - hailstones[2].p.y,
    ];
    c[5] = hailstones[2].p.z * hailstones[2].v.y
        - hailstones[2].p.y * hailstones[2].v.z
        - hailstones[0].p.z * hailstones[0].v.y
        + hailstones[0].p.y * hailstones[0].v.z;

    // Solve the system
    let solution = solve_system(&mut a, &mut c, d)?;

    // Validate the solution
    let sol_line = Hail {
        p: Vector {
            x: solution[0],
            y: solution[1],
            z: solution[2],
        },
        v: Vector {
            x: solution[3],
            y: solution[4],
            z: solution[5],
        },
    };

    // for hail in &hailstones {
    //     let mut t = -1.0;
    //     if !eq(sol_line.v.x, hail.v.x) {
    //         t = (sol_line.p.x - hail.p.x) / (hail.v.x - sol_line.v.x);
    //     }
    //     if !eq(sol_line.v.y, hail.v.y) {
    //         t = (sol_line.p.y - hail.p.y) / (hail.v.y - sol_line.v.y);
    //     }
    //     if !eq(sol_line.v.z, hail.v.z) {
    //         t = (sol_line.p.z - hail.p.z) / (hail.v.z - sol_line.v.z);
    //     }

    //     let p1 = sol_line.position_at(t);
    //     let p2 = hail.position_at(t);

    //     if t < f64::EPSILON || !eq(p1.x, p2.x) || !eq(p1.y, p2.y) || !eq(p1.z, p2.z) {
    //         return None; // Invalid solution
    //     }
    // }

    Some((sol_line.p.x + sol_line.p.y + sol_line.p.z).round() as u64)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let hail = input
            .lines()
            .map(|line| Hail::from_str(line).unwrap())
            .collect::<Vec<_>>();
        let intersections = intersections_xy(&hail);
        let result = intersections
            .iter()
            .flatten()
            .filter(|c| c.contained_xy(7, 27))
            .count();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }

    #[test]
    fn parse() {
        let s = "19, 13, 30 @ -2,  1, -2";
        let hail = Hail {
            p: Vector { x: 19.0, y: 13.0, z: 30.0 },
            v: Vector { x: -2.0, y: 1.0, z: -2.0 },
        };
        let result = Hail::from_str(s).unwrap();
        assert_eq!(result, hail);
    }

    #[test]
    fn advance_time() {
        let hail = Hail {
            p: Vector { x: 20.0, y: 19.0, z: 15.0 },
            v: Vector { x: 1.0, y: -5.0, z: -3.0 },
        };
        let result = hail.position_at(1.0);
        assert_eq!(result, Vector { x: 21.0, y: 14.0, z: 12.0 })
    }

    #[test]
    fn intersect_paths() {
        let a = Hail::from_str("19, 13, 30 @ -2, 1, -2").unwrap();
        let b = Hail::from_str("18, 19, 22 @ -1, -1, -2").unwrap();
        let result = a.intersect_xy(&b);
        assert_eq!(result, Some(Vector { x: 14.333, y: 15.333, z: 0.0 }));

        let a = Hail::from_str("19, 13, 30 @ -2, 1, -2").unwrap();
        let b = Hail::from_str("20, 25, 34 @ -2, -2, -4").unwrap();
        let result = a.intersect_xy(&b);
        assert_eq!(result, Some(Vector { x: 11.667, y: 16.667, z: 0.0 }));
    }
}

#[allow(dead_code)]
trait Round {
    fn round_n(self, n: usize) -> f64;
}

impl Round for f64 {
    fn round_n(self, n: usize) -> f64 {
        let shift = n as i32 /* - self.abs().log10().ceil() as i32 */;
        let sf = (10_f64).powi(shift);
        (self * (sf as f64)).round() / (sf as f64)
    }
}

#[derive(Debug, PartialEq)]
struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    #[allow(dead_code)]
    fn contained_xy(&self, min: usize, max: usize) -> bool {
        // ignoring the z axis, is self within the bounds of min..=max
        !(
            self.x < (min as f64) ||
            self.x > (max as f64) ||
            self.y < (min as f64) ||
            self.y > (max as f64)
        )
    }
}


#[derive(Debug, PartialEq)]
struct Hail {
    p: Vector,
    v: Vector,
}
impl Hail {
    #[allow(dead_code)]
    fn position_at(&self, nanoseconds: f64) -> Vector {
        Vector {
            x: self.p.x + self.v.x * nanoseconds,
            y: self.p.y + self.v.y * nanoseconds,
            z: self.p.z + self.v.z * nanoseconds,
        }
    }

    #[allow(dead_code)]
    fn intersect_xy(&self, other: &Hail) -> Option<Vector> {
        // Calculate the determinant to check if lines are parallel
        let det = self.v.x * other.v.y - self.v.y * other.v.x;
        if det.abs() < f64::EPSILON {
            return None; // Lines are parallel, no intersection
        }

        // Calculate the intersection point
        let dx = other.p.x - self.p.x;
        let dy = other.p.y - self.p.y;
        let t = (dx * other.v.y - dy * other.v.x) / det;

        // if t < 0.0 {
        //     return None; // Only allow forward movement in time
        // }
        let x = (self.p.x + t * self.v.x).round_n(3);
        let y = (self.p.y + t * self.v.y).round_n(3);

        // for each coordinate component, check the temporal sign
        // i.e. The sign of delta x and the sign of velocity x should match
        // if inconsistent, the intersection happens backwards in time
        let self_x = (x - self.p.x < 0.0) == (self.v.x < 0.0);
        let self_y = (y - self.p.y < 0.0) == (self.v.y < 0.0);
        let other_x = (x - other.p.x < 0.0) == (other.v.x < 0.0);
        let other_y = (y - other.p.y < 0.0) == (other.v.y < 0.0);

        let intersects_in_future = self_x && self_y && other_x && other_y;

        if !intersects_in_future {
            return None;
        }
        Some(Vector {
            x,
            y,
            z: 0.0,
        })
    }
}

impl FromStr for Hail {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // px, py, pz, @ vx, vy, vz
        let p: Vec<f64> = s
            .replace("@", "")
            .replace(",", "")
            .split_whitespace()
            .map(|e| e.trim().parse::<f64>().unwrap())
            .collect();
        Ok(Self {
            p: Vector { x: p[0], y: p[1], z: p[2] },
            v: Vector { x: p[3], y: p[4], z: p[5] },
        })
    }
}

fn intersections_xy(hail: &[Hail]) -> Vec<Option<Vector>> {
    let mut res = Vec::new();
    for (i, h1) in hail.iter().enumerate() {
        for h2 in hail.iter().skip(i + 1) {
            res.push(h1.intersect_xy(h2));
        }
    }
    res
}

fn abs(x: f64) -> f64 {
    if x > 0.0 { x } else { -x }
}

fn eq(a: f64, b: f64) -> bool {
    // a == b
    abs(a-b) < f64::EPSILON * (abs(a) + abs(b) )
}


fn solve_system(a: &mut [Vec<f64>], c: &mut [f64], d: usize) -> Option<Vec<f64>> {
    let mut x = vec![0.0; d];

    // Row reduction
    for i in 0..d {
        // Find the pivot
        let mut imax = i;
        let mut max_val = 0.0;
        for j in i..d {
            if abs(a[j][i]) > max_val {
                max_val = abs(a[j][i]);
                imax = j;
            }
        }
        if eq(max_val, 0.0) {
            return None; // No unique solution
        }

        // Swap rows
        a.swap(i, imax);
        c.swap(i, imax);

        // Reduce rows
        for ii in (i + 1)..d {
            let r = a[ii][i] / a[i][i];
            for k in i..d {
                a[ii][k] -= r * a[i][k];
            }
            c[ii] -= r * c[i];
        }
    }

    // Back substitution
    for i in (0..d).rev() {
        x[i] = c[i];
        for j in (i + 1)..d {
            x[i] -= a[i][j] * x[j];
        }
        x[i] /= a[i][i];
    }

    Some(x)
}
