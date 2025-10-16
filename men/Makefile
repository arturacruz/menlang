CC = gcc
CFLAGS = -g

all: menc

parser.tab.c parser.tab.h: parser.y
	bison -d parser.y

lex.yy.c: lexer.l parser.tab.h
	flex lexer.l

menc: parser.tab.c lex.yy.c
	$(CC) $(CFLAGS) -o $@ parser.tab.c lex.yy.c -lfl

clean:
	rm -f menc lex.yy.c parser.tab.c parser.tab.h parser.output

.PHONY: all clean
