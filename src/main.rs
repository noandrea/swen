mod cities;
use crate::cities::CityDb;

fn main() {
    match CityDb::load("_private/worldcitiespop.csv") {
        Err(e) => println!("something went wrong: {}", e),
        Ok(cd) => {
            let dir = cities::Direction::Lat;
            let input = vec![
                "turin/it",
                "milan/it",
                "paris/fr",
                "berlin/de",
                "london/en",
                "madrid/es",
            ];
            input.clone().into_iter().for_each(|n| println!("{}", n));

            let output = cd.sort(input, &dir);
            output.into_iter().for_each(|c| println!("{}", c))
        }
    }
}
