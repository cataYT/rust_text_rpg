use std::ops::Add;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Armor
{
	resistance_force: u32,
	armor_hp: u32,
	armor_max_hp: u32,
	pub armor_name: String
}

impl Armor
{
	pub fn new(armor_name: String, resistance_force: u32, armor_hp: u32, armor_max_hp: u32) -> Result<Armor, String>
	{
		if armor_hp > armor_max_hp { return Err(String::from("Armor HP is greater than max HP")) }

		Ok(Armor {
			armor_name,
			resistance_force,
			armor_hp,
			armor_max_hp,
		})
	}

	pub fn enhance_armor(&mut self, armor_hp: u32) { self.armor_hp += armor_hp; }

	pub fn check_hp(&self) -> u32 { self.armor_hp }

	pub fn get_resistance(&self) -> u32 { self.resistance_force }

	pub fn repair_armor(&mut self, repair_amount: u32)
	{
		if self.armor_hp + repair_amount > self.armor_max_hp { return; }
		self.armor_hp += repair_amount;
	}
	
	pub fn get_stats(&self)
	{
		println!("Armor name: {}", self.armor_name);
		println!("Armor hp: {}", self.armor_hp);
		println!("Armor max hp: {}", self.armor_max_hp);
		println!("Armor resistance: {}", self.resistance_force);
	}
}

impl Add for &Armor 
{
	type Output = Armor;

	fn add(self, other: &Armor) -> Armor
	{
		Armor
		{
			armor_name: format!("{}:{}", self.armor_name, other.armor_name),
			resistance_force: self.resistance_force + other.resistance_force,
			armor_hp: self.armor_hp + other.armor_hp,
			armor_max_hp: self.armor_max_hp + other.armor_max_hp,
		}
	}
}