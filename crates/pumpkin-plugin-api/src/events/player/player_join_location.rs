use crate::wit::pumpkin::plugin::event::{Event, EventType, PlayerJoinLocationEventData};

use super::super::FromIntoEvent;

/// An event that occurs when a player joins, before they are added to a world.
///
/// Change the world, position or rotation in [`PlayerJoinLocationEventData`] to spawn
/// the player somewhere else without teleporting them after the join. The location from
/// the player's saved data is in `saved_location`.
pub struct PlayerJoinLocationEvent;
impl FromIntoEvent for PlayerJoinLocationEvent {
    const EVENT_TYPE: EventType = EventType::PlayerJoinLocationEvent;
    type Data = PlayerJoinLocationEventData;

    fn data_from_event(event: Event) -> Self::Data {
        match event {
            Event::PlayerJoinLocationEvent(data) => data,
            _ => panic!("unexpected event"),
        }
    }

    fn data_into_event(data: Self::Data) -> Event {
        Event::PlayerJoinLocationEvent(data)
    }
}
