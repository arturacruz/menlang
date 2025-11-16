#ifndef CODEGEN_H
#define CODEGEN_H

#include "ast.h"

int semantic_check(AST *root); /* 0 ok, non-zero errors */
int codegen_emit(AST *root, const char *outpath);

#endif

