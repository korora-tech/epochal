use cucumber::World;

mod behaviours;

fn main() {
    use behaviours::common::TestWorld;

    // Ensure step modules are linked (they contain cucumber step definitions)
    #[allow(unused_imports)]
    use behaviours::steps::{app_steps, tray_steps, ui_steps, workflow_steps};

    futures::executor::block_on(TestWorld::run("../../features"));
}
