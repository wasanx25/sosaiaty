use cards::card::Card;

pub struct Monster {
    pub name: String,
    pub level: u8,
    pub effect: String,
}

impl Monster {
    pub fn get_level(&self) -> u8 {
        return self.level.to_owned();
    }
}

impl Card for Monster {
    fn get_name(&self) -> String {
        return self.name.to_owned();
    }
    fn get_effect(&self) -> String {
        return self.effect.to_owned();
    }
}
