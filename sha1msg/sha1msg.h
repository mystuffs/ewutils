#ifndef SHA1SUM_H
#define SHA1SUM_H
#endif

typedef struct {
    unsigned int state[5];
    unsigned int count[2];
    unsigned char buffer[64];
} SHA1_SUM_CTX;

typedef union {
    unsigned char c[64];
    unsigned int l[16];
} CHAR64LONG16;
