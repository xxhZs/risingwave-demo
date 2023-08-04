use std::time::{SystemTime, UNIX_EPOCH};
use tokio_postgres::NoTls;
use tonic::{Response, Status};
use crate::model::{RecallRequest, RecallResponse};
use crate::recommender::ReportActionResponse;
use crate::{ModelClient, Recwave};

pub const  RECALL_SQL: &str = "
select count from user_mfa_change_count where userid = $1 order by window_start limit 1;
";

impl Recwave {
    pub async fn recall(&self, userid: String) -> Result<u64, Status> {
        let request = RecallRequest {
            userid,
        };

        //"dbname=dev user=root host=127.0.0.1 port=4566"
        let con = format!("dbname=dev user=root host=127.0.0.1 port=4566");
        let (client,connection) = tokio_postgres::connect(&con, NoTls).await.unwrap();
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        let data = client.query(RECALL_SQL, &[&request.userid]).await.unwrap();
        let count: i64 = data[0].get(0);
        Ok(count as u64)
    }
    pub async fn recall_with_py(&self, userid: String) -> Result<u64, Status> {
        let request = RecallRequest {
            userid,
        };
        
        let mut model_client = ModelClient::connect("http://localhost:8080")
            .await
            .expect("Failed to connect to model server");
        let response = model_client.recall(request)
            .await;
        match response {
            Ok(resp) => {
                Ok(resp.into_inner().count)
            }
            Err(e) => {
                Err(e)
            }
        }
    }

}
