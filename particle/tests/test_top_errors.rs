use particle::error::Error;
use pretty_assertions::assert_eq;

// ------------------------------------------------------------------------------------------------
// Integration Tests
// ------------------------------------------------------------------------------------------------

#[test]
fn test_unknown_error_display() {
    let err = Error::Unknown {
        message: "test".to_string(),
    };
    assert_eq!(
        err.to_string(),
        "An unknown error occurred; message: test".to_string()
    );
}

#[test]
fn test_unknown_error_fn_constructor() {
    let err = Error::Unknown {
        message: "test".to_string(),
    };

    let err_2 = Error::unknown("test");
    assert_eq!(err, err_2);
}

#[test]
fn test_unknown_error_from_string() {
    let err = Error::Unknown {
        message: "test".to_string(),
    };

    let err_3: Error = "test".into();
    assert_eq!(err, err_3);
}
