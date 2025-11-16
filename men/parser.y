/* parser.y (corrigido para incluir ast.h no header gerado) */
%{
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* mantemos includes que vão para parser.tab.c */
#include "codegen.h"
#include "symtab.h"

/* Externs do lexer */
extern int yylex();
extern FILE *yyin;
void yyerror(const char *s);

/* top-level statements array */
AST **top_stmts = NULL;
int top_count = 0;

/* helper to push top-level statement */
static void push_top(AST *s) {
    AST **n = realloc(top_stmts, sizeof(AST*) * (top_count + 1));
    if (!n) { perror("realloc"); exit(1); }
    top_stmts = n;
    top_stmts[top_count++] = s;
}
%}

/* garantimos que o header gerado conheça AST */
%code requires {
  #include "ast.h"
}

%union {
    int number;
    int bool_val;
    char *iden;
    AST *ast;
    AST **astlist;
}

/* token declarations */
%token <iden> IDENTIFIER
%token <number> NUMBER
%token <bool_val> BOOLEAN
%token IF THEN ENDBLOCK WHILE UNTIL ELSE
%token THAN TO
%token DECLARE ASSIGN OR AND
%token EQUALS GREATER LESSER
%token PLUS MINUS NOT INC DEC DIVIDE MULT
%token PRINT
%token ENDLINE
%token LPAREN RPAREN ERROR

/* nonterminals types */
%type <ast> statement declaration print increment decrement conditional conditional_loop bool_expr bool_term rel_expr expr term factor block noend_block elseif_conditional else_conditional

/* Precedence */
%left OR
%left AND
%left EQUALS GREATER LESSER
%left PLUS MINUS
%left MULT DIVIDE
%right NOT UNARY_MINUS

%%

program:
    /* empty */ { /* explicit: nothing to push here */ }
    | program statement
    ;

statement:
    declaration
    | print
    | increment
    | decrement
    | conditional
    | conditional_loop
    | ENDLINE
    {
        /* produce an empty AST node for blank line and push it */
        $$ = ast_new(N_EMPTY);
        push_top($$);
    }
    ;

declaration:
    DECLARE IDENTIFIER ASSIGN expr ENDLINE
    {
        AST *d = ast_new_decl($2, $4);
        push_top(d);
        free($2);
        $$ = d;
    }
    ;

print:
    PRINT expr ENDLINE
    {
        AST *p = ast_new_print($2);
        push_top(p);
        $$ = p;
    }
    ;

increment:
    IDENTIFIER INC ENDLINE
    {
        AST *n = ast_new_inc($1);
        push_top(n);
        free($1);
        $$ = n;
    }
    ;

decrement:
    IDENTIFIER DEC ENDLINE
    {
        AST *n = ast_new_dec($1);
        push_top(n);
        free($1);
        $$ = n;
    }
    ;

/* conditional supports multiple else-if and optional else */
conditional:
    IF bool_expr THEN noend_block elseif_conditional else_conditional ENDBLOCK
    {
        AST *elseblock = $6;
        /* elseif_conditional already converted to a block if present */
        AST *ifnode = ast_new_if($2, $4, elseblock);
        push_top(ifnode);
        $$ = ifnode;
    }
    ;

elseif_conditional:
    /* empty */
    { $$ = NULL; }
    | IF ELSE bool_expr THEN noend_block
    {
        /* transform this elseif into an else that contains an if node */
        AST *inner_if = ast_new_if($3, $5, NULL);
        AST **arr = malloc(sizeof(AST*));
        arr[0] = inner_if;
        $$ = ast_new_block(arr, 1);
    }
    ;

else_conditional:
    /* empty */
    { $$ = NULL; }
    | ELSE THEN noend_block
    {
        $$ = $3;
    }
    ;

conditional_loop:
    WHILE UNTIL bool_expr block
    {
        AST *w = ast_new_while($3, $4);
        push_top(w);
        $$ = w;
    }
    ;

bool_expr:
    bool_term
    | bool_expr OR bool_term
    {
        $$ = ast_new_binop("OR", $1, $3);
    }
    ;

bool_term:
    rel_expr
    | bool_term AND rel_expr
    {
        $$ = ast_new_binop("AND", $1, $3);
    }
    ;

rel_expr:
    expr
    | expr EQUALS TO expr
    {
        $$ = ast_new_binop("==", $1, $4);
    }
    | expr GREATER THAN expr
    {
        $$ = ast_new_binop(">", $1, $4);
    }
    | expr LESSER THAN expr
    {
        $$ = ast_new_binop("<", $1, $4);
    }
    ;

expr:
    term
    | expr PLUS term
    {
        $$ = ast_new_binop("+", $1, $3);
    }
    | expr MINUS term
    {
        $$ = ast_new_binop("-", $1, $3);
    }
    | PLUS factor %prec UNARY_MINUS
    {
        $$ = $2; /* unary plus -> no-op */
    }
    | MINUS factor %prec UNARY_MINUS
    {
        $$ = ast_new_unop("u-", $2);
    }
    | NOT factor
    {
        $$ = ast_new_unop("nao", $2);
    }
    ;

term:
    factor
    | term MULT factor
    {
        $$ = ast_new_binop("*", $1, $3);
    }
    | term DIVIDE factor
    {
        $$ = ast_new_binop("/", $1, $3);
    }
    ;

factor:
    NUMBER
    {
        $$ = ast_new_number($1);
    }
    | BOOLEAN
    {
        $$ = ast_new_bool($1);
    }
    | IDENTIFIER
    {
        $$ = ast_new_ident($1);
        free($1);
    }
    | LPAREN bool_expr RPAREN
    {
        $$ = $2;
    }
    ;

block:
    noend_block ENDBLOCK
    {
        $$ = $1;
    }
    ;

noend_block:
    statement
    {
        AST **arr = malloc(sizeof(AST*));
        arr[0] = $1;
        $$ = ast_new_block(arr, 1);
    }
    | noend_block statement
    {
        AST *prev = $1;
        int n = prev->stmt_count;
        prev->stmts = realloc(prev->stmts, sizeof(AST*) * (n + 1));
        prev->stmts[n] = $2;
        prev->stmt_count = n + 1;
        $$ = prev;
    }
    ;

%%

void yyerror(const char *s) {
    extern char *yytext;
    fprintf(stderr, "Erro sintático: %s no token '%s'\n", s, yytext);
}

/* main unchanged... (same as previous full file) */
int main(int argc, char *argv[]) {
    if (argc > 1) {
        yyin = fopen(argv[1], "r");
        if (!yyin) {
            fprintf(stderr, "Não foi possível abrir o arquivo: %s\n", argv[1]);
            return 1;
        }
    } else {
        printf("Digite seu código (Ctrl+D para finalizar):\n");
    }

    fprintf(stderr, "DEBUG: starting parse\n");
    int result = yyparse();
    fprintf(stderr, "DEBUG: parsed, result=%d\n", result);

    if (yyin != stdin) fclose(yyin);

    /* Build root block from top_stmts */
    AST *root = NULL;
    if (top_count > 0) {
        AST **arr = calloc(top_count, sizeof(AST*));
        for (int i=0;i<top_count;i++) arr[i] = top_stmts[i];
        root = ast_new_block(arr, top_count);
    } else {
        AST **arr = calloc(1, sizeof(AST*));
        root = ast_new_block(arr, 0);
    }

    fprintf(stderr, "DEBUG: built AST, top_count=%d\n", top_count);

    fprintf(stderr, "DEBUG: AST dump:\n");
    ast_dump(root, 0);
    fflush(stderr);

    int v = ast_validate(root);
    if (v != 0) {
        fprintf(stderr, "AST validation failed (%d errors). Aborting compilation.\n", v);
        ast_free(root);
        return 1;
    } else {
        fprintf(stderr, "AST validation: OK\n");
    }

    if (semantic_check(root) != 0) {
        fprintf(stderr, "Erros semânticos detectados. Abortando geração de código.\n");
        ast_free(root);
        return 1;
    }

    fprintf(stderr, "DEBUG: semantic ok\n");

    if (codegen_emit(root, "out.invm") != 0) {
        fprintf(stderr, "Erro durante codegen\n");
        ast_free(root);
        return 1;
    }

    fprintf(stderr, "Código gerado em out.invm\n");
    ast_free(root);

    return result;
}

