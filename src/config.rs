#[derive(Clone)]
pub struct Config {
    pub query_list: bool,
    pub query_select: Vec<String>,
    pub query_groupby: String,
}
