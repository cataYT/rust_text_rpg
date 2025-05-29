use std::{
	io::Write,
	sync::{Mutex, MutexGuard}
};

use crate::{
	armor::Armor,
	weapon::Weapon,
	player::Player,
	player_manager::PlayerManager
};

mod player;
mod armor;
mod player_manager;
mod weapon;

fn get_input(input: &mut String)
{
	std::io::stdout().flush().expect("Failed to flush stdout");
	std::io::stdin()
		.read_line(input)
		.unwrap();
}

fn main()
{
	let mut player_name: String = String::new();
	print!("Enter player name: ");
	get_input(&mut player_name);
	player_name = player_name.trim().to_string();

	let mut weapon_name: String = String::new();
	print!("Enter weapon name: ");
	get_input(&mut weapon_name);
	weapon_name = weapon_name.trim().to_string();

	let mut weapon_dmg: u32 = 0;
	let mut weapon_dmg_input: String = String::new();

	print!("Enter weapon damage: ");
	get_input(&mut weapon_dmg_input);
	weapon_dmg_input = weapon_dmg_input.trim().to_string();

	match weapon_dmg_input.parse::<u32>()
	{
		Ok(n) => weapon_dmg = n,
		Err(e) => eprintln!("Failed to parse weapon damage: {}", e)
	}
	drop(weapon_dmg_input);

	let basic_armor: Armor = Armor::new("BASIC".to_string(), 10, 1, 100)
		.unwrap();

	let mut main_plr: Player = Player::new(player_name, 100, Weapon::new(weapon_name, 100, weapon_dmg), &basic_armor);

	let plr_man: &Mutex<PlayerManager> = PlayerManager::instance();
	let mut plr_man: MutexGuard<PlayerManager> = plr_man.lock().unwrap();
	plr_man.add_player(main_plr.clone());

	let mut enemy: Player = Player::new("Enemy".to_string(), 50, Weapon::new("Sword".to_string(), 10, 10), &basic_armor);
	plr_man.get_total_players();

	enemy.attack(&mut main_plr, "Sword");

	let mut weapon_use: String = String::new();
	print!("Enter weapon name to use: ");
	get_input(&mut weapon_use);

	main_plr.attack(&mut enemy, &weapon_use.trim());
	drop(weapon_use);
	let winner: &Player = plr_man.get_winner().unwrap();

	println!("Winner is: {}!", winner.plr_name);
}