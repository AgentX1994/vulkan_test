use std::error;
use std::fmt;
use std::sync::Arc;

use vulkano::device::{Device, DeviceCreationError, DeviceExtensions, Features, Queue};
use vulkano::instance::{
    ApplicationInfo, Instance, InstanceCreationError, InstanceExtensions, PhysicalDevice,
};

#[derive(Debug)]
pub enum RenderContextError {
    InstanceError(InstanceCreationError),
    DeviceError(DeviceCreationError),
    NoSupportedDevice,
}

impl fmt::Display for RenderContextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RenderContextError::InstanceError(ref e) => e.fmt(f),
            RenderContextError::DeviceError(ref e) => e.fmt(f),
            RenderContextError::NoSupportedDevice => {
                write!(f, "No physical device supports desired feature set")
            }
        }
    }
}

impl error::Error for RenderContextError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            RenderContextError::InstanceError(ref e) => Some(e),
            RenderContextError::DeviceError(ref e) => Some(e),
            RenderContextError::NoSupportedDevice => None,
        }
    }
}

impl From<InstanceCreationError> for RenderContextError {
    fn from(err: InstanceCreationError) -> RenderContextError {
        RenderContextError::InstanceError(err)
    }
}

impl From<DeviceCreationError> for RenderContextError {
    fn from(err: DeviceCreationError) -> RenderContextError {
        RenderContextError::DeviceError(err)
    }
}

pub struct RenderContext {
    instance: Arc<Instance>,
    physical_device_index: usize,
    device: Arc<Device>,
    queue: Arc<Queue>,
}

impl RenderContext {
    pub fn new(
        app_info: Option<&ApplicationInfo>,
        required_features: &Features,
        instance_extensions: &InstanceExtensions,
        device_extensions: &DeviceExtensions,
        layers: Vec<&str>,
    ) -> Result<Arc<Self>, RenderContextError> {
        let instance = Instance::new(app_info, instance_extensions, layers)?;
        let (physical_device_index, device, mut queues) = {
            let physical_device = PhysicalDevice::enumerate(&instance)
                .find(|p| p.supported_features().superset_of(&required_features))
                .ok_or(RenderContextError::NoSupportedDevice)?;

            // TODO how to choose queue families?
            let queue_family = physical_device
                .queue_families()
                .find(|&q| q.supports_graphics())
                .expect("couldn't find a graphical queue family");

            let (device, queues) = Device::new(
                physical_device,
                &Features::none(),
                device_extensions,
                [(queue_family, 0.5)].iter().cloned(),
            )?;

            (physical_device.index(), device, queues)
        };

        Ok(Arc::new(RenderContext {
            instance,
            physical_device_index,
            device,
            queue: queues.next().expect("No queue, this should never happen"),
        }))
    }

    pub fn instance(&self) -> Arc<Instance> {
        self.instance.clone()
    }

    pub fn physical_device(&self) -> PhysicalDevice {
        PhysicalDevice::from_index(&self.instance, self.physical_device_index)
            .expect("Physical device disappeared, this shouldn't happen")
    }

    pub fn device(&self) -> Arc<Device> {
        self.device.clone()
    }

    pub fn queue(&self) -> Arc<Queue> {
        self.queue.clone()
    }
}
