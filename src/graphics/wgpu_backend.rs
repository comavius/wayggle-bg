use uuid::Uuid;
use wgpu::*;

pub struct Graphics<'a>
{
    device: Device,
    queue: Queue,
    surface: Surface<'a>,
    render_passes: Vec<PipelinePass<'a>>,
}

enum PipelinePass<'a>
{
    ComputePass(ComputePassConfigurationBundle<'a>),
    RenderPass(RenderPassConfigurationBundle<'a>),
}

struct ComputePassConfigurationBundle<'a>
{
    descriptor: wgpu::ComputePassDescriptor<'a>,
}

struct RenderPassConfigurationBundle<'a>
{
    descriptor: wgpu::RenderPassDescriptor<'a>,
}

#[derive(Debug, thiserror::Error)]
pub enum GraphicsInitializationError
{
    #[error("Adapter request error: {0}")]
    AdapterRequestError(#[from] wgpu::RequestAdapterError),
    #[error("Device request error: {0}")]
    DeviceRequestError(#[from] RequestDeviceError),
    #[error("Surface creation error: {0}")]
    SurfaceCreationError(#[from] CreateSurfaceError),
}

impl<'a> Graphics<'a>
{
    pub async fn new(
        surface_generator: impl Fn(
            Instance,
        )
            -> Result<Surface<'a>, CreateSurfaceError>
        + 'a,
    ) -> Result<Self, GraphicsInitializationError>
    {
        // Initialize wgpu instance, adapter, device and queue
        // (blocking for simplicity)
        let instance =
            Instance::new(&InstanceDescriptor::from_env_or_default());
        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await
            .map_err(GraphicsInitializationError::AdapterRequestError)?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor { ..Default::default() })
            .await
            .map_err(|e| GraphicsInitializationError::DeviceRequestError(e))?;
        let surface = surface_generator(instance).map_err(|e| {
            GraphicsInitializationError::SurfaceCreationError(e)
        })?;
        return Ok(Graphics { device, queue, surface, render_passes: vec![] });
    }

    pub fn render(&self) -> ()
    {
        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                ..Default::default()
            });
        for render_pass in &self.render_passes {
            match render_pass {
                PipelinePass::ComputePass(conf) => {
                    let mut pass = encoder.begin_compute_pass(&conf.descriptor);
                }
                PipelinePass::RenderPass(conf) => {
                    let mut pass = encoder.begin_render_pass(&conf.descriptor);
                }
            }
        }
        let buffer = encoder.finish();
        self.queue
            .submit(Some(buffer));
    }
}
