use super::client::ApiClient;
use super::model::SignInInfo;

fn sign_in_info() -> SignInInfo {
    SignInInfo { email: "xxxx@xxx.com".to_string(), password: "xxxxx".to_string() }
}

async fn sign_in_client() -> ApiClient {
    let mut client = ApiClient::new().unwrap();
    let sign_in_info = sign_in_info();
    client.sign_in(&sign_in_info).await.unwrap();
    client
}

#[test]
fn test_new() {
    assert!(ApiClient::new().is_ok());
}

#[tokio::test]
async fn test_sign_in() {
    let mut client = ApiClient::new().unwrap();
    let sign_in_info = sign_in_info();
    assert!(client.sign_in(&sign_in_info).await.is_ok());
}

#[tokio::test]
async fn test_get_authentication() {
    let client = sign_in_client().await;
    assert!(client.get_authentication().await.is_ok());
}

#[tokio::test]
async fn test_delete_authentication() {
    let client = sign_in_client().await;
    assert!(client.delete_authentication().await.is_ok());
    assert!(client.get_authentication().await.is_err());
}

#[tokio::test]
async fn test_option_simulations() {
    let client = sign_in_client().await;
    assert!(client.option_simulations().await.is_ok());
}

#[tokio::test]
async fn test_post_simulations() {
    let client = sign_in_client().await;
    let simulation_obj = r#"
           [ {
            "type":"REGULAR",
            "settings":{
                "maxTrade":"OFF",
                "nanHandling":"OFF",
                "instrumentType":"EQUITY",
                "delay":1,
                "universe":"TOP500",
                "truncation":0.08,
                "unitHandling":"VERIFY",
                "maxPosition":"OFF",
                "testPeriod":"P1Y",
                "pasteurization":"ON",
                "region":"USA",
                "language":"FASTEXPR",
                "decay":0,
                "neutralization":"SUBINDUSTRY",
                "visualization":false
            },
            "regular":"zscore(cash_st / debt_st)"
        },
        {
            "type":"REGULAR",
            "settings":{
                "maxTrade":"OFF",
                "nanHandling":"OFF",
                "instrumentType":"EQUITY",
                "delay":1,
                "universe":"TOP500",
                "truncation":0.08,
                "unitHandling":"VERIFY",
                "maxPosition":"OFF",
                "testPeriod":"P1Y",
                "pasteurization":"ON",
                "region":"USA",
                "language":"FASTEXPR",
                "decay":0,
                "neutralization":"SUBINDUSTRY",
                "visualization":false
            },
            "regular":"close"
        }
        ]
        "#;
    assert!(client.post_simulations(simulation_obj).await.is_ok());
}

#[tokio::test]
async fn test_get_alphas() {
    let client = sign_in_client().await;
    assert!(client.alphas("78KkV3oQ").await.is_ok());
}

#[tokio::test]
async fn test_alpha_recordsets() {
    let client = sign_in_client().await;
    assert!(client.alpha_recordsets("78KkV3oQ").await.is_ok());
}

#[tokio::test]
async fn test_alpha_recordsets_name() {
    let client = sign_in_client().await;
    assert!(client.alpha_recordsets_name("78KkV3oQ", "pnl").await.is_ok());
}

#[tokio::test]
async fn test_user_activities_diversities() {
    let client = sign_in_client().await;
    assert!(client.user_activities_diversities().await.is_ok());
}
