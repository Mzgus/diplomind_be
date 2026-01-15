use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

fn default_page() -> u32 { 1 }
fn default_per_page() -> u32 { 20 }

impl PaginationParams {
    pub fn validate(&mut self) {
        if self.page < 1 {
            self.page = 1;
        }
        if self.per_page > 100 {
            self.per_page = 100;
        }
        if self.per_page < 1 {
            self.per_page = 20;
        }
    }
    
    pub fn offset(&self) -> i64 {
        ((self.page - 1) * self.per_page) as i64
    }
    
    pub fn limit(&self) -> i64 {
        self.per_page as i64
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub page: u32,
    pub per_page: u32,
    pub total: i64,
    pub total_pages: u32,
}
