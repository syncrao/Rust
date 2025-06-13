fn main() {
    let x = 44;
    let y = Box::new(88);

    {
        let z = (x, y); // x is copied into z, whereas y is moved into z
        println!("{:?}", z)
    }

    println!("{}", x); // so this work
    // println!("{}", y) this won't work because value borrowed here after move

    fn replace_with_84(s: &mut Box<i32>) {

        let was = std::mem::take(s);
        println!("{}", was);
        println!("{}", *s);
        *s = was;
        println!("{}", *s);

        let mut r = Box::new(84);
        println!("{}", r);
        std::mem::swap(s, &mut r);
        println!("{}", *s);
        println!("{}", r);
        assert_ne!(*r, 84);
    }

    let mut s = Box::new(42);
    replace_with_84(&mut s);

    let mut k = Box::new(66);
    let mut j = &k;
    for i in 0..100 {
        println!("{}", j);
        k = Box::new(i);
        j = &k;
    }
    println!("{}", j);
}
