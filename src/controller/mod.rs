use bevy::prelude::*;

pub enum ControlPhyState {
    UP,
    DOWN,
    STALE,
}

trait ControlSystem {
    fn tick(&self, state: &mut ControlState) {}
}
struct ControlState {
    state: [ControlPhyState; 8],
}

impl ControlState {
    // why
    #[inline]
    fn clamp(i: u8) -> usize {
        i as usize & 0x1
    }

    pub fn get(&self, node: u8) -> &ControlPhyState {
        &self.state[Self::clamp(node)]
    }

    pub fn set(&mut self, node: u8, state: ControlPhyState) {
        self.state[Self::clamp(node)] = state
    }
}

#[derive(Resource, Default)]
pub struct ControlManager(Vec<(Box<dyn ControlSystem + Sync + Send + 'static>, ControlState)>);

#[derive(Default)]
pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ControlManager>()
            .add_systems(Startup, Self::init)
            .add_systems(Update, Self::tick);
    }
}

impl ControlPlugin {
    fn init(mut cman: ResMut<ControlManager>) {}

    fn tick(mut cman: ResMut<ControlManager>) {
        for m in &mut cman.0 {
            m.0.tick(&mut m.1)
        }
    }
}
