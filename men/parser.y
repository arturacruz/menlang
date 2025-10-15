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
    PRINT expr ENDLINE
    // {
    //     printf("Print: %d\n", $2);
    //     free($2);
    // }
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
    IF bool_expr THEN noend_block elseif_conditional else_conditional ENDBLOCK
    ;

elseif_conditional:
    /* empty */
    | IF ELSE bool_expr THEN noend_block
    ;

else_conditional:
    /* empty */
    | ELSE THEN noend_block
    ;

conditional_loop:
    WHILE UNTIL bool_expr block
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
    factor
    | expr bin_op factor
    | PLUS factor %prec UNARY_MINUS
    | MINUS factor %prec UNARY_MINUS
    | NOT factor
    ;

bin_op:
    PLUS | MINUS | DIVIDE | MULT
    ;

factor:
    NUMBER
    | BOOLEAN
    | IDENTIFIER
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
    fprintf(stderr, "Erro: %s\n", s);
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
