mod applicator;

pub use self::applicator::*;

use async_trait::async_trait;
use spectroscopy_core::resolver::ResolveMapping;
use crate::Context;
use crate::errors::AgentError;

#[async_trait]
pub(crate) trait Applier<T: ResolveMapping>: 'static + Sync + Send {
    async fn apply(self: Box<Self>, entity: &mut T, ctx: &mut Context) -> Result<(), AgentError>;
}
