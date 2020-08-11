// how to define relationships in rust? anything similar to oops
// normal trait
#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Hero {
    name: &'static str,
    health: u8,
    position: Position,
}

trait Player {
    fn new(name: &'static str) -> Self;
    fn walk(&mut self);
    fn shoot(&self) -> bool;
    fn heal(&self);
}

impl Player for Hero {
    fn new(name: &'static str) -> Hero {
        return Hero {
            name,
            health: 0,
            position: Position {
                x: 0,
                y: 0,
                z: 0,
            },
        };
    }

    fn walk(&mut self) {
        self.position.x += 1;
        self.position.y += 1;
        self.position.z += 1;
    }

    fn shoot(&self) -> bool {
        println!("shoot");
        return true;
    }

    fn heal(&self) {
        unimplemented!()
    }
}

pub fn test_trait() {
    let mut h: Hero = Player::new("luke");
    h.walk();
    h.shoot();
    println!("Position {:?}", h.position);
}

// supertrait

trait Person {
    fn name(&self) -> String;
}

// Student is a supertrait of Person.
// Implementing Student requires you to also impl Person.
trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a supertrait of both Programmer
// and Student. Implementing CompSciStudent requires you to impl both subtraits.
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

struct GradStudent {
    name: &'static str,
    university: &'static str,
    git_username: &'static str
}

impl Person for GradStudent {
    fn name(&self) -> String {
        return self.name.to_string();
    }
}

impl Student for GradStudent {
    fn university(&self) -> String {
        return self.university.to_string();
    }
}

impl Programmer for GradStudent {
    fn fav_language(&self) -> String {
        return "rust".to_string();
    }
}

impl CompSciStudent for GradStudent {
    fn git_username(&self) -> String { return self.git_username.to_string(); }
}

fn comp_sci_student_greeting(student: GradStudent) -> String {
    format!(
        "My name is {} and I attend {}. My Git username is {}",
        student.name(),
        student.university(),
        student.git_username()
    )
}

pub fn test_super_trait() {
    let student =  GradStudent {
        name: "Aritra",
        university: "Skywalker academy",
        git_username: "Ar11rA"
    };
    println!("{}", comp_sci_student_greeting(student));
}
