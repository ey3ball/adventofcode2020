use std::collections::HashMap;
use std::collections::HashSet;
use counter::Counter;
use regex::Regex;

static EMPTY_BAG: &str = " bags contain no other bags.";
type Bags = HashMap<String, HashMap<String, usize>>;

#[aoc_generator(day7)]
pub fn generator(input: &str) -> Bags {
    let re_contains = Regex::new(r"^([a-z ]*) bags contain (.*)").unwrap();
    let re_contents = Regex::new(r"(\d)+ ([a-z ]*) bag[s]?[ .,]?").unwrap();
    input.lines().map(
        |line| {
            if line.ends_with(EMPTY_BAG) {
                (line.replace(EMPTY_BAG, ""), HashMap::new())
            } else {
                let rule = re_contains.captures(line).unwrap();
                let bag = &rule[1];
                let contents = &rule[2];
                let parsed_contents: HashMap<String, usize> =
                    re_contents.captures_iter(contents).map(|x| {
                        (x[2].to_owned(), x[1].parse::<usize>().unwrap())
                    }).collect();
                (bag.to_owned(), parsed_contents)
            }
        }
    ).collect()
}

#[aoc(day7, part1)]
fn part1(input: &Bags) -> usize {
    let mut keys: usize = 0;
    let mut target: HashSet<&str> = ["shiny gold"].iter().cloned().collect();
    loop {
        let new: HashSet<&str> = input.iter().filter(|(_k, v)| {
            let contents: HashSet<&str> = v.keys().map(|s| &s[..]).collect();
            ! target.is_disjoint(&contents)
        }).map(|(k,_v)| k.as_str()).collect();
        target = target.union(&new).copied().collect();

        if target.iter().count() == keys {
            break
        } else {
            keys = target.iter().count()
        }
    }
    keys - 1
}

#[aoc(day7, part2)]
fn part2(input: &Bags) -> usize {
    let mut total_bags: usize = 0;
    let mut totals = Counter::<&str>::new();
    let mut step = vec![("shiny gold", 1)];
    loop {
        let next_step = step.iter().flat_map(|(x, count)| {
            (input[*x]).iter().map(move |(y, i)| (y, (count*i)))
        }).fold(Counter::<&str>::new(), |mut c, (y,i)| {
            c[&y.as_str()] += i;
            c
        });
        totals += next_step.clone();

        let next_total_bags = totals.iter().fold(0, |c, (_k,v)| c + v);
        step = next_step.clone().into_map().iter().map(|(k,v)| (*k, *v)).collect();
        if total_bags == next_total_bags {
            break
        } else {
            total_bags = next_total_bags
        }
    }
    total_bags
}
