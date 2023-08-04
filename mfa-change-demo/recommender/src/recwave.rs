use std::collections::HashMap;
use std::time::{UNIX_EPOCH, SystemTime, SystemTimeError, Duration};
use serde_json::Map;
use tonic::{Request, Response, Status};
use tonic::transport::Channel;
use crate::kafka::KafkaSink;
use crate::model::model_server::Model;
use crate::model::{ RecallRequest, RecallResponse};
use crate::ModelClient;
use crate::recommender::{GetRecommendationRequest, GetRecommendationResponse, ReportActionRequest, ReportActionResponse};
use crate::recommender::recommender_server::Recommender;

pub struct Recwave{
    pub(crate) kafka: KafkaSink,
    pub(crate) record_id: usize, 
    pub(crate) mock: bool, 
}

pub struct RecwaveModelClient {}


#[tonic::async_trait]
impl Recommender for Recwave {
    async fn get_recommendation(&self, request: Request<GetRecommendationRequest>)
                                -> Result<Response<GetRecommendationResponse>, Status> {
        let userid = request.into_inner().userid;
        println!("Recwave::get_recommendation: userid={}", userid);
        let recall_count = self.recall(userid.clone()).await.unwrap();
        Ok(Response::new(GetRecommendationResponse {
            count: recall_count,
        }))
        // match recall_response {
        //     Ok(item_ids) => {
        //
        //     }
        //     Err(e) => {
        //         Err(e)
        //     }
        // }
    }

    async fn report_action(&self, request: Request<ReportActionRequest>)
        -> Result<Response<ReportActionResponse>, Status> {
        let message = request.into_inner();
        self.mock_report_action(&message).await
    }
}

impl Recwave{
    async fn mock_report_action(&self, message: &ReportActionRequest) -> Result<Response<ReportActionResponse>, Status> {
        let duration = SystemTime::now().duration_since(UNIX_EPOCH);
        // todo: format duration to SQL acceptable timestamp before SQLization
        // println!("received action from user `{}` on item `{}`", &message.userid, &message.itemid);
        match duration {
            Ok(dur) => {
                let timestamp = dur.as_micros();
                let json = Self::create_sink_json(message, timestamp as u64);
                println!("timestamp: {}, payload: {}", timestamp, json.clone());
                self.kafka.send("0".to_string(), json).await;
                Ok(Response::new(ReportActionResponse {
                    timestamp: timestamp as u64
                }))
            }
            Err(e) => {
                Err(Status::unknown("Failed to generate timestamp".to_string()))
            }
        }
    }

    fn mock_get_recommendation(userid: String) -> Result<Response<GetRecommendationResponse>, Status> {
        // println!("{} requested recommendations", userid);
        Ok(Response::new(GetRecommendationResponse {
            count: 0
        }))
    }


    pub(crate) fn create_sink_json(message: &ReportActionRequest, timestamp: u64) -> String {
        format!("{{\"userid\": {}, \"eventype\": {}, \"changenum\": {}, \"timestamp\": {}}}",
            message.userid, message.eventtype, message.changenum, timestamp).to_string()
    }
}