use cucumber::World;

mod behaviours;

use behaviours::{common::TestWorld, steps};

fn main() {
    futures::executor::block_on(TestWorld::run("../../../features"));
}