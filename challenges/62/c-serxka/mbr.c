#include <byteswap.h>
#include <errno.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "types.h"

#define DIE(...)                              \
	do {                                  \
		fprintf(stderr, __VA_ARGS__); \
		exit(EXIT_FAILURE);           \
	} while (0)

#define DIE_ERR(FN) DIE(#FN ": %s\n", strerror(errno))

uint8_t* bootsector(const char* file) {
	FILE* disk = fopen(file, "r");
	if (disk == NULL)
		DIE("fopen (%s): %s\n", file, strerror(errno));
	uint8_t* data = malloc(512);
	if (data == NULL)
		DIE_ERR(malloc);
	if (fread(data, 1, 512, disk) < 0)
		DIE_ERR(fread);
	fclose(disk);

	return data;
}

int main(int argc, const char* argv[]) {
	if (argc < 2)
		DIE("Usage: %s [DISK]\nDisk file was not specified.\n",
		    argv[0]);

	// Read the bootsector (first 512-bytes)
	uint8_t* bs = bootsector(argv[1]);
	// Check we have the boot signature
	if (bs[510] != 0x55 || bs[511] != 0xAA)
		DIE("File \"%s\" does not appear to be a valid MBR disk\n",
		    argv[1]);
	// Loop through partition entries
	for (size_t i = 0; i < 4; ++i) {
		size_t off = 0x1BE + 0x10 * i;

		printf("Partition %d:\n", i + 1);

		// Is the partition present?
		if (bs[off + 0x4] == 0x0) {
			printf("\tPresent: no\n");
			continue;
		}
		printf("\tPresent: yes\n");

		// Is the disk bootable?
		printf("\tBootable: %s\n", (bs[off] == 0x80) ? "yes" : "no");

		uint8_t type = bs[off + 0x4];
		bool found = false;
		for (size_t j = 0; partition_types[j].name; ++j) {
			if (type == partition_types[j].type) {
				printf("\tPartition Type: %d %s\n",
				       partition_types[j].type,
				       partition_types[j].name);
				found = true;
				break;
			}
		}
		if (found == false)
			printf("\tPartition Type: %d (Unknown)\n", type);

		// Print LBA address and sector length
		uint32_t lba_address = *(uint32_t*)(bs + off + 0x8);
		uint32_t sector_len = *(uint32_t*)(bs + off + 0xC);
#if __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
		lba_address = bswap_32(lba_address);
		sector_len = bswap_32(sector_len);
#endif
		printf("\tLBA Address: %x\n\tLength (in sectors): %d\n",
		       lba_address, sector_len);
	}

	free(bs);
}
