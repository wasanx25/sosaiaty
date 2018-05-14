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

    let grinder_golem = Monster {
        name: "grinder golem".to_string(),
        level: 8,
        effect: "Generate Grinder tokens on my field.".to_string()
    };
    println!("name: {}, level: {}, effect: {}",
             grinder_golem.get_name(),
             grinder_golem.get_level(),
             grinder_golem.get_effect());

    let ritual_sanctuary = Magic {
        name: "ritual sanctuary".to_string(),
        spell: MagicSpell::FieldSpell,
        effect: "Search RitualSpell Magic Card.".to_string()
    };
    println!("name: {}, effect: {}, card_type: {}",
             ritual_sanctuary.get_name(),
             ritual_sanctuary.get_effect(),
             ritual_sanctuary.get_spell());
}

trait Card {
    fn get_name(&self) -> String;
    fn get_effect(&self) -> String;
}

struct Monster {
    name: String,
    level: u8,
    effect: String,
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

struct Magic {
    name: String,
    spell: MagicSpell,
    effect: String,
}

enum MagicSpell {
    QuickPlaySpell,
    FieldSpell,
    EquipSpell,
    NormalSpell,
    RitualSpell,
    ContinuousSpell,
}

impl MagicSpell {
    pub fn get_spell_name(self) -> String {
        match self {
            MagicSpell::QuickPlaySpell => "Quick-Play Spell".to_string(),
            MagicSpell::FieldSpell => "Field Spell".to_string(),
            MagicSpell::EquipSpell => "Eauip Spell".to_string(),
            MagicSpell::NormalSpell => "Normal Spell".to_string(),
            MagicSpell::RitualSpell => "Ritual Spell".to_string(),
            MagicSpell::ContinuousSpell => "Continuous Spell".to_string(),
        }
    }
}

impl Magic {
    pub fn get_spell(self) -> String {
        return self.spell.get_spell_name();
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
