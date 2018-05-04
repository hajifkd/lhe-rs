#[derive(Debug, PartialEq)]
pub struct Event {
    pub n: i32,
    pub id: i32,
    pub event_weight: f64,
    pub scale: f64,
    pub alpha_qed: f64,
    pub alpha_qcd: f64,
    pub particles: Vec<Particle>,
}

#[derive(Debug, PartialEq)]
pub struct Particle {
    pub id: i32,
    pub status: i32,
    pub mother_id0: i32,
    pub mother_id1: i32,
    pub color0: i32,
    pub color1: i32,
    pub px: f64,
    pub py: f64,
    pub pz: f64,
    pub e: f64,
    pub m: f64,
    pub lifetime: f64,
    pub spin: f64,
}
