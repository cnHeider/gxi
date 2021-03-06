//use serdie::{Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateParams {
    pub view_id: String,
    pub update: UpdateUpdate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUpdate {
    pub ops: Vec<UpdateOp>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateOp {
    pub op: String,
    pub n: u64,
    pub lines: Option<Vec<UpdateLines>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateLines {
    pub cursor: Option<Vec<usize>>,
    pub text: String,
    pub styles: Option<Vec<usize>>,
}
