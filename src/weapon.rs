#[derive(Clone)]
pub struct Weapon
{
	name: String,
	weapon_health: u32,
	damage: u32,
}

impl Weapon
{
	pub fn new(name: String, weapon_health: u32, damage: u32) -> Self
	{
		Self
		{
			name,
			weapon_health,
			damage
		}
	}

	pub fn get_name(&self) -> &str { &self.name }
	pub fn get_damage(&self) -> u32 { self.damage }

	pub fn use_weapon(&mut self, dmg: u32)
	{
		let _ = self.weapon_health.saturating_sub(dmg);
	}
}