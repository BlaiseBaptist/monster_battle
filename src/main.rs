use pad::{Alignment, PadStr};
use std::io;
use std::sync::mpsc::channel;
use std::thread;
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
        if round % self.logic.wait == 0 {
            if 3 % (self.logic.wait + round % self.logic.wait) == 0 {
                println!("{} is cloaked!", self.logic.name);
                return true;
            }
        }
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

fn main() {
    let (client_s, server_r) = channel();
    let (server_s, client_r) = channel();
    thread::spawn(move || loop {
        let op = get_data("1: battle\n2: buy monster".to_string(),1,2);
        if op == 2 {
            client_s.send((op, get_data("1: Ninja\n2: Golem".to_string(),1,2)));
        }
        else {
            client_s.send((op, 0));
            println!("{}",client_r.recv().unwrap());
        }
        print!("{}", client_r.recv().unwrap());
        wait_for_key();
        print!("{}[2J", 27 as char);
    });
    let mut team1 = Team::new("you".to_string());
    loop {
        let (op, data) = server_r.recv().unwrap();
        println!("op: {}, data: {}", op, data);
        let result = 
        match op {
            1 => {
                let mut team2 = set_up();
                if !team1.monsters.len() == 0 {
                    server_s.send(print(&team1, &team2));
                    if battle(&mut team1, &mut team2) {
                        "you lose"
                    } else {
                        "you win"
                    }
                }
                else {
                    server_s.send("you lose".to_string());
                    "you have no monsters, go buy some"
                }
            }
            2 => { 
                if team1.monsters.len() != 5 {
                    match data {
                        1 => team1.monsters.push(Box::new(Ninja::new(30))),
                        2 => team1.monsters.push(Box::new(Ninja::new(30))),
                        _ => {},
                        };
                    "added"
                }
                else {
                    "you have 5 please battle"
                }
            }
            _ => "HOW!?!?!?!"
        };
        server_s.send(result.to_string());
    }
}
fn wait_for_key() {
    let mut nothing = String::new();
    println!(" (press enter to continue)");
    io::stdin().read_line(&mut nothing);
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
fn set_up() -> (Team) {
    let mut team = Team::new("enemy".to_string());
    team.monsters.push(Box::new(Ninja::new(30)));
    team.monsters.push(Box::new(Ninja::new(30)));
    team.monsters.push(Box::new(Ninja::new(30)));
    team.monsters.push(Box::new(Ninja::new(30)));
    team.monsters.push(Box::new(Ninja::new(30)));
    (team)
}
fn battle(mut team1: &mut Team, mut team2: &mut Team) -> bool {
    let mut round = 0;
    print(&team1, &team2);
    while !team1.is_dead() && !team2.is_dead() {
        round += 1;
        println!("                                     round {}", round);
        team1.attack(&mut team2, round);
        team2.attack(&mut team1, round);
        print(&team1, &team2);
    }
    team1.is_dead()
}
fn print(team1: &Team, team2: &Team)->String {
    let mut print = String::new();
    print.push_str(&format!(
        "{}{}",
        team1.name.pad(40, ' ', Alignment::Left, true),
        team2.name.pad(40, ' ', Alignment::Right, true)
    ));
    let mut i = 0;
    loop {
        if i < team1.monsters.len() {
            print.push_str(&format!(
                "{}",
                team1.monsters[i]
                    .shout()
                    .pad(40, ' ', Alignment::Left, true)
            ));
        } else {
            print.push_str(&format!("{}","".pad(40, ' ', Alignment::Left, true)));
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
