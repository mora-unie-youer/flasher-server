pub struct DatabaseSettings {
	pub host: String,
	pub username: String,
	pub password: String,
	pub name: String,
}

pub struct Settings {
	pub database: DatabaseSettings,
}
