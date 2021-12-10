CC		:= clang
CFLAGS	:= -Wall -std=c11 -pedantic -O2

SRCS	:= $(wildcard *.c)
OBJS	:= $(SRCS:.c=.o)
TBJS	:= $(SRCS:.c=.to)

BIN		:= prog
TEST	:= prog.test

all: $(BIN) $(TEST)

$(BIN): $(OBJS)
	$(CC) $(CFLAGS) -o $@ $^

$(TEST): $(TBJS)
	$(CC) $(CFLAGS) -o $@ $^

%.o: %.c
	$(CC) -c $(CFLAGS) -o $@ $<

%.to: %.c
	$(CC) -c -DTEST $(CFLAGS) -o $@ $<
