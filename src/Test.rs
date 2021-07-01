
use chrono::{Duration, Utc};

use crate::GetHistData::{
    GetTickDukas,
};

#[test]
fn it_works() {
    test();
    assert_eq!(2 + 2, 4);
}

fn test() {
    let sym = GetTickDukas(
        "USDJPY".to_string(),
        Utc::now() - Duration::days(10),
        Utc::now(),
    );
}
