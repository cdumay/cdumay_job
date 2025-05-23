#[cfg(test)]
mod test {
    use cdumay_job::ResultBuilder;
    use serde_value::Value;
    use std::collections::BTreeMap;

    #[test]
    fn test_builder() {
        let result = ResultBuilder::default()
            .stdout("Test".to_string())
            .retval({
                let mut extra = BTreeMap::new();
                extra.insert("Hello".into(), Value::String("World".to_string()));
                extra
            })
            .build();
        assert_eq!(result.stdout, Some("Test".to_string()));
        assert!(result.retval.contains_key("Hello"))
    }
}
