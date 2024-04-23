use wgsl_testing::run;

pub fn main() {
    pollster::block_on(run());
}
