use pad::{PadStr, Alignment};
trait Monster {
    fn shout(&self)-> String;
    fn attack(&self, other: &mut Monster, round: i32);
    fn hurt(&mut self, a: i32, round: i32);
    fn is_dead(&self) -> bool;
    fn star_check(&mut self);
    fn special(&self, round: i32)->bool;
}
struct Ninja {
    health: i32,
    attack: i32,
    wait: i32,
    name: String,
    level: i32,
    stars: i32,
}
struct Golem {
    health: i32,
    attack: i32,
    wait: i32,
    name: String,
    level: i32,
    stars: i32,
}
struct Team {
    monsters: Vec<Box<Monster>>,
    name: String
}
impl Team {
    fn new(name:String)-> Team{
        Team {
            monsters: Vec::new(),
            name: name,
        }
    }
    fn cleanup(&mut self) {
        self.monsters.retain(|m| !m.is_dead());
    }

    fn is_dead(&self) -> bool {
        if self.monsters.len() == 0 {
            return true;
        }
        return false;
    }
    fn attack(&self, other: &mut Team, round: i32) {
        for m in &self.monsters {
            m.attack(&mut *other.monsters[0], round);
            other.cleanup();
            if other.is_dead(){ return; }
        }
    }
}
impl Ninja {
    fn new(name: &str, level: i32) -> Ninja {
        Ninja {
            stars: 3,
            level: level,
            health: (level*1),
            attack: (level*5),
            wait: 1,
            name: name.to_string(),
        }
    }
}

impl Monster for Ninja {
    fn star_check(&mut self) {
        if self.level > self.stars * 10 {
            self.level = self.stars * 10;
        }
    }
    fn is_dead(&self) -> bool {
        if self.health <= 0 {
            return true;
        } else {
            return false;
        }
    }
    fn shout(&self)->String {
        format!("Ninja {} h:({}), a:({}), l({})", self.name, self.health, self.attack, self.level)
    }
    fn hurt(&mut self, a: i32, round: i32) {
        if !self.special(round){
            self.health -= a;
        }
    }
    fn attack(&self, other: &mut Monster, round: i32) {
        if round%self.wait == 0{
            other.hurt(self.attack, round);
        }
    }
    fn special (&self, round: i32)->bool{
        if round%self.wait == 0{
            if 3 % (self.wait + round%self.wait) == 0 {
                println!("ninja {} is cloaked! no damage", self.name);
                return true;
            }
        }
        return false;
    }
        
}
impl Golem {
    fn new(name: &str, level: i32) -> Golem {
        Golem {
            stars: 3,
            level: level,
            health: (level*55),
            attack: (level*7),
            wait: 6,
            name: name.to_string(),
        }
    }
}
impl Monster for Golem {
    fn star_check(&mut self) {
        if self.level > self.stars * 10 {
            self.level = self.stars * 10;
        }
    }
    fn is_dead(&self) -> bool {
        if self.health <= 0 {
            return true;
        } else {
            return false;
        }
    }
    fn shout(&self)->String {
        format!("Golem {} h:({}), a:({}), l:({})", self.name, self.health, self.attack, self.level)
    }
    fn hurt(&mut self, a: i32, round: i32) {
        self.health -= a;
    }
    fn attack(&self, other: &mut Monster, round: i32) {
        if round%self.wait == 0{
            other.hurt(self.attack, round);
        }
    }
    fn special(&self, round: i32)->bool{return true;}
}

fn main() {
    let mut team1 = Team::new("your team".to_string());
    let mut team2 = Team::new("enemy".to_string());
    let mut round = 0;
    team1.monsters.push(Box::new(Ninja::new("teddy", 30)));
    team1.monsters.push(Box::new(Ninja::new("blaise", 30)));
    team1.monsters.push(Box::new(Ninja::new("marcus",30)));
    team1.monsters.push(Box::new(Ninja::new("clara", 30)));
    team1.monsters.push(Box::new(Ninja::new("bea", 30)));

    team2.monsters.push(Box::new(Golem::new("teddy1",30)));
    team2.monsters.push(Box::new(Golem::new("teddy2",30)));
    team2.monsters.push(Box::new(Golem::new("teddy3",30)));
    team2.monsters.push(Box::new(Golem::new("teddy4",30)));
    team2.monsters.push(Box::new(Golem::new("teddy5",30)));
    print(&team1, &team2);
    while !team1.is_dead() && !team2.is_dead() {
        round += 1;
        println!("                                     round {}", round);
        team1.attack(&mut team2, round);
        team2.attack(&mut team1, round);
        print(&team1, &team2);
    }
    if team1.is_dead(){
        println!("you lose");
    }
    else{
        println!("you win");
    }
}
fn print(team1: &Team, team2: &Team) {
    println!("{}{}", team1.name.pad(40, ' ', Alignment::Left, true), team2.name.pad(40, ' ', Alignment::Right, true));
    let mut i = 0;
    loop {
        if i < team1.monsters.len(){
            print!("{}",team1.monsters[i].shout().pad(40, ' ', Alignment::Left, true));
        }
        else { 
            print!("                                        ");
        }
        if i < team2.monsters.len(){
            print!("{}",team2.monsters[i].shout().pad(40, ' ', Alignment::Right, true));
        }
        println!();
        if i > team1.monsters.len() && i > team2.monsters.len() {
            return;
        }
        i+=1;
    }
}
