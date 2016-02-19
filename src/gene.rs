// A struct representing a gene
// Each type of gene can store extra fields (through Variant enum)
pub struct Gene {
    weight: f64,
    id: usize,
    variant: Variant
}

pub enum Variant {
    Neuron(f64, usize),
    Input(f64),
    Forward,
    Recurrent,
    Bias
}
