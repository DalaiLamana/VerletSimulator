pub struct VerletSimulator {
    all_positions: Vec<i32>,
    all_accelerations: Vec<i32>,
    all_velocities: Vec<i32>,
    all_masses: Vec<i32>,
    particles: Vec<i32>,
    _delta_t: i32,
}

trait Simulator {
    fn create_particle(&self, posx: i32, posy: i32, vx: i32, vy: i32);
    fn simulate_step(&self);
    fn _calculate_accelerations(&self, positions: &Vec<i32>) -> &Vec <i32>;
    fn _calculate_movement(&self);
    fn basic_force_function(p1 :&Vec<i32>, p2 :&Vec<i32>) -> i32{
        
    }
    fn distance(x1: i32, y1: i32, x2: i32, y2: i32) -> u32{

    }
}

impl Simulator for VerletSimulator {

}