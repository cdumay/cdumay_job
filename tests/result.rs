#[cfg(test)]
mod test {
    use cdumay_core::Error;
    use cdumay_error_standard::Unexpected;
    use cdumay_job::{ResultBuilder, Result};
    use serde_value::Value;
    use std::collections::BTreeMap;
    use std::ops::Add;
    use uuid::Uuid;

    #[test]
    fn test_error() {
        let err: Error = Unexpected::new()
            .with_details({
                let mut extra = BTreeMap::new();
                extra.insert("context".into(), Value::String("Example".to_string()));
                extra
            })
            .into();
        let r1 = Result::from(err);
        assert_eq!(r1.is_error(), true);
        assert_eq!(r1.stderr, Some("Unexpected error".to_string()))
    }

    #[test]
    fn test_add_err_ok() {
        let r1 = ResultBuilder::default()
            .retcode(1)
            .stderr("Something wrong!".into())
            .retval({
                let mut retval = BTreeMap::new();
                retval.insert("Error".into(), Value::String("Err-46892".to_string()));
                retval
            })
            .build();
        assert_eq!(r1.is_error(), true);
        assert_eq!(
            format!("{}", r1),
            "Err(1, stderr: Some(\"Something wrong!\"))".to_string()
        );

        let r2 = ResultBuilder::default()
            .stdout("Ok !".into())
            .retval({
                let mut retval = BTreeMap::new();
                retval.insert("Success".into(), Value::String("Yes!".to_string()));
                retval
            })
            .build();
        assert_eq!(
            format!("{}", r2),
            "Ok(0, stdout: Some(\"Ok !\"))".to_string()
        );

        let r3 = r1.add(&r2);
        assert_eq!(r3.is_error(), true);
        let r4 = r2.add(&r1);
        assert_eq!(r4.is_error(), true);
    }
    #[test]
    fn test_add_stdout() {
        let r1 = ResultBuilder::default().stdout("Ok!".into()).build();
        let r2 = ResultBuilder::default().stdout("Ok!".into()).build();
        let r3 = r1.add(&r2);
        assert_eq!(r3.is_error(), false);
        assert_eq!(r3.stdout, Some("Ok!\nOk!".to_string()));
        assert_eq!(r3.stderr, None);
    }
    #[test]
    fn test_add_stderr() {
        let r1 = ResultBuilder::default()
            .retcode(1)
            .stderr("Error!".into())
            .build();
        let r2 = ResultBuilder::default().stderr("Error!".into()).build();
        let r3 = r1.add(&r2);
        assert_eq!(r3.is_error(), true);
        assert_eq!(r3.stdout, None);
        assert_eq!(r3.stderr, Some("Error!\nError!".to_string()));
    }
    #[test]
    fn test_ser() {
        let r1 = ResultBuilder::default()
            .uuid(Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap())
            .stdout("Ok!".into())
            .retval({
                let mut retval = BTreeMap::new();
                retval.insert("context".into(), Value::String("Example".to_string()));
                retval
            })
            .build();
        let out = format!("{}", serde_json::to_string(&Result::from(r1)).unwrap());
        assert_eq!(out, "{\"uuid\":\"550e8400-e29b-41d4-a716-446655440000\",\"retcode\":0,\"stdout\":\"Ok!\",\"stderr\":null,\"retval\":{\"context\":\"Example\"}}".to_string())
    }
}
