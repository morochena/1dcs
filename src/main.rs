extern crate rand;

use std::io;
use rand::Rng;

struct Entity {
	health: i32,
	attack: i32,
	accuracy: i32,
}

fn main() {
	println!("Welcome to the 1d combat simulator!");

	println!("Input the number of humans:");

	let mut human_army_size = String::new();
	let mut goblin_army_size = String::new();
	let mut human_army: Vec<Entity> = Vec::new();
	let mut goblin_army: Vec<Entity> = Vec::new();

	io::stdin().read_line(&mut human_army_size)
		.ok()
		.expect("Failed to read line");

	println!("Input number of goblins:");

	io::stdin().read_line(&mut goblin_army_size)
		.ok()
		.expect("Failed to read line.");

	let human_army_size: u32 = human_army_size.trim().parse()
		.ok()
		.expect("Failed to convert");


	let goblin_army_size: u32 = goblin_army_size.trim().parse()
		.ok()
		.expect("Failed to convert");

	println!("Filling up the ranks...");

	for _x in 0..human_army_size {
		human_army.push(Entity { health: 100, attack: 30, accuracy: 60 })
	}

	for _x in 0..goblin_army_size {
		goblin_army.push(Entity { health: 60, attack: 30, accuracy: 90 })
	}

	println!("Beginning combat!");

	loop {
		// two champions come out to battle

		let mut human = human_army.pop().unwrap();
		let mut goblin = goblin_army.pop().unwrap();

		// they battle to the death
		loop {
			let chance = rand::thread_rng().gen_range(1, 101);

			if chance < human.accuracy {
				goblin.health -= human.attack;
			}

			if chance < goblin.accuracy {
				human.health -= goblin.attack;
			}

			if (human.health < 1) || (goblin.health < 1) {
				if human.health > 0	{
					human_army.push(human);
					println!("A human lives to fight again!");
				}

				if goblin.health > 0 {
					goblin_army.push(goblin);
					println!("The goblin victor returns to his army.");
				}
				println!("There are {} humans and {} goblins left.", human_army.len(), goblin_army.len() );

				break;
			}
		}

	}

	println!("Combat ended!");

}
