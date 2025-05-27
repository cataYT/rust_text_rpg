use crate::player::Player;
use std::sync::{Mutex, OnceLock};

pub struct PlayerManager
{
	players: Vec<Player>
}

impl PlayerManager
{
	fn new() -> PlayerManager
	{
		PlayerManager { players: Vec::new() }
	}

	pub fn instance() -> &'static Mutex<PlayerManager> 
	{
		static INSTANCE: OnceLock<Mutex<PlayerManager>> = OnceLock::new();
		INSTANCE.get_or_init(|| Mutex::new(PlayerManager::new()))
	}
	
	pub fn add_player(&mut self, plr: Player)
	{
		self.players.push(plr)
	}

	pub fn remove_player(&mut self, plr: &Player)
	{
		self.players.retain(|p| p.plr_name != plr.plr_name)
	}

	pub fn get_total_players(&self) -> usize
	{
		self.players.len()
	}

	pub fn get_winner(&self) -> Option<&Player>
	{
		if self.players.len() == 1
		{
			return Some(&self.players[0])
		}
		None
	}
}