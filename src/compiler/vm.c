#include <stdarg.h>

#include "debug.h"
#include "compiler/compiler.h"
#include "compiler/result.h"
#include "compiler/stack.h" /* This header will eventually be implemented in it's own stack.c file */
#include "compiler/vm.h"
#include "compiler/values.h"

static void reset_stack();
static result run();
static uint8_t read_byte();
static value read_constant();
static value peek(int offset);
static void print_stack_trace();
static void runtime_error(string format, ...);
static bool is_falsey(value val);
static bool values_equal(value a, value b);

/* Static instance of VM. Rewrite this to take a pointer and pass that around
 * instead. */
vm _dvm;

void init_vm() { reset_stack(); }

static void reset_stack() { _dvm.sp = _dvm.stack; }

void push(value value) {
    *_dvm.sp = value;
    _dvm.sp++;
}

value pop() {
    _dvm.sp--;
    return *_dvm.sp;
}

void free_vm() {}

result interpret(string source) {
    chunk chunk;
    init_chunk(&chunk);

    if (!compile(source, &chunk)) {
        free_chunk(&chunk);
        return COMPILER_ERROR;
    }

    _dvm.chunk = &chunk;
    _dvm.ip = _dvm.chunk->bytes;

    result result = run();

    free_chunk(&chunk);
    return result;
}

uint8_t read_byte() { return *_dvm.ip++; }

value read_constant() { return _dvm.chunk->constants.values[read_byte()]; }

value peek(int offset) { return _dvm.sp[-1 - offset]; }

bool is_falsey(value val) {
    return IS_NONE(val) || (IS_BOOL(val) && !AS_BOOL(val));
}

static bool values_equal(value a, value b) {
    if (a.type != b.type)
        return false;
    switch (a.type) {
    case VAL_BOOL:
        return AS_BOOL(a) == AS_BOOL(b);
    case VAL_NONE:
        return true;
    case VAL_NUMBER:
        return AS_NUMBER(a) == AS_NUMBER(b);
    default:
        return false;
    }
}

result run() {
#define BINARY_OP(val_type, op)                                                \
    do {                                                                       \
        if (!IS_NUMBER(peek(0)) || !IS_NUMBER(peek(1))) {                      \
            runtime_error("Operands must be numbers.");                        \
            return RUNTIME_ERROR;                                              \
        }                                                                      \
        double b = AS_NUMBER(pop());                                           \
        double a = AS_NUMBER(pop());                                           \
        push(val_type(a op b));                                                \
    } while (false)

    for (;;) {
#ifdef DEBUG_TRACE_EXECUTION
        print_stack_trace();
        disassemble_instruction(_dvm.chunk, (int)(_dvm.ip - _dvm.chunk->bytes));
#endif
        uint8_t instruction = read_byte();
        switch (instruction) {
        case OP_CONSTANT:
            push(read_constant());
            break;
        case OP_ADD:
            BINARY_OP(NUMBER_VAL, +);
            break;
        case OP_SUBTRACT:
            BINARY_OP(NUMBER_VAL, -);
            break;
        case OP_MULTIPLY:
            BINARY_OP(NUMBER_VAL, *);
            break;
        case OP_DIVIDE:
            BINARY_OP(NUMBER_VAL, /);
            break;
        case OP_NEGATE:
            if (!IS_NUMBER(peek(0))) {
                runtime_error("Operand must be a number");
                return RUNTIME_ERROR;
            }
            push(NUMBER_VAL(-AS_NUMBER(pop())));
            break;
        case OP_NONE:
            push(NONE_VAL);
            break;
        case OP_NOT:
            push(BOOL_VAL(is_falsey(pop())));
            break;
        case OP_TRUE:
            push(BOOL_VAL(true));
            break;
        case OP_FALSE:
            push(BOOL_VAL(false));
            break;
        case OP_EQUAL:
            value b = pop();
            value a = pop();
            push(BOOL_VAL(values_equal(a, b)));
            break;
        case OP_GREATER:
            BINARY_OP(BOOL_VAL, >);
            break;
        case OP_LESS:
            BINARY_OP(BOOL_VAL, <);
            break;
        case OP_RETURN:
            print_value(pop());
            printf("\n");
            return OK;
            break;
        default:
            return RUNTIME_ERROR;
            break;
        }
    }
#undef BINARY_OP
}

void print_stack_trace() {
    printf("          ");
    for (value* val = _dvm.stack; val < _dvm.sp; ++val) {
        printf("[ ");
        print_value(*val);
        printf(" ]");
    }
    printf("\n");
}

static void runtime_error(const char* format, ...) {
    va_list args;
    va_start(args, format);
    vfprintf(stderr, format, args);
    va_end(args);
    fputs("\n", stderr);

    size_t instruction = _dvm.ip - _dvm.chunk->bytes - 1;
    int line = _dvm.chunk->lines[instruction];
    fprintf(stderr, "[line %d] in script\n", line);
    reset_stack();
}