mod kitty;
mod card;

use kitty::Kitty;



fn regen_kittys_mana(kittys: &mut Vec<Kitty>) {
    for kitty in kittys {
        kitty.regen_mana();
    }
}


fn game_loop() {
    let mut kittys = vec![Kitty::new(), Kitty::new()];


    loop {
        for kitty in kittys.iter_mut() {
            kitty.regen_mana();
        }
    }
}


fn main() {
    println!("Hello, world!");
}
