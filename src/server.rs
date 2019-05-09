#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use pad::{Alignment, PadStr};
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{str, thread};
trait Monster {
    fn get_logic(&self) -> &Logic;
    fn get_mut_logic(&mut self) -> &mut Logic;
    fn shout(&self) -> String {
        self.get_logic().shout()
    }
    fn attack(&self, other: &mut Monster, round: i32) {
        self.get_logic().attack(other, round)
    }
    fn hurt(&mut self, amount: i32, round: i32) {
        if !self.special(round) {
            self.get_mut_logic().hurt(amount, round)
        }
    }
    fn is_dead(&self) -> bool {
        self.get_logic().is_dead()
    }
    fn star_check(&mut self) {
        self.get_mut_logic().star_check()
    }
    fn special(&self, round: i32) -> bool;
}
struct Logic {
    health: i32,
    attack: i32,
    wait: i32,
    name: String,
    level: i32,
    stars: i32,
}
struct Ninja {
    logic: Logic,
}
struct Golem {
    logic: Logic,
}
struct Team {
    monsters: Vec<Box<Monster>>,
    name: String,
}
impl Team {
    fn new(name: String) -> Team {
        Team {
            monsters: Vec::new(),
            name: name,
        }
    }
    fn cleanup(&mut self) {
        self.monsters.retain(|m| !m.is_dead());
    }

    fn is_dead(&self) -> bool {
        self.monsters.len() == 0
    }
    fn attack(&self, other: &mut Team, round: i32) {
        for m in &self.monsters {
            m.attack(&mut *other.monsters[0], round);
            other.cleanup();
            if other.is_dead() {
                return;
            }
        }
    }
}
impl Logic {
    fn star_check(&mut self) {
        if self.level > self.stars * 10 {
            self.level = self.stars * 10;
        }
    }
    fn is_dead(&self) -> bool {
        self.health <= 0
    }
    fn shout(&self) -> String {
        format!(
            "{} h:({}), a:({}), l({})",
            self.name, self.health, self.attack, self.level
        )
    }
    fn hurt(&mut self, a: i32, round: i32) {
        self.health -= a;
    }
    fn attack(&self, other: &mut Monster, round: i32) {
        if round % self.wait == 0 {
            other.hurt(self.attack, round);
        }
    }
}
impl Ninja {
    fn new(level: i32) -> Ninja {
        let logic = Logic {
            stars: 3,
            level: level,
            health: (level * 1),
            attack: (level * 5),
            wait: 1,
            name: "Ninja".to_string(),
        };
        Ninja { logic }
    }
}

impl Monster for Ninja {
    fn get_logic(&self) -> &Logic {
        &self.logic
    }
    fn get_mut_logic(&mut self) -> &mut Logic {
        &mut self.logic
    }
    fn special(&self, round: i32) -> bool {
        return false;
    }
}
impl Golem {
    fn new(level: i32) -> Golem {
        let logic = Logic {
            stars: 3,
            level: level,
            health: (level * 55),
            attack: (level * 7),
            wait: 6,
            name: "Golem".to_string(),
        };
        Golem { logic }
    }
}
impl Monster for Golem {
    fn get_logic(&self) -> &Logic {
        &self.logic
    }
    fn get_mut_logic(&mut self) -> &mut Logic {
        &mut self.logic
    }
    fn special(&self, round: i32) -> bool {
        return false;
    }
}

#[derive(Serialize, Deserialize)]
enum Request {
    Battle,
    AddMonster(u32),
}
#[derive(Serialize, Deserialize)]
enum Response {
    Battle(Result<bool, String>),
    AddMonster(Result<(), String>),
}
fn main() {
    let (client_s, server_r) = channel();
    let mut team1 = Team::new("you".to_string());
    let listener = TcpListener::bind("0.0.0.0:55555").expect("Could not bind");
    for stream in listener.incoming() {
        match stream {
            Err(e) => continue,
            Ok(stream) => {
                let (server_s, client_r) = channel();
                create_client_proxy(stream, client_s, client_r);
                loop {
                    let request = server_r.recv().unwrap();
                    server_s.send(process_request(request, &mut team1)).unwrap();
                }
            }
        }
    }
}
fn process_request(request: Request, mut team1: &mut Team) -> Response {
    match request {
        Request::Battle => {
            let mut team2 = set_up();
            if team1.monsters.len() != 0 {
                if battle(&mut team1, &mut team2) {
                    Response::Battle(Ok(false))
                } else {
                    Response::Battle(Ok(true))
                }
            } else {
                Response::Battle(Err("you have no monsters go buy some!".to_string()))
            }
        }
        Request::AddMonster(data) => {
            if team1.monsters.len() != 5 {
                match data {
                    1 => team1.monsters.push(Box::new(Ninja::new(1))),
                    2 => team1.monsters.push(Box::new(Golem::new(1))),
                    _ => {}
                };
                Response::AddMonster(Ok(()))
            } else {
                Response::AddMonster(Err("you have 5 please battle".to_string()))
            }
        }
    }
}
fn set_up() -> (Team) {
    let mut team = Team::new("enemy".to_string());
    team.monsters.push(Box::new(Ninja::new(1)));
    team.monsters.push(Box::new(Ninja::new(1)));
    team.monsters.push(Box::new(Ninja::new(1)));
    team.monsters.push(Box::new(Ninja::new(1)));
    team.monsters.push(Box::new(Ninja::new(1)));
    (team)
}
fn battle(mut team1: &mut Team, mut team2: &mut Team) -> bool {
    let mut round = 0;
    display(&team1, &team2);
    while !team1.is_dead() && !team2.is_dead() {
        round += 1;
        team1.attack(&mut team2, round);
        team2.attack(&mut team1, round);
        display(&team1, &team2);
    }
    team1.is_dead()
}
fn create_client_proxy(stream: TcpStream, send: Sender<Request>, recv: Receiver<Response>) {
    let mut stream = BufReader::new(stream);
    thread::spawn(move || loop {
        let mut data = Vec::new();
        stream.read_until(b'\n', &mut data);
        let imput: Request = serde_json::from_slice(&data).unwrap();
        send.send(imput);
        let rres = recv.recv().unwrap();
        let res = serde_json::to_string(&rres).unwrap();
        write!(stream.get_mut(), "{}\n", res).unwrap();
    });
}
fn display(team1: &Team, team2: &Team) -> String {
    let mut print = String::new();
    print.push_str(&format!(
        "{}{}",
        team1.name.pad(40, ' ', Alignment::Left, true),
        team2.name.pad(40, ' ', Alignment::Right, true)
    ));
    let mut i = 0;
    loop {
        print.push_str(&format!(
            "round: {}{}",
            (i + 1),
            "".pad(40, ' ', Alignment::Left, true)
        ));
        if i < team1.monsters.len() {
            print.push_str(&format!(
                "{}",
                team1.monsters[i]
                    .shout()
                    .pad(40, ' ', Alignment::Left, true)
            ));
        } else {
            print.push_str(&format!("{}", "".pad(40, ' ', Alignment::Left, true)));
        }
        if i < team2.monsters.len() {
            print.push_str(&format!(
                "{}\n",
                team2.monsters[i]
                    .shout()
                    .pad(40, ' ', Alignment::Right, true)
            ));
        }
        if i > team1.monsters.len() && i > team2.monsters.len() {
            return print;
        }
        i += 1;
    }
}