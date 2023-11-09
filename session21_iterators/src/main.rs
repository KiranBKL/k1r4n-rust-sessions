fn main() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();
    for val in v1_iter.into_iter() { //here also moved as it calls implicitly .into_iter()
        println!("Got: {}", val);
    }
    // let v = v1_iter.next();//this will borrow v1_iter as mutable
    // println!("{:?}", v);
    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }

    //.sum()
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();//here copy
    let total: i32 = v1_iter.sum();
    println!("Total: {:?}",v1);
    // assert_eq!(total, 6);

    //methods that produce other vectors based on the vectors
    //.map
    let v2:Vec<_> = v1.iter().map(|x| (*x as f32)*2.0).collect();//it will give error as it needed annotation
    println!("Total: {:?}",v2);
    //.filter
    let v3:Vec<_> =v2.into_iter().filter(|x| *x== 6.0).collect();
    println!("Total: {:?}",v3);
    
}
//actually iterator is an trait
// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;

//     // methods with default implementations elided
// }