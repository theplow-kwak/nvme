use crossterm::event::{self, KeyCode, KeyEvent};

pub mod bitarray;
pub mod datafifo;
pub mod dev;
pub mod multicast;
pub mod packet;
pub mod receiver_a;
pub mod receiver_s;
pub mod sender;
pub mod slice;
// pub mod statistics;

pub const RUNNING: bool = true;
pub const ENDLOOP: bool = false;

pub const MI_BYTES: usize = 1024 * 1024;
pub const SECTOR_SIZE: usize = 512;
pub const CHUNK_SIZE: usize = 512 * SECTOR_SIZE;
pub const READ_CHUNK: usize = 16 * CHUNK_SIZE;
pub const MAX_BUFFER_SIZE: usize = 128 * CHUNK_SIZE;
pub const MAX_READ_PIPE: usize = 1024 * MI_BYTES;
pub const MAX_WRITE_PIPE: usize = 128 * MI_BYTES;

pub const BLOCK_SIZE: u32 = 1456;
pub const UDP_PACK_SIZE: usize = 2048;

pub const MAX_CLIENTS: u32 = 128;
pub const MAX_SLICE_SIZE: u32 = 2048;
pub const BITS_PER_CHAR: u32 = 8;

pub const CAP_NEW_GEN: u32 = 0x0001;
pub const CAP_BIG_ENDIAN: u32 = 0x0008;
pub const CAP_LITTLE_ENDIAN: u32 = 0x0010;
pub const CAP_ASYNC: u32 = 0x0020;
pub const SENDER_CAPABILITIES: u32 = CAP_NEW_GEN | CAP_BIG_ENDIAN;
pub const RECEIVER_CAPABILITIES: u32 = CAP_NEW_GEN | CAP_BIG_ENDIAN;

pub const FLAG_PASSIVE: u16 = 0x0010;
pub const FLAG_NOSYNC: u16 = 0x0040;
pub const FLAG_NOKBD: u16 = 0x0080;
pub const FLAG_SYNC: u16 = 0x0100;
pub const FLAG_STREAMING: u16 = 0x200;
pub const FLAG_IGNORE_LOST_DATA: u16 = 0x400;

pub const PORTBASE: u16 = 9000;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn getch(secs: u64) -> Option<char> {
    if event::poll(std::time::Duration::from_secs(secs)).unwrap() {
        if let event::Event::Key(KeyEvent {
            code, modifiers: _, ..
        }) = event::read().unwrap()
        {
            if let KeyCode::Char(c) = code {
                return Some(c);
            }
            if let KeyCode::Enter = code {
                return Some('\r');
            }
        }
    }
    return None;
}

use std::time::{Duration, Instant};

pub struct DiskLatency {
    pub start: Instant,
    pub end: Instant,
    pub size: usize,
}

impl DiskLatency {
    pub fn new(size: usize) -> Self {
        Self {
            start: Instant::now(),
            end: Instant::now(),
            size,
        }
    }

    pub fn end(&mut self) -> &Self {
        self.end = Instant::now();
        self
    }

    pub fn get(&self) -> (Instant, Instant, usize) {
        (self.start, self.end, self.size)
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

use std::fs::File;
use std::io::Write;

pub fn save_trace(
    filename: &str,
    mut events: Vec<(String, Instant, Instant)>,
    start_time: Instant,
) {
    events.sort_by(|a, b| a.1.cmp(&b.1));

    if let Ok(mut outfile) = File::create(filename) {
        write!(outfile, "io_type,start,end,latency\n").unwrap();
        for val in events {
            let start = val.1.saturating_duration_since(start_time).as_nanos();
            let end = val.2.saturating_duration_since(start_time).as_nanos();
            write!(outfile, "{},{},{},{}\n", val.0, start, end, end - start).unwrap();
        }
    }
}
