pub struct UtilizationMetric {
    pub cpu_usage: f32,
    pub memory_usage: f32,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: u32,
    pub ip: String,
    pub healthy: bool,
}