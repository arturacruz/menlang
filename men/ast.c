#include "ast.h"
#include <string.h>
#include <stdio.h>
#include <stdlib.h>

/* basic allocation */
AST *ast_new(NodeKind kind) {
    AST *a = calloc(1, sizeof(AST));
    a->kind = kind;
    a->stmt_count = 0;
    a->inferred = T_UNKNOWN;
    return a;
}
AST *ast_new_ident(const char *name) {
    AST *a = ast_new(N_IDENT);
    if (name) a->name = strdup(name);
    return a;
}
AST *ast_new_number(int n) {
    AST *a = ast_new(N_NUMBER);
    a->number = n;
    a->inferred = T_INT;
    return a;
}
AST *ast_new_bool(int b) {
    AST *a = ast_new(N_BOOL);
    a->bool_val = b ? 1 : 0;
    a->inferred = T_BOOL;
    return a;
}
AST *ast_new_binop(const char *op, AST *l, AST *r) {
    AST *a = ast_new(N_BINOP);
    strncpy(a->op, op, sizeof(a->op)-1);
    a->left = l;
    a->right = r;
    return a;
}
AST *ast_new_unop(const char *op, AST *child) {
    AST *a = ast_new(N_UNOP);
    strncpy(a->op, op, sizeof(a->op)-1);
    a->left = child;
    return a;
}
AST *ast_new_decl(const char *name, AST *expr) {
    AST *a = ast_new(N_DECL);
    if (name) a->name = strdup(name);
    a->left = expr;
    return a;
}
AST *ast_new_print(AST *expr) {
    AST *a = ast_new(N_PRINT);
    a->left = expr;
    return a;
}
AST *ast_new_inc(const char *name) {
    AST *a = ast_new(N_INC);
    if (name) a->name = strdup(name);
    return a;
}
AST *ast_new_dec(const char *name) {
    AST *a = ast_new(N_DEC);
    if (name) a->name = strdup(name);
    return a;
}
AST *ast_new_if(AST *cond, AST *then_block, AST *else_block) {
    AST *a = ast_new(N_IF);
    a->left = cond;
    a->right = then_block;
    a->next = else_block;
    return a;
}
AST *ast_new_while(AST *cond, AST *block) {
    AST *a = ast_new(N_WHILE);
    a->left = cond;
    a->right = block;
    return a;
}
AST *ast_new_block(AST **stmts, int count) {
    AST *a = ast_new(N_BLOCK);
    a->stmts = stmts;
    a->stmt_count = count;
    return a;
}

void ast_free(AST *a) {
    if (!a) return;
    if (a->name) free(a->name);
    if (a->left) ast_free(a->left);
    if (a->right) ast_free(a->right);
    if (a->next) ast_free(a->next);
    if (a->stmts) {
        for (int i=0;i<a->stmt_count;i++) if (a->stmts[i]) ast_free(a->stmts[i]);
        free(a->stmts);
    }
    free(a);
}

/* debug print */
static void indent_print(int indent) { for (int i=0;i<indent;i++) putchar(' '); }
void ast_dump(AST *a, int indent) {
    if (!a) { indent_print(indent); printf("(null)\n"); return; }
    indent_print(indent);
    printf("AST %p kind=%d name=%s stmt_count=%d\n", (void*)a, a->kind, a->name?a->name:"(null)", a->stmt_count);
    switch(a->kind) {
        case N_NUMBER: indent_print(indent+2); printf("NUMBER=%d\n", a->number); break;
        case N_BOOL:   indent_print(indent+2); printf("BOOL=%d\n", a->bool_val); break;
        case N_IDENT:  indent_print(indent+2); printf("IDENT=%s\n", a->name?a->name:"(null)"); break;
        case N_BINOP:
            indent_print(indent+2); printf("OP='%s'\n", a->op);
            indent_print(indent+2); printf("LEFT:\n"); ast_dump(a->left, indent+4);
            indent_print(indent+2); printf("RIGHT:\n"); ast_dump(a->right, indent+4);
            break;
        case N_UNOP:
            indent_print(indent+2); printf("UNOP='%s'\n", a->op);
            indent_print(indent+2); printf("CHILD:\n"); ast_dump(a->left, indent+4);
            break;
        case N_DECL:
            indent_print(indent+2); printf("DECL name=%s\n", a->name?a->name:"(null)");
            indent_print(indent+2); printf("INIT:\n"); ast_dump(a->left, indent+4);
            break;
        case N_PRINT:
            indent_print(indent+2); printf("PRINT expr:\n"); ast_dump(a->left, indent+4);
            break;
        case N_IF:
            indent_print(indent+2); printf("IF cond:\n"); ast_dump(a->left, indent+4);
            indent_print(indent+2); printf("THEN:\n"); ast_dump(a->right, indent+4);
            indent_print(indent+2); printf("ELSE:\n"); ast_dump(a->next, indent+4);
            break;
        case N_WHILE:
            indent_print(indent+2); printf("WHILE cond:\n"); ast_dump(a->left, indent+4);
            indent_print(indent+2); printf("BODY:\n"); ast_dump(a->right, indent+4);
            break;
        case N_BLOCK:
            for (int i=0;i<a->stmt_count;i++) {
                indent_print(indent+2); printf("stmt[%d]:\n", i);
                ast_dump(a->stmts[i], indent+4);
            }
            break;
        case N_INC: indent_print(indent+2); printf("INC %s\n", a->name?a->name:"(null)"); break;
        case N_DEC: indent_print(indent+2); printf("DEC %s\n", a->name?a->name:"(null)"); break;
        default: break;
    }
}

/* simple validation recursion */
static int validate_rec(AST *a, AST **seen, int seen_count);
static int already_seen(AST *a, AST **seen, int seen_count) {
    for (int i=0;i<seen_count;i++) if (seen[i]==a) return 1;
    return 0;
}

int ast_validate(AST *a) {
    return validate_rec(a, NULL, 0);
}

static int add_seen(AST ***pseen, int *pscount, AST *node) {
    AST **seen = *pseen;
    int count = *pscount;
    AST **n = realloc(seen, sizeof(AST*) * (count + 1));
    if (!n) return -1;
    n[count] = node;
    *pseen = n;
    *pscount = count + 1;
    return 0;
}

static int validate_rec(AST *a, AST **seen, int seen_count) {
    if (!a) return 0;
    if (already_seen(a, seen, seen_count)) {
        fprintf(stderr, "AST validation: cycle detected at %p\n", (void*)a);
        return 1;
    }
    /* build new seen list */
    AST **nseen = NULL;
    int ncount = 0;
    if (seen && seen_count>0) {
        nseen = malloc(sizeof(AST*)*seen_count);
        for (int i=0;i<seen_count;i++) nseen[i]=seen[i];
        ncount = seen_count;
    }
    add_seen(&nseen, &ncount, a);

    int errors = 0;
    switch(a->kind) {
        case N_DECL:
            if (!a->name) { fprintf(stderr,"AST validate: DECL missing name\n"); errors++; }
            if (!a->left) { fprintf(stderr,"AST validate: DECL '%s' missing init\n", a->name?a->name:"(null)"); errors++; }
            else errors += validate_rec(a->left, nseen, ncount);
            break;
        case N_PRINT:
            if (!a->left) { fprintf(stderr,"AST validate: PRINT missing expr\n"); errors++; }
            else errors += validate_rec(a->left, nseen, ncount);
            break;
        case N_IF:
            if (!a->left) { fprintf(stderr,"AST validate: IF missing cond\n"); errors++; }
            else errors += validate_rec(a->left, nseen, ncount);
            if (!a->right) { fprintf(stderr,"AST validate: IF missing then-block\n"); errors++; }
            else errors += validate_rec(a->right, nseen, ncount);
            if (a->next) errors += validate_rec(a->next, nseen, ncount);
            break;
        case N_WHILE:
            if (!a->left) { fprintf(stderr,"AST validate: WHILE missing cond\n"); errors++; }
            else errors += validate_rec(a->left, nseen, ncount);
            if (!a->right) { fprintf(stderr,"AST validate: WHILE missing body\n"); errors++; }
            else errors += validate_rec(a->right, nseen, ncount);
            break;
        case N_BINOP:
            if (!a->left || !a->right) { fprintf(stderr,"AST validate: BINOP missing child\n"); errors++; }
            else { errors += validate_rec(a->left, nseen, ncount); errors += validate_rec(a->right, nseen, ncount); }
            break;
        case N_UNOP:
            if (!a->left) { fprintf(stderr,"AST validate: UNOP missing child\n"); errors++; }
            else errors += validate_rec(a->left, nseen, ncount);
            break;
        case N_BLOCK:
            if (a->stmt_count < 0) { fprintf(stderr,"AST validate: BLOCK negative count\n"); errors++; }
            if (a->stmt_count > 0 && !a->stmts) { fprintf(stderr,"AST validate: BLOCK stmts==NULL\n"); errors++; }
            for (int i=0;i<a->stmt_count;i++) {
                if (!a->stmts[i]) { fprintf(stderr,"AST validate: BLOCK null stmt at %d\n", i); errors++; }
                else errors += validate_rec(a->stmts[i], nseen, ncount);
            }
            break;
        case N_IDENT:
        case N_NUMBER:
        case N_BOOL:
        case N_INC:
        case N_DEC:
        case N_EMPTY:
            /* ok */
            break;
        default:
            fprintf(stderr,"AST validate: unknown kind %d\n", a->kind);
            errors++;
    }

    if (nseen) free(nseen);
    return errors;
}

