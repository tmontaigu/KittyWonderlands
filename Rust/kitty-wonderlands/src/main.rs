extern crate rand;


mod kitty;
mod card;
mod deck;

use kitty::Kitty;



fn regen_kittys_mana(kittys: &mut Vec<Kitty>) {
    for kitty in kittys {
        kitty.regen_mana();
    }
}


fn game_loop() {
    let mut kittys = vec![Kitty::new(), Kitty::new()];
    for kitty in kittys.iter_mut() {
        kitty.create_deck();
        kitty.fill_hand();
    }

    loop {
        for kitty in kittys.iter_mut() {
            kitty.regen_mana();
        }
    }
}


fn main() {
    println!("Hello, world!");
    game_loop();
}
