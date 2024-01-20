#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InfoType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Info {
    info_type: InfoType,
    id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Almanac {
    seeds: Vec<Info>,
    soils: Vec<Info>,
    fertilizers: Vec<Info>,
    waters: Vec<Info>,
    lights: Vec<Info>,
    temperatures: Vec<Info>,
    humidities: Vec<Info>,
    locations: Vec<Info>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MapRule {
    from_index: usize,
    to_index: usize,
    range: usize,
}

impl MapRule {
    fn get_map(&self, from_index: &usize) -> Option<usize> {
        if from_index >= &self.from_index && from_index < &(self.from_index + self.range) {
            return Some(self.to_index + from_index - self.from_index);
        }
        None
    }

    fn map(map_rules: &mut Vec<Self>, from_index: &usize) -> usize {
        for rule in map_rules {
            if let Some(to_index) = rule.get_map(from_index) {
                return to_index;
            }
        }
        from_index.clone()
    }
}

pub fn main() {
    let input = r"src\inputs\input.txt";
    let content = std::fs::read_to_string(input).unwrap();
    let result = process(&content).unwrap();
    println!("part1: {}", result);
}

fn process(input: &str) -> Result<usize, String> {
    let almanac = parse_input(input)?;
    let lowest_location = almanac.locations.iter().min_by_key(|l| l.id).unwrap();
    Ok(lowest_location.id)
}

fn parse_input(input: &str) -> Result<Almanac, String> {
    let mut almanac = Almanac {
        seeds: Vec::new(),
        soils: Vec::new(),
        fertilizers: Vec::new(),
        waters: Vec::new(),
        lights: Vec::new(),
        temperatures: Vec::new(),
        humidities: Vec::new(),
        locations: Vec::new(),
    };

    let mut sections = input.split("\n\n");

    let seeds = sections.next().unwrap();
    let test = seeds.split(" ").skip(1).collect::<Vec<_>>();
    let test2 = test.chunks(2).collect::<Vec<_>>();
    for t in test2 {
        let start = t[0].parse::<usize>().unwrap();
        let rng = t[1].parse::<usize>().unwrap();
        almanac.seeds.extend(
            (start..(start + rng))
                .into_iter()
                .map(|s| Info {
                    info_type: InfoType::Seed,
                    id: s,
                })
                .collect::<Vec<_>>(),
        );
    }

    let seed_to_soil_map = sections.next().unwrap();
    let mut seed_to_soil_map_rules = map_to_rules(seed_to_soil_map);
    almanac.soils = map_to_parent(&almanac.seeds, InfoType::Soil, seed_to_soil_map_rules);

    let soil_to_fertilizer_map = sections.next().unwrap();
    let mut soil_to_fertilizer_map_rules = map_to_rules(soil_to_fertilizer_map);
    almanac.fertilizers = map_to_parent(
        &almanac.soils,
        InfoType::Fertilizer,
        soil_to_fertilizer_map_rules,
    );

    let fertilizer_to_water_map = sections.next().unwrap();
    let mut fertilizer_to_water_map_rules = map_to_rules(fertilizer_to_water_map);
    almanac.waters = map_to_parent(
        &almanac.fertilizers,
        InfoType::Water,
        fertilizer_to_water_map_rules,
    );

    let water_to_light_map = sections.next().unwrap();
    let mut water_to_light_map_rules = map_to_rules(water_to_light_map);
    almanac.lights = map_to_parent(&almanac.waters, InfoType::Light, water_to_light_map_rules);

    let light_to_temperature_map = sections.next().unwrap();
    let mut light_to_temperature_map_rules = map_to_rules(light_to_temperature_map);
    almanac.temperatures = map_to_parent(
        &almanac.lights,
        InfoType::Temperature,
        light_to_temperature_map_rules,
    );

    let temperature_to_humidity_map = sections.next().unwrap();
    let mut temperature_to_humidity_map_rules = map_to_rules(temperature_to_humidity_map);
    almanac.humidities = map_to_parent(
        &almanac.temperatures,
        InfoType::Humidity,
        temperature_to_humidity_map_rules,
    );

    let humidity_to_location_map = sections.next().unwrap();
    let mut humidity_to_location_map_rules = map_to_rules(humidity_to_location_map);
    almanac.locations = map_to_parent(
        &almanac.humidities,
        InfoType::Location,
        humidity_to_location_map_rules,
    );

    Ok(almanac)
}

fn map_to_parent(
    parents: &Vec<Info>,
    child_type: InfoType,
    mut map_rules: Vec<MapRule>,
) -> Vec<Info> {
    parents
        .iter()
        .map(|parent| {
            let child_id = MapRule::map(&mut map_rules, &parent.id);
            let mut child = Info {
                info_type: child_type,
                id: child_id,
            };
            child
        })
        .collect::<Vec<_>>()
}

fn map_to_rules(seed_to_soil_map: &str) -> Vec<MapRule> {
    let mut seed_to_soil_map_rules = seed_to_soil_map
        .split("\n")
        .skip(1)
        .map(|m| {
            let mut rule_info = m.split(" ").map(|s| s.parse().unwrap());
            let rule = MapRule {
                to_index: rule_info.next().unwrap(),
                from_index: rule_info.next().unwrap(),
                range: rule_info.next().unwrap(),
            };
            rule
        })
        .collect::<Vec<_>>();
    seed_to_soil_map_rules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37";
        let result = process(input).unwrap();
        assert_eq!(result, 46);
    }
}
