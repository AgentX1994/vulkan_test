use std::error;
use std::sync::Arc;

use vulkano::command_buffer::{AutoCommandBuffer, DynamicState};
use vulkano::descriptor::DescriptorSet;
use vulkano::device::Queue;

use nalgebra_glm as glm;

use super::SceneObject;
use crate::drawable::Drawable;

pub struct SceneGraph {
    parent_transform: glm::Mat4,
    world_transform: glm::Mat4,
    object: Option<SceneObject>,
    children: Vec<SceneGraph>,
}

impl Default for SceneGraph {
    fn default() -> Self {
        SceneGraph {
            parent_transform: glm::identity(),
            world_transform: glm::identity(),
            object: None,
            children: vec![],
        }
    }
}

impl SceneGraph {
    pub fn new(
        parent_transform: glm::Mat4,
        object: Option<SceneObject>,
        children: Vec<SceneGraph>,
    ) -> Self {
        SceneGraph {
            parent_transform,
            world_transform: parent_transform,
            object,
            children,
        }
    }

    pub fn get_parent_transform(&self) -> glm::Mat4 {
        self.parent_transform
    }

    pub fn set_parent_transform(
        &mut self,
        parent_transform: glm::Mat4,
        parents_transform: glm::Mat4,
    ) {
        self.parent_transform = parent_transform;
        self.update_transform(parents_transform);
    }

    pub fn get_object(&self) -> &Option<SceneObject> {
        &self.object
    }

    pub fn set_object(&mut self, object: SceneObject) {
        self.object = Some(object);
    }

    pub fn add_child(&mut self, mut child: SceneGraph) {
        child.update_transform(self.world_transform);
        self.children.push(child);
    }

    pub fn add_child_object(&mut self, child_object: SceneObject) {
        let mut node = SceneGraph::default();
        node.set_object(child_object);
        self.children.push(node);
    }

    pub fn get_children(&self) -> &[SceneGraph] {
        &self.children[..]
    }

    pub fn get_children_mut(&mut self) -> &mut [SceneGraph] {
        &mut self.children[..]
    }

    fn update_transform(&mut self, parents_transform: glm::Mat4) {
        self.world_transform = parents_transform * self.parent_transform;
        if let Some(ref mut object) = self.object {
            object.set_transform(self.world_transform);
        }

        for child in &mut self.children {
            child.update_transform(self.world_transform);
        }
    }

    pub fn draw(
        &self,
        queue: Arc<Queue>,
        dynamic_state: &DynamicState,
        view_set: Arc<dyn DescriptorSet + Send + Sync>,
        lighting_set: Arc<dyn DescriptorSet + Send + Sync>,
    ) -> Result<Vec<AutoCommandBuffer>, Box<dyn error::Error + Send + Sync>> {
        let mut vec = Vec::new();
        if let Some(ref object) = self.object {
            vec.push(object.draw(
                queue.clone(),
                dynamic_state,
                view_set.clone(),
                lighting_set.clone(),
            )?);
        }

        for child in &self.children {
            vec.append(&mut child.draw(
                queue.clone(),
                dynamic_state,
                view_set.clone(),
                lighting_set.clone(),
            )?);
        }

        Ok(vec)
    }
}
