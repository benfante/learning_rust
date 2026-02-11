pub fn gcd(n: u64, m: u64) -> u64 {
    let mut tn = n;
    let mut tm = m;
    while tm != 0 {
        if tm < tn {
            swap(&mut tm, &mut tn);
        }
        tm = tm % tn;
    }
    tn
}

fn swap(a: &mut u64, b: &mut u64) {
    let t = *a;
    *a = *b;
    *b = t;
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(10, 15), 5);
    assert_eq!(gcd(100, 80), 20);
    assert_eq!(gcd(17, 13), 1);
    assert_eq!(gcd(0, 0), 0);
}

#[test]
fn test_swap() {
    let mut a = 10;
    let mut b = 20;
    swap(&mut a, &mut b);
    assert_eq!(a, 20);
    assert_eq!(b, 10);
}
