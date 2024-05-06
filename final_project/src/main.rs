//Importing necessary libraries that will be used 
use flate2::bufread::GzDecoder;
use rand::prelude::*;
use serde::Deserialize;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::time::Instant;

//Creating a struct named "Review" to group data
//Deserialize tells Serde how to interpret the data
//Identifiying the traits/fields used for this code
#[derive(Deserialize, Debug)]
struct Review {
    #[serde(rename = "reviewerID")]
    reviewer_id: String,
    asin: String,
}

//Creatign a "Graph" struct
//To review the record with a reviewer ID and ASIN
//Represents an undirected graph using an adjacency list
#[derive(Debug)]
struct Graph{
    outedges: HashMap<String, HashSet<String>>,
}

//Define methods for the Graph struct
impl Graph {

    //Creates and returns a new graph
    //Initialization of graph by return an empty adjacency list
    //creating a new hashmap
    fn new() -> Graph {
        Graph {
            outedges: HashMap::new(),
        }
    }

    //Adds an undirected edge between vertices u and v
    fn add_edges(&mut self, u: String, v: String) {
        self.outedges.entry(u.clone()).or_insert_with(HashSet::new).insert(v.clone());
        self.outedges.entry(v).or_insert_with(HashSet::new).insert(u);
    }

    //Creates undriected graph from a list of edges
    //Iterates over each tuple and adds each edge to the graph
    fn create_undirected(edges: &[(String, String)]) -> Graph {
        let mut g = Graph::new();
        for &(ref u, ref v) in edges {
            g.add_edges(u.clone(), v.clone());
        }
        g
    }

    //Breadth-first search to calculate shortest path from start mode
    fn bfs_shortpath(&self, start: &str) -> HashMap<String,usize> {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();
        distances.insert(start.to_string(), 0);
        queue.push_back(start.to_string());

        while let Some(current) = queue.pop_front() {
            let current_distance = distances[&current];
            for neighbor in self.outedges.get(&current).unwrap_or(&HashSet::new()) {
                if !distances.contains_key(neighbor) {
                    distances.insert(neighbor.to_string(), current_distance + 1);
                    queue.push_back(neighbor.to_string());
                }
            }
        }
        distances
    }

    //Calculates the average shortest path for the graph
    fn average_shortpath(&self) -> f64 {
        let mut total_length = 0;
        let mut total_path = 0;
        for node in self.outedges.keys() {
            let distances = self.bfs_shortpath(node);
            for &distance in distances.values() {
                if distance > 0 {
                    total_length += distance;
                    total_path += 1;
                }
            }
        }
        total_length as f64/ total_path as f64
    }
}

//Read Jsonfile
fn read_jsonfile(file_path: &str) -> Vec<Review> {
    let file = File::open(file_path).expect("Could not open file");
    let buf_reader = BufReader::new(file);
    let decoder = GzDecoder::new(buf_reader);
    let reader = BufReader::new(decoder);

    let mut result: Vec<Review> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error reading line from file");
        let review: Review = serde_json::from_str(&line).expect("Invalid Json");
        result.push(review);
    }
    result
}

//Randomly samples a subset of reviews to reduce the size of the graph
//Stores randomly selected reviews in sample_ids 
//Creating a Hashset from the selected sample_ids into sample_id_set
//Filters reviews to only include reviewer ID and ASIN
fn sample_reviews(reviews: &[Review], _target_size: usize) -> Vec<(String, String)> {
    let mut rng = rand::thread_rng();
    let unique_ids: HashSet<_> = reviews
        .iter()
        .flat_map(|r| vec![r.reviewer_id.clone(), r.asin.clone()])
        .collect();
    let sample_ids: Vec<_> = unique_ids.into_iter().collect::<Vec<_>>().choose_multiple(&mut rng, _target_size).cloned().collect();

    let sample_id_set: HashSet<_> = sample_ids.into_iter().collect();
    reviews
        .iter()
        .filter(|r| sample_id_set.contains(&r.reviewer_id) || sample_id_set.contains(&r.asin))
        .map(|r| (r.reviewer_id.clone(), r.asin.clone()))
        .collect()
}

//Comparing two randomly selected sample sets
//Evaluates whether the shortest path length is 6 or few
fn compare_average_shortpaths(graph1: &Graph, graph2: &Graph) {
    let avg_length1 = graph1.average_shortpath();
    let avg_length2 = graph2.average_shortpath();

    println!("Graph 1: Average Shortest Path Length = {:.2}", avg_length1);

    let six_degrees_1 = avg_length1 <= 6.0;
    if six_degrees_1 {
        println!("Six degrees of separation hold true for Graph 1.");
    } else {
        println!("Six degrees of separation do not hold true for Graph 1.");
    }

    println!("Graph 2: Average Shortest Path Length = {:.2}", avg_length2);

    let six_degrees_2 = avg_length2 <= 6.0;
    if six_degrees_2 {
        println!("Six degrees of separation hold true for Graph 2.");
    } else {
        println!("Six degrees of separation do not hold true for Graph 2.");
    }

    if avg_length1 < avg_length2 {
        println!("Graph 1 has a shorter average shortest path.");
    } else if avg_length1 > avg_length2 {
        println!{"Graph 2 has a shorter average shortest path."};
    } else {
        println!("Both graphs have the same average shortest path.");
    }
}

//Processing the file
//Outputs: Execution time of the code
//Samples a subset of reviews to construct two different graphs
//Creates two undirect graphs from sample reviews
//Compare the average shortest path
fn main() {
    let start = Instant::now();

    let reviews = read_jsonfile("Video_Games_5.json.gz");

    let edges1 = sample_reviews(&reviews,1500);
    let graph1 = Graph::create_undirected(&edges1);
    
    let edges2 = sample_reviews(&reviews,1500);
    let graph2 = Graph::create_undirected(&edges2);

    compare_average_shortpaths(&graph1, &graph2);
    
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bfs_shortpath() {
        let edges = vec! [
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
            ("C".to_string(), "D".to_string()),
        ];

        let graph = Graph::create_undirected(&edges);
        let distances = graph.bfs_shortpath("A");
        assert_eq!(dsitances.get("B"), Some(&1));
        assert_eq!(dsitances.get("C"), Some(&2));
        assert_eq!(dsitances.get("D"), Some(&3));
    }

    #[test]
    fn test_average_shortpath() {
        let edges = vec![
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
            ("C".to_string(), "D".to_string()),
        ];
        let graph = Graph::create_undirected(&edges);
        let avg_shortpath = graph.average_shortpath();
        assert_eq!(avg_shortpath, 1.5);
    }

    #[test]
    fn test_sample_reviews() {
        let reveiws = vec![
            Review {
                reviewer_id: "A1".to_string(),
                asin: "B1".to_string(),
            },

            Review {
                reviewer_id: "A2".to_string(),
                asin: "B2".to_string(),
            },
            
            Review {
                reviewer_id: "A3".to_string(),
                asin: "B3".to_string(),
            },
        ];
        let samples = sample_reviews(&reviews,2);
        assert_eq!(samples.len(),2)
    }

    #[test]
    fn test_compare_average_shortpaths() {
        let edges1 = vec![
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "C".to_string()),
        ];

        let edges2 = vec![
            ("A".to_string(), "B".to_string()),
            ("B".to_string(), "D".to_string()),
            ("D".to_string(), "E".to_string()),
        ];

        let graph1 = Graph::create_undirected(&edges1);
        let graph2 = Graph::create_undirected(&edges2);
        let avg_length1 = graph1.average_shortpath();
        let avg_length2 = graph2.average_shortpath();
        assert!(avg_length1 < avg_length2);
    }
}
