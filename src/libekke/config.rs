use serde       :: { Serialize, Deserialize } ;
use std         :: { path::PathBuf          } ;

#[ derive( Serialize, Deserialize, Debug, Clone ) ]
//
pub struct Settings
{
	pub apps: Vec< AppConfig >
}

#[ derive( Serialize, Deserialize, Debug, Clone ) ]
//
pub struct AppConfig
{
	pub name: String ,
	pub path: PathBuf,
}
