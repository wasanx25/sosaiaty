use cards::card::Card;
mod cards;

fn main() {
    let gargoyle = cards::card::Monster {
        name: "goagoyle".to_string(),
        level: 3,
        effect: "".to_string()
    };
    println!("name: {}, level: {}, effect: {effect}",
             gargoyle.get_name(),
             gargoyle.get_level(),
             effect=gargoyle.get_effect());

    let grinder_golem = cards::card::Monster {
        name: "grinder golem".to_string(),
        level: 8,
        effect: "Generate Grinder tokens on my field.".to_string()
    };
    println!("name: {}, level: {}, effect: {}",
             grinder_golem.get_name(),
             grinder_golem.get_level(),
             grinder_golem.get_effect());

    let ritual_sanctuary = cards::card::Magic {
        name: "ritual sanctuary".to_string(),
        spell: cards::card::MagicSpell::FieldSpell,
        effect: "Search RitualSpell Magic Card.".to_string()
    };
    println!("name: {}, card_type: {}, effect: {}",
             ritual_sanctuary.get_name(),
             ritual_sanctuary.get_spell(),
             ritual_sanctuary.get_effect());
}
