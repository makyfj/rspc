use std::marker::PhantomData;

use futures::{Stream, StreamExt};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;
use specta::{DefOpts, Type, TypeDefs};

use crate::{
    internal::{ProcedureDataType, RequestResult, StreamFuture, TypedRequestFuture},
    ExecError,
};

pub trait RequestResolver<TCtx, TMarker, TResultMarker>: Send + Sync + 'static {
    type Arg: DeserializeOwned + Type;
    type Result: RequestResult<TResultMarker>;
    type Data;

    fn exec(
        &self,
        ctx: TCtx,
        input: Self::Arg,
    ) -> Result<TypedRequestFuture<<Self::Result as RequestResult<TResultMarker>>::Data>, ExecError>;

    fn typedef(defs: &mut TypeDefs) -> ProcedureDataType;
}

pub struct DoubleArgMarker<TArg, TResultMarker>(
    /* private */ PhantomData<(TArg, TResultMarker)>,
);
impl<TFunc, TCtx, TArg, TResult, TResultMarker>
    RequestResolver<TCtx, DoubleArgMarker<TArg, TResultMarker>, TResultMarker> for TFunc
where
    TArg: DeserializeOwned + Type,
    TFunc: Fn(TCtx, TArg) -> TResult + Send + Sync + 'static,
    TResult: RequestResult<TResultMarker>,
{
    type Result = TResult;
    type Arg = TArg;
    type Data = TResult::Data;

    fn exec(
        &self,
        ctx: TCtx,
        input: Self::Arg,
    ) -> Result<TypedRequestFuture<TResult::Data>, ExecError> {
        self(ctx, input).into_request_future()
    }

    fn typedef(defs: &mut TypeDefs) -> ProcedureDataType {
        ProcedureDataType {
            arg_ty: <TArg as Type>::reference(
                DefOpts {
                    parent_inline: false,
                    type_map: defs,
                },
                &[],
            ),
            result_ty: <TResult::Data as Type>::reference(
                DefOpts {
                    parent_inline: false,
                    type_map: defs,
                },
                &[],
            ),
            inline_arg_ty: <TArg as Type>::inline(
                DefOpts {
                    parent_inline: true,
                    type_map: defs,
                },
                &[],
            ),
            inline_result_ty: <TResult::Data as Type>::inline(
                DefOpts {
                    parent_inline: true,
                    type_map: defs,
                },
                &[],
            ),
        }
    }
}

pub trait StreamResolver<TCtx, TMarker> {
    fn exec(&self, ctx: TCtx, input: Value) -> Result<StreamFuture, ExecError>;

    fn typedef(defs: &mut TypeDefs) -> ProcedureDataType;
}

pub struct DoubleArgStreamMarker<TArg, TResult, TStream>(
    /* private */ PhantomData<(TArg, TResult, TStream)>,
);
impl<TFunc, TCtx, TArg, TResult, TStream>
    StreamResolver<TCtx, DoubleArgStreamMarker<TArg, TResult, TStream>> for TFunc
where
    TArg: DeserializeOwned + Type,
    TFunc: Fn(TCtx, TArg) -> TStream,
    TStream: Stream<Item = TResult> + Send + Sync + 'static,
    TResult: Serialize + Type,
{
    fn exec(&self, ctx: TCtx, input: Value) -> Result<StreamFuture, ExecError> {
        let input = serde_json::from_value(input).map_err(ExecError::DeserializingArgErr)?;

        Ok(Box::pin(self(ctx, input).map(|v| {
            serde_json::to_value(&v).map_err(ExecError::SerializingResultErr)
        })))
    }

    fn typedef(defs: &mut TypeDefs) -> ProcedureDataType {
        ProcedureDataType {
            arg_ty: <TArg as Type>::reference(
                DefOpts {
                    parent_inline: false,
                    type_map: defs,
                },
                &[],
            ),
            result_ty: <TResult as Type>::reference(
                DefOpts {
                    parent_inline: false,
                    type_map: defs,
                },
                &[],
            ),
            inline_arg_ty: <TArg as Type>::inline(
                DefOpts {
                    parent_inline: true,
                    type_map: defs,
                },
                &[],
            ),
            inline_result_ty: <TResult as Type>::inline(
                DefOpts {
                    parent_inline: true,
                    type_map: defs,
                },
                &[],
            ),
        }
    }
}
