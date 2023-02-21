
// importing rust standard libraries
use std::collections::HashMap;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::{Duration, Instant};

// Defining TTL as a constant
const CACHE_TTL: Duration = Duration::from_secs(30);

// Defining custom struct for creating cache-entry, for storing the cache data in hashmap

struct CacheEntry {
    data: Vec<u8>,
    timestamp: Instant,
}

struct Cache {
    data: HashMap<String, CacheEntry>,
}

// implementation of cache

impl Cache {
    fn new() -> Self{
        Self{
            data: HashMap::new(),
        }
    }

    fn get(&self, key: &str) -> Option<&CacheEntry> {
        self.data.get(key).filter(|entry|{
            entry.timestamp + CACHE_TTL > Instant::now()
        })
    }

    fn put(&mut self, key: String, data: Vec<u8>) {
        self.data.insert(key, CacheEntry { data, timestamp: Instant::now() });
    }
}

// creating a function for handeling the client

fn handle_client(mut stream: TcpStream, cache: &mut Cache) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let mut lines = request.lines();

    let first_line = lines.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let _method = parts.next().unwrap();
    let path = parts.next().unwrap();
    let _version = parts.next().unwrap();

    let mut headers = String::new();
    for line in lines {
        if line == "\r" {
            break;
        }
        headers.push_str(line);
    }

    let mut origin_stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    origin_stream.write_all(request.as_bytes()).unwrap();
    origin_stream.flush().unwrap();

    let mut origin_response = Vec::new();
    origin_stream.read_to_end(&mut origin_response).unwrap();

    let response = origin_response.clone();
    cache.put(path.to_string(), response);

    stream.write_all(&origin_response).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").unwrap();
    let mut cache = Cache::new();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                if let Some(entry) = cache.get("/") {
                    stream.write_all(&entry.data).unwrap();
                } else {
                    handle_client(stream.try_clone().unwrap(), &mut cache);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}