//structures are three types structures,Tuple like structres and unit structures
#[derive(Debug)]//to view struct
struct Rectangle {
    width : u32,
    height: u32,
    id: String,//here we have used String rather than &str as &str is a reference to str, and references need lifetimes we will look into lifetimes in next sessions
}

impl Rectangle {
    fn new(width:u32,height:u32,id:String) -> Rectangle {
        Rectangle {
            width,
            height,
            id,
        }
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn perimeter(&self) -> u32 {
        2 * (self.height + self.width)
    }

    fn is_it_will_fit(&self,rec:&Rectangle) -> bool {
        let area_self = self.area();
        let area_rec = rec.area();
        if area_self >= area_rec {
            return  true;
        }
        else { false}
    }
}

fn main() {
    let rec1 = Rectangle::new(4,5,"1".into());
    let width = rec1.width;
    let height=rec1.height;
    let id = &rec1.id;
    let rec2 = Rectangle{
        width,
        height,
    //4 ways convert &str to String
    //    String::from("2")
    //"blue".to_string()
    //"blue".into()
    //"blue".to_owned()
        id:id.into(),//here varaible not same so we have to explicitly
    };

    println!("area of rec1 {}",rec1.area());
    println!("area of rec2 {}",rec2.area());
    println!("rec1 will fit in rec2 {}",rec2.is_it_will_fit(&rec1));

    println!("rec1:{rec1:?},rec2:{rec2:?}")

}
