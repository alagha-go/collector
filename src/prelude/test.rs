use super::*;

#[tokio::test]
#[ignore]
async fn load_ids_test()  {
    let date = collecting_date();
    let url = format!("http://files.tmdb.org/p/exports/tv_series_ids_{date}.json.gz");
    laoad_ids(&url).await.unwrap();
}

#[test]
fn str_from_number_test() {
    assert_eq!(str_from_number(8), String::from("08"));
    assert_eq!(str_from_number(18), String::from("18"));
}