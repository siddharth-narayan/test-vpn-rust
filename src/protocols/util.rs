use std::sync::Arc;

use tokio::{io::{ReadHalf, WriteHalf}, sync::Mutex};
use tun::AsyncDevice;

pub type TunRead = ReadHalf<AsyncDevice>;
pub type TunWrite = Arc<Mutex<WriteHalf<AsyncDevice>>>; // Wrap with Arc<Mutex<>> so that multiple server recv threads can write to it
