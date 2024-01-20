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
    println!("part2: {}", result);
}

fn process(input: &str) -> Result<usize, String> {
    let locations: Vec<usize> = parse_input(input)?;
    let lowest_location = locations.iter().min();
    match lowest_location {
        Some(l) => return Ok(*l),
        None => return Err("No locations found".to_string()),
    }
}

fn parse_input(input: &str) -> Result<Vec<usize>, String> {
    let mut output: Vec<usize> = Vec::new();
    let mut sections = input.split("\n\n");

    let seeds = sections.next().unwrap();
    let test = seeds.split(" ").skip(1).collect::<Vec<_>>();
    let test2 = test.chunks(2).collect::<Vec<_>>();
    for t in test2 {
        let start = t[0].parse::<usize>().unwrap();
        let rng = t[1].parse::<usize>().unwrap();
        output.extend((start..(start + rng)).collect::<Vec<_>>());
    }
    println!("seeds finished");

    let seed_to_soil_map = sections.next().unwrap();
    let mut seed_to_soil_map_rules = map_to_rules(seed_to_soil_map);
    output = output
        .iter()
        .map(|seed| MapRule::map(&mut seed_to_soil_map_rules, seed))
        .collect::<Vec<_>>();
    println!("soil finished");

    let soil_to_fertilizer_map = sections.next().unwrap();
    let mut soil_to_fertilizer_map_rules = map_to_rules(soil_to_fertilizer_map);
    output = output
        .iter()
        .map(|soil| MapRule::map(&mut soil_to_fertilizer_map_rules, soil))
        .collect::<Vec<_>>();
    println!("fertilizer finished");

    let fertilizer_to_water_map = sections.next().unwrap();
    let mut fertilizer_to_water_map_rules = map_to_rules(fertilizer_to_water_map);
    output = output
        .iter()
        .map(|fertilizer| MapRule::map(&mut fertilizer_to_water_map_rules, fertilizer))
        .collect::<Vec<_>>();
    println!("water finished");

    let water_to_light_map = sections.next().unwrap();
    let mut water_to_light_map_rules = map_to_rules(water_to_light_map);
    output = output
        .iter()
        .map(|water| MapRule::map(&mut water_to_light_map_rules, water))
        .collect::<Vec<_>>();
    println!("light finished");

    let light_to_temperature_map = sections.next().unwrap();
    let mut light_to_temperature_map_rules = map_to_rules(light_to_temperature_map);
    output = output
        .iter()
        .map(|light| MapRule::map(&mut light_to_temperature_map_rules, light))
        .collect::<Vec<_>>();
    println!("temperature finished");

    let temperature_to_humidity_map = sections.next().unwrap();
    let mut temperature_to_humidity_map_rules = map_to_rules(temperature_to_humidity_map);
    output = output
        .iter()
        .map(|temperature| MapRule::map(&mut temperature_to_humidity_map_rules, temperature))
        .collect::<Vec<_>>();
    println!("humidity finished");

    let humidity_to_location_map = sections.next().unwrap();
    let mut humidity_to_location_map_rules = map_to_rules(humidity_to_location_map);
    output = output
        .iter()
        .map(|humidity| MapRule::map(&mut humidity_to_location_map_rules, humidity))
        .collect::<Vec<_>>();
    println!("location finished");

    Ok(output)
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
