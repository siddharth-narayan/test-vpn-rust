// use std::{io::{Read, Write}, sync::mpsc::{self, Receiver, Sender}};

// pub struct WriteHalf {
//     tx: Sender<Box<[u8]>>
// }

// pub struct ReadHalf<T> {
//     rx: Receiver<Box<[u8]>>
// }

// pub fn split<T: Read + Write>(io: T) -> (WriteHalf<T>, ReadHalf<T>) {
//     let (tx, rx) = mpsc::channel::<T>();

//     todo!()
// }

// // struct ReadWriter<T: Read + Write> {
    
// // }

// // impl<T> ReadWriter<T> {
// //     pub fn spawn()
// // }

