use std::fs;

#[derive(Debug, Clone)]
struct Range {
    start: u64,
    end: u64,
}

#[derive(Debug, Clone)]
struct MapRange {
    //inclusive
    start: u64,
    //inclusive
    end: u64,
    difference: i64,

    dest_start: u64,
    dest_end: u64,
    name: String,
}

#[derive(Debug)]
struct Mappings { 
    name: String,
    ranges: Vec<MapRange>
}

#[derive(Debug, Default)]
struct AllRanges {
    ranges: Vec<MapRange>
}

impl AllRanges {
    fn get_most_intersecting(&self) {
        self.ranges.iter().filter(|range| range.difference < 0).for_each(|range| {

        })
    } 
}

impl Mappings {
    pub fn new(input: &str, all_ranges: &mut AllRanges) -> Mappings {
        let (name_str, map_str) = input.split_once(":").unwrap();
        let ranges: Vec<MapRange> = map_str.lines().filter_map(|line| {
            if line.is_empty() {
                return None
            }

            let num_vec: Vec<u64> = line.split(" ").map(|num_str| num_str.parse::<u64>().unwrap()).collect();
            
            let (start, end): (u64, u64) = (num_vec[1], num_vec[1] + num_vec[2] - 1);

            let (dest_start, dest_end): (u64, u64) = (num_vec[0], num_vec[0] + num_vec[2] - 1);
            let difference = i64::try_from(num_vec[0]).unwrap() - i64::try_from(num_vec[1]).unwrap();

            Some(MapRange { start, end, dest_start, dest_end, difference, name: name_str.to_owned() })
        }).collect();

        let name = name_str.to_owned();

        all_ranges.ranges.append(&mut ranges.clone());

        Mappings { name, ranges }
    }

    pub fn get_dest(&self, source: u64) -> u64 {
        for range in self.ranges.iter() {
            if source >= range.start && source <= range.end {
                // I'd be concerned if it returned negative, so ok to panic
                return u64::try_from(i64::try_from(source).unwrap() + range.difference).unwrap()
            }
        }

        return source
    }


    pub fn get_source(&self, dest: u64) -> u64 {
        for range in self.ranges.iter() {
            if dest >= range.dest_end && dest <= range.dest_end {
                // I'd be concerned if it returned negative, so ok to panic
                return u64::try_from(i64::try_from(dest).unwrap() - range.difference).unwrap()
            }
        }

        return dest
    }

    // pub fn get_min_dest_from_range(&self, source_start: u64, source_end: u64) -> u64 {
    //     let dests = self.ranges.iter().filter_map(|range| {
    //         if range.difference > 0 {
    //             return None
    //         }

    //         if range.start >= source_start && range.start <= source_end {
    //             return Some(self.get_dest(range.start)) 
    //         }

    //         if range.end >= source_start && range.end <= source_end {
    //             return Some(self.get_dest(range.end)) 
    //         }
    //         None

    //     });

    //     todo!()

    // }
}


#[derive(Debug)]
struct Pipeline {
    seeds: Vec<u64>,
    seeds_part_2: Vec<Range>,
    seed_to_soil: Mappings,
    soil_to_fert: Mappings,
    fert_to_water: Mappings,
    water_to_light: Mappings,
    light_to_temp: Mappings,
    temp_to_humidity: Mappings,
    humidity_to_location: Mappings,
    all_ranges: AllRanges,
}

fn parse_input(input: &str) -> Pipeline {
    let categories: Vec<&str> = input.split("\n\n").collect();
    let mut all_ranges = AllRanges::default();
    let mut seeds_part_2: Vec<Range> = Vec::new();

    let seeds: Vec<u64> = categories[0].split_once(":").unwrap().1.split(" ").filter_map(|num_str| num_str.parse::<u64>().ok()).collect();

    for idx in (0..seeds.len()).step_by(2) {
        seeds_part_2.push(Range { start: seeds[idx], end: seeds[idx] + seeds[idx + 1]});
    }
    
    // yummy hardcoding
    let seed_to_soil = Mappings::new(categories[1], &mut all_ranges);
    let soil_to_fert = Mappings::new(categories[2], &mut all_ranges);
    let fert_to_water = Mappings::new(categories[3], &mut all_ranges);
    let water_to_light = Mappings::new(categories[4], &mut all_ranges);
    let light_to_temp = Mappings::new(categories[5], &mut all_ranges);
    let temp_to_humidity = Mappings::new(categories[6], &mut all_ranges);
    let humidity_to_location = Mappings::new(categories[7], &mut all_ranges);

    Pipeline {
        seeds,
        seeds_part_2,
        seed_to_soil,
        soil_to_fert,
        fert_to_water,
        water_to_light,
        light_to_temp,
        temp_to_humidity,
        humidity_to_location,
        all_ranges,

    }
}

impl Pipeline {
    pub fn send(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.get_dest(seed);
        let fert = self.soil_to_fert.get_dest(soil);
        let water = self.fert_to_water.get_dest(fert);
        let light = self.water_to_light.get_dest(water);
        let temp = self.light_to_temp.get_dest(light);
        let humidity = self.temp_to_humidity.get_dest(temp);
        self.humidity_to_location.get_dest(humidity)
    }

    pub fn reverse(&self, location: u64) -> u64 {
        let humidity = self.humidity_to_location.get_source(location);
        let temp = self.temp_to_humidity.get_source(humidity);
        let light = self.light_to_temp.get_source(temp);
        let water = self.water_to_light.get_source(light);
        let fert = self.fert_to_water.get_source(water);
        let soil = self.soil_to_fert.get_source(fert);
        self.seed_to_soil.get_source(soil)
    }

    pub fn mark_skip(&mut self, location u64) {
        let skip_range = Range { start: location, end: u64::MAX };

        self.humidity_to_location.ranges.iter().filter(|range| {
            if range.dest_start > skip_range.start {
                   
            }
        })
        
    }
}


fn part1(pipeline: &Pipeline) -> u64 {
    let locations = pipeline.seeds.iter().map(|seed| {
        pipeline.send(seed.to_owned())
    }).min().unwrap();

    locations
}

fn part2(pipeline: &Pipeline) -> u64 {
    seeds_part_2.for_each(|range| {
        for seed in (range.start..=range.end) {
            location = pipeline.send(seed);
            pipeline.mark_skip(location);
        }
    })

}



fn main() {
    let input = fs::read_to_string("sample").unwrap();

    let pipeline = parse_input(&input);
    println!("{}", part1(&pipeline));

    part2(&pipeline);

}
