use std::collections::HashMap;
use std::io::Write;
use std::ops::Add;
use std::sync::MutexGuard;
use rand::Rng;
use rand::rngs::ThreadRng;
use armor::Armor;
use crate::armor;
use crate::player_manager::PlayerManager;

#[derive(Clone)]
pub struct Player
{
	weapons: HashMap<String, u32>,
	is_wearing_armor: bool,
	pub plr_name: String,
	pub plr_hp: u32,
	pub plr_armor: Armor
}

impl Player
{
	pub fn new(plr_name: String, plr_hp: u32, weapons: HashMap<String, u32>, plr_armor: &Armor) -> Player
	{
		Player
		{
			plr_name,
			plr_hp,
			weapons,
			is_wearing_armor: false,
			plr_armor: plr_armor.clone(),
		}
	}

	pub fn get_stats(&self)
	{
		println!("----GETTING STATS FOR {}----", self.plr_name);
		println!("Name: {}, Health: {}, Weapons:\n\n", self.plr_name, self.plr_hp);
		for (key, value) in &self.weapons
		{
			println!("{} : {}", key, value);
		}
		println!("\nTotal weapons count: {}", self.weapons.len());
		println!("----STATS END----");
	}

	pub fn update_weapons(&mut self, state_type: &str, value: (String, u32))
	{
		match state_type
		{
			"add" => {
				self.weapons.insert(value.0, value.1);
			}
			"remove" => {
				self.weapons.remove(&value.0);
			}
			_ => {}
		}
	}

	fn crit(weapon_dmg: u32, armor_resistance: u32) -> f64
	{
		let mut rng: ThreadRng = rand::rng();
		let random_number: u32 = rng.random_range(1..=4);

		let damage: f64 = if random_number == 1
		{
			(weapon_dmg as f64 / armor_resistance as f64) * 1.5
		}
		else
		{
			weapon_dmg as f64 / armor_resistance as f64
		};

		damage
	}

	pub fn attack(&self, target: &mut Player, weapon_name: &str)
	{
		let weapon_dmg_option: Option<&u32> = self.weapons.get(weapon_name);
		let mut weapon_dmg: u32 = 0;
		match weapon_dmg_option
		{
			Some(dmg) => weapon_dmg = *dmg,
			None => {
				std::io::stdout().flush().expect("Failed to flush stdout");
				println!("Weapon {weapon_name} does not exist")
			}
		}
		let armor_resistance: u32 = target.plr_armor.get_resistance();
		let damage: f64 = Self::crit(weapon_dmg, armor_resistance);
		target.plr_hp = target.plr_hp.saturating_sub(damage as u32);
		if target.plr_hp == 0
		{
			let mut plr_man: MutexGuard<PlayerManager> = PlayerManager::instance().lock().unwrap();
			plr_man.remove_player(target);
			println!("Player {} has died!", target.plr_name)
		}
	}

	pub fn heal(target: &mut Player, amount: u32)
	{
		target.plr_hp += amount;
	}

	pub fn equip_armor(&mut self, armor: Armor)
	{
		if self.is_wearing_armor { return; }
		self.plr_armor = armor;
		self.is_wearing_armor = true;
	}
}

impl Add for &Player
{
	type Output = Player;

	fn add(self, other: &Player) -> Player
	{
		Player
		{
			plr_name: format!("{}:{}", self.plr_name, other.plr_name),
			plr_hp: self.plr_hp + other.plr_hp,
			weapons: self.weapons.clone(),
			is_wearing_armor: self.is_wearing_armor,
			plr_armor: &self.plr_armor + &other.plr_armor,
		}
	}
}