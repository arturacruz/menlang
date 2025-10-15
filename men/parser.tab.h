/* A Bison parser, made by GNU Bison 3.8.2.  */

/* Bison interface for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015, 2018-2021 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

/* DO NOT RELY ON FEATURES THAT ARE NOT DOCUMENTED in the manual,
   especially those whose name start with YY_ or yy_.  They are
   private implementation details that can be changed or removed.  */

#ifndef YY_YY_PARSER_TAB_H_INCLUDED
# define YY_YY_PARSER_TAB_H_INCLUDED
/* Debug traces.  */
#ifndef YYDEBUG
# define YYDEBUG 0
#endif
#if YYDEBUG
extern int yydebug;
#endif

/* Token kinds.  */
#ifndef YYTOKENTYPE
# define YYTOKENTYPE
  enum yytokentype
  {
    YYEMPTY = -2,
    YYEOF = 0,                     /* "end of file"  */
    YYerror = 256,                 /* error  */
    YYUNDEF = 257,                 /* "invalid token"  */
    IDENTIFIER = 258,              /* IDENTIFIER  */
    NUMBER = 259,                  /* NUMBER  */
    BOOLEAN = 260,                 /* BOOLEAN  */
    IF = 261,                      /* IF  */
    THEN = 262,                    /* THEN  */
    ENDBLOCK = 263,                /* ENDBLOCK  */
    WHILE = 264,                   /* WHILE  */
    UNTIL = 265,                   /* UNTIL  */
    ELSE = 266,                    /* ELSE  */
    THAN = 267,                    /* THAN  */
    TO = 268,                      /* TO  */
    DECLARE = 269,                 /* DECLARE  */
    ASSIGN = 270,                  /* ASSIGN  */
    OR = 271,                      /* OR  */
    AND = 272,                     /* AND  */
    EQUALS = 273,                  /* EQUALS  */
    GREATER = 274,                 /* GREATER  */
    LESSER = 275,                  /* LESSER  */
    PLUS = 276,                    /* PLUS  */
    MINUS = 277,                   /* MINUS  */
    NOT = 278,                     /* NOT  */
    INC = 279,                     /* INC  */
    DEC = 280,                     /* DEC  */
    DIVIDE = 281,                  /* DIVIDE  */
    MULT = 282,                    /* MULT  */
    PRINT = 283,                   /* PRINT  */
    ENDLINE = 284,                 /* ENDLINE  */
    LPAREN = 285,                  /* LPAREN  */
    RPAREN = 286,                  /* RPAREN  */
    ERROR = 287,                   /* ERROR  */
    LESS = 288,                    /* LESS  */
    UNARY_MINUS = 289              /* UNARY_MINUS  */
  };
  typedef enum yytokentype yytoken_kind_t;
#endif

/* Value type.  */
#if ! defined YYSTYPE && ! defined YYSTYPE_IS_DECLARED
union YYSTYPE
{
#line 12 "parser.y"

    int number;
    int bool_val;
    char *iden;

#line 104 "parser.tab.h"

};
typedef union YYSTYPE YYSTYPE;
# define YYSTYPE_IS_TRIVIAL 1
# define YYSTYPE_IS_DECLARED 1
#endif


extern YYSTYPE yylval;


int yyparse (void);


#endif /* !YY_YY_PARSER_TAB_H_INCLUDED  */
