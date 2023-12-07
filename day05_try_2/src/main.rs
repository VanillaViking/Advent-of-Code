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
    //exclusive
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

impl Mappings {
    pub fn new(input: &str) -> Mappings {
        let (name_str, map_str) = input.split_once(":").unwrap();
        let ranges: Vec<MapRange> = map_str.lines().filter_map(|line| {
            if line.is_empty() {
                return None
            }

            let num_vec: Vec<u64> = line.split(" ").map(|num_str| num_str.parse::<u64>().unwrap()).collect();
            
            let (start, end): (u64, u64) = (num_vec[1], num_vec[1] + num_vec[2]);

            let (dest_start, dest_end): (u64, u64) = (num_vec[0], num_vec[0] + num_vec[2]);
            let difference = i64::try_from(num_vec[0]).unwrap() - i64::try_from(num_vec[1]).unwrap();

            Some(MapRange { start, end, dest_start, dest_end, difference, name: name_str.to_owned() })
        }).collect();

        let name = name_str.to_owned();

        Mappings { name, ranges }
    }

    pub fn get_dest(&self, source: u64) -> u64 {
        for range in self.ranges.iter() {
            if source >= range.start && source < range.end {
                // I'd be concerned if it returned negative, so ok to panic
                return u64::try_from(i64::try_from(source).unwrap() + range.difference).unwrap()
            }
        }

        return source
    }

    fn get_dest_ranges(&self, source_ranges: &[Range]) -> Vec<Range> {
        source_ranges.iter().map(|src_range| {
            let mut subranges: Vec<Range> = self.ranges.iter().filter_map(|range| {
                if (range.start >= src_range.start && range.start < src_range.end) && (range.end >= src_range.start && range.end <= src_range.end) {
                    return Some(Range {start: self.get_dest(range.start), end: self.get_dest(range.end-1)+1})
                } else if range.start >= src_range.start && range.start < src_range.end {
                    return Some(Range {start: self.get_dest(range.start), end: self.get_dest(src_range.end-1)+1})
                } else if range.end >= src_range.start && range.end <= src_range.end {
                    return Some(Range {start: self.get_dest(src_range.start), end: self.get_dest(range.end-1)+1})
                } else {
                    return None
                }
            }).collect();

            if subranges.is_empty() {
                subranges.push(Range {start: self.get_dest(src_range.start), end: self.get_dest(src_range.end-1)+1})
            }

            if subranges.iter().find(|range| range.start == self.get_dest(src_range.start)).is_none() {
                subranges.push(Range {start: self.get_dest(src_range.start), end: self.get_dest(subranges.iter().min_by_key(|range| range.start).unwrap().start -1)+1})
            }

            if subranges.iter().find(|range| range.end == self.get_dest(src_range.end)).is_none() {
                subranges.push(Range {end: self.get_dest(src_range.end-1) +1, start: self.get_dest(subranges.iter().max_by_key(|range| range.end).unwrap().end)})
            }
            subranges

        }).flatten().collect()
    }


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
}

fn parse_input(input: &str) -> Pipeline {
    let categories: Vec<&str> = input.split("\n\n").collect();
    let mut seeds_part_2: Vec<Range> = Vec::new();

    let seeds: Vec<u64> = categories[0].split_once(":").unwrap().1.split(" ").filter_map(|num_str| num_str.parse::<u64>().ok()).collect();

    for idx in (0..seeds.len()).step_by(2) {
        seeds_part_2.push(Range { start: seeds[idx], end: seeds[idx] + seeds[idx + 1]});
    }
    
    // yummy hardcoding
    let seed_to_soil = Mappings::new(categories[1]);
    let soil_to_fert = Mappings::new(categories[2]);
    let fert_to_water = Mappings::new(categories[3]);
    let water_to_light = Mappings::new(categories[4]);
    let light_to_temp = Mappings::new(categories[5]);
    let temp_to_humidity = Mappings::new(categories[6]);
    let humidity_to_location = Mappings::new(categories[7]);

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
    }
}



fn main() {
    let input = fs::read_to_string("input").unwrap();
    let pipeline = parse_input(&input);

    println!("{}", part2(&pipeline));
}


fn part2(pipeline: &Pipeline) -> u64 {
    let soil_ranges: Vec<Range> = pipeline.seed_to_soil.get_dest_ranges(&pipeline.seeds_part_2); 
    let fert_ranges: Vec<Range> = pipeline.soil_to_fert.get_dest_ranges(&soil_ranges); 
    let water_ranges: Vec<Range> = pipeline.fert_to_water.get_dest_ranges(&fert_ranges); 
    let light_ranges: Vec<Range> = pipeline.water_to_light.get_dest_ranges(&water_ranges); 
    let temp_ranges: Vec<Range> = pipeline.light_to_temp.get_dest_ranges(&light_ranges); 
    let humidity_ranges: Vec<Range> = pipeline.temp_to_humidity.get_dest_ranges(&temp_ranges); 
    let location_ranges: Vec<Range> = pipeline.humidity_to_location.get_dest_ranges(&humidity_ranges); 
    location_ranges.iter().min_by_key(|range| range.start).unwrap().start
}




