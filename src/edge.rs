pub struct Edge {
    pub property1_id: String,
    pub property2_id: String,
    pub weight: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_edge() {
        let edge = Edge {
            property1_id: "123".to_string(),
            property2_id: "456".to_string(),
            weight: 0.5,
        };
        assert_eq!(edge.property1_id, "123");
        assert_eq!(edge.property2_id, "456");
        assert_eq!(edge.weight, 0.5);
    }
}
