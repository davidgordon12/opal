#include "vm/repl.h"
#include "vm/vm.h"
#include "vm/values.h"

static void run(string path);
static mut_string read_file(string path);

int main(int argc, string argv[]) {
    init_vm();

    if(argc == 1) {
        repl();
    } else if(argc == 2) {
        run(argv[1]);
    } else {
        fprintf(stderr, "Usage: opal [path]\n");
        exit(64);
    }

    free_vm();
    return 0;
}

static void run(string path) {
    mut_string source = read_file(path);
    result result = interpret((string)source);
    free(source);

    if(result == COMPILER_ERROR)
        exit(65);
    if(result == RUNTIME_ERROR)
        exit(70);
}

static mut_string read_file(string path) {
    FILE* file = fopen(path, "rb");

    if(file == NULL) {
        fprintf(stderr, "Could not open file \"%s\".\n", path);
        exit(74);
    }

    fseek(file, 0L, SEEK_END);
    size_t file_size = ftell(file);
    rewind(file);

    mut_string buffer = (mut_string)malloc(file_size + 1);

    if(buffer == NULL) {
        fprintf(stderr, "Not enough memory to read \"%s\"\n", path);
        exit(74);
    }

    size_t bytes_read = fread(buffer, sizeof(char), file_size, file);

    if(bytes_read < file_size) {
        fprintf(stderr, "Could not read file \"%s\"\n", path);
    }

    buffer[bytes_read] = '\0';

    fclose(file);

    return buffer;
}
