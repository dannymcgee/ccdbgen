#pragma once
#include <memory>

struct MultiBuf;

class BlobstoreClient {
public:
	BlobstoreClient();

	auto put(MultiBuf& buf) const -> uint64_t;
};

auto new_blobstore_client() -> std::unique_ptr<BlobstoreClient>;
