#![allow(dead_code)]
trait Run {
    fn top_speed(&self) -> f32;

    fn desc(&self) {
        println!("top speed reach up to {}", self.top_speed());
    }
}

struct Porsche {
    name: String,
    model: String,
}

struct Ferrari {
    year: i32,
    model: String,
}

impl Run for Porsche {
    fn top_speed(&self) -> f32 {
        350.0
    }

    fn desc(&self) {
        println!(
            "Porsche {} can run up to {} km/h",
            self.model,
            self.top_speed()
        )
    }
}

impl Run for Ferrari {
    fn top_speed(&self) -> f32 {
        340.0
    }

    fn desc(&self) {
        println!(
            "Ferrari {} year {} can run up to {} km/h",
            self.model,
            self.year,
            self.top_speed()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_car_startup() {
        let porsche = Porsche {
            name: String::from("911"),
            model: String::from("Carrera"),
        };
        let ferrari = Ferrari {
            year: 2021,
            model: String::from("488"),
        };

        let cars: Vec<Box<dyn Run>> = vec![Box::new(porsche), Box::new(ferrari)];
        for car in cars {
            car.desc();
        }
    }
}
