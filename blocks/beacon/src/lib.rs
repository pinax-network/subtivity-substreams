pub mod pb;
use pb::sf::beacon::r#type::v1::{block::Body::*, Block as BeaconBlock};
use pb::subtivity::v1::BlockStats;
use substreams::errors::Error;

#[substreams::handlers::map]
fn map_block_stats(blk: BeaconBlock) -> Result<BlockStats, Error> {
    let blobs = match blk.body.unwrap() {
        Deneb(body) => body
            .embedded_blobs
            .into_iter()
            .collect(),
        _ => vec![],
    };

    Ok(BlockStats {
        transaction_traces: 0,
        trace_calls: 0,
        uaw: vec![],
        blobs: blobs.len() as i64,
    })
}
