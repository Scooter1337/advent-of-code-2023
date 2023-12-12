use std::{collections::btree_map::Range, env, fs::File, io::BufRead};
#[allow(dead_code, unused_variables)]
fn main() {
    let now = std::time::Instant::now();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a file path as a command line argument");
    }
    let file_path = &args[1];

    let mut lines = Vec::new();

    let file = File::open(file_path).expect("Could not open file");
    let reader = std::io::BufReader::new(file);
    for line in reader.lines() {
        lines.push(line.expect("Could not read line"));
    }

    let seeds: Seeds = lines[0].split("seeds: ").collect::<Vec<&str>>()[1]
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut seeds_ranges = Vec::with_capacity(seeds.len() / 2);

    let mut i = 0;
    while i < seeds.len() {
        seeds_ranges.push(seeds[i]..(seeds[i] + seeds[i + 1]));
        i += 2;
    }

    let mut seed_to_soil: SeedToSoil = Vec::new();
    let mut soil_to_fertilizer: SoilToFertilizer = Vec::new();
    let mut fertilizer_to_water: FertilizerToWater = Vec::new();
    let mut water_to_light: WaterToLight = Vec::new();
    let mut light_to_temperature: LightToTemperature = Vec::new();
    let mut temperature_to_humidity: TemperatureToHumidity = Vec::new();
    let mut humidity_to_location: HumidityToLocation = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        match line {
            l if l.contains("seed-to-soil") => {
                let mut j = i + 1;
                while !lines[j].is_empty() {
                    let mut split = lines[j].split(' ').collect::<Vec<&str>>();
                    let mapped = split
                        .iter()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                    seed_to_soil.push((mapped[0], mapped[1], mapped[2]));
                    j += 1;
                }
            }
            l if l.contains("soil-to-fertilizer") => {
                let mut j = i + 1;
                while !lines[j].is_empty() {
                    let mut split = lines[j].split(' ').collect::<Vec<&str>>();
                    let mapped = split
                        .iter()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                    soil_to_fertilizer.push((mapped[0], mapped[1], mapped[2]));
                    j += 1;
                }
            }
            l if l.contains("fertilizer-to-water") => {
                let mut j = i + 1;
                while !lines[j].is_empty() {
                    let split = lines[j].split(' ').collect::<Vec<&str>>();
                    let mapped = split
                        .iter()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                    fertilizer_to_water.push((mapped[0], mapped[1], mapped[2]));
                    j += 1;
                }
            }
            l if l.contains("water-to-light") => {
                let mut j = i + 1;
                while !lines[j].is_empty() {
                    let split = lines[j].split(' ').collect::<Vec<&str>>();
                    let mapped = split
                        .iter()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                    water_to_light.push((mapped[0], mapped[1], mapped[2]));
                    j += 1;
                }
            }
            l if l.contains("light-to-temperature") => {
                let mut j = i + 1;
                while !lines[j].is_empty() {
                    let split = lines[j].split(' ').collect::<Vec<&str>>();
                    let mapped = split
                        .iter()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                    light_to_temperature.push((mapped[0], mapped[1], mapped[2]));
                    j += 1;
                }
            }
            l if l.contains("temperature-to-humidity") => {
                let mut j = i + 1;
                while !lines[j].is_empty() {
                    let split = lines[j].split(' ').collect::<Vec<&str>>();
                    let mapped = split
                        .iter()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                    temperature_to_humidity.push((mapped[0], mapped[1], mapped[2]));
                    j += 1;
                }
            }
            l if l.contains("humidity-to-location") => {
                let mut j = i + 1;
                while j < lines.len() && !lines[j].is_empty() {
                    let split = lines[j].split(' ').collect::<Vec<&str>>();
                    let mapped = split
                        .iter()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect::<Vec<i64>>();
                    humidity_to_location.push((mapped[0], mapped[1], mapped[2]));
                    j += 1;
                }
            }
            _ => {}
        }
    }
    // dbg!(seed_to_soil);
    // dbg!(soil_to_fertilizer);
    // dbg!(fertilizer_to_water);
    // dbg!(water_to_light);
    // dbg!(light_to_temperature);
    // dbg!(temperature_to_humidity);
    // dbg!(humidity_to_location);
    let mut lowest_location = i64::MAX;
    for seed_range in seeds_ranges {
        dbg!(&seed_range);
        for seed in seed_range {
            let mut soil = -1;
            let mut fertilizer = -1;
            let mut water = -1;
            let mut light = -1;
            let mut temperature = -1;
            let mut humidity = -1;
            let mut location = -1;

            for (dest_range_start, source_range_start, range_length) in &seed_to_soil {
                if (source_range_start..&(source_range_start + range_length)).contains(&&seed) {
                    soil = dest_range_start + (seed - source_range_start);
                    break;
                }
            }
            if soil == -1 {
                soil = seed;
            }

            for (dest_range_start, source_range_start, range_length) in &soil_to_fertilizer {
                if (source_range_start..&(source_range_start + range_length)).contains(&&soil) {
                    fertilizer = dest_range_start + (soil - source_range_start);
                    break;
                }
            }
            if fertilizer == -1 {
                fertilizer = soil;
            }

            for (dest_range_start, source_range_start, range_length) in &fertilizer_to_water {
                if (source_range_start..&(source_range_start + range_length)).contains(&&fertilizer)
                {
                    water = dest_range_start + (fertilizer - source_range_start);
                    break;
                }
            }
            if water == -1 {
                water = fertilizer;
            }

            for (dest_range_start, source_range_start, range_length) in &water_to_light {
                if (source_range_start..&(source_range_start + range_length)).contains(&&water) {
                    light = dest_range_start + (water - source_range_start);
                    break;
                }
            }
            if light == -1 {
                light = water;
            }

            for (dest_range_start, source_range_start, range_length) in &light_to_temperature {
                if (source_range_start..&(source_range_start + range_length)).contains(&&light) {
                    temperature = dest_range_start + (light - source_range_start);
                    break;
                }
            }
            if temperature == -1 {
                temperature = light;
            }

            for (dest_range_start, source_range_start, range_length) in &temperature_to_humidity {
                if (source_range_start..&(source_range_start + range_length))
                    .contains(&&temperature)
                {
                    humidity = dest_range_start + (temperature - source_range_start);
                    break;
                }
            }
            if humidity == -1 {
                humidity = temperature;
            }

            for (dest_range_start, source_range_start, range_length) in &humidity_to_location {
                if (source_range_start..&(source_range_start + range_length)).contains(&&humidity) {
                    location = dest_range_start + (humidity - source_range_start);
                    break;
                }
            }
            if location == -1 {
                location = humidity;
            }
            if location < lowest_location {
                lowest_location = location;
            }

            // println!(
            //     "Seed: {}, Soil: {}, Fertilizer: {}, Water: {}, Light: {}, Temperature: {}, Humidity: {}, Location: {}",
            //     seed, soil, fertilizer, water, light, temperature, humidity, location
            // );
        }
    }

    dbg!(lowest_location);
    println!("Time elapsed: {:?}", now.elapsed());
}

type Seeds = Vec<i64>;

type SeedToSoil = Vec<(i64, i64, i64)>;

type SoilToFertilizer = Vec<(i64, i64, i64)>;

type FertilizerToWater = Vec<(i64, i64, i64)>;

type WaterToLight = Vec<(i64, i64, i64)>;

type LightToTemperature = Vec<(i64, i64, i64)>;

type TemperatureToHumidity = Vec<(i64, i64, i64)>;

type HumidityToLocation = Vec<(i64, i64, i64)>;
