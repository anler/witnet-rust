use actix::prelude::*;

use crate::actors::worker;
use crate::{model, types};

pub struct GetPendingTransactions(
    pub types::SessionWallet,
    /// Offset
    pub u32,
    /// Limit
    pub u32,
);

impl Message for GetPendingTransactions {
    type Result = worker::Result<model::Transactions>;
}

impl Handler<GetPendingTransactions> for worker::Worker {
    type Result = <GetPendingTransactions as Message>::Result;

    fn handle(
        &mut self,
        GetPendingTransactions(wallet, offset, limit): GetPendingTransactions,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        self.pending_transactions(&wallet, offset, limit)
    }
}
