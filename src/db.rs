

use std:: sync::Arc;

use libsql::{Builder, Connection};

#[derive(Clone)]
pub struct DB {
    pub conn: Arc<Connection>,
}

impl DB {
    pub async fn init() -> Result<Self, Box<dyn std::error::Error>> {
        let db = Builder::new_local("local.db").build().await?;
        let conn = db.connect().unwrap();

        Ok(Self { conn: Arc::new(conn) })
    }

    pub async fn create_table(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS
                            users( 
                                id INTEGER AUTOINCREAMENT PRIMARY KEY,
                                token TEXT NOT NULL,
                                email TEXT NOT NULL,
                                verified BOOLEAN DEFAULT FALSE,
                                requests INTEGER DEFAULT 10,
                                remaining INTEGER DEFAULT 10000
                            )",
                (),
            )
            .await?;
        println!("Table Created Succesfully");
        Ok(())
    }
}
