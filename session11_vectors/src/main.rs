//
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    //vector is dynmically sized and stores same type of data.
   let v:Vec<i32> = Vec::new();//we have to declare type explicitly here
   let mut v = vec![1,2,  ];//we dont need to specify type explicitly here
//    let e=&v[2];
   let k=v.get(2);
   println!("{:?}",k);//OPtion does not implement fmt::Display

   //reading elements from vector

//    let e=&v[2];
//    let k=v.get(2).expect("number has been expected");
// let k=v.get(2);
// let k = match k {
//     Some(k)=>k,
//     None=>{
//         println!("bhhg");
//         return;
//     }
// };
//    println!("hihii");
//    println!("{k}")

let k=v.get(2);
let mut x=9;
match k {
    Some(k)=>
    {
        x=*k;}

    None=>{
        println!("bhhg");
    
    }
};
   println!("hihii");
   println!("{x}");

   //iterating over vector  elements
   for i in &v{
       println!("{i}");
   }

   //mutating vector elements
   for i in &mut v {//we have not declared vector as mutable so it cannot be borrowed as a mutable
        *i+=27;
   }

   //vector of enumerations
let mut y: Vec<SpreadsheetCell> = Vec::new();
y.push(SpreadsheetCell::Int(1));
y.push(SpreadsheetCell::Float(2.0));
y.push(SpreadsheetCell::Text("hello".to_string()));

}

