use leetcode::part1::part1_0::main336;

mod script;

use protobuf::Message;

pub use script::Users;
pub use script::User;


fn main() {
    let mut user = User::new();
    let mut users = Users::new();

    user.name = "123".to_string();
    user.password = "123".to_string();

    users.users.push(user);

    let out_bytes: Vec<u8> = users.write_to_bytes().unwrap();

    println!("{:?}", out_bytes);
    // main336();
}
