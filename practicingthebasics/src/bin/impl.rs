struct Sword {
    sword_name: String,
    sword_damage: u32,
    sword_type: String,
}

impl Sword {
    fn bask_sword() -> Self {
        Self {
            sword_name: String::from("Baskerville's Requim"),
            sword_damage: 200,
            sword_type: String::from("Broadsword"),
        }
    }

    fn sword_info(&self) {
        println!("Sword Name: {}", self.sword_name);
        println!("Sword Damage: {}", self.sword_damage);
        println!("Sword Type: {}", self.sword_type);
    }
}
fn main() {
    let s1 = Sword {
        sword_name: String::from("Shrek's Sword"),
        sword_damage: 999,
        sword_type: String::from("World Destroyer"),
    };

    let s2 = Sword::bask_sword();
    s2.sword_info();
    s1.sword_info();
}
