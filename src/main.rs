use nexus::Nexus;
use vacuum::Vacuum;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- FORGEX4 PROJECT ARES-X INITIALIZING ---");

    // 1. Initialize Secured Nexus with a Master Password
    let nexus = Arc::new(Nexus::new("forgex4_sovereign_2025")?);

    // 2. Initialize The Vacuum
    let v = Vacuum::new(nexus.clone());

    // 3. Start Ingestion Loop
    println!("ARES-X: Starting Live OpenSky Ingestion...");
    v.pull_opensky().await?;

    println!("ARES-X: Initialization complete. Sentinel is watching.");
    Ok(())
}