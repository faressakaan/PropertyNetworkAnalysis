pub struct Property {
    pub id: String,
    pub sale_amount: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_property() {
        let property = Property { id: "123".to_string(), value: 100000.0 };
        assert_eq!(property.id, "123");
        assert_eq!(property.value, 100000.0);
    }
}
