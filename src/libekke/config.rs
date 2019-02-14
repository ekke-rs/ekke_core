use std::path::PathBuf;
use std::sync::RwLock;
use config::{ Config };
use lazy_static::lazy_static;

lazy_static!
{
	pub static ref SETTINGS: RwLock<Config> = RwLock::new( Config::default() );
}




struct Settings
{
	apps: Vec< Application >
}


struct Application
{
	  name: String
	, path: PathBuf
	, args: Vec< String >
}

