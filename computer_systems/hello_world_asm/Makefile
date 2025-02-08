ASM = nasm
LD = ld
ASM_FLAGS = -f elf64
LD_FLAGS = -o

TARGET = hello_linux
SRC = hello_linux.asm
OBJ = hello_linux.o

all: $(TARGET)

$(TARGET): $(OBJ)
	$(LD) $(LD_FLAGS) $@ $^

$(OBJ): $(SRC)
	$(ASM) $(ASM_FLAGS) -o $@ $<

run: $(TARGET)
	./$(TARGET)

clean:
	rm -f $(OBJ) $(TARGET)
