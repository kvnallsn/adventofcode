CC		:= clang
CFLAGS	:= -Wall -std=c11 -pedantic -O2

SRCS	:= $(wildcard *.c)
OBJS	:= $(SRCS:.c=.o)

BIN		:= prog

$(BIN): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^

%.o: %.c
	$(CC) -c $(CFLAGS) -o $@ $<
