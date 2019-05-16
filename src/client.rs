#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
//use pad::{Alignment, PadStr};
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

#[derive(Serialize, Deserialize)]
enum Request {
    Battle,
    AddMonster(u32),
}
fn main() {
    let mut stream = TcpStream::connect("10.0.1.99:4444").expect("fail");
    loop {
        let op = get_data("1: battle\n2: buy monster".to_string(), 1, 2);
        let rdata = if op == 2 {
            Request::AddMonster(get_data("1: Ninja\n2: Golem".to_string(), 1, 2))
        } else {
            Request::Battle
        };
        let data = serde_json::to_string(&rdata).unwrap();
        stream.write_all(data.as_bytes()).expect("fail");
        stream.write_all(&[b'\n']).unwrap();
        let mut reader = BufReader::new(&stream);
        let mut buffer: Vec<u8> = Vec::new();
        reader.read_until(b'\n', &mut buffer).unwrap();
        let input = str::from_utf8(&buffer).expect("fail");
        println!("{}", input);
        wait_for_key();
        print!("{}[2J", 27 as char);
    }
}
fn wait_for_key() {
    let mut nothing = String::new();
    println!("(press enter to continue)");
    io::stdin().read_line(&mut nothing).unwrap();
}
fn get_data(message: String, lb: u32, ub: u32) -> u32 {
    loop {
        println!("{}", message);
        let mut snum = String::new();
        io::stdin().read_line(&mut snum).expect("fail");
        let snum: u32 = match snum.trim().parse() {
            Ok(snum) => snum,
            Err(_) => continue,
        };
        if snum > ub || snum < lb {
            println!("bad choice, choose between {} and {}!!", lb, ub);
            continue;
        }
        return snum;
    }
}
