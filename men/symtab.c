#include "symtab.h"
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

static Sym **table = NULL;
static int table_count = 0;
static int next_slot = 0;

void symtab_init(void) {
    for (int i=0;i<table_count;i++) {
        if (table[i]) {
            free(table[i]->name);
            free(table[i]);
        }
    }
    free(table);
    table = NULL;
    table_count = 0;
    next_slot = 0;
}

int symtab_insert(const char *name, TypeTag t) {
    if (!name) return -1;
    for (int i=0;i<table_count;i++) {
        if (strcmp(table[i]->name, name)==0) return -1; /* duplicate */
    }
    Sym *s = malloc(sizeof(Sym));
    s->name = strdup(name);
    s->address = next_slot++;
    s->type = t;
    Sym **n = realloc(table, sizeof(Sym*) * (table_count + 1));
    table = n;
    table[table_count++] = s;
    return s->address;
}

Sym *symtab_lookup(const char *name) {
    if (!name) return NULL;
    for (int i=0;i<table_count;i++) if (strcmp(table[i]->name, name)==0) return table[i];
    return NULL;
}

