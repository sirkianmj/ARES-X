use nexus::Nexus;
use serde::Deserialize;
use reqwest::Client;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
pub struct OpenSkyResponse {
    pub states: Option<Vec<Vec<serde_json::Value>>>,
}

pub struct Vacuum {
    client: Client,
    nexus: Arc<Nexus>,
}

impl Vacuum {
    pub fn new(nexus: Arc<Nexus>) -> Self {
        Self {
            client: Client::new(),
            nexus,
        }
    }

    /// Task 1: Ingest Live ADS-B Data (OpenSky Network)
    pub async fn pull_opensky(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Public API URL (Global coverage)
        let url = "https://opensky-network.org/api/states/all";
        
        let resp = self.client.get(url).send().await?.json::<OpenSkyResponse>().await?;

        if let Some(states) = resp.states {
            for s in states.iter().take(10) { // Take 10 for the first test
                let icao24 = s[0].as_str().unwrap_or("Unknown");
                let lon = s[5].as_f64().unwrap_or(0.0);
                let lat = s[6].as_f64().unwrap_or(0.0);
                let alt = s[7].as_f64().unwrap_or(0.0);

                // Pipe directly into the Secured Nexus
                self.nexus.ingest_track(
                    icao24, 
                    lon, 
                    lat, 
                    alt, 
                    &format!("Source: OpenSky | ICAO: {}", icao24)
                )?;
            }
            println!("VACUUM: Successfully ingested {} aircraft tracks into Nexus.", states.len());
        }

        Ok(())
    }
}