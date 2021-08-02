type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
use serde::Serialize;
use serde::Deserialize;
use std::fs;
use std::str;
use std::io::{self, Cursor};



#[derive(Debug, Clone,Serialize, Deserialize)]
pub struct Book{
    pub code: String,
    pub name: String, 
    pub pdf: String,
    pub web: String, 
}

impl Book{
    pub fn load(path: &str)-> Vec<Self>{
        let data = fs::read_to_string(path).expect("Unable to read books listings...");
        let v: Vec<Self> = serde_json::from_str(&data).unwrap();
        v
    }
    pub async fn get(&self, format: &str)-> Result<()>{
        let mut url = "";
        if format == "pdf"{
            url = &self.pdf;
        } else if format == "web" {
            url = &self.web;
        }
        let response = reqwest::get(url).await?;
        let mut file = std::fs::File::create(&format!("{}-{}.zip", self.name.replace(" ", "_"), format))?;
        let mut content =  Cursor::new(response.bytes().await?);
        io::copy(&mut content, &mut file)?;
        Ok(())
    }
}