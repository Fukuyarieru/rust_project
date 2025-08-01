mod prelude;
mod res;

use crate::prelude::*;
use bevy::prelude::*;

fn main() {
    let app = App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .run();
}
