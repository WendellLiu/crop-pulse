use std::fmt::Debug;

pub fn log<T: Debug>(message: T) {
    println!("log: {:?}", message);
}

pub fn error<T: Debug>(message: T) {
    println!("error: {:?}", message);
}
