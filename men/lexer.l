%{
#include "parser.tab.h"
#include <stdlib.h>
#include <string.h>
#include <time.h>
void yyerror(const char *s);

%}

%%
"se"            { return IF; }
"ent"           { return THEN; }
"ne"            { return ENDBLOCK; }
"grind"         { return WHILE; }
"enquanto"      { return UNTIL; }
"pah"           { return ELSE; }
"literal"       { return DECLARE; }
"literalmente"  { return DECLARE; }
"eh"            { return ASSIGN; }
"ou"            { return OR; }
"e"             { return AND; }
"identico"      { return EQUALS; }
"a"             { return TO; }
"maior"         { return GREATER; }
"menor"         { return LESSER; }
"que"           { return THAN; }
"mais"          { return PLUS; }
"menos"         { return MINUS; }
"nao"           { return NOT; }
"win"           { return INC; }
"loss"          { return DEC; }
"divide"        { return DIVIDE; }
"vezes"         { return MULT; }

"po"            { return PRINT; }
"puts"          { return PRINT; }
"puta"          { return PRINT; }

"bizarro"       { yylval.bool_val = 0; return BOOLEAN; }
"certeza"       { yylval.bool_val = 1; return BOOLEAN; }
"depende"       { srand(time(NULL));
    yylval.bool_val = rand() % 2; return BOOLEAN; }

"men"           { return ENDLINE; }
"mano"          { return ENDLINE; }
"meo"           { return ENDLINE; }
"menzinho"      { return ENDLINE; }
"meu"           { return ENDLINE; }
"bro"           { return ENDLINE; }
"vei"           { return ENDLINE; }

[ \t\r\n]+      ;

"#".*           ;

[a-zA-Z_][a-zA-Z0-9_]* {
    yylval.iden = strdup(yytext);
    return IDENTIFIER;
}

[0-9]+          {
    yylval.number = atoi(yytext);
    return NUMBER;
}

"("             { return LPAREN; }
")"             { return RPAREN; }

.               {
    printf("Caractere desconhecido: %c\n", *yytext);
    return ERROR;
}

%%

int yywrap() { return 1; }

