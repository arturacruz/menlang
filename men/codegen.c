#define _GNU_SOURCE
#include "codegen.h"
#include "symtab.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdarg.h>

static FILE *outf = NULL;
static int temp_slot = 0;
static int label_counter = 0;

static char *new_label(const char *pref) {
    char *s;
    if (asprintf(&s, "%s%d", pref, ++label_counter) < 0) exit(1);
    return s;
}

static int new_slot(void) {
    return temp_slot++;
}

static void emitf(const char *fmt, ...) {
    va_list ap;
    va_start(ap, fmt);
    vfprintf(outf, fmt, ap);
    va_end(ap);
}

/* type inference and sem check */
static TypeTag infer_type(AST *e);

/* generate expression to a slot index (literal slot or symbol slot or temporary) */
static int gen_expr_to_slot(AST *e);

/* top-level functions */
int semantic_check(AST *root) {
    if (!root) return 0;
    symtab_init();
    /* expect root to be N_BLOCK with stmts */
    if (root->kind != N_BLOCK) {
        fprintf(stderr, "semantic_check: program root must be block\n");
        return 1;
    }
    int errs = 0;
    for (int i=0;i<root->stmt_count;i++) {
        AST *s = root->stmts[i];
        if (!s) continue;
        if (s->kind == N_DECL) {
            TypeTag t = infer_type(s->left);
            if (t == T_UNKNOWN) { fprintf(stderr, "semantic: cannot infer initializer for %s\n", s->name?s->name:"(null)"); errs++; }
            else {
                if (symtab_insert(s->name, t) == -1) { fprintf(stderr, "semantic: duplicate declaration %s\n", s->name); errs++; }
            }
        } else if (s->kind == N_PRINT) {
            TypeTag t = infer_type(s->left);
            if (t == T_UNKNOWN) { fprintf(stderr, "semantic: print of unknown type\n"); errs++; }
        } else if (s->kind == N_INC || s->kind == N_DEC) {
            Sym *sym = symtab_lookup(s->name);
            if (!sym) { fprintf(stderr, "semantic: use of undeclared %s\n", s->name); errs++; }
            else if (sym->type != T_INT) { fprintf(stderr, "semantic: inc/dec applied to non-int %s\n", s->name); errs++; }
        } else if (s->kind == N_IF) {
            TypeTag ct = infer_type(s->left);
            if (ct != T_BOOL) { fprintf(stderr, "semantic: if condition not boolean\n"); errs++; }
        } else if (s->kind == N_WHILE) {
            TypeTag ct = infer_type(s->left);
            if (ct != T_BOOL) { fprintf(stderr, "semantic: while condition not boolean\n"); errs++; }
        }
    }
    return errs;
}

/* type inference (simple) */
static TypeTag infer_type(AST *e) {
    if (!e) return T_UNKNOWN;
    switch(e->kind) {
        case N_NUMBER: return T_INT;
        case N_BOOL: return T_BOOL;
        case N_IDENT: {
            Sym *s = symtab_lookup(e->name);
            return s ? s->type : T_UNKNOWN;
        }
        case N_UNOP: {
            if (strcmp(e->op,"nao")==0) return T_BOOL;
            if (strcmp(e->op,"u-")==0) return T_INT;
            return T_UNKNOWN;
        }
        case N_BINOP: {
            if (strcmp(e->op,"+")==0 || strcmp(e->op,"-")==0 || strcmp(e->op,"*")==0 || strcmp(e->op,"/")==0)
                return T_INT;
            if (strcmp(e->op,"<")==0 || strcmp(e->op,">")==0 || strcmp(e->op,"==")==0 || strcmp(e->op,"AND")==0 || strcmp(e->op,"OR")==0)
                return T_BOOL;
            return T_UNKNOWN;
        }
        default: return T_UNKNOWN;
    }
}

/* gen_expr_to_slot:
   returns a slot index (0..N) where the value is stored after generation.
   The convention: symtab addresses are used as slots for variables.
   Temporary slots are generated via new_slot() and addressed as *<slot>.
*/
static int gen_expr_to_slot(AST *e) {
    if (!e) return -1;
    if (e->kind == N_NUMBER) {
        int s = new_slot();
        emitf("SET *%d %d\n", s, e->number);
        return s;
    }
    if (e->kind == N_BOOL) {
        int s = new_slot();
        emitf("SET *%d %d\n", s, e->bool_val ? 1 : 0);
        return s;
    }
    if (e->kind == N_IDENT) {
        Sym *sym = symtab_lookup(e->name);
        if (!sym) { fprintf(stderr, "codegen: unknown ident %s\n", e->name); return -1; }
        return sym->address;
    }
    if (e->kind == N_UNOP) {
        if (strcmp(e->op,"nao")==0) {
            int s = gen_expr_to_slot(e->left);
            if (s < 0) return -1;
            int out = new_slot();
            char *Ltrue = new_label("not_true_");
            char *Lend = new_label("not_end_");
            emitf("SET FUND1 *%d\n", s);
            emitf("GOIF == FUND1 %s\n", Ltrue);
            emitf("SET *%d 0\n", out);
            emitf("GOTO %s\n", Lend);
            emitf("%s:\n", Ltrue);
            emitf("SET *%d 1\n", out);
            emitf("%s:\n", Lend);
            free(Ltrue); free(Lend);
            return out;
        } else if (strcmp(e->op,"u-")==0) {
            int s = gen_expr_to_slot(e->left);
            if (s < 0) return -1;
            int out = new_slot();
            emitf("SET FUND1 *%d\n", s);
            emitf("SET *%d 0\n", out);
            emitf("SUB FUND1 *%d\n", out); /* FUND1 = 0 - val; then store? we'll store FUND1 into out */
            emitf("SET *%d FUND1\n", out);
            return out;
        }
    }
    if (e->kind == N_BINOP) {
        /* arithmetic */
        if (strcmp(e->op,"+")==0 || strcmp(e->op,"-")==0 || strcmp(e->op,"*")==0 || strcmp(e->op,"/")==0) {
            int L = gen_expr_to_slot(e->left);
            int R = gen_expr_to_slot(e->right);
            if (L < 0 || R < 0) return -1;
            int out = new_slot();
            emitf("SET FUND1 *%d\n", L);
            emitf("SET FUND2 *%d\n", R);
            if (strcmp(e->op,"+")==0) emitf("ADD FUND1 FUND2\n");        /* FUND2 = R + L */
            else if (strcmp(e->op,"-")==0) { emitf("SUB FUND2 FUND1\n"); } /* FUND2 = R - L ??? -> we want L - R; to avoid confusion do different */
            else if (strcmp(e->op,"*")==0) emitf("MULT FUND1 FUND2\n");
            else if (strcmp(e->op,"/")==0) emitf("DIV FUND2 FUND1\n");
            /* normalize: after operation, we place result in *out. To keep consistent, set *out FUND2 or FUND1 depending */
            /* For simplicity: after ADD/MULT we assume result in FUND2; after SUB/DIV we'll compute properly */
            if (strcmp(e->op,"+")==0 || strcmp(e->op,"*")==0) {
                emitf("SET *%d FUND2\n", out);
            } else if (strcmp(e->op,"-")==0) {
                /* we want left - right: set FUND1 left, FUND2 right, SUB FUND1 FUND2 gives FUND2 = FUND2 - FUND1 (bad).
                   So do: SET FUND1 *L ; SET *out FUND1 ; SET FUND2 *R ; SUB *out FUND2 -> out = left - right */
                emitf("SET FUND1 *%d\n", L);
                emitf("SET *%d FUND1\n", out);
                emitf("SET FUND2 *%d\n", R);
                emitf("SUB *%d FUND2\n", out); /* *out = *out - FUND2 => left - right */
                /* Now *out has left-right */
            } else if (strcmp(e->op,"/")==0) {
                emitf("SET FUND1 *%d\n", L);
                emitf("SET *%d FUND1\n", out);
                emitf("SET FUND2 *%d\n", R);
                emitf("DIV *%d FUND2\n", out); /* *out = *out / FUND2 -> left / right */
            }
            return out;
        }

        /* relational: produce bool in slot */
        if (strcmp(e->op,"<")==0 || strcmp(e->op,">")==0 || strcmp(e->op,"==")==0) {
            int L = gen_expr_to_slot(e->left);
            int R = gen_expr_to_slot(e->right);
            if (L < 0 || R < 0) return -1;
            int out = new_slot();
            emitf("SET FUND1 *%d\n", L);
            emitf("SET *%d FUND1\n", out);
            emitf("SET FUND2 *%d\n", R);
            emitf("SUB FUND2 *%d\n", out); /* FUND2 = right - left? we use GOIF comparing FUND1 to 0 semantics; keep consistent */
            emitf("SET FUND1 *%d\n", out);
            char *Ltrue = new_label("rel_true_");
            char *Lend = new_label("rel_end_");
            if (strcmp(e->op,"<")==0) emitf("GOIF < FUND1 %s\n", Ltrue);
            else if (strcmp(e->op,">")==0) emitf("GOIF > FUND1 %s\n", Ltrue);
            else emitf("GOIF == FUND1 %s\n", Ltrue);
            emitf("SET *%d 0\n", out);
            emitf("GOTO %s\n", Lend);
            emitf("%s:\n", Ltrue);
            emitf("SET *%d 1\n", out);
            emitf("%s:\n", Lend);
            free(Ltrue); free(Lend);
            return out;
        }

        /* logical AND/OR */
        if (strcmp(e->op,"AND")==0 || strcmp(e->op,"OR")==0) {
            int L = gen_expr_to_slot(e->left);
            if (L < 0) return -1;
            int out = new_slot();
            char *Lfalse = new_label("logic_false_");
            char *Lafter = new_label("logic_after_");
            emitf("SET FUND1 *%d\n", L);
            if (strcmp(e->op,"AND")==0) {
                emitf("GOIF == FUND1 %s\n", Lfalse);
                int R = gen_expr_to_slot(e->right);
                if (R < 0) { free(Lfalse); free(Lafter); return -1; }
                emitf("SET FUND2 *%d\n", R);
                emitf("SET *%d FUND2\n", out);
                emitf("GOTO %s\n", Lafter);
                emitf("%s:\n", Lfalse);
                emitf("SET *%d 0\n", out);
                emitf("%s:\n", Lafter);
            } else {
                char *Ltrue = new_label("logic_true_");
                emitf("GOIF == FUND1 %s\n", Ltrue);
                emitf("SET *%d 1\n", out);
                emitf("GOTO %s\n", Lafter);
                emitf("%s:\n", Ltrue);
                int R = gen_expr_to_slot(e->right);
                if (R < 0) { free(Ltrue); free(Lfalse); free(Lafter); return -1; }
                emitf("SET FUND2 *%d\n", R);
                emitf("SET *%d FUND2\n", out);
                emitf("%s:\n", Lafter);
                free(Ltrue);
            }
            free(Lfalse); free(Lafter);
            return out;
        }
    }

    fprintf(stderr, "codegen: unhandled expr kind=%d\n", e->kind);
    return -1;
}

/* helpers to generate statements and blocks */
static void gen_stmt(AST *s);
static void gen_block(AST *b) {
    if (!b) return;
    if (b->kind != N_BLOCK) { gen_stmt(b); return; }
    for (int i=0;i<b->stmt_count;i++) {
        if (!b->stmts[i]) continue;
        gen_stmt(b->stmts[i]);
    }
}

static void gen_stmt(AST *s) {
    if (!s) return;
    if (s->kind == N_DECL) {
        int slot = gen_expr_to_slot(s->left);
        Sym *sym = symtab_lookup(s->name);
        if (!sym) { fprintf(stderr,"codegen: missing sym %s\n", s->name); return; }
        emitf("SET *%d *%d\n", sym->address, slot);
        return;
    }
    if (s->kind == N_PRINT) {
        int slot = gen_expr_to_slot(s->left);
        TypeTag t = infer_type(s->left);
        emitf("SET FUND1 *%d\n", slot);
        if (t == T_INT) emitf("PRINT FUND1 int\n");
        else if (t == T_BOOL) emitf("PRINT FUND1 bool\n");
        else emitf("PRINT FUND1 int\n");
        return;
    }
    if (s->kind == N_INC || s->kind == N_DEC) {
        Sym *sym = symtab_lookup(s->name);
        if (!sym) { fprintf(stderr,"codegen: inc/dec unknown %s\n", s->name); return; }
        emitf("SET FUND1 *%d\n", sym->address);
        if (s->kind == N_INC) emitf("ADD 1 FUND1\n");
        else emitf("SUB 1 FUND1\n");
        emitf("SET *%d FUND1\n", sym->address);
        return;
    }
    if (s->kind == N_IF) {
        int cond = gen_expr_to_slot(s->left);
        char *Lthen = new_label("then_");
        char *Lend = new_label("ifend_");
        emitf("SET FUND1 *%d\n", cond);
        emitf("GOIF == FUND1 %s\n", Lend);
        emitf("%s:\n", Lthen);
        gen_block(s->right);
        emitf("GOTO %s\n", Lend);
        emitf("%s:\n", Lend);
        if (s->next) gen_block(s->next);
        free(Lthen); free(Lend);
        return;
    }
    if (s->kind == N_WHILE) {
        char *Lstart = new_label("while_start_");
        char *Lend = new_label("while_end_");
        emitf("%s:\n", Lstart);
        int cond = gen_expr_to_slot(s->left);
        emitf("SET FUND1 *%d\n", cond);
        emitf("GOIF == FUND1 %s\n", Lend);
        gen_block(s->right);
        emitf("GOTO %s\n", Lstart);
        emitf("%s:\n", Lend);
        free(Lstart); free(Lend);
        return;
    }
    if (s->kind == N_BLOCK) { gen_block(s); return; }
}

/* entry */
int codegen_emit(AST *root, const char *outpath) {
    if (!root) return 1;
    outf = fopen(outpath, "w");
    if (!outf) { perror("fopen"); return 1; }
    temp_slot = 0; label_counter = 0;
    emitf("# generated InVM code\n");
    gen_block(root);
    fclose(outf);
    return 0;
}

