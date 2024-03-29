use crate::model::Component;
use crate::model::ComponentTypeId;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::PropertyType;

#[derive(Debug)]
pub enum ComponentManagerError {
    InitializationError,
}

#[derive(Debug)]
pub enum ComponentCreationError {
    Failed,
}

#[derive(Debug)]
pub enum ComponentImportError {
    Failed,
}

pub trait ComponentManager: Send + Sync {
    /// Returns all components
    fn get_all(&self) -> Vec<Component>;

    /// Returns all components of the given namespace.
    fn get_by_namespace(&self, namespace: &str) -> Vec<Component>;

    /// Returns true, if a component with the given name exists.
    fn has(&self, ty: &ComponentTypeId) -> bool;

    /// Returns true, if a component with the given fully qualified name exists.
    fn has_by_type(&self, namespace: &str, name: &str) -> bool;

    /// Returns the component with the given name or empty.
    fn get(&self, ty: &ComponentTypeId) -> Option<Component>;

    /// Returns the component with the given fully qualified name or empty.
    fn get_by_type(&self, namespace: &str, name: &str) -> Option<Component>;

    /// Returns all components whose names matches the given search string.
    fn find(&self, search: &str) -> Vec<Component>;

    /// Returns the count of components.
    fn count(&self) -> usize;

    /// Returns the count of components of the given namespace.
    fn count_by_namespace(&self, namespace: &str) -> usize;

    /// Creates a new component with the given name and the given properties.
    fn create(
        &self,
        ty: &ComponentTypeId,
        description: &str,
        properties: Vec<PropertyType>,
        extensions: Vec<Extension>,
    ) -> Result<Component, ComponentCreationError>;

    /// Replaces the component with the given name with the given component.
    fn replace(&self, ty: &ComponentTypeId, component: Component);

    /// Adds a property to the component with the given name.
    fn add_property(&self, ty: &ComponentTypeId, property: PropertyType);

    /// Removes the property with the given property_name from the component with the given name.
    fn remove_property(&self, ty: &ComponentTypeId, property_name: &str);

    /// Adds an extension to the component with the given name.
    fn add_extension(&self, ty: &ComponentTypeId, extension: Extension);

    /// Removes the extension with the given type from the component with the given type.
    fn remove_extension(&self, component_ty: &ComponentTypeId, extension_ty: &ExtensionTypeId);

    /// Deletes the component with the given name.
    fn delete(&self, ty: &ComponentTypeId);

    /// Imports a component from a JSON file located at the given path.
    fn import(&self, path: &str) -> Result<Component, ComponentImportError>;

    /// Exports the component with the given name to a JSON file located at the given path.
    fn export(&self, ty: &ComponentTypeId, path: &str);
}
