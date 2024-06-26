use reqwest;
use serde::{Deserialize};

use crate::helpers::date;

#[derive(Deserialize, Debug)]
pub struct CropDataResponse {
    #[serde(rename = "交易日期")]
    pub transaction_date: Option<String>,
    #[serde(rename = "種類代碼")]
    pub type_code: Option<String>,
    #[serde(rename = "作物代號")]
    pub crop_code: Option<String>,
    #[serde(rename = "作物名稱")]
    pub crop_name: Option<String>,
    #[serde(rename = "市場代號")]
    pub market_code: Option<String>,
    #[serde(rename = "市場名稱")]
    pub market_name: Option<String>,
    #[serde(rename = "上價")]
    pub high_price: Option<f32>,
    #[serde(rename = "中價")]
    pub mid_price: Option<f32>,
    #[serde(rename = "下價")]
    pub low_price: Option<f32>,
    #[serde(rename = "平均價")]
    pub average_price: Option<f32>,
    #[serde(rename = "交易量")]
    pub trading_volume: Option<f32>,
}

static TC_TYPE_ALLOWLIST: &'static [&str] = &["N04", "N05"];

pub async fn get_crop_transaction_history(
    top: u16,
    skip: u16,
    start_date: &date::RocDateString,
    end_date: &date::RocDateString,
) -> Result<Vec<CropDataResponse>, reqwest::Error> {
    let url = format!(
        "https://data.moa.gov.tw/Service/OpenData/FromM/FarmTransData.aspx?$top={}&$skip={}&StartDate={}&EndDate={}&UnitId=037", 
        top,
        skip,
        start_date, 
        end_date, 
    );

    let resp = reqwest::get(url)
        .await?
        .json::<Vec<CropDataResponse>>()
        .await?;

    let filterd_resp = resp.into_iter().filter(|crop_data| TC_TYPE_ALLOWLIST.contains(&crop_data.type_code.clone().unwrap_or(String::from("")).as_str())).collect();

    Ok(filterd_resp)
}
