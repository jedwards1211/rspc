use std::{future::ready, marker::PhantomData};

use futures::{FutureExt, Stream, TryStreamExt};
use serde_json::Value;

use crate::{
    error::ExecError,
    internal::middleware::{new_mw_ctx, MiddlewareFn, RequestContext},
};

use super::Layer;

#[doc(hidden)]
pub struct MiddlewareLayer<TLayerCtx, TNewCtx, TNextLayer, TNewMiddleware> {
    pub(crate) next: TNextLayer,
    pub(crate) mw: TNewMiddleware,
    pub(crate) phantom: PhantomData<(TLayerCtx, TNewCtx)>,
}

impl<TLayerCtx, TNewCtx, TNextMiddleware, TNewMiddleware> Layer<TLayerCtx>
    for MiddlewareLayer<TLayerCtx, TNewCtx, TNextMiddleware, TNewMiddleware>
where
    TLayerCtx: Send + Sync + 'static,
    TNewCtx: Send + Sync + 'static,
    TNextMiddleware: Layer<TNewCtx> + Sync + 'static,
    TNewMiddleware: MiddlewareFn<TLayerCtx, TNewCtx> + Send + Sync + 'static,
{
    async fn call(
        &self,
        ctx: TLayerCtx,
        input: Value,
        req: RequestContext,
    ) -> Result<impl Stream<Item = Result<Value, ExecError>> + Send + 'static, ExecError> {
        todo!();
        // let y = self.mw.execute(ctx, new_mw_ctx(input, req)).await;s

        // TODO: In this case `resp_fn` is being borrowed. Can we avoid that???
        // self.next.call(ctx, input, req).await.map(move |stream| {
        //     stream.and_then(move |v| {
        //         match &resp_fn {
        //             Some(resp_fn) => resp_fn.call(v).left_future(),
        //             None => ready(v).right_future(),
        //         }
        //         .map(Ok)
        //     })
        // })

        Ok(futures::stream::iter([]))
    }
}
