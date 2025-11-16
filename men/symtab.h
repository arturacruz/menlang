#ifndef SYMTAB_H
#define SYMTAB_H

#include "ast.h"

typedef struct Sym {
    char *name;
    int address; /* slot: 0,1,2,... */
    TypeTag type;
} Sym;

void symtab_init(void);
int symtab_insert(const char *name, TypeTag t); /* returns address or -1 on dup */
Sym *symtab_lookup(const char *name);

#endif
 
