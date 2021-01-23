use std::fmt::Display;

fn pt1(val : impl Display) {
    println!("p1 {}", val);
}

fn pt2(val : &dyn Display) {
    println!("p2 {}", val);
}

struct Person(String);


fn main() {
    let v = vec![0, 1,2,3,4];
    let vr = &v;
    let mut cs = vec![];
    for n in 0..5 {
        let c = std::thread::spawn( move ||{
            println!("{}", vr[n]);
        });
        cs.push(c);
    }

    for c in cs {
        c.join().unwrap();
    }
}
