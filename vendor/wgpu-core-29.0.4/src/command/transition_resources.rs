use alloc::{sync::Arc, vec::Vec};

use thiserror::Error;
use wgt::error::{ErrorType, WebGpuError};

use crate::{
    command::{encoder::EncodingState, ArcCommand, CommandEncoder, EncoderStateError},
    device::DeviceError,
    global::Global,
    id::{BufferId, CommandEncoderId, TextureId},
    resource::{Buffer, InvalidResourceError, ParentDevice, Texture},
    track::ResourceUsageCompatibilityError,
};

impl Global {
    pub fn command_encoder_transition_resources(
        &self,
        command_encoder_id: CommandEncoderId,
        buffer_transitions: impl Iterator<Item = wgt::BufferTransition<BufferId>>,
        texture_transitions: impl Iterator<Item = wgt::TextureTransition<TextureId>>,
    ) -> Result<(), EncoderStateError> {
        profiling::scope!("CommandEncoder::transition_resources");

        let hub = &self.hub;

        // Lock command encoder for recording
        let cmd_enc = hub.command_encoders.get(command_encoder_id);
        let mut cmd_buf_data = cmd_enc.data.lock();
        cmd_buf_data.push_with(|| -> Result<_, TransitionResourcesError> {
            Ok(ArcCommand::TransitionResources {
                buffer_transitions: buffer_transitions
                    .map(|t| {
                        Ok(wgt::BufferTransition {
                            buffer: self.resolve_buffer_id(t.buffer)?,
                            state: t.state,
                        })
                    })
                    .collect::<Result<_, TransitionResourcesError>>()?,
                texture_transitions: texture_transitions
                    .map(|t| {
                        Ok(wgt::TextureTransition {
                            texture: self.resolve_texture_id(t.texture)?,
                            selector: t.selector,
                            state: t.state,
                        })
                    })
                    .collect::<Result<_, TransitionResourcesError>>()?,
            })
        })
    }
}

pub(crate) fn transition_resources(
    state: &mut EncodingState,
    buffer_transitions: Vec<wgt::BufferTransition<Arc<Buffer>>>,
    texture_transitions: Vec<wgt::TextureTransition<Arc<Texture>>>,
) -> Result<(), TransitionResourcesError> {
    let mut usage_scope = state.device.new_usage_scope();
    let indices = &state.device.tracker_indices;
    usage_scope.buffers.set_size(indices.buffers.size());
    usage_scope.textures.set_size(indices.textures.size());

    // Process buffer transitions
    for buffer_transition in buffer_transitions {
        buffer_transition.buffer.same_device(state.device)?;

        usage_scope
            .buffers
            .merge_single(&buffer_transition.buffer, buffer_transition.state)?;
    }

    // Process texture transitions
    for texture_transition in texture_transitions {
        texture_transition.texture.same_device(state.device)?;

        unsafe {
            usage_scope.textures.merge_single(
                &texture_transition.texture,
                texture_transition.selector,
                texture_transition.state,
            )
        }?;
    }

    // Record any needed barriers based on tracker data
    CommandEncoder::insert_barriers_from_scope(
        state.raw_encoder,
        state.tracker,
        &usage_scope,
        state.snatch_guard,
    );
    Ok(())
}

/// Error encountered while attempting to perform [`Global::command_encoder_transition_resources`].
#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum TransitionResourcesError {
    #[error(transparent)]
    Device(#[from] DeviceError),
    #[error(transparent)]
    EncoderState(#[from] EncoderStateError),
    #[error(transparent)]
    InvalidResource(#[from] InvalidResourceError),
    #[error(transparent)]
    ResourceUsage(#[from] ResourceUsageCompatibilityError),
}

impl WebGpuError for TransitionResourcesError {
    fn webgpu_error_type(&self) -> ErrorType {
        match self {
            Self::Device(e) => e.webgpu_error_type(),
            Self::EncoderState(e) => e.webgpu_error_type(),
            Self::InvalidResource(e) => e.webgpu_error_type(),
            Self::ResourceUsage(e) => e.webgpu_error_type(),
        }
    }
}
