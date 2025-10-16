%{
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
extern int yylex();
extern int yyparse();
extern FILE *yyin;
void yyerror(const char *s);
int yylex_destroy(void);
%}

%union {
    int number;
    int bool_val;
    char *iden;
}

/* Tokens */
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

/* Precedence and associativity */
%left OR
%left AND
%left EQUALS GREATER LESSER
%left PLUS MINUS
%left MULT DIVIDE
%right NOT LESS UNARY_MINUS

%%

program: 
    /* empty */
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
    ;

declaration:
    DECLARE IDENTIFIER ASSIGN expr ENDLINE
    {
        printf("Declared variable: %s\n", $2);
        free($2);
    }
    ;

print:
    PRINT bool_expr ENDLINE
    {
        printf("Print var\n");
    }
    ;

increment:
    IDENTIFIER INC ENDLINE
    {
        printf("Increment: %s\n", $1);
        free($1);
    }
    ;

decrement:
    IDENTIFIER DEC ENDLINE
    {
        printf("Decrement: %s\n", $1);
        free($1);
    }
    ;

conditional:
    {
        printf("if start\n");
    }
    IF bool_expr THEN noend_block elseif_conditional else_conditional ENDBLOCK
    {
        printf("if end\n");
    }
    ;

elseif_conditional:
    {
        printf("Else if start\n");
    }
    | IF ELSE bool_expr THEN noend_block
    {
        printf("Else if end\n");
    }
    ;

else_conditional:
    {
        printf("else start\n");
    }
    | ELSE THEN noend_block
    {
        printf("else end\n");
    }
    ;

conditional_loop:
    {
        printf("while start\n");
    }
    WHILE UNTIL bool_expr block
    {
        printf("while end\n");
    }
    ;

bool_expr:
    bool_term
    | bool_expr OR bool_term
    ;

bool_term:
    rel_expr
    | bool_term AND rel_expr
    ;

rel_expr:
    expr
    | expr bool_bin_op expr
    ;

bool_bin_op:
    EQUALS TO
    | GREATER THAN
    | LESSER THAN
    ;

expr:
    term
    | term PLUS term
    | term MINUS term
    ;

term: 
    factor
    | factor MULT factor
    | factor DIVIDE factor
    ;

factor:
    NUMBER
    | BOOLEAN
    | IDENTIFIER
    | MINUS factor
    | PLUS factor
    | NOT factor
    | LPAREN bool_expr RPAREN
    ;

block:
    noend_block ENDBLOCK
    ;

noend_block:
    statement
    | noend_block statement
    ;

%%

void yyerror(const char *s) {
    extern char *yytext;
    fprintf(stderr, "Erro sintático: %s no token '%s'\n", s, yytext);
}


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
    
    int result = yyparse();
    
    if (yyin != stdin) {
        fclose(yyin);
    }
    
    yylex_destroy();
    
    return result;
}
