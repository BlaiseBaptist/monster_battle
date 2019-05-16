#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use pad::{Alignment, PadStr};
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::MutexGuard;
use std::sync::{Arc, Mutex};
use std::{str, thread};
trait Monster: Send {
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
struct Player {
    team: Team,
    money: u32,
    name: String,
    level: u32,
    id: u128,
}
struct Ninja {
    logic: Logic,
}
struct Golem {
    logic: Logic,
}
struct Team {
    monsters: Vec<Box<Monster>>,
}
impl Team {
    fn new() -> Team {
        Team {
            monsters: Vec::new(),
        }
    }
    fn cleanup(&mut self) {
        self.monsters.retain(|m| !m.is_dead());
    }
    fn is_dead(&self) -> bool {
        self.monsters.is_empty()
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
impl Player {
    fn new(name: String, money: u32, id: u128) -> Player {
        Player {
            team: Team::new(),
            money,
            name,
            id,
            level: 1,
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
    fn hurt(&mut self, a: i32, _round: i32) {
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
            level,
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
    fn special(&self, _round: i32) -> bool {
        false
    }
}
impl Golem {
    fn new(level: i32) -> Golem {
        let logic = Logic {
            stars: 3,
            level,
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
    fn special(&self, _round: i32) -> bool {
        false
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
    let listener = TcpListener::bind("0.0.0.0:4444").expect("Could not bind");
    let mut players = HashMap::new();
    let mut next_id: u128 = 1;
    for stream in listener.incoming() {
        match stream {
            Err(_e) => continue,
            Ok(stream) => {
                let (client_s, server_r) = channel();
                let (server_s, client_r) = channel();
                create_client_proxy(stream, client_s, client_r);
                let player = Arc::new(Mutex::new(Player::new("blaise".to_string(), 100, next_id)));
                let player_clone = Arc::clone(&player);
                players.insert(next_id, player);
                thread::spawn(move || loop {
                    let request = server_r.recv().unwrap();
                    let player_clone = player_clone.lock().unwrap();
                    server_s
                        .send(process_request(request, player_clone))
                        .unwrap();
                });
            }
        }
        next_id += 1;
    }
}
fn process_request(request: Request, mut player: MutexGuard<Player>) -> Response {
    match request {
        Request::Battle => {
            let mut team2 = set_up();
            if !player.team.monsters.is_empty() {
                if battle(&mut player.team, &mut team2) {
                    Response::Battle(Ok(false))
                } else {
                    Response::Battle(Ok(true))
                }
            } else {
                Response::Battle(Err("you have no monsters go buy some!".to_string()))
            }
        }
        Request::AddMonster(data) => {
            if player.team.monsters.len() != 5 {
                match data {
                    1 => player.team.monsters.push(Box::new(Ninja::new(1))),
                    2 => player.team.monsters.push(Box::new(Golem::new(1))),
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
    let mut team = Team::new();
    team.monsters.push(Box::new(Ninja::new(30)));
    team.monsters.push(Box::new(Ninja::new(30)));
    team.monsters.push(Box::new(Ninja::new(30)));
    team.monsters.push(Box::new(Ninja::new(30)));
    team.monsters.push(Box::new(Ninja::new(30)));
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
        stream.read_until(b'\n', &mut data).unwrap();
        let imput: Request = serde_json::from_slice(&data).unwrap();
        send.send(imput).unwrap();
        let rres = recv.recv().unwrap();
        let res = serde_json::to_string(&rres).unwrap();
        writeln!(stream.get_mut(), "{}", res).unwrap();
    });
}
fn display(team1: &Team, team2: &Team) -> String {
    let mut print = String::new();
    let mut i = 0;
    loop {
        print.push_str(&format!(
            "round: {}{}",
            (i + 1),
            "".pad(40, ' ', Alignment::Left, true)
        ));
        if i < team1.monsters.len() {
            print.push_str(
                &team1.monsters[i]
                    .shout()
                    .pad(40, ' ', Alignment::Left, true),
            );
        } else {
            print.push_str(&"".pad(40, ' ', Alignment::Left, true));
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
