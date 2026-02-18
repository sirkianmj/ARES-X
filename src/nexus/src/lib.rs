pub mod shield;

use duckdb::{params, Connection, Result};
use shield::SovereignShield;

pub struct Nexus {
    pub conn: Connection,
    shield: SovereignShield,
}

impl Nexus {
    pub fn new(password: &str) -> Result<Self> {
        let conn = Connection::open_in_memory()?; 
        let salt = b"FORGEX4_STATIC_SALT"; 
        let shield = SovereignShield::new(password, salt);
        
        let mut nexus = Nexus { conn, shield };
        nexus.init_schema()?;
        Ok(nexus)
    }

    fn init_schema(&mut self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE live_tracks (
                id           VARCHAR PRIMARY KEY,
                lon          DOUBLE,
                lat          DOUBLE,
                alt          DOUBLE,
                encrypted_metadata BLOB
            );"
        )?;
        Ok(())
    }

    pub fn ingest_track(&self, id: &str, lon: f64, lat: f64, alt: f64, metadata: &str) -> Result<()> {
        let nonce = [0u8; 12]; 
        let encrypted_data = self.shield.encrypt(metadata.as_bytes(), &nonce);

        // FIX: Use the params! macro for DuckDB
        self.conn.execute(
            "INSERT OR REPLACE INTO live_tracks (id, lon, lat, alt, encrypted_metadata) 
             VALUES (?, ?, ?, ?, ?)",
            params![id, lon, lat, alt, encrypted_data],
        )?;
        Ok(())
    }

    pub fn kill_all(&mut self) {
        self.shield.trigger_kill_switch();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shielded_ingestion() {
        let mut n = Nexus::new("top_secret_forgex4").unwrap();
        let res = n.ingest_track("UAV-ALPHA", 12.34, 56.78, 10000.0, "Pilot: Kian");
        assert!(res.is_ok());
        
        n.kill_all();
    }
}