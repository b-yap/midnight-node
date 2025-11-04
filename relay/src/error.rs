use subxt::ext::subxt_rpcs;

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Failed to read Beefy keys from {0}")]
	InvalidKeysFile(String),

	#[error("Failed to parse {0}")]
	JsonDecodeError(String),

	// --------- Subxt errors ---------
	#[error("Subxt Error: {0}")]
	Subxt(#[from] subxt::Error),

	#[error("Rpc Error: {0}")]
	Rpc(#[from] subxt_rpcs::Error),

	#[error("Codec Error: {0}")]
	Codec(#[from] parity_scale_codec::Error),
}
