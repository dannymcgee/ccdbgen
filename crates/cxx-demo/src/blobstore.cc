#include "cxx-demo/include/blobstore.h"
#include "cxx-demo/src/main.rs.h"
#include <functional>
#include <string>

BlobstoreClient::BlobstoreClient()
{}

auto new_blobstore_client() -> std::unique_ptr<BlobstoreClient>
{
	return std::make_unique<BlobstoreClient>();
}

// Upload a new blob and return a blobid that serves as a handle to the blob.
auto BlobstoreClient::put(MultiBuf& buf) const -> uint64_t
{
	// Traverse the caller's chunk iterator.
	std::string contents;
	while (true) {
		auto chunk = next_chunk(buf);
		if (chunk.size() == 0)
			break;

		contents.append(reinterpret_cast<char const*>(chunk.data()), chunk.size());
	}

	// Pretend we did something useful to persist the data.
	auto blobid = std::hash<std::string>{}(contents);
	return blobid;
}
