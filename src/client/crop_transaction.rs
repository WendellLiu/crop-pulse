use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct CropDataResponse {
    #[serde(rename = "交易日期")]
    pub transaction_date: String,
    #[serde(rename = "種類代碼")]
    pub type_code: String,
    #[serde(rename = "作物代號")]
    pub crop_code: String,
    #[serde(rename = "作物名稱")]
    pub crop_name: String,
    #[serde(rename = "市場代號")]
    pub market_code: String,
    #[serde(rename = "市場名稱")]
    pub market_name: String,
    #[serde(rename = "上價")]
    pub high_price: f32,
    #[serde(rename = "中價")]
    pub mid_price: f32,
    #[serde(rename = "下價")]
    pub low_price: f32,
    #[serde(rename = "平均價")]
    pub average_price: f32,
    #[serde(rename = "交易量")]
    pub trading_volume: f32,
}

pub async fn get_crop_transaction_history() -> Result<Vec<CropDataResponse>, reqwest::Error> {
    let resp = reqwest::get("https://data.moa.gov.tw/Service/OpenData/FromM/FarmTransData.aspx?$top=100&$skip=0&StartDate=113.01.01&EndDate=113.01.02&TcType=N05&UnitId=037")
        .await?
        .json::<Vec<CropDataResponse>>()
        .await?;

    Ok(resp)
}
