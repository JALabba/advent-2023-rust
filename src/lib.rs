pub mod template;

// Use this file to add helper functions and additional modules.

pub fn lcm(a: usize, b: usize)-> usize {
    a * b / gcd(a,b)
}

pub fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        std::mem::swap(&mut b, &mut a);
    }
    while b> 0 {
        let t = a;
        a = b;
        b = t % b;
    }
    a
}
