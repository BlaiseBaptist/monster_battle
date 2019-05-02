/*
        todo
*   give team name and print it
*   level
*   player
*   main screen
*   pvc
*   pvp
*   money
*   save
*   make more simple monsters
*   buy more monsters
*   monster ability
*   inventory
*   items
*   buy items
*   make more complated monsters
*   GRAFICS!
*

*/

trait Monster {
    fn shout(&self);
    fn attack(&self, other: &mut Monster, round: i32);
    fn hurt(&mut self, a: i32);
    fn is_dead(&self) -> bool;
}
struct Ninja {
    health: i32,
    attack: i32,
    wait: i32,
    name: String,
}
struct Golem {
    health: i32,
    attack: i32,
    wait: i32,
    name: String,
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
    fn print(&self) {
        for (n, m) in self.monsters.iter().enumerate() {
            print!("{}'s monster {} is a ",self.name, n + 1);
            m.shout();
        }
        println!();
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
    fn new(name: &str) -> Ninja {
        Ninja {
            health: 1,
            attack: 5,
            wait: 1,
            name: name.to_string(),
        }
    }
}

impl Monster for Ninja {
    fn is_dead(&self) -> bool {
        if self.health <= 0 {
            return true;
        } else {
            return false;
        }
    }
    fn shout(&self) {
        println!("Ninja {} ({})", self.name, self.health);
    }
    fn hurt(&mut self, a: i32) {
        self.health -= a;
    }
    fn attack(&self, other: &mut Monster, round: i32) {
        if round%self.wait == 0{
            other.hurt(self.attack);
        }
    }
}

impl Golem {
    fn new(name: &str) -> Golem {
        Golem {
            health: 50,
            attack: 10,
            wait: 15,
            name: name.to_string(),
        }
    }
}
impl Monster for Golem {
    fn is_dead(&self) -> bool {
        if self.health <= 0 {
            return true;
        } else {
            return false;
        }
    }
    fn shout(&self) {
        println!("Golem {} ({})", self.name, self.health);
    }
    fn hurt(&mut self, a: i32) {
        self.health -= a;
    }
    fn attack(&self, other: &mut Monster, round: i32) {
        if round%self.wait == 0{
            other.hurt(self.attack);
        }
    }
}

fn main() {
    let mut team1 = Team::new("team1".to_string());
    let mut team2 = Team::new("team2".to_string());
    let mut round = 0;
    team1.monsters.push(Box::new(Golem::new("teddy")));
    team1.monsters.push(Box::new(Ninja::new("blaise")));
    team1.monsters.push(Box::new(Ninja::new("marcus")));
    team1.monsters.push(Box::new(Ninja::new("clara")));
    team1.monsters.push(Box::new(Ninja::new("bea")));

    team2.monsters.push(Box::new(Golem::new("teddy1")));
    team2.monsters.push(Box::new(Golem::new("teddy2")));
    team2.monsters.push(Box::new(Golem::new("teddy3")));
    team2.monsters.push(Box::new(Golem::new("teddy4")));
    team2.monsters.push(Box::new(Golem::new("teddy5")));
    team2.monsters.push(Box::new(Golem::new("teddy6")));
    team2.monsters.push(Box::new(Golem::new("teddy7")));
    team2.monsters.push(Box::new(Golem::new("teddy8")));
    team2.monsters.push(Box::new(Golem::new("teddy9")));
    team2.monsters.push(Box::new(Golem::new("teddy10")));
    team1.print();
    team2.print();
    while !team1.is_dead() && !team2.is_dead() {
        round += 1;
        println!("round {}", round);
        team1.attack(&mut team2, round);
        team2.attack(&mut team1, round);
        team1.print();
        team2.print();
    }
}
