#ifndef AST_H
#define AST_H

#include <stdlib.h>

typedef enum {
    N_PROGRAM,
    N_DECL,
    N_PRINT,
    N_INC,
    N_DEC,
    N_IF,
    N_WHILE,
    N_BLOCK,
    N_BINOP,
    N_UNOP,
    N_IDENT,
    N_NUMBER,
    N_BOOL,
    N_EMPTY
} NodeKind;

typedef enum {
    T_INT,
    T_BOOL,
    T_UNKNOWN
} TypeTag;

typedef struct AST {
    NodeKind kind;
    char *name;           /* identifier name, or NULL */
    int number;           /* for N_NUMBER */
    int bool_val;         /* for N_BOOL */
    char op[16];          /* operator for BINOP/UNOP */
    struct AST *left;     /* left child (or expr in many nodes) */
    struct AST *right;    /* right child or then-block */
    struct AST *next;     /* extra (else block) */
    struct AST **stmts;   /* for block */
    int stmt_count;
    TypeTag inferred;
} AST;

/* constructors / destructors */
AST *ast_new(NodeKind kind);
AST *ast_new_ident(const char *name);
AST *ast_new_number(int n);
AST *ast_new_bool(int b);
AST *ast_new_binop(const char *op, AST *l, AST *r);
AST *ast_new_unop(const char *op, AST *child);
AST *ast_new_decl(const char *name, AST *expr);
AST *ast_new_print(AST *expr);
AST *ast_new_inc(const char *name);
AST *ast_new_dec(const char *name);
AST *ast_new_if(AST *cond, AST *then_block, AST *else_block);
AST *ast_new_while(AST *cond, AST *block);
AST *ast_new_block(AST **stmts, int count);
void ast_free(AST *a);

/* debug + validation */
void ast_dump(AST *a, int indent);
int ast_validate(AST *a); /* 0 ok, >0 errors */

#endif

