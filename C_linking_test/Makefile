CC = gcc

OUT = test
SRC = test.c

INC = -I../src
LDFLAGS = -L. -l:./libfrodokexp.so

all: $(OUT)

$(OUT): $(SRC)
	cp ../target/release/libfrodokexp.so ./
	$(CC) -o $@ $(SRC) $(INC) $(LDFLAGS)

clean:
	rm $(OUT) libfrodokexp.so
