use std::cmp;

use actix::prelude::*;
use serde::{Deserialize, Serialize};

use crate::actors::app;
use crate::{constants, model, types};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPendingTransactionsRequest {
    session_id: types::SessionId,
    wallet_id: String,
    offset: Option<u32>,
    limit: Option<u32>,
}

pub type GetPendingTransactionsResponse = model::Transactions;

impl Message for GetPendingTransactionsRequest {
    type Result = app::Result<GetPendingTransactionsResponse>;
}

impl Handler<GetPendingTransactionsRequest> for app::App {
    type Result = app::ResponseActFuture<GetPendingTransactionsResponse>;

    fn handle(
        &mut self,
        msg: GetPendingTransactionsRequest,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let offset = msg
            .offset
            .unwrap_or_else(|| constants::DEFAULT_PAGINATION_OFFSET);
        let limit = cmp::min(
            msg.limit
                .unwrap_or_else(|| constants::DEFAULT_PAGINATION_LIMIT),
            constants::MAX_PAGINATION_LIMIT,
        );
        let f = self.get_pending_transactions(msg.session_id, msg.wallet_id, offset, limit);

        Box::new(f)
    }
}
