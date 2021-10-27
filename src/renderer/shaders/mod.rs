pub mod vertex;
pub mod fragment;

use wgpu::ShaderModule;
use wgpu::Device;

use log::{info, warn};

use super::super::error::Error as GompError;

/// Defines the language of a shader.
#[derive(Debug)]
pub enum ShaderSourceType {
    /// A shader written in the WGSL language.
    Wgsl,

    /// __DO NOT USE. NOT IMPLEMENTED!__
    Glsl,

    // __DO NOT USE. NOT IMPLEMENTED!__
    Hlsl,
}

/// Helps to build a shader.
#[derive(Debug)]
pub struct ShaderBuilder<'a> {
    /// The source of the shader.
    source: String,

    /// The entry point of the shader. Not used in compilation,
    /// but still a nice value to be able to store along with
    /// this builder.
    pub entry: String,

    /// The language the shader is written in.
    source_type: ShaderSourceType,

    /// The label of the shader. An example could be:
    ///     `default vertex shader`
    label: String,

    /// The device to compile the shader under.
    device: Option<&'a Device>,
}

impl<'a> ShaderBuilder<'a> {
    /// Creates a new shader builder.
    pub fn new() -> Self {
        Self {
            source: "".to_owned(),
            source_type: ShaderSourceType::Wgsl,
            entry: "main".to_owned(),
            label: "".to_owned(),
            device: None,
        }
    }

    /// Adds source to the shader. Required.
    pub fn with_source(self, source_type: ShaderSourceType, source: &str) -> Self {
        Self {
            source: source.to_owned(),
            source_type,
            .. self
        }
    }

    /// Gives the shader an entry point. Not used in
    /// compilation, but rather to be put into the
    /// builder to be used later in client code.
    pub fn with_entry_point(self, entry: &str) -> Self {
        Self {
            entry: entry.to_owned(),
            .. self
        }
    }

    /// Gives the shader a label. Not required, but
    /// highly recommended.
    pub fn with_label(self, label: &str) -> Self {
        Self {
            label: label.to_owned(),
            .. self
        }
    }

    /// 
    pub fn with_device(self, device: &'a Device) -> Self {
        Self {
            device: Some(device),
            .. self
        }
    }
}

impl<'a> ShaderBuilder<'a> {
    pub fn compile(self) -> Result<ShaderModule, GompError> {
        match self.source_type {
            ShaderSourceType::Wgsl => {},
            _ => {
                info!("Supported shader types: WGSL (GLSL/HSLS not supported, as are other languages). This may be supported in future versions");
                return Err(GompError::UnsupportedShaderType);
            }
        }

        if self.source.is_empty() {
            return Err(GompError::NoShaderSource);
        }

        if self.device.is_none() {
            return Err(GompError::NoDeviceSupplied);
        }

        if self.label.is_empty() {
            warn!("No label given for new shader (before compilation). Consider giving it one to ease in debugging");
        }

        let shader = self.device.expect("unreachable panic on unwrapping of device reference in ShaderBuilder")
            .create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some(&self.label),
            source: wgpu::ShaderSource::Wgsl(self.source.into()),
        });

        Ok(shader)
    }
}

