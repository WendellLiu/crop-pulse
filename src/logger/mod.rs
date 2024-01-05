use std::fmt::Debug;

pub fn log<T: Debug>(message: T) {
    println!("log: {:?}", message);
}
