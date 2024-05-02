fn main() {
    let i = 5;
    let j = i;

    println!("i is {}", i);
    println!("j is {}", j);

    let v = vec![1, 2, 3, 4, 5];
    // let w = &v;
    // println!("{:?}", w);
    // println!("{:?}", v);
    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("vector used in foo. ownership has been transferred here. bleh");
        v
    };

    let v = foo(v);

    println!("{:?}", v);
}
