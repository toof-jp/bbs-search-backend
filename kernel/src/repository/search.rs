use shared::error::Error;

use crate::model::search::{Count, Res, SearchOptions};

trait SearchRepository {
    fn search(&self, options: SearchOptions) -> AppResult<Vec<Res>>;
    fn count(&self, options: SearchOptions) -> AppResult<Count>;
}
