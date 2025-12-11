use reqwest;

#[tokio::test]
async fn test_country_cities() {
    let resp = reqwest::get("http://127.0.0.1:8000/countries/England").await.unwrap();
    assert_eq!(resp.status(), 200);
    let cities: Vec<String> = resp.json().await.unwrap();
    assert_eq!(cities, vec!["London"]);

    let resp = reqwest::get("http://127.0.0.1:8000/countries/France").await.unwrap();
    assert_eq!(resp.status(), 200);
    let cities: Vec<String> = resp.json().await.unwrap();
    assert_eq!(cities, vec!["Paris"]);

    let resp = reqwest::get("http://127.0.0.1:8000/countries/Unknown").await.unwrap();
    assert_eq!(resp.status(), 404);
}
