use std::os::unix::net::UnixListener;
use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::collections::HashMap;

struct Timer {
    client: i32, //The ID of the client who owns this timer 
    time_remaining: f32, //amount of time remaining on this timer 
    expired: bool // false initially, set to true once timer hits zero 
}


fn main() -> std::io::Result<()> {
    let mut client_timers: HashMap<i32, Timer> = HashMap::new();
    let listener = UnixListener::bind("/tmp/pluggable_timer.sock")?;
    loop {
        match listener.accept() {
            Ok((mut socket, addr)) => {
                println!("Got a client: {:?}", addr);
                let mut buf = [0; 1024];
                let count = socket.read(&mut buf).unwrap();
                let res = String::from_utf8(buf[..count].to_vec()).unwrap();
                println!("{}", res);
            },
        Err(e) => println!("accept function failed: {:?}", e),
        }
    }
}
