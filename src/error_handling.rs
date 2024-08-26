

#[derive(Debug)]
pub enum AppError {
    HttpRequestError(reqwest::Error),
    DatabaseError(sea_orm::DbErr),
}

impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::DatabaseError(err)
    }
    
}


impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::HttpRequestError(err)
    }
}
