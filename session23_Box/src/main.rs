//we will use box pointer to store data on heap
// fn main() {
//     let b = Box::new(5);
//     println!("b = {}", b);
// }
//more often Pointers will be used when recursive structs has been used
//there are some use smart pointers:
        // 1.Box
        // 2.Rc (Reference Counted Smart Pointer)
        // 3.RefCell




enum List {
    Cons(i32, Rc<List>),//Box pointer will stored in stack and its size can be defined at compile time
    Nil,
}
use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // let list = Cons(1, Cons(2, Cons(3, Nil)));//this will generate error as compiler has to know size but here it is not known
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))).into());   we can use .into method convert Box<LIst> into Rc<List>

    // let x:i32 =5;
    // let y = &x;//here just it will take copy and
    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    // println!("{},{}",x,*y); 

    // let x = 5;
    // let y = Box::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));//here this will give error box only allows single ownership 
    //but when we have to deal with sjngle value and multiple owners then we will go for Rc (Referrence Counter Smart Pointer)
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a));
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    //lets look into refcell

    let mut x= 5;
    let mut y =&mut x;
    println!("{}",y);
    println!("{}",x)

    // above code executed perfectly 



    // Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
    // Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
    // Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

    // let x = 5;
    // let y = &mut x;//but this wont

    // we can do that using refcell
    // lets discuss this in next session

}