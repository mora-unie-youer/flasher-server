use crate::Settings;

use sea_orm::{Database, DatabaseConnection};

pub async fn establish_connection(config: Settings) -> DatabaseConnection {
	let url = format!(
		"{}://{}:{}@{}/{}",
		config.database.db_type,
		config.database.username,
		config.database.password,
		config.database.host,
		config.database.db_name
	);

	Database::connect(&url)
		.await
		.unwrap_or_else(|_| panic!("Failed connecting to {}", url))
}
