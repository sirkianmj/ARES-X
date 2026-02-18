use nexus::Nexus;
use vacuum::Vacuum;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- FORGEX4 PROJECT ARES-X (SENTINEL KERNEL) ---");

    // 1. Initialize Secured Nexus with a Master Password
    // This creates the encrypted storage in RAM for our truth engine.
    let nexus = Arc::new(Nexus::new("forgex4_sovereign_2025")?);

    // 2. Initialize The Vacuum
    let v = Vacuum::new(nexus.clone());

    // 3. Connect to OSINT Stream
    println!("ARES-X: Connecting to OpenSky Live OSINT Feed...");
    match v.pull_opensky().await {
        Ok(_) => println!("ARES-X: Live aircraft tracks synchronized successfully."),
        Err(e) => eprintln!("ARES-X ERROR: OSINT connection failed: {}. (Check internet connection)", e),
    }

    println!("ARES-X: Sentinel initialization complete. Status: WATCHING.");
    Ok(())
}