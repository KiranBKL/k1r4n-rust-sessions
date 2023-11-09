fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {//here T is type that implemts PartialOrd
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

//implementing generics in struct
struct Point<T> {
    x: T,
    y: T,
}


impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Point2<T,U> {
    x: T,
    y: U,
}

impl<T,U> Point2<T,U>
    where T:PartialOrd
{
    fn greater(&self, other: &Point2<T,U>) -> bool {
        if self.x < other.x {
            return false;
        }
        return false;
    }
}
//As we discussed earlier Options and Result are Enum types
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p = Point { x: 5, y: 10 };
    // let q = Point { x: 5, y: 10.0 };this will give error as we implementd only one generic

    let p2 = Point2 { x: 5, y: 10. };
    let q2 = Point2 { x: 5, y: 11.0 };

    println!("{}",p2.greater(&q2));

    
}