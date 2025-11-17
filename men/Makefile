CC = gcc
CFLAGS = -g -Wall -Wextra
LDFLAGS = -lfl

OBJS = parser.tab.o lex.yy.o ast.o symtab.o codegen.o

all: comp

parser.tab.c parser.tab.h: parser.y
	bison -d -o parser.tab.c parser.y

lex.yy.c: lexer.l parser.tab.h
	flex -o lex.yy.c lexer.l

ast.o: ast.c ast.h
	$(CC) $(CFLAGS) -c ast.c

symtab.o: symtab.c symtab.h ast.h
	$(CC) $(CFLAGS) -c symtab.c

codegen.o: codegen.c codegen.h ast.h symtab.h
	$(CC) $(CFLAGS) -c codegen.c

parser.tab.o: parser.tab.c ast.h codegen.h
	$(CC) $(CFLAGS) -c parser.tab.c

lex.yy.o: lex.yy.c
	$(CC) $(CFLAGS) -c lex.yy.c

comp: parser.tab.o lex.yy.o ast.o symtab.o codegen.o
	$(CC) -o comp parser.tab.o lex.yy.o ast.o symtab.o codegen.o $(LDFLAGS)

clean:
	rm -f comp parser.tab.c parser.tab.h lex.yy.c *.o out.vm

