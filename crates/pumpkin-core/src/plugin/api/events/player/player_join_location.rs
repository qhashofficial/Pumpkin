use pumpkin_macros::Event;
use pumpkin_util::math::vector3::Vector3;
use std::sync::Arc;

use crate::{entity::player::Player, world::World};

use super::PlayerEvent;

/// An event that occurs when a player joins, before they are added to a world.
///
/// The player spawns directly in `world` at `position` with the given rotation, so a
/// plugin can place them somewhere else without teleporting them after the join.
#[derive(Event, Clone)]
pub struct PlayerJoinLocationEvent {
    /// The player who is joining.
    pub player: Arc<Player>,

    /// The world the player spawns in.
    pub world: Arc<World>,

    /// The position the player spawns at.
    pub position: Vector3<f64>,

    /// The yaw the player spawns with.
    pub yaw: f32,

    /// The pitch the player spawns with.
    pub pitch: f32,

    /// The location stored in the player's saved data, or `None` on a first join.
    /// Changing it has no effect.
    pub saved_location: Option<PlayerSavedLocation>,
}

/// Where a player was when their data was last saved.
#[derive(Clone)]
pub struct PlayerSavedLocation {
    pub world: Arc<World>,
    pub position: Vector3<f64>,
    pub yaw: f32,
    pub pitch: f32,
}

impl PlayerEvent for PlayerJoinLocationEvent {
    fn get_player(&self) -> &Arc<Player> {
        &self.player
    }
}
