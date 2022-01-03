use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::time::Instant;

pub enum Direction {
    Lat,
    Lon,
}

/// The City maps the csv fileds that are expected to be
/// Country,City,AccentCity,Region,Population,Latitude,Longitude
///
/// The city input file is take from https://www.kaggle.com/max-mind/world-cities-database
#[derive(Debug, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
pub struct City {
    country: String,
    city: String,
    accent_city: String,
    region: String,
    population: Option<f32>,
    latitude: f32,
    longitude: f32,
}

impl City {
    pub fn id(&self) -> String {
        format!("{}/{}", &self.city, &self.country)
    }

    pub fn label(&self) -> &String {
        &self.accent_city
    }

    pub fn lat(&self) -> &f32 {
        &self.latitude
    }

    pub fn lon(&self) -> &f32 {
        &self.longitude
    }
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:10} {}  https://www.openstreetmap.org/#map=8/{}/{}",
            self.label(),
            self.population.unwrap_or(0.0),
            self.lat(),
            self.lon(),
        )
    }
}

/// Simple in-memory database for the allowed cities
pub struct CityDb {
    db: HashMap<String, City>,
}

impl CityDb {
    pub fn load(file: &str) -> Result<Self, Box<dyn Error>> {
        // track execution
        // start time, num success, num errors, num ignored
        let mut exec = (Instant::now(), 0, 0, 0);
        // create the map to hold the data
        let mut db: HashMap<String, City> = HashMap::new();

        // create the path from input string
        let p = Path::new(file);
        // open the reader to the path
        let mut reader = csv::Reader::from_path(p)?;

        // structure of the content
        for blb in reader.deserialize() {
            match blb {
                Err(e) => {
                    // increment failures
                    exec.2 += 1;
                    println!("{}", e);
                }
                Ok(x) => {
                    let c: City = x;
                    match c.population {
                        Some(_) => {
                            // increment success
                            exec.1 += 1;
                            // add the city to the db
                            db.insert(c.id(), c);
                            // db.insert(c.id().to_owned(), c);
                        }
                        None => exec.3 += 1, //ignore records without population
                    }
                }
            }
        }
        println!(
            "{} records ({} errors, {}ignored) processed in {}s, databse has {} records",
            exec.1,
            exec.2,
            exec.3,
            exec.0.elapsed().as_secs(),
            db.len(),
        );
        Ok(CityDb { db })
    }

    pub fn sort(&self, city_names: Vec<&str>, direction: &Direction) -> Vec<&City> {
        let mut res = city_names
            .into_iter()
            .filter_map(|n| self.db.get(n))
            .collect::<Vec<&City>>();
        res.sort_by(|a, b| match direction {
            Direction::Lat => b.lat().partial_cmp(a.lat()).unwrap(),
            Direction::Lon => b.lon().partial_cmp(a.lon()).unwrap(),
        });
        res
    }
}
