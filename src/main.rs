#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
}

impl Clone for CubeSat {
    fn clone(&self) -> Self {
        CubeSat { id: self.id }
    }
}

#[derive(Debug, Copy)]
enum StatusMessage {
    Ok,
}

impl Clone for StatusMessage {
    fn clone(&self) -> Self {
        *self
    }
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn main() {
    let sat_a = CubeSat { id: 0 };

    let a_status = check_status(sat_a.clone());
    println!("a: {:?}", a_status.clone());

    let a_status = check_status(sat_a);
    println!("a: {:?}", a_status);


}
