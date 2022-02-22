#![doc = include_str!("../README.md")]

pub mod terminate;
pub use futility_try_catch::try_;

#[test]
fn try_catch() {
    use std::error::Error;
    let mut errored: Option<Box<dyn Error>> = None;

    try_!({
      test_failure()?;
    } catch Box<dyn Error> as err {
      errored = Some(err);
    });

    assert!(errored.is_some());
    fn test_failure() -> Result<(), Box<dyn Error>> {
        Err("Always Fails".into())
    }
}

#[test]
fn try_catch_ret_val() {
    use std::error::Error;
    let mut errored: Option<Box<dyn Error>> = None;

    let val = try_!({
      "Will not fail"
    } catch Box<dyn Error> as err {
      errored = Some(err);
      "use this if failed"
    });

    assert!(errored.is_none());
    assert_eq!(val, "Will not fail");
}
