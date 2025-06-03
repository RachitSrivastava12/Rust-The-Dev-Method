#[derive(Debug)]

struct Rectangle{
    width: u32,
    height: u32
}
// struct  User{
//     active:bool,
//     username: String,
//     email:String,
//     sign_in_count: u64,
// }
imple Rectangle{
    fn area(&self) -> u32{
        self.width *self.height
    }
}


fn main() {
    let scale = 2;
    let rect1 = Rectangle{
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1);
    // let mut user1 = User{
    //     email: String::from("some@example.com"),
    //     username: String::from("someuser"),
    //     active: true,
    //     sign_in_count: 2,
    // };
}

