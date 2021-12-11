fn main() {
    let mut x:i128 = 0;
    let mut y:i128 = 0;
    let mut q = true;
    for i in 1..100000080 {
        if q {
            y+=1;
            if y%3 == 0 && y%5 != 0 {
                q = false;
            }
        }else {
            x+=1;
            if x%5 == 0 && x%3 != 0 {
                q = true;
            }
        }
    }
    eprintln!("x = {:?}", x);
    eprintln!("y = {:?}", y);
}
