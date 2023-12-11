mod property;
mod graph;
mod edge;

use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Serial Number")]
    serial_number: String,
    #[serde(rename = "List Year")]
    list_year: String,
    #[serde(rename = "Date Recorded")]
    date_recorded: String,
    #[serde(rename = "Town")]
    town: String,
    #[serde(rename = "Address")]
    address: String,
    #[serde(rename = "Assessed Value")]
    assessed_value: f64,
    #[serde(rename = "Sale Amount")]
    sale_amount: f64,
    #[serde(rename = "Sales Ratio")]
    sales_ratio: f64,
    #[serde(rename = "Property Type")]
    property_type: String,
    #[serde(rename = "Residential Type")]
    residential_type: String,
    // Add other fields as necessary
}

#[derive(Debug)]
struct Property {
    id: String,
    sale_amount: f64,
}

#[derive(Debug)]
struct Edge {
    serial_number1: String,
    serial_number2: String,
    weight: f64,
}

struct Graph {
    properties: Vec<Property>,
    edges: Vec<Edge>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            properties: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    fn add_edges_based_on_sale_amount(&mut self, id_to_index: &HashMap<String, usize>, sale_amount_threshold: f64) {
        for i in 0..self.properties.len() {
            for j in (i + 1)..self.properties.len() {
                let difference = (self.properties[i].sale_amount - self.properties[j].sale_amount).abs();
                if difference <= sale_amount_threshold {
                    self.edges.push(Edge {
                        serial_number1: self.properties[i].id.clone(),
                        serial_number2: self.properties[j].id.clone(),
                        weight: 1.0 / difference,
                    });
                }
            }
        }
    }
}

// Define the calculate_median function
fn calculate_median(values: &mut Vec<f64>) -> f64 {
    values.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = values.len() / 2;
    if values.len() % 2 == 0 {
        (values[mid - 1] + values[mid]) / 2.0
    } else {
        values[mid]
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "/Users/faressakaan/Desktop/sampled_data.csv"; // Update this path
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let mut graph = Graph::new();
    let mut sale_amounts = Vec::new();
    let mut id_to_index = HashMap::new(); // Ensure HashMap is declared

    for result in rdr.deserialize() {
        let record: Record = result?;
        sale_amounts.push(record.sale_amount);
        let property = Property {
            id: record.serial_number.clone(),
            sale_amount: record.sale_amount,
        };
        let index = graph.properties.len(); // The current index before adding the new property
        id_to_index.insert(record.serial_number, index); // Map the serial number to its index
        graph.add_property(property);
    }

    let median_sale_amount = calculate_median(&mut sale_amounts); // Ensure this function is defined
    let sale_amount_threshold = median_sale_amount * 0.01;
    graph.add_edges_based_on_sale_amount(&id_to_index, sale_amount_threshold);


    println!("Median Sale Amount: {}", median_sale_amount);
    println!("Similarity Threshold: {}", sale_amount_threshold);
    println!("Graph has been created with {} properties and {} edges.", graph.properties.len(), graph.edges.len());

    let mut degrees = vec![0; graph.properties.len()];
    for edge in &graph.edges {
        let index1 = *id_to_index.get(&edge.serial_number1).expect("Serial number 1 not found");
        let index2 = *id_to_index.get(&edge.serial_number2).expect("Serial number 2 not found");
        degrees[index1] += 1;
        degrees[index2] += 1;
    }

    // Print out the degree distribution
    for (index, degree) in degrees.iter().enumerate() {
        println!("Property ID: {}, Degree: {}", graph.properties[index].id, degree);
    }

    // Optionally, you could also sort and print out the top N properties by degree
    // ...

    Ok(())
}
