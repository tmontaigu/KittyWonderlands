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
        regen_kittys_mana(&mut kittys);
        unimplemented!();
    }
}


fn main() {
    println!("Hello, world!");
}
