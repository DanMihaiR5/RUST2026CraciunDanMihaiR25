fn prime(n: i32) {
    if n==0 || n==1 {
        return;
    }
    if n % 2 != 0 {
        let mut i: i32 = 3;
        while i <= n / 2 {
            if n % i == 0 {
                return;
            }
            i += 2;
        }
        println!("{}", n);
    }
}

fn coprime(n: i32, m: i32) {
    let mut i: i32 = 2;
    while i <= n.min(m) / 2 {
        if n % i == 0 && m % i == 0 {
            return;
        }
        i += 1;
    }
    println!("({},{})", n, m);
}

fn main() {
    let mut a = 0;
    while a <= 100 {
        prime(a);
        a += 1;
    }
    let mut j = 0;
    let mut v = 0;
    loop {
        loop {
            coprime(j, v);
            v += 1;
            if v > 100 {
                break;
            }
        }
        v = 0;
        if j == 100 {
            break;
        }
        j += 1;
    }
    let mut b = 99;
    let mut c = 99;
    let mut d = 98;
    loop {
        if b < 0 || c < 0 || d < 0 {
            break;
        }
        if b == 1 {
            println!("{} bottles of beer on the wall,", b);
            println!("{} bottles of beer.", c);
            println!("Take one down, pass it around,");
            println!("No bottles of beer on the wall.");
            break;
        }
        println!("{} bottles of beer on the wall,", b);
        println!("{} bottles of beer.", c);
        println!("Take one down, pass it around,");
        println!("{} bottles of beer on the wall.", d);
        println!(" ");
        b -= 1;
        c -= 1;
        d -= 1;
    }
}
