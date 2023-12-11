use crate::property::Property;
use crate::edge::Edge;

pub struct Graph {
    pub properties: Vec<Property>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            properties: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_property(&mut self, property: Property) {
        self.properties.push(property);
    }

    pub fn add_edges_based_on_sale_amount(&mut self, sale_amount_threshold: f64) {
        for i in 0..self.properties.len() {
            for j in (i + 1)..self.properties.len() {
                let difference = (self.properties[i].sale_amount - self.properties[j].sale_amount).abs();
                if difference <= sale_amount_threshold {
                    let weight = 1.0 / difference; // Weight is inverse of the difference
                    self.edges.push(Edge {
                        property1_id: self.properties[i].id.clone(),
                        property2_id: self.properties[j].id.clone(),
                        weight,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_properties_and_edges() {
        let mut graph = Graph::new();
        graph.add_property(Property { id: "123".to_string(), value: 100000.0 });
        graph.add_property(Property { id: "456".to_string(), value: 105000.0 });

        graph.add_edges_based_on_value();
        assert_eq!(graph.properties.len(), 2);
        assert!(graph.edges.len() > 0);
        assert!(graph.edges[0].weight > 0.0);
    }
}
