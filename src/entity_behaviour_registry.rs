use std::sync::Arc;

use crate::model::EntityBehaviourTypeId;
use crate::model::ReactiveEntityInstance;
use crate::reactive::BehaviourFactory;

pub trait EntityBehaviourRegistry: Send + Sync {
    /// Registers a factory for creating entity behaviours.
    /// If an entity instance is of the entity type then the given behaviour is applied.
    /// The behaviour will be created using the given EntityBehaviourCreator.
    #[allow(unused_variables)]
    fn register(&self, entity_behaviour_ty: EntityBehaviourTypeId, factory: Arc<dyn BehaviourFactory<ReactiveEntityInstance> + Send + Sync>) {}

    /// Unregisters a factory for creating entity behaviours.
    #[allow(unused_variables)]
    fn unregister(&self, entity_behaviour_ty: &EntityBehaviourTypeId) {}
}
