pub trait Card {
    fn get_name(&self) -> String;
    fn get_effect(&self) -> String;
}

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

pub struct Magic {
    pub name: String,
    pub spell: MagicSpell,
    pub effect: String,
}

pub enum MagicSpell {
    QuickPlaySpell,
    FieldSpell,
    EquipSpell,
    NormalSpell,
    RitualSpell,
    ContinuousSpell,
}

impl Magic {
    pub fn get_spell(&self) -> String {
        match self.spell {
            MagicSpell::QuickPlaySpell => "Quick-Play Spell".to_string(),
            MagicSpell::FieldSpell => "Field Spell".to_string(),
            MagicSpell::EquipSpell => "Eauip Spell".to_string(),
            MagicSpell::NormalSpell => "Normal Spell".to_string(),
            MagicSpell::RitualSpell => "Ritual Spell".to_string(),
            MagicSpell::ContinuousSpell => "Continuous Spell".to_string(),
        }
    }
}

impl Card for Magic {
    fn get_name(&self) -> String {
        return self.name.to_owned();
    }
    fn get_effect(&self) -> String {
        return self.effect.to_owned();
    }
}
