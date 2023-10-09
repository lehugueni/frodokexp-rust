#include "frodokexp.h"
#include <stdio.h>

int main(void)
{
	unsigned char seed[SEED_SIZE_BYTES];
	unsigned char sk_a[SK_SIZE_BYTES];
	unsigned char f_a[F_SIZE_BYTES];
	unsigned char pk_a[PK_SIZE_BYTES];
	unsigned char sk_b[SK_SIZE_BYTES];
	unsigned char f_b[F_SIZE_BYTES];
	unsigned char pk_b[PK_SIZE_BYTES];
	unsigned char key_enc[SS_SIZE_BYTES];
	unsigned char key_dec[SS_SIZE_BYTES];
	unsigned long long ct;

	frodokexp_gen_pp(seed);
	frodokexp_gen_a(seed, sk_a, f_a, pk_a);
	frodokexp_gen_b(seed, sk_b, f_b, pk_b);
	frodokexp_encaps(pk_a, sk_b, key_enc, &ct); 
	frodokexp_decaps(pk_b, sk_a, f_a, &ct, key_dec); 

	printf("A's key: ");
	for (int i = 0; i < SS_SIZE_BYTES; i++) {
		printf("%02x", key_enc[i]);
	}
	printf("\nB's key: ");
	for (int i = 0; i < SS_SIZE_BYTES; i++) {
		printf("%02x", key_dec[i]);
	}
	printf("\n");
	return 0;
}

