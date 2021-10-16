pub mod transaction;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct BaseResponse<T> {
    status: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}
