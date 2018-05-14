fn main() {
    let gargoyle = Monster {
        name: "goagoyle".to_string(),
        level: 3,
        effect: "".to_string()
    };
    println!("name: {}, level: {}, effect: {effect}",
             gargoyle.get_name(),
             gargoyle.get_level(),
             effect=gargoyle.get_effect());
}

trait Card {
    fn get_name(&self) -> String;
    fn get_level(&self) -> u8;
    fn get_effect(&self) -> String;
}

struct Monster {
    name: String,
    level: u8,
    effect: String,
}

impl Card for Monster {
    fn get_name(&self) -> String {
        return self.name.to_owned();
    }
    fn get_level(&self) -> u8 {
        return self.level.to_owned();
    }
    fn get_effect(&self) -> String {
        return self.effect.to_owned();
    }
}
