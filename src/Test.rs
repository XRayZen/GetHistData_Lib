
use chrono::{Duration, Utc};

use crate::{GetHistData::{
    GetTickDukas,
}, Get_HistricRate};

#[tokio::test]
async fn it_works() {
    test().await;
    assert_eq!(2 + 2, 4);
}

async fn test() {
    let from=Utc::now() - Duration::days(10);
    let to=Utc::now();
    let sym = Get_HistricRate("USDJPY", &from,
     &to, &crate::DataProviderType::Dukascopy, &crate::GetHistDataType::Tick).await;

}
