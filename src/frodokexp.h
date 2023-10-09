#define SEED_SIZE_BYTES 32
#define SK_SIZE_BYTES 23232 
#define PK_SIZE_BYTES 23232
#define F_SIZE_BYTES 128
#define SS_SIZE_BYTES 32

void frodokexp_gen_pp(unsigned char *seed);
void frodokexp_gen_a(const unsigned char *seed, unsigned char *sk_out, unsigned char *f_out, unsigned char *pk_out);
void frodokexp_gen_b(const unsigned char *seed, unsigned char *sk_out, unsigned char *f_out, unsigned char *pk_out);

void frodokexp_encaps(const unsigned char *pk_a, const unsigned char *sk_b, unsigned char *key_out, unsigned long long *ct_out);

void frodokexp_decaps(const unsigned char *pk_b, const unsigned char *sk_a, unsigned char *f_a, const unsigned long long *ct_out, unsigned char *key_out);
