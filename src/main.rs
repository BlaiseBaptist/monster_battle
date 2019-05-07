use pad::{Alignment, PadStr};
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
    let mut team1 = Team::new("your team".to_string());
    let mut team2 = Team::new("enemy".to_string());
    let mut round = 0;
    team1.monsters.push(Box::new(Ninja::new(30)));
    /*    team1.monsters.push(Box::new(Ninja::new(30)));
        team1.monsters.push(Box::new(Ninja::new(30)));
        team1.monsters.push(Box::new(Ninja::new(30)));
        team1.monsters.push(Box::new(Ninja::new(30)));

        team2.monsters.push(Box::new(Golem::new(30)));
        team2.monsters.push(Box::new(Golem::new(30)));
        team2.monsters.push(Box::new(Golem::new(30)));
        team2.monsters.push(Box::new(Golem::new(30)));
    */
    team2.monsters.push(Box::new(Golem::new(30)));
    print(&team1, &team2);
    while !team1.is_dead() && !team2.is_dead() {
        round += 1;
        println!("                                     round {}", round);
        team1.attack(&mut team2, round);
        team2.attack(&mut team1, round);
        print(&team1, &team2);
    }
    if team1.is_dead() {
        println!("you lose");
    } else {
        println!("you win");
    }
}
fn print(team1: &Team, team2: &Team) {
    println!(
        "{}{}",
        team1.name.pad(40, ' ', Alignment::Left, true),
        team2.name.pad(40, ' ', Alignment::Right, true)
    );
    let mut i = 0;
    loop {
        if i < team1.monsters.len() {
            print!(
                "{}",
                team1.monsters[i]
                    .shout()
                    .pad(40, ' ', Alignment::Left, true)
            );
        } else {
            print!("                                        ");
        }
        if i < team2.monsters.len() {
            print!(
                "{}",
                team2.monsters[i]
                    .shout()
                    .pad(40, ' ', Alignment::Right, true)
            );
        }
        println!();
        if i > team1.monsters.len() && i > team2.monsters.len() {
            return;
        }
        i += 1;
    }
}
